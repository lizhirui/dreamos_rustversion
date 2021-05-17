use core::u8;

use crate::memory::phypage;
use core::alloc::*;

use crate::*;
use crate::attribute::*;

struct allocator_t;

impl allocator_t
{
    pub const fn new() -> allocator_t
    {
        allocator_t{}
    }
}

unsafe impl GlobalAlloc for allocator_t
{
    unsafe fn alloc(&self,layout: Layout) -> *mut u8
    {
        assert!(phypage::is_initialized());
        let r = phypage::alloc(layout.size(),layout.size());
        println!("alloc {:#018x},{},{}",r as usize,layout.size(),layout.align());
        r
    }

    unsafe fn dealloc(&self,ptr: *mut u8,layout: Layout)
    {
        println!("free {:#018x},{},{}",ptr as usize,layout.size(),layout.align());
        phypage::free(ptr,layout.size(),layout.align());
    }
}

#[global_allocator]
static allocator: allocator_t = allocator_t::new();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> !
{
    terminal_color::set(terminal_color::fg_red,terminal_color::bg_current);
    println!("[Memory Not Enough]");
    terminal_color::set(terminal_color::fg_white,terminal_color::bg_current);
    println!("{:#?}!",layout);
    loop{}
}