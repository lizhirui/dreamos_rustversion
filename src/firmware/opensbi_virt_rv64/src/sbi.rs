#![allow(dead_code)]

use core::usize;

#[inline(always)]
fn syscall(id: usize,arg0: usize,arg1: usize,arg2: usize) -> usize
{
    let ret: usize;

    unsafe
    {
        asm!("ecall",
            in("a7") id,
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            lateout("a0") ret
        );
    }

    return ret;
}

#[inline(always)]
fn syscall0(id: usize) -> usize
{
    syscall(id,0,0,0)
}

#[inline(always)]
fn syscall1(id: usize,arg0: usize) -> usize
{
    syscall(id,arg0,0,0)
}

#[inline(always)]
fn syscall2(id: usize,arg0: usize,arg1: usize) -> usize
{
    syscall(id,arg0,arg1,0)
}

#[inline(always)]
fn syscall3(id: usize,arg0: usize,arg1: usize,arg2: usize) -> usize
{
    syscall(id,arg0,arg1,arg2)
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

pub fn console_putchar(ch: u8)
{
    syscall1(SBI_CONSOLE_PUTCHAR,ch as usize);
}

pub fn console_getchar() -> u8
{
    syscall0(SBI_CONSOLE_GETCHAR) as u8
}

pub fn set_timer(stime_value: usize)
{
    syscall1(SBI_SET_TIMER,stime_value);
}

pub fn shutdown()
{
    syscall0(SBI_SHUTDOWN);
}

pub fn send_ipi(hart_mask: &mut usize)
{
    syscall1(SBI_SEND_IPI,hart_mask as *mut usize as usize);
}

pub fn remote_fence_i(hart_mask: &mut usize)
{
    syscall1(SBI_REMOTE_FENCE_I,hart_mask as *mut usize as usize);
}

pub fn remote_sfence_vma(hart_mask: &mut usize,_start: usize,_size: usize)
{
    syscall1(SBI_REMOTE_SFENCE_VMA,hart_mask as *mut usize as usize);
}

pub fn remote_sfence_vma_asid(hart_mask: &mut usize,_start: usize,_size: usize,_asid: usize)
{
    syscall1(SBI_REMOTE_SFENCE_VMA_ASID,hart_mask as *mut usize as usize);
}
