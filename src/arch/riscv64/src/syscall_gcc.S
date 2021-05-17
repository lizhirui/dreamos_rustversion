    #include "encoding.h"
    #include "stackframe.h"
    .option norelax
    .section .text
    .align 2
    .global syscall_entry
    
syscall_entry:
    j syscall_entry