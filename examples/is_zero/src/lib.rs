#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(postgres_extension)]
#![feature(libc)]
#[macro_use] extern crate postgres_extension;


extern crate libc;

use libc::{c_int};

pg_module!(version: 90500);

#[pg_export]
pub fn is_zero(a: c_int) -> c_int {
    if a == 0 {
        return 0
    } else {
        return 0
    }
}
