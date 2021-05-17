use dreamos::*;
use arch::register::*;
use firmware::sbi;

static mut tick_cycles: usize = 0;

pub fn get_ticks() -> usize
{
    let r: usize;
    unsafe{asm!("rdtime {rd}",rd = out(reg)r)};
    r
}

pub fn init(config_table: &kernel::kernel_config_table_t)
{
    let interval: usize = 10000 / (*config_table).tick_per_second;
    let mut sie = sie_t::new();
    sie.set_stie(false);
    sie.write();

    unsafe
    {
        tick_cycles = 3686400 / 5000 * interval - 1;
        sbi::set_timer(get_ticks() + tick_cycles);
    }

    sie.set_stie(true);
    sie.write();
}

pub fn tick_isr()
{
    kernel::tick();

    unsafe 
    {
        sbi::set_timer(get_ticks() + tick_cycles);
    }
}