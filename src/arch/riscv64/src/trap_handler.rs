use core::{str, usize};
use dreamos::*;
use crate::register::*;

#[repr(C)]
pub struct TrapFrame
{
    pub sepc: usize,
    pub ra: usize,
    pub sstatus: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0_fp: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub user_sp_exc_stack: usize
}

pub enum exception_type
{
    instruction_address_misaligned = 0,
    instruction_access_fault = 1,
    illegal_instruction = 2,
    breakpoint = 3,
    load_address_misaligned = 4,
    load_access_fault = 5,
    store_amo_address_misaligned = 6,
    store_amo_access_fault = 7,
    environment_call_from_umode = 8,
    environment_call_from_smode = 9,
    reserved_10 = 10,
    reserved_11 = 11,
    instruction_page_fault = 12,
    load_page_fault = 13,
    reserved_14 = 14,
    store_amo_page_fault = 15
}

#[allow(dead_code)]
impl exception_type
{
    pub fn from(n: usize) -> Option<exception_type>
    {
        match n
        {
            0 => Some(exception_type::instruction_address_misaligned),
            1 => Some(exception_type::instruction_access_fault),
            2 => Some(exception_type::illegal_instruction),
            3 => Some(exception_type::breakpoint),
            4 => Some(exception_type::load_address_misaligned),
            5 => Some(exception_type::load_access_fault),
            6 => Some(exception_type::store_amo_address_misaligned),
            7 => Some(exception_type::store_amo_access_fault),
            8 => Some(exception_type::environment_call_from_umode),
            9 => Some(exception_type::environment_call_from_smode),
            10 => Some(exception_type::reserved_10),
            11 => Some(exception_type::reserved_11),
            12 => Some(exception_type::instruction_page_fault),
            13 => Some(exception_type::load_page_fault),
            14 => Some(exception_type::reserved_14),
            15 => Some(exception_type::store_amo_page_fault),
            _ => None
        }
    }

    pub fn to(self) -> usize
    {
        self as usize
    }
}

pub enum interrupt_type
{
    user_software_interrupt = 0,
    supervisor_software_interrupt = 1,
    reserved_2 = 2,
    reserved_3 = 3,
    user_timer_interrupt = 4,
    supervisor_timer_interrupt = 5,
    reserved_6 = 6,
    reserved_7 = 7,
    user_external_interrupt = 8,
    supervisor_external_interrupt = 9,
    reserved_10 = 10,
    reserved_11 = 11
}

#[allow(dead_code)]
impl interrupt_type
{
    pub fn from(n: usize) -> Option<interrupt_type>
    {
        match n
        {
            0 => Some(interrupt_type::user_software_interrupt),
            1 => Some(interrupt_type::supervisor_software_interrupt),
            2 => Some(interrupt_type::reserved_2),
            3 => Some(interrupt_type::reserved_3),
            4 => Some(interrupt_type::user_timer_interrupt),
            5 => Some(interrupt_type::supervisor_timer_interrupt),
            6 => Some(interrupt_type::reserved_6),
            7 => Some(interrupt_type::reserved_7),
            8 => Some(interrupt_type::user_external_interrupt),
            9 => Some(interrupt_type::supervisor_external_interrupt),
            10 => Some(interrupt_type::reserved_10),
            11 => Some(interrupt_type::reserved_11),
            _ => None
        }
    }

    pub fn to(self) -> usize
    {
        self as usize
    }
}

fn get_exception_name(t: exception_type) -> &'static str
{
    match t
    {
        exception_type::instruction_address_misaligned => "Instruction Address Misaligned",
        exception_type::instruction_access_fault => "Instruction Access Fault",
        exception_type::illegal_instruction => "Illegal Instruction",
        exception_type::breakpoint => "Breakpoint",
        exception_type::load_address_misaligned => "Load Address Misaligned",
        exception_type::load_access_fault => "Load Access Fault",
        exception_type::store_amo_address_misaligned => "Store/AMO Address Misaligned",
        exception_type::store_amo_access_fault => "Store/AMO Access Fault",
        exception_type::environment_call_from_umode => "Environment call from U-mode",
        exception_type::environment_call_from_smode => "Environment call from S-mode",
        exception_type::reserved_10 => "Reserved-10",
        exception_type::reserved_11 => "Reserved-11",
        exception_type::instruction_page_fault => "Instruction Page Fault",
        exception_type::load_page_fault => "Load Page Fault",
        exception_type::reserved_14 => "Reserved-14",
        exception_type::store_amo_page_fault => "Store/AMO Page Fault",
    }
}

fn get_interrupt_name(t: interrupt_type) -> &'static str
{
    match t
    {
        interrupt_type::user_software_interrupt => "User Software Interrupt",
        interrupt_type::supervisor_software_interrupt => "Supervisor Software Interrupt",
        interrupt_type::reserved_2 => "Reserved-2",
        interrupt_type::reserved_3 => "Reserved-3",
        interrupt_type::user_timer_interrupt => "User Timer Interrupt",
        interrupt_type::supervisor_timer_interrupt => "Supervisor Timer Interrupt",
        interrupt_type::reserved_6 => "Reserved-6",
        interrupt_type::reserved_7 => "Reserved-7",
        interrupt_type::user_external_interrupt => "User External Interrupt",
        interrupt_type::supervisor_external_interrupt => "Supervisor External Interrupt",
        interrupt_type::reserved_10 => "Reserved-10",
        interrupt_type::reserved_11 => "Reserved-11",
    }
}

fn get_trap_name(scause: scause_t) -> &'static str
{
    if scause.is_interrupt()
    {
        let t = interrupt_type::from(scause.get_id());

        match t
        {
            Some(value) => get_interrupt_name(value),
            _ => "Unknown Interrupt"
        }
    }
    else
    {
        let t = exception_type::from(scause.get_id());

        match t
        {
            Some(value) => get_exception_name(value),
            _ => "Unknown Exception"
        }
    }
}

#[no_mangle]
fn trap_handler(scause: usize,stval: usize,sepc: usize,regs: &mut TrapFrame)
{
    let mut reg_scause = scause_t::new();
    reg_scause.set(scause);

    if reg_scause.is_interrupt()
    {
        let t = interrupt_type::from(reg_scause.get_id());

        match t
        {
            Some(value) => 
            {
                unsafe 
                {
                    if crate::bsp_helper::bsp_helper_func.interrupt(value)
                    {
                        return;
                    }
                }
            }

            _ => {}
        }
    }
    
    println!("Unhandled {} {}:{}",if reg_scause.is_interrupt() {"Interrupt"} else {"Exception"},reg_scause.get_id(),get_trap_name(reg_scause));
    println!("scause = {:#018x}\tstval = {:#018x}\tsepc = {:#018x}\n",scause,stval,sepc);
    println!("-----------------------------Dump Registers-----------------------------");
    println!("Function Registers:");
    println!("\tra(x1) = {:#018x}\tuser_sp = {:#018x}",regs.ra,regs.user_sp_exc_stack);
    println!("\tgp(x3) = {:#018x}\ttp(x4) = {:#018x}",regs.gp,regs.tp);
    println!("Temporary Registers:");
    println!("\tt0(x5) = {:#018x}\tt1(x6) = {:#018x}",regs.t0,regs.t1);
    println!("\tt2(x7) = {:#018x}",regs.t2);
    println!("\tt3(x28) = {:#018x}\tt4(x29) = {:#018x}",regs.t3,regs.t4);
    println!("\tt5(x30) = {:#018x}\tt6(x31) = {:#018x}",regs.t5,regs.t6);
    println!("Saved Registers:");
    println!("\ts0/fp(x8) = {:#018x}\ts1(x9) = {:#018x}",regs.s0_fp,regs.s1);
    println!("\ts2(x18) = {:#018x}\ts3(x19) = {:#018x}",regs.s2,regs.s3);
    println!("\ts4(x20) = {:#018x}\ts5(x21) = {:#018x}",regs.s4,regs.s5);
    println!("\ts6(x22) = {:#018x}\ts7(x23) = {:#018x}",regs.s6,regs.s7);
    println!("\ts8(x24) = {:#018x}\ts9(x25) = {:#018x}",regs.s8,regs.s9);
    println!("\ts10(x26) = {:#018x}\ts11(x27) = {:#018x}",regs.s10,regs.s11);
    println!("Function Arguments Registers:");
    println!("\ta0(x10) = {:#018x}\ta1(x11) = {:#018x}",regs.a0,regs.a1);
    println!("\ta2(x12) = {:#018x}\ta3(x13) = {:#018x}",regs.a2,regs.a3);
    println!("\ta4(x14) = {:#018x}\ta5(x15) = {:#018x}",regs.a4,regs.a5);
    println!("\ta6(x16) = {:#018x}\ta7(x17) = {:#018x}",regs.a6,regs.a7);
    println!("sstatus = {:#018x}",regs.sstatus);
    /*println!("\t%s\n",(regs.sstatus & SSTATUS_SIE) ? "Supervisor Interrupt Enabled" : "Supervisor Interrupt Disabled");
    println!("\t%s\n",(regs.sstatus & SSTATUS_SPIE) ? "Last Time Supervisor Interrupt Enabled" : "Last Time Supervisor Interrupt Disabled");
    println!("\t%s\n",(regs.sstatus & SSTATUS_SPP) ? "Last Privilege is Supervisor Mode" : "Last Privilege is User Mode");
    println!("\t%s\n",(regs.sstatus & SSTATUS_PUM) ? "Permit to Access User Page" : "Not Permit to Access User Page");
    println!("\t%s\n",(regs.sstatus & (1 << 19)) ? "Permit to Read Executable-only Page" : "Not Permit to Read Executable-only Page");
    rt_size_t satp_v = read_csr(satp);
    println!("satp = {:#018x}\n",satp_v);
    println!("\tCurrent Page Table(Physical) = {:#018x}\n",__MASKVALUE(satp_v,__MASK(44)) << PAGE_OFFSET_BIT);
    println!("\tCurrent ASID = {:#018x}\n",__MASKVALUE(satp_v >> 44,__MASK(16)) << PAGE_OFFSET_BIT);
    const char *mode_str = "Unknown Address Translation/Protection Mode";
    
    switch(__MASKVALUE(satp_v >> 60,__MASK(4)))
    {
        case 0:
            mode_str = "No Address Translation/Protection Mode";
            break;

        case 8:
            mode_str = "Page-based 39-bit Virtual Addressing Mode";
            break;

        case 9:
            mode_str = "Page-based 48-bit Virtual Addressing Mode";
            break;
    }

    println!("\tMode = %s\n",mode_str);*/
    println!("---------------------------------Dump OK--------------------------------");
    loop{};
}