use core::{mem::size_of, ptr::null_mut, u8, usize};
use crate::kernel::config_table;
use crate::common;

#[repr(C)]
#[derive(Copy,Clone)]
struct page_metainfo
{
    order: usize,
    prev: *mut page_metainfo,
    next: *mut page_metainfo
}

extern "C" 
{
    static _heap_start: usize;
}

const buddy_order_max: usize = core::mem::size_of::<usize>() << 3;
const buddy_order_uplimit: usize = buddy_order_max + 1;
static mut page_list: [page_metainfo; buddy_order_uplimit] = [page_metainfo{order: 0,prev: null_mut(),next: null_mut()}; buddy_order_uplimit];
static mut page_metainfo_start: usize = 0;
static mut page_metainfo_end: usize = 0;
static mut page_memory_start: usize = 0;
static mut page_memory_end: usize = 0;
static mut page_metainfo_bits_aligned: usize = 0;
static mut page_allocated: usize = 0;

static mut initialized: bool = false;

pub fn is_initialized() -> bool
{
    unsafe
    {
        initialized
    }
}

pub fn init()
{
    unsafe 
    {
        let heap_start: usize = get_address!(_heap_start);
        let mem_start: usize = align_up!(heap_start,config_table.page_size);
        let mem_end: usize = align_down!(config_table.memory_base + config_table.memory_size,config_table.page_size);
        let mem_size: usize = mem_end - mem_start;
        println!("memory layout:\nmem_start = {:#018x}\nmem_end = {:#018x}\nmem_size = {:#018x}",mem_start,mem_end,mem_size);

        for i in 0..buddy_order_uplimit
        {
            page_list[i].order = i;
            page_list[i].prev = null_mut();
            page_list[i].next = null_mut();
        }

        page_metainfo_bits_aligned = align_up_min!(size_of::<page_metainfo>());
        let meta_size = size!(page_metainfo_bits_aligned);
        let page_size = config_table.page_size;
        let page_num = mem_size / (meta_size + page_size);
        page_metainfo_start = mem_start;
        page_metainfo_end = page_metainfo_start + (page_num << page_metainfo_bits_aligned);
        page_memory_start = align_up!(page_metainfo_end,config_table.page_size);
        page_memory_end = page_memory_start + (page_num << config_table.page_bits);
        println!("phypage layout:\nmeta_size = {}\npage_size = {}\npage_num = {}",meta_size,page_size,page_num);
        println!("page_metainfo_start = {:#018x}\npage_metainfo_end = {:#018x}\npage_memory_start = {:#018x}\npage_memory_end = {:#018x}",page_metainfo_start,page_metainfo_end,page_memory_start,page_memory_end);

        for i in 0..page_num
        {
            let page = (page_metainfo_start + (i << page_metainfo_bits_aligned)) as *mut page_metainfo;
            (*page).order = buddy_order_uplimit;
            (*page).prev = null_mut();
            (*page).next = null_mut();
        }

        page_allocated = page_num;

        let mut cur_page_addr = page_memory_start;

        while cur_page_addr < page_memory_end
        {
            let mut size_bits = align_down_max!(cur_page_addr);
            let align_bits = cur_page_addr.trailing_zeros() as usize;

            if align_bits < size_bits
            {
                size_bits = align_bits;
            }

            if size_bits > buddy_order_max
            {
                size_bits = buddy_order_max;
            }

            assert!(size_bits >= config_table.page_bits);

            //println!("page: {:#018x},size_bits: {}",cur_page_addr,size_bits);

            _free(cur_page_addr,size_bits);
            cur_page_addr += 1 << size_bits;
        }
        
        assert!(page_allocated == 0);
        initialized = true;
        common::sync_data();
        test();
        test();
    }
}

unsafe fn test()
{
    let mem1 = alloc(131072,4096);
    println!("mem1 = {:#018x}",mem1 as usize);
    let mem2 = alloc(4096,4096);
    println!("mem2 = {:#018x}",mem2 as usize);
    let mem3 = alloc(4096,131072);
    println!("mem3 = {:#018x}",mem3 as usize);
    free(mem1,131072,4096);
    let mem4 = alloc(4096,4096);
    println!("mem4 = {:#018x}",mem4 as usize);
    free(mem2,4096,4096);
    free(mem3,4096,131072);
    free(mem4,4096,4096);
    assert!(page_allocated == 0);
}

unsafe fn addr_to_page_metainfo(addr: usize) -> *mut page_metainfo
{
    if addr < page_memory_start
    {
        null_mut()
    }
    else
    {
        let r = (((addr - page_memory_start) >> config_table.page_bits) << page_metainfo_bits_aligned) + page_metainfo_start;

        if r >= page_metainfo_end
        {
            null_mut()
        }
        else
        {
            r as *mut page_metainfo
        }
    }
}

unsafe fn page_metainfo_to_addr(page_metainfo: *mut page_metainfo) -> usize
{
    let r = ((((page_metainfo as usize) - page_metainfo_start) >> page_metainfo_bits_aligned) << config_table.page_bits) + page_memory_start;

    if r >= page_memory_end
    {
        0
    }
    else
    {
        r
    }
}

unsafe fn size_to_order(size: usize) -> usize
{
    let pos = align_up_min!(size);

    if pos < config_table.page_bits
    {
        config_table.page_bits
    }
    else
    {
        pos
    }
}

unsafe fn buddy_get(page: *mut page_metainfo,order: usize) -> *mut page_metainfo
{
    addr_to_page_metainfo(page_metainfo_to_addr(page) ^ size!(order))
}

unsafe fn page_insert(order: usize,page: *mut page_metainfo)
{
    (*page).prev = get_address!(page_list[order]) as *mut page_metainfo;
    (*page).next = page_list[order].next;
    page_list[order].next = page;
    (*page).order = order;

    if !(*page).next.is_null()
    {
        let next_page = (*page).next;

        (*next_page).prev = page;
    }
}

unsafe fn page_remove(page: *mut page_metainfo)
{
    (*(*page).prev).next = (*page).next;

    if !(*page).next.is_null()
    {
        (*(*page).next).prev = (*page).prev;
    }

    (*page).order = buddy_order_uplimit;
}

#[inline]
fn get_big_page(page: *mut page_metainfo,buddy: *mut page_metainfo) -> *mut page_metainfo
{
    if page as usize <= buddy as usize
    {
        page
    }
    else
    {
        buddy
    }
}

fn _alloc(order: usize, align: usize) -> *mut u8
{
    unsafe 
    {
        assert!(align.is_power_of_two());

        for mut i in order..buddy_order_uplimit
        {
            if !page_list[i].next.is_null()
            {
                let mut page = page_list[i].next;
                let addr = page_metainfo_to_addr(page);
                
                while !page.is_null()
                {
                    if align_down!(addr,align) == addr
                    {
                        page_remove(page);

                        while i > order
                        {
                            i -= 1;
                            let right_new_addr = addr + size!(i);
                            let right_new_page = addr_to_page_metainfo(right_new_addr);
                            page_insert(i,right_new_page);

                        }
                        
                        page_allocated += size!(order - config_table.page_bits);
                        common::sync_data();
                        return page_metainfo_to_addr(page) as *mut u8;
                    }

                    page = (*page).next;
                }
            }
        }
        
        common::sync_data();
        return null_mut();
    }
}

pub fn alloc(size: usize, align: usize) -> *mut u8
{
    unsafe 
    {
        _alloc(size_to_order(size),align)
    }
}

fn _free(addr: usize,old_order: usize)
{
    unsafe 
    {
        let mut page = addr_to_page_metainfo(addr);
        
        for i in old_order..buddy_order_uplimit
        {
            let buddy = buddy_get(page,i);

            if !buddy.is_null() && ((*buddy).order == i) && (i < buddy_order_max)
            {
                page_remove(buddy);
                page = get_big_page(page,buddy);
            }
            else
            {
                page_allocated -= size!(old_order - config_table.page_bits);
                page_insert(i,page);
                break;
            }
        }

        common::sync_data();
    }
}

pub fn free(addr: *mut u8,old_size: usize,_align: usize)
{
    unsafe 
    {
        _free(addr as usize,size_to_order(old_size))
    }
}

pub fn get_allocated_page_count() -> usize
{
    unsafe 
    {
        page_allocated
    }
}

pub fn get_total_page_count() -> usize
{
    unsafe 
    {
        (page_memory_end - page_memory_start) >> config_table.page_bits
    }
}

pub fn get_free_page_count() -> usize
{
    get_total_page_count() - get_allocated_page_count()
}