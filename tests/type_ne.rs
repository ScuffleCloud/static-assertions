#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate static_assertions_next;

assert_type_ne_all!(u8, u16, u32);
