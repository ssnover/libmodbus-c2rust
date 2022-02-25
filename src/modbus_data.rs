use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/*
 * Copyright © 2010-2014 Stéphane Raimbault <stephane.raimbault@gmail.com>
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */
/* Sets many bits from a single byte value (all 8 bits of the byte value are
   set) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_bits_from_byte(mut dest: *mut uint8_t,
                                                   mut idx: libc::c_int,
                                                   value: uint8_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *dest.offset((idx + i) as isize) =
            if value as libc::c_int & (1 as libc::c_int) << i != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as uint8_t;
        i += 1
    };
}
/* Sets many bits from a table of bytes (only the bits between idx and
   idx + nb_bits are set) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_bits_from_bytes(mut dest: *mut uint8_t,
                                                    mut idx: libc::c_int,
                                                    mut nb_bits: libc::c_uint,
                                                    mut tab_byte:
                                                        *const uint8_t) {
    let mut i: libc::c_uint = 0;
    let mut shift: libc::c_int = 0 as libc::c_int;
    i = idx as libc::c_uint;
    while i < (idx as libc::c_uint).wrapping_add(nb_bits) {
        *dest.offset(i as isize) =
            if *tab_byte.offset(i.wrapping_sub(idx as
                                                   libc::c_uint).wrapping_div(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                    as isize) as libc::c_int &
                   (1 as libc::c_int) << shift != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as uint8_t;
        /* gcc doesn't like: shift = (++shift) % 8; */
        shift += 1;
        shift %= 8 as libc::c_int;
        i = i.wrapping_add(1)
    };
}
/* Gets the byte value from many bits.
   To obtain a full byte, set nb_bits to 8. */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_byte_from_bits(mut src: *const uint8_t,
                                                   mut idx: libc::c_int,
                                                   mut nb_bits: libc::c_uint)
 -> uint8_t {
    let mut i: libc::c_uint = 0;
    let mut value: uint8_t = 0 as libc::c_int as uint8_t;
    if nb_bits > 8 as libc::c_int as libc::c_uint {
        /* Assert is ignored if NDEBUG is set */
        if nb_bits < 8 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"nb_bits < 8\x00" as *const u8 as
                              *const libc::c_char,
                          b"/home/ssnover/dev/libmodbus/src/modbus-data.c\x00"
                              as *const u8 as *const libc::c_char,
                          110 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"uint8_t modbus_get_byte_from_bits(const uint8_t *, int, unsigned int)\x00")).as_ptr());
        }
        nb_bits = 8 as libc::c_int as libc::c_uint
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < nb_bits {
        value =
            (value as libc::c_int |
                 (*src.offset((idx as libc::c_uint).wrapping_add(i) as isize)
                      as libc::c_int) << i) as uint8_t;
        i = i.wrapping_add(1)
    }
    return value;
}
/* Get a float from 4 bytes (Modbus) without any conversion (ABCD) */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_float_abcd(mut src: *const uint16_t)
 -> libc::c_float {
    let mut f: libc::c_float = 0.;
    let mut i: uint32_t = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    a =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    b =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    c =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    d =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    i =
        ((a as libc::c_int) << 24 as libc::c_int |
             (b as libc::c_int) << 16 as libc::c_int |
             (c as libc::c_int) << 8 as libc::c_int |
             (d as libc::c_int) << 0 as libc::c_int) as uint32_t;
    memcpy(&mut f as *mut libc::c_float as *mut libc::c_void,
           &mut i as *mut uint32_t as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    return f;
}
/* Get a float from 4 bytes (Modbus) in inversed format (DCBA) */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_float_dcba(mut src: *const uint16_t)
 -> libc::c_float {
    let mut f: libc::c_float = 0.;
    let mut i: uint32_t = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    a =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    b =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    c =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    d =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    i =
        ((d as libc::c_int) << 24 as libc::c_int |
             (c as libc::c_int) << 16 as libc::c_int |
             (b as libc::c_int) << 8 as libc::c_int |
             (a as libc::c_int) << 0 as libc::c_int) as uint32_t;
    memcpy(&mut f as *mut libc::c_float as *mut libc::c_void,
           &mut i as *mut uint32_t as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    return f;
}
/* Get a float from 4 bytes (Modbus) with swapped bytes (BADC) */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_float_badc(mut src: *const uint16_t)
 -> libc::c_float {
    let mut f: libc::c_float = 0.;
    let mut i: uint32_t = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    a =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    b =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    c =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    d =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    i =
        ((b as libc::c_int) << 24 as libc::c_int |
             (a as libc::c_int) << 16 as libc::c_int |
             (d as libc::c_int) << 8 as libc::c_int |
             (c as libc::c_int) << 0 as libc::c_int) as uint32_t;
    memcpy(&mut f as *mut libc::c_float as *mut libc::c_void,
           &mut i as *mut uint32_t as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    return f;
}
/* Get a float from 4 bytes (Modbus) with swapped words (CDAB) */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_float_cdab(mut src: *const uint16_t)
 -> libc::c_float {
    let mut f: libc::c_float = 0.;
    let mut i: uint32_t = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    a =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    b =
        (*src.offset(0 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    c =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    d =
        (*src.offset(1 as libc::c_int as isize) as libc::c_int >>
             0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    i =
        ((c as libc::c_int) << 24 as libc::c_int |
             (d as libc::c_int) << 16 as libc::c_int |
             (a as libc::c_int) << 8 as libc::c_int |
             (b as libc::c_int) << 0 as libc::c_int) as uint32_t;
    memcpy(&mut f as *mut libc::c_float as *mut libc::c_void,
           &mut i as *mut uint32_t as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    return f;
}
/* DEPRECATED - Get a float from 4 bytes in sort of Modbus format */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_float(mut src: *const uint16_t)
 -> libc::c_float {
    let mut f: libc::c_float = 0.;
    let mut i: uint32_t = 0;
    i =
        ((*src.offset(1 as libc::c_int as isize) as uint32_t) <<
             16 as
                 libc::c_int).wrapping_add(*src.offset(0 as libc::c_int as
                                                           isize) as
                                               libc::c_uint);
    memcpy(&mut f as *mut libc::c_float as *mut libc::c_void,
           &mut i as *mut uint32_t as *const libc::c_void,
           ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
    return f;
}
/* Set a float to 4 bytes for Modbus w/o any conversion (ABCD) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_float_abcd(mut f: libc::c_float,
                                               mut dest: *mut uint16_t) {
    let mut i: uint32_t = 0;
    let mut out: *mut uint8_t = dest as *mut uint8_t;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    memcpy(&mut i as *mut uint32_t as *mut libc::c_void,
           &mut f as *mut libc::c_float as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    a =
        (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    b =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    c =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    d =
        (i >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *out.offset(0 as libc::c_int as isize) = a;
    *out.offset(1 as libc::c_int as isize) = b;
    *out.offset(2 as libc::c_int as isize) = c;
    *out.offset(3 as libc::c_int as isize) = d;
}
/* Set a float to 4 bytes for Modbus with byte and word swap conversion (DCBA) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_float_dcba(mut f: libc::c_float,
                                               mut dest: *mut uint16_t) {
    let mut i: uint32_t = 0;
    let mut out: *mut uint8_t = dest as *mut uint8_t;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    memcpy(&mut i as *mut uint32_t as *mut libc::c_void,
           &mut f as *mut libc::c_float as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    a =
        (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    b =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    c =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    d =
        (i >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *out.offset(0 as libc::c_int as isize) = d;
    *out.offset(1 as libc::c_int as isize) = c;
    *out.offset(2 as libc::c_int as isize) = b;
    *out.offset(3 as libc::c_int as isize) = a;
}
/* Set a float to 4 bytes for Modbus with byte swap conversion (BADC) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_float_badc(mut f: libc::c_float,
                                               mut dest: *mut uint16_t) {
    let mut i: uint32_t = 0;
    let mut out: *mut uint8_t = dest as *mut uint8_t;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    memcpy(&mut i as *mut uint32_t as *mut libc::c_void,
           &mut f as *mut libc::c_float as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    a =
        (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    b =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    c =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    d =
        (i >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *out.offset(0 as libc::c_int as isize) = b;
    *out.offset(1 as libc::c_int as isize) = a;
    *out.offset(2 as libc::c_int as isize) = d;
    *out.offset(3 as libc::c_int as isize) = c;
}
/* Set a float to 4 bytes for Modbus with word swap conversion (CDAB) */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_float_cdab(mut f: libc::c_float,
                                               mut dest: *mut uint16_t) {
    let mut i: uint32_t = 0;
    let mut out: *mut uint8_t = dest as *mut uint8_t;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut c: uint8_t = 0;
    let mut d: uint8_t = 0;
    memcpy(&mut i as *mut uint32_t as *mut libc::c_void,
           &mut f as *mut libc::c_float as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    a =
        (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    b =
        (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    c =
        (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    d =
        (i >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as
            uint8_t;
    *out.offset(0 as libc::c_int as isize) = c;
    *out.offset(1 as libc::c_int as isize) = d;
    *out.offset(2 as libc::c_int as isize) = a;
    *out.offset(3 as libc::c_int as isize) = b;
}
/* DEPRECATED - Set a float to 4 bytes in a sort of Modbus format! */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_float(mut f: libc::c_float,
                                          mut dest: *mut uint16_t) {
    let mut i: uint32_t = 0;
    memcpy(&mut i as *mut uint32_t as *mut libc::c_void,
           &mut f as *mut libc::c_float as *const libc::c_void,
           ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    *dest.offset(0 as libc::c_int as isize) = i as uint16_t;
    *dest.offset(1 as libc::c_int as isize) =
        (i >> 16 as libc::c_int) as uint16_t;
}
