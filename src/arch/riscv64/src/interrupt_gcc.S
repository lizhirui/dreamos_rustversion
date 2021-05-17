    #include "encoding.h"
    #include "stackframe.h"
    .option norelax
    .section .text
    .extern syscall_entry
    .global trap_entry
    .align 4
trap_entry:
    //backup user sp,switch to interrupt stack for current cpu
    csrrw sp, sscratch, sp
    
    //backup a little context only
    addi sp, sp, -3 * REGBYTES
    STORE t0, 0 * REGBYTES(sp)
    STORE t1, 1 * REGBYTES(sp)
    STORE gp, 2 * REGBYTES(sp)

    //restore gp
    RESTORE_SYS_GP
    
    //check syscall
    csrr t0, scause
    li t1, 8//environment call from u-mode
    beq t0, t1, syscall_entry

    //restore a little context only
    LOAD t0, 0 * REGBYTES(sp)
    LOAD t1, 1 * REGBYTES(sp)
    LOAD gp, 2 * REGBYTES(sp)
    addi sp, sp, 3 * REGBYTES

    //backup context
    SAVE_ALL_AND_RESTORE_SSCRATCH

    //restore gp
    RESTORE_SYS_GP

    //prepare arguments for trap handler
    csrr a0, scause
    csrrc a1, stval, zero
    csrr a2, sepc
    mv a3, sp

    call trap_handler

    //restore context
    RESTORE_ALL
    sret
