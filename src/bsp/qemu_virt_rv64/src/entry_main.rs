use dreamos::*;
use crate::init;

#[no_mangle]
pub extern "C" fn main() -> ! 
{
    let config_table = kernel::kernel_config_table_t
    {
        page_bits: 12,
        page_size: 4096,
        memory_base: 0x80000000,
        memory_size: 128 * 0x100000,
        tick_per_second: 100
    };
    
    init::init(&config_table);
    kernel::start(&config_table);
    loop {}
}
