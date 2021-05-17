use core::panic::PanicInfo;
use crate::*;
use crate::attribute::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    terminal_color::set(terminal_color::fg_red,terminal_color::bg_current);
    println!("[System Panic]");
    terminal_color::set(terminal_color::fg_white,terminal_color::bg_current);
    println!("{:#?}!",info);
    loop {}
}