pub trait bsp_helper_func_t
{
    fn early_init(&self);
    fn after_heap_init(&self);
    fn after_task_scheduler_init(&self);
    fn main_thread_body(&self);
    fn puts(&self,str: &str);
    fn interrupt_enable(&self,enabled: bool);
    fn interrupt_disable(&self) -> bool;
}

struct default_bsp_helper_func;

#[allow(unused_variables)]
impl bsp_helper_func_t for default_bsp_helper_func
{
    fn early_init(&self){}
    fn after_heap_init(&self){}
    fn after_task_scheduler_init(&self){}
    fn main_thread_body(&self){}
    fn puts(&self,str: &str){}
    fn interrupt_enable(&self,enabled: bool){}
    fn interrupt_disable(&self) -> bool{false}
}

pub static mut bsp_helper_func: &dyn bsp_helper_func_t = &default_bsp_helper_func;