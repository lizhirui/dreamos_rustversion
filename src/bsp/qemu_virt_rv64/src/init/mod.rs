mod arch_bsp_helper;
mod kernel_bsp_helper;
use crate::tick;
use dreamos::kernel;

pub fn init(config_table: &kernel::kernel_config_table_t)
{
    arch_bsp_helper::init();
    kernel_bsp_helper::init();
    tick::init(config_table);
}