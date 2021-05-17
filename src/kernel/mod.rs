use crate::bsp_helper::*;
use crate::memory;
use core::mem::size_of;

#[allow(dead_code)]
#[derive(Copy,Clone)]
#[repr(C)]
pub struct kernel_config_table_t
{
    pub page_bits: usize,
    pub page_size: usize,
    pub memory_base: usize,
    pub memory_size: usize,
    pub tick_per_second: usize
}

impl kernel_config_table_t
{
    pub const fn new() -> kernel_config_table_t
    {
        kernel_config_table_t
        {
            page_bits: 12,
            page_size: 4096,
            memory_base: 0x80000000,
            memory_size: 0x400000,
            tick_per_second: 100
        }
    }
}

pub static mut config_table: kernel_config_table_t = kernel_config_table_t::new();

pub fn start(_config_table: &kernel_config_table_t)
{
    unsafe{bsp_helper_func.early_init();}
    print_system_info();

    unsafe 
    {
        println!("kernel_config_table size = {}",size_of::<kernel_config_table_t>() as usize);
        config_table = *_config_table;
        assert!(size!(config_table.page_bits) == config_table.page_size);
    }
    
    memory::init();
    unsafe{bsp_helper_func.after_heap_init();}
    unsafe{bsp_helper_func.after_task_scheduler_init();}
    unsafe{bsp_helper_func.interrupt_enable(true);}

    loop
    {
        println!("abc");
    }

    assert!(false);
}

fn print_logo()
{
    println!();
    println!("************************************************");
    println!("   ____                            ___  ____");
    println!("  |  _ \\ _ __ ___  __ _ _ __ ___  / _ \\/ ___|");
    println!("  | | | | '__/ _ \\/ _` | '_ ` _ \\| | | \\___ \\");
    println!("  | |_| | | |  __/ (_| | | | | | | |_| |___) |");
    println!("  |____/|_|  \\___|\\__,_|_| |_| |_|\\___/|____/");
    println!();
    println!("************************************************");
}

fn print_system_info()
{
    print_logo();
    println!();
    println!("DreamOS 1.0");
    println!("Author: lizhirui <exiis@126.com>");
    println!();
    println!("************************************************");
    println!();
}

pub fn tick()
{
    println!("tick");
}