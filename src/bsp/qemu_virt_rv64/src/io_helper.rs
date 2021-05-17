use firmware::sbi;

pub fn putchar(ch: char)
{
    sbi::console_putchar(ch as u8);
}

pub fn puts(s: &str)
{
    for ch in s.chars()
    {
        putchar(ch);
    }
}