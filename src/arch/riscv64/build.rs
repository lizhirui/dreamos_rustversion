use cc;

fn main()
{
    cc::Build::new()
            .file("src/entry_gcc.S")
            .file("src/interrupt_gcc.S")
            .file("src/syscall_gcc.S")
            .file("src/function.S")
            .compile("libarch.a");
}