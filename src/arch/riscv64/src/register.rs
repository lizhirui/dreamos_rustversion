#![allow(dead_code)]

use core::usize;

#[macro_use]
use dreamos::*;

#[macro_export]
macro_rules! read_csr
{
    ($csr: ident) =>
    {
        {
            let r: usize;
            unsafe{asm!(concat!("csrr {rd},",stringify!($csr)),rd = out(reg)r)}
            r
        }
    }
}

#[macro_export]
macro_rules! write_csr
{
    ($csr: ident,$value: expr) =>
    {
        unsafe{asm!(concat!("csrw ",stringify!($csr),",{rd}"),rd = in(reg)$value);}
    }
}
pub trait register_port<V,T>
{
    fn get(&self) -> V;
    fn set(&mut self,val: V);
    fn read(&mut self);
    fn write(&self);
}

pub struct scause_t
{
    value: usize
}

impl scause_t
{
    pub const fn new() -> scause_t
    {
        scause_t{value: 0}
    }

    pub fn is_interrupt(&self) -> bool
    {
        (self.value >> 63) != 0
    }

    pub fn get_id(&self) -> usize
    {
        self.value & ((1 << 63) - 1)
    }
}

impl register_port<usize,scause_t> for scause_t
{
    fn get(&self) -> usize
    {
        self.value
    }

    fn set(&mut self,val: usize)
    {
        self.value = val;
    }

    fn read(&mut self)
    {
        self.value = read_csr!(scause);
    }

    fn write(&self)
    {
        write_csr!(scause,self.value);
    }
}

pub struct sie_t
{
    value: usize
}

impl sie_t
{
    pub const fn new() -> sie_t
    {
        sie_t{value: 0}
    }

    pub fn get_seie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,9))
    }

    pub fn get_ueie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,8))
    }
    
    pub fn get_stie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,5))
    }

    pub fn get_utie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,4))
    }

    pub fn get_ssie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,1))
    }

    pub fn get_usie(&self) -> bool
    {
        usize_to_bool!(get_bit!(self.value,0))
    }
    
    pub fn set_seie(&mut self,enabled: bool)
    {
        set_bit!(self.value,9,bool_to_usize!(enabled))
    }

    pub fn set_ueie(&mut self,enabled: bool)
    {
        set_bit!(self.value,8,bool_to_usize!(enabled))
    }
    
    pub fn set_stie(&mut self,enabled: bool)
    {
        set_bit!(self.value,5,bool_to_usize!(enabled))
    }

    pub fn set_utie(&mut self,enabled: bool)
    {
        set_bit!(self.value,4,bool_to_usize!(enabled))
    }

    pub fn set_ssie(&mut self,enabled: bool)
    {
        set_bit!(self.value,1,bool_to_usize!(enabled))
    }

    pub fn set_usie(&mut self,enabled: bool)
    {
        set_bit!(self.value,0,bool_to_usize!(enabled))
    }
}

impl register_port<usize,sie_t> for sie_t
{
    fn get(&self) -> usize
    {
        self.value
    }

    fn set(&mut self,val: usize)
    {
        self.value = val;
    }

    fn read(&mut self)
    {
        self.value = read_csr!(sie);
    }

    fn write(&self)
    {
        write_csr!(sie,self.value);
    }
}