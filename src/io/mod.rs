use core::{fmt::{self,Write}};
use crate::bsp_helper::*;

struct Stdout;

impl fmt::Write for Stdout
{
    fn write_str(&mut self,s: &str) -> fmt::Result
    {
        unsafe 
        {
            bsp_helper_func.puts(s);
        }

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments)
{
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print 
{
    ($($arg:tt)*) => 
    ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println 
{
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}