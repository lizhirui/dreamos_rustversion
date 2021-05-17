use crate::trap_handler::*;

pub trait bsp_helper_func_t
{
    fn interrupt(&self,int_type: interrupt_type) -> bool;
}

struct default_bsp_helper_func;

#[allow(unused_variables)]
impl bsp_helper_func_t for default_bsp_helper_func
{
    fn interrupt(&self,int_type: interrupt_type) -> bool 
    {
        false
    }
}

pub static mut bsp_helper_func: &dyn bsp_helper_func_t = &default_bsp_helper_func;