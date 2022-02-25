#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]



extern crate libc;

#[path = "../src/modbus_data.rs"]
pub mod modbus_data;
#[path = "../src/modbus_rtu.rs"]
pub mod modbus_rtu;
#[path = "../src/modbus_tcp.rs"]
pub mod modbus_tcp;
#[path = "../src/modbus.rs"]
pub mod modbus;

