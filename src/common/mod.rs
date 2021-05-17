#![allow(dead_code)]
#![allow(unused_macros)]

const nullptr: usize = 0;

#[macro_export]
macro_rules! align_down
{
    ($value: expr,$align_bound: expr) =>
    {
        $value & (!($align_bound - 1))
    }
}

#[macro_export]
macro_rules! align_up
{
    ($value: expr,$align_bound: expr) =>
    {
        ($value + ($align_bound - 1)) & (!($align_bound - 1))
    }
}

#[macro_export]
macro_rules! align_down_max
{
    ($value: expr) =>
    {
        {
            (size_of::<usize>() << 3) - $value.leading_zeros() as usize - 1
        }
    }
}

#[macro_export]
macro_rules! align_up_min
{
    ($value: expr) =>
    {
        {
            let pos = (size_of::<usize>() << 3) - $value.leading_zeros() as usize - 1;

            if $value.is_power_of_two()
            {
                pos
            }
            else
            {
                pos + 1
            }
        }
    }
}

#[macro_export]
macro_rules! size
{
    ($bits: expr) =>
    {
        1 << $bits
    }
}

#[macro_export]
macro_rules! mask
{
    ($bits: expr) =>
    {
        (1 << $bits) - 1
    }
}

#[macro_export]
macro_rules! get_address
{
    ($variable: expr) => 
    {
        &$variable as *const _ as usize
    };
}

#[macro_export]
macro_rules! get_pointer
{
    ($variable: expr) =>
    {
        unsafe{&$variable as *const _ as *mut _}
    }
}

#[macro_export]
macro_rules! get_pointer_const
{
    ($variable: expr) =>
    {
        unsafe{&$variable as *const _}
    }
}

#[macro_export]
macro_rules! get_reference
{
    ($variable: expr) =>
    {
        unsafe{&*(&$variable as *const _ as *mut _)}
    }
}

#[macro_export]
macro_rules! pointer_to_reference
{
    ($variable: expr) =>
    {
        unsafe{&*($variable as *const_ *mut _)}
    }
}

#[macro_export]
macro_rules! get_reference_const
{
    ($variable: expr) =>
    {
        unsafe{&*(&$variable as *const _)}
    }
}

#[macro_export]
macro_rules! pointer_to_reference_const
{
    ($variable: expr) =>
    {
        unsafe{&*($variable as *const_ *mut _)}
    }
}

#[macro_export]
macro_rules! offsetof
{
    ($ty: ty,$field: ident) => 
    {
        {
            let t = core::ptr::null() as *const $ty;
            get_address!((*t).$field) - get_address!(*t)
        }
    };
}

#[macro_export]
macro_rules! get_bit
{
    ($value: expr,$bit: expr) => 
    {
        ($value >> $bit) & 0x01
    };
}

#[macro_export]
macro_rules! set_bit
{
    ($value: expr,$bit: expr,$enabled: expr) => 
    {
        $value = ($value & (!(1 << $bit))) | (($enabled & 0x01) << $bit);
    };
}

#[macro_export]
macro_rules! bool_to_usize
{
    ($value: expr) => 
    {
        if $value
        {
            1
        }
        else
        {
            0
        }
    };
}

#[macro_export]
macro_rules! usize_to_bool
{
    ($value: expr) => 
    {
        if $value != 0
        {
            true
        }
        else
        {
            false
        }
    };
}

extern "C"
{
    pub fn sync();
    pub fn sync_data();
    pub fn sync_instruction();
}

#[macro_use]
pub mod list;
