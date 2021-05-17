use arch::trap_handler::interrupt_type;
use crate::tick;

struct arch_bsp_helper_func_t;

impl arch::bsp_helper::bsp_helper_func_t for arch_bsp_helper_func_t
{
    fn interrupt(&self,int_type: interrupt_type) -> bool 
    {
        match int_type
        {
            interrupt_type::supervisor_timer_interrupt => 
            {
                tick::tick_isr();
                true
            }

            _ => {false}
        }
    }
}

pub fn init()
{
    unsafe 
    {
        arch::bsp_helper::bsp_helper_func = &arch_bsp_helper_func_t;
    }
}