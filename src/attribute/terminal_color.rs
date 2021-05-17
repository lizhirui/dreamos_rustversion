pub const fg_black: &str = "30m";
pub const fg_red: &str = "31m";
pub const fg_green: &str = "32m";
pub const fg_yellow: &str = "33m";
pub const fg_cyan: &str = "34m";
pub const fg_purple: &str = "35m";
pub const fg_blue: &str = "36m";
pub const fg_white: &str = "37m";

pub const bg_black: &str = "40;";
pub const bg_red: &str = "41;";
pub const bg_green: &str = "42;";
pub const bg_yellow: &str = "43;";
pub const bg_cyan: &str = "44;";
pub const bg_purple: &str = "45;";
pub const bg_blue: &str = "46;";
pub const bg_white: &str = "47;";

pub const fg_current: &str = "";
pub const bg_current: &str = "";

pub fn set(fg: &str,bg: &str)
{
    print!("{}[{}{}",0o33 as char,fg,bg);
}