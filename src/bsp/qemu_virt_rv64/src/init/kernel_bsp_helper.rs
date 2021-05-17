use dreamos::{*, common::list::list_node};
use dreamos::common::list;
use crate::io_helper;

struct test_entry
{
    id: usize,
    node: list_node
}

struct kernel_bsp_helper_func_t;

impl bsp_helper::bsp_helper_func_t for kernel_bsp_helper_func_t
{
    fn early_init(&self)
    {
        println!("early_init");
    }

    fn after_heap_init(&self)
    {

    }

    fn after_task_scheduler_init(&self)
    {
        let mut header = list_node::new();
        list_init!(header);
        let mut a = test_entry{id: 1,node: list_node::new()};
        let mut b = test_entry{id: 2,node: list_node::new()};
        let mut c = test_entry{id: 3,node: list_node::new()};
        //a.node.insert_after(&mut header);
        //b.node.insert_after(&mut a.node);
        //c.node.insert_after(&mut b.node);
        list_insert_tail!(header,a.node);
        list_insert_tail!(header,b.node);
        list_insert_tail!(header,c.node);
        //b.node.remove_from_list();

        list_entry_foreach!(header,test_entry,node,entry,
        {
            println!("list_node_test:{}",entry.id);
        });

        println!("list_head:{}",list_entry!(list_head!(header),test_entry,node).id);
        println!("list_tail:{}",list_entry!(list_tail!(header),test_entry,node).id);

        loop{};
    }

    fn main_thread_body(&self){}

    fn puts(&self,str: &str)
    {
        io_helper::puts(str)
    }

    fn interrupt_enable(&self,enabled: bool)
    {
        unsafe 
        {
            if(enabled)
            {
                asm!("csrrsi x0,sstatus,2");
            }
            else
            {
                asm!("csrrci x0,sstatus,2");
            }
        }
    }

    fn interrupt_disable(&self) -> bool
    {
        let r: usize;
        unsafe{asm!("csrrci {rd},sstatus,2",rd = out(reg)r)};
        usize_to_bool!((r >> 2) & 0x01)
    }
}

pub fn init()
{
    unsafe 
    {
        bsp_helper::bsp_helper_func = &kernel_bsp_helper_func_t;
    }
}