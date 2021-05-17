#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]
#![feature(const_fn_fn_ptr_basics)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod lang_items;
#[macro_use]
pub mod common;
#[macro_use]
pub mod io;
pub mod kernel;
pub mod bsp_helper;
pub mod attribute;
pub mod memory;
pub mod task;

extern crate alloc;