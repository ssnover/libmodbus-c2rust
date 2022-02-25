use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn modbus_free(ctx: *mut modbus_t);
    #[no_mangle]
    fn _modbus_receive_msg(ctx: *mut modbus_t, msg: *mut uint8_t,
                           msg_type: msg_type_t) -> libc::c_int;
    #[no_mangle]
    fn _modbus_init_common(ctx: *mut modbus_t);
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
    #[no_mangle]
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t)
     -> libc::c_int;
    #[no_mangle]
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t)
     -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type C2RustUnnamed = libc::c_uint;
pub const MODBUS_EXCEPTION_MAX: C2RustUnnamed = 12;
pub const MODBUS_EXCEPTION_GATEWAY_TARGET: C2RustUnnamed = 11;
pub const MODBUS_EXCEPTION_GATEWAY_PATH: C2RustUnnamed = 10;
pub const MODBUS_EXCEPTION_NOT_DEFINED: C2RustUnnamed = 9;
pub const MODBUS_EXCEPTION_MEMORY_PARITY: C2RustUnnamed = 8;
pub const MODBUS_EXCEPTION_NEGATIVE_ACKNOWLEDGE: C2RustUnnamed = 7;
pub const MODBUS_EXCEPTION_SLAVE_OR_SERVER_BUSY: C2RustUnnamed = 6;
pub const MODBUS_EXCEPTION_ACKNOWLEDGE: C2RustUnnamed = 5;
pub const MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE: C2RustUnnamed = 4;
pub const MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE: C2RustUnnamed = 3;
pub const MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS: C2RustUnnamed = 2;
pub const MODBUS_EXCEPTION_ILLEGAL_FUNCTION: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus {
    pub slave: libc::c_int,
    pub s: libc::c_int,
    pub debug: libc::c_int,
    pub error_recovery: libc::c_int,
    pub response_timeout: timeval,
    pub byte_timeout: timeval,
    pub indication_timeout: timeval,
    pub backend: *const modbus_backend_t,
    pub backend_data: *mut libc::c_void,
}
pub type modbus_backend_t = _modbus_backend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus_backend {
    pub backend_type: libc::c_uint,
    pub header_length: libc::c_uint,
    pub checksum_length: libc::c_uint,
    pub max_adu_length: libc::c_uint,
    pub set_slave: Option<unsafe extern "C" fn(_: *mut modbus_t,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub build_request_basis: Option<unsafe extern "C" fn(_: *mut modbus_t,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *mut uint8_t)
                                        -> libc::c_int>,
    pub build_response_basis: Option<unsafe extern "C" fn(_: *mut sft_t,
                                                          _: *mut uint8_t)
                                         -> libc::c_int>,
    pub prepare_response_tid: Option<unsafe extern "C" fn(_: *const uint8_t,
                                                          _: *mut libc::c_int)
                                         -> libc::c_int>,
    pub send_msg_pre: Option<unsafe extern "C" fn(_: *mut uint8_t,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub send: Option<unsafe extern "C" fn(_: *mut modbus_t, _: *const uint8_t,
                                          _: libc::c_int) -> ssize_t>,
    pub receive: Option<unsafe extern "C" fn(_: *mut modbus_t,
                                             _: *mut uint8_t) -> libc::c_int>,
    pub recv: Option<unsafe extern "C" fn(_: *mut modbus_t, _: *mut uint8_t,
                                          _: libc::c_int) -> ssize_t>,
    pub check_integrity: Option<unsafe extern "C" fn(_: *mut modbus_t,
                                                     _: *mut uint8_t,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub pre_check_confirmation: Option<unsafe extern "C" fn(_: *mut modbus_t,
                                                            _: *const uint8_t,
                                                            _: *const uint8_t,
                                                            _: libc::c_int)
                                           -> libc::c_int>,
    pub connect: Option<unsafe extern "C" fn(_: *mut modbus_t)
                            -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut modbus_t) -> ()>,
    pub flush: Option<unsafe extern "C" fn(_: *mut modbus_t) -> libc::c_int>,
    pub select: Option<unsafe extern "C" fn(_: *mut modbus_t, _: *mut fd_set,
                                            _: *mut timeval, _: libc::c_int)
                           -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut modbus_t) -> ()>,
}
pub type modbus_t = _modbus;
pub type sft_t = _sft;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sft {
    pub slave: libc::c_int,
    pub function: libc::c_int,
    pub t_id: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MODBUS_ERROR_RECOVERY_PROTOCOL: C2RustUnnamed_0 = 4;
pub const MODBUS_ERROR_RECOVERY_LINK: C2RustUnnamed_0 = 2;
pub const MODBUS_ERROR_RECOVERY_NONE: C2RustUnnamed_0 = 0;
pub type modbus_rtu_t = _modbus_rtu;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus_rtu {
    pub device: *mut libc::c_char,
    pub baud: libc::c_int,
    pub data_bit: uint8_t,
    pub stop_bit: uint8_t,
    pub parity: libc::c_char,
    pub old_tios: termios,
    pub serial_mode: libc::c_int,
    pub rts: libc::c_int,
    pub rts_delay: libc::c_int,
    pub onebyte_time: libc::c_int,
    pub set_rts: Option<unsafe extern "C" fn(_: *mut modbus_t, _: libc::c_int)
                            -> ()>,
    pub confirmation_to_ignore: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type msg_type_t = libc::c_uint;
pub const MSG_CONFIRMATION: msg_type_t = 1;
pub const MSG_INDICATION: msg_type_t = 0;
pub const _MODBUS_BACKEND_TYPE_RTU: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct serial_rs485 {
    pub flags: __u32,
    pub delay_rts_before_send: __u32,
    pub delay_rts_after_send: __u32,
    pub padding: [__u32; 5],
}
pub type __u32 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _MODBUS_BACKEND_TYPE_TCP: C2RustUnnamed_1 = 1;
/*
 * Copyright © 2001-2011 Stéphane Raimbault <stephane.raimbault@gmail.com>
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */
/* Table of CRC values for high-order byte */
static mut table_crc_hi: [uint8_t; 256] =
    [0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x41 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0xc1 as libc::c_int as uint8_t,
     0x81 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t];
/* Table of CRC values for low-order byte */
static mut table_crc_lo: [uint8_t; 256] =
    [0 as libc::c_int as uint8_t, 0xc0 as libc::c_int as uint8_t,
     0xc1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0xc3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0xc2 as libc::c_int as uint8_t,
     0xc6 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x7 as libc::c_int as uint8_t, 0xc7 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0xc5 as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0xcc as libc::c_int as uint8_t, 0xc as libc::c_int as uint8_t,
     0xd as libc::c_int as uint8_t, 0xcd as libc::c_int as uint8_t,
     0xf as libc::c_int as uint8_t, 0xcf as libc::c_int as uint8_t,
     0xce as libc::c_int as uint8_t, 0xe as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t, 0xca as libc::c_int as uint8_t,
     0xcb as libc::c_int as uint8_t, 0xb as libc::c_int as uint8_t,
     0xc9 as libc::c_int as uint8_t, 0x9 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0xc8 as libc::c_int as uint8_t,
     0xd8 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0x19 as libc::c_int as uint8_t, 0xd9 as libc::c_int as uint8_t,
     0x1b as libc::c_int as uint8_t, 0xdb as libc::c_int as uint8_t,
     0xda as libc::c_int as uint8_t, 0x1a as libc::c_int as uint8_t,
     0x1e as libc::c_int as uint8_t, 0xde as libc::c_int as uint8_t,
     0xdf as libc::c_int as uint8_t, 0x1f as libc::c_int as uint8_t,
     0xdd as libc::c_int as uint8_t, 0x1d as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x14 as libc::c_int as uint8_t, 0xd4 as libc::c_int as uint8_t,
     0xd5 as libc::c_int as uint8_t, 0x15 as libc::c_int as uint8_t,
     0xd7 as libc::c_int as uint8_t, 0x17 as libc::c_int as uint8_t,
     0x16 as libc::c_int as uint8_t, 0xd6 as libc::c_int as uint8_t,
     0xd2 as libc::c_int as uint8_t, 0x12 as libc::c_int as uint8_t,
     0x13 as libc::c_int as uint8_t, 0xd3 as libc::c_int as uint8_t,
     0x11 as libc::c_int as uint8_t, 0xd1 as libc::c_int as uint8_t,
     0xd0 as libc::c_int as uint8_t, 0x10 as libc::c_int as uint8_t,
     0xf0 as libc::c_int as uint8_t, 0x30 as libc::c_int as uint8_t,
     0x31 as libc::c_int as uint8_t, 0xf1 as libc::c_int as uint8_t,
     0x33 as libc::c_int as uint8_t, 0xf3 as libc::c_int as uint8_t,
     0xf2 as libc::c_int as uint8_t, 0x32 as libc::c_int as uint8_t,
     0x36 as libc::c_int as uint8_t, 0xf6 as libc::c_int as uint8_t,
     0xf7 as libc::c_int as uint8_t, 0x37 as libc::c_int as uint8_t,
     0xf5 as libc::c_int as uint8_t, 0x35 as libc::c_int as uint8_t,
     0x34 as libc::c_int as uint8_t, 0xf4 as libc::c_int as uint8_t,
     0x3c as libc::c_int as uint8_t, 0xfc as libc::c_int as uint8_t,
     0xfd as libc::c_int as uint8_t, 0x3d as libc::c_int as uint8_t,
     0xff as libc::c_int as uint8_t, 0x3f as libc::c_int as uint8_t,
     0x3e as libc::c_int as uint8_t, 0xfe as libc::c_int as uint8_t,
     0xfa as libc::c_int as uint8_t, 0x3a as libc::c_int as uint8_t,
     0x3b as libc::c_int as uint8_t, 0xfb as libc::c_int as uint8_t,
     0x39 as libc::c_int as uint8_t, 0xf9 as libc::c_int as uint8_t,
     0xf8 as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0x28 as libc::c_int as uint8_t, 0xe8 as libc::c_int as uint8_t,
     0xe9 as libc::c_int as uint8_t, 0x29 as libc::c_int as uint8_t,
     0xeb as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0x2a as libc::c_int as uint8_t, 0xea as libc::c_int as uint8_t,
     0xee as libc::c_int as uint8_t, 0x2e as libc::c_int as uint8_t,
     0x2f as libc::c_int as uint8_t, 0xef as libc::c_int as uint8_t,
     0x2d as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0x2c as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0x24 as libc::c_int as uint8_t,
     0x25 as libc::c_int as uint8_t, 0xe5 as libc::c_int as uint8_t,
     0x27 as libc::c_int as uint8_t, 0xe7 as libc::c_int as uint8_t,
     0xe6 as libc::c_int as uint8_t, 0x26 as libc::c_int as uint8_t,
     0x22 as libc::c_int as uint8_t, 0xe2 as libc::c_int as uint8_t,
     0xe3 as libc::c_int as uint8_t, 0x23 as libc::c_int as uint8_t,
     0xe1 as libc::c_int as uint8_t, 0x21 as libc::c_int as uint8_t,
     0x20 as libc::c_int as uint8_t, 0xe0 as libc::c_int as uint8_t,
     0xa0 as libc::c_int as uint8_t, 0x60 as libc::c_int as uint8_t,
     0x61 as libc::c_int as uint8_t, 0xa1 as libc::c_int as uint8_t,
     0x63 as libc::c_int as uint8_t, 0xa3 as libc::c_int as uint8_t,
     0xa2 as libc::c_int as uint8_t, 0x62 as libc::c_int as uint8_t,
     0x66 as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0xa7 as libc::c_int as uint8_t, 0x67 as libc::c_int as uint8_t,
     0xa5 as libc::c_int as uint8_t, 0x65 as libc::c_int as uint8_t,
     0x64 as libc::c_int as uint8_t, 0xa4 as libc::c_int as uint8_t,
     0x6c as libc::c_int as uint8_t, 0xac as libc::c_int as uint8_t,
     0xad as libc::c_int as uint8_t, 0x6d as libc::c_int as uint8_t,
     0xaf as libc::c_int as uint8_t, 0x6f as libc::c_int as uint8_t,
     0x6e as libc::c_int as uint8_t, 0xae as libc::c_int as uint8_t,
     0xaa as libc::c_int as uint8_t, 0x6a as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0xab as libc::c_int as uint8_t,
     0x69 as libc::c_int as uint8_t, 0xa9 as libc::c_int as uint8_t,
     0xa8 as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0x78 as libc::c_int as uint8_t, 0xb8 as libc::c_int as uint8_t,
     0xb9 as libc::c_int as uint8_t, 0x79 as libc::c_int as uint8_t,
     0xbb as libc::c_int as uint8_t, 0x7b as libc::c_int as uint8_t,
     0x7a as libc::c_int as uint8_t, 0xba as libc::c_int as uint8_t,
     0xbe as libc::c_int as uint8_t, 0x7e as libc::c_int as uint8_t,
     0x7f as libc::c_int as uint8_t, 0xbf as libc::c_int as uint8_t,
     0x7d as libc::c_int as uint8_t, 0xbd as libc::c_int as uint8_t,
     0xbc as libc::c_int as uint8_t, 0x7c as libc::c_int as uint8_t,
     0xb4 as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x75 as libc::c_int as uint8_t, 0xb5 as libc::c_int as uint8_t,
     0x77 as libc::c_int as uint8_t, 0xb7 as libc::c_int as uint8_t,
     0xb6 as libc::c_int as uint8_t, 0x76 as libc::c_int as uint8_t,
     0x72 as libc::c_int as uint8_t, 0xb2 as libc::c_int as uint8_t,
     0xb3 as libc::c_int as uint8_t, 0x73 as libc::c_int as uint8_t,
     0xb1 as libc::c_int as uint8_t, 0x71 as libc::c_int as uint8_t,
     0x70 as libc::c_int as uint8_t, 0xb0 as libc::c_int as uint8_t,
     0x50 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0x91 as libc::c_int as uint8_t, 0x51 as libc::c_int as uint8_t,
     0x93 as libc::c_int as uint8_t, 0x53 as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0x92 as libc::c_int as uint8_t,
     0x96 as libc::c_int as uint8_t, 0x56 as libc::c_int as uint8_t,
     0x57 as libc::c_int as uint8_t, 0x97 as libc::c_int as uint8_t,
     0x55 as libc::c_int as uint8_t, 0x95 as libc::c_int as uint8_t,
     0x94 as libc::c_int as uint8_t, 0x54 as libc::c_int as uint8_t,
     0x9c as libc::c_int as uint8_t, 0x5c as libc::c_int as uint8_t,
     0x5d as libc::c_int as uint8_t, 0x9d as libc::c_int as uint8_t,
     0x5f as libc::c_int as uint8_t, 0x9f as libc::c_int as uint8_t,
     0x9e as libc::c_int as uint8_t, 0x5e as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0x9a as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x5b as libc::c_int as uint8_t,
     0x99 as libc::c_int as uint8_t, 0x59 as libc::c_int as uint8_t,
     0x58 as libc::c_int as uint8_t, 0x98 as libc::c_int as uint8_t,
     0x88 as libc::c_int as uint8_t, 0x48 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x89 as libc::c_int as uint8_t,
     0x4b as libc::c_int as uint8_t, 0x8b as libc::c_int as uint8_t,
     0x8a as libc::c_int as uint8_t, 0x4a as libc::c_int as uint8_t,
     0x4e as libc::c_int as uint8_t, 0x8e as libc::c_int as uint8_t,
     0x8f as libc::c_int as uint8_t, 0x4f as libc::c_int as uint8_t,
     0x8d as libc::c_int as uint8_t, 0x4d as libc::c_int as uint8_t,
     0x4c as libc::c_int as uint8_t, 0x8c as libc::c_int as uint8_t,
     0x44 as libc::c_int as uint8_t, 0x84 as libc::c_int as uint8_t,
     0x85 as libc::c_int as uint8_t, 0x45 as libc::c_int as uint8_t,
     0x87 as libc::c_int as uint8_t, 0x47 as libc::c_int as uint8_t,
     0x46 as libc::c_int as uint8_t, 0x86 as libc::c_int as uint8_t,
     0x82 as libc::c_int as uint8_t, 0x42 as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0x83 as libc::c_int as uint8_t,
     0x41 as libc::c_int as uint8_t, 0x81 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0x40 as libc::c_int as uint8_t];
/* Define the slave ID of the remote device to talk in master mode or set the
 * internal slave ID in slave mode */
unsafe extern "C" fn _modbus_set_slave(mut ctx: *mut modbus_t,
                                       mut slave: libc::c_int)
 -> libc::c_int {
    /* Broadcast address is 0 (MODBUS_BROADCAST_ADDRESS) */
    if slave >= 0 as libc::c_int && slave <= 247 as libc::c_int {
        (*ctx).slave = slave
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Builds a RTU request header */
unsafe extern "C" fn _modbus_rtu_build_request_basis(mut ctx: *mut modbus_t,
                                                     mut function:
                                                         libc::c_int,
                                                     mut addr: libc::c_int,
                                                     mut nb: libc::c_int,
                                                     mut req: *mut uint8_t)
 -> libc::c_int {
    if (*ctx).slave != -(1 as libc::c_int) {
    } else {
        __assert_fail(b"ctx->slave != -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"/home/ssnover/dev/libmodbus/src/modbus-rtu.c\x00" as
                          *const u8 as *const libc::c_char,
                      110 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"int _modbus_rtu_build_request_basis(modbus_t *, int, int, int, uint8_t *)\x00")).as_ptr());
    }
    *req.offset(0 as libc::c_int as isize) = (*ctx).slave as uint8_t;
    *req.offset(1 as libc::c_int as isize) = function as uint8_t;
    *req.offset(2 as libc::c_int as isize) =
        (addr >> 8 as libc::c_int) as uint8_t;
    *req.offset(3 as libc::c_int as isize) =
        (addr & 0xff as libc::c_int) as uint8_t;
    *req.offset(4 as libc::c_int as isize) =
        (nb >> 8 as libc::c_int) as uint8_t;
    *req.offset(5 as libc::c_int as isize) =
        (nb & 0xff as libc::c_int) as uint8_t;
    return 6 as libc::c_int;
}
/* Builds a RTU response header */
unsafe extern "C" fn _modbus_rtu_build_response_basis(mut sft: *mut sft_t,
                                                      mut rsp: *mut uint8_t)
 -> libc::c_int {
    /* In this case, the slave is certainly valid because a check is already
     * done in _modbus_rtu_listen */
    *rsp.offset(0 as libc::c_int as isize) =
        (*sft).slave as uint8_t; /* high CRC byte initialized */
    *rsp.offset(1 as libc::c_int as isize) =
        (*sft).function as uint8_t; /* low CRC byte initialized */
    return 2 as libc::c_int; /* will index into CRC lookup */
}
unsafe extern "C" fn crc16(mut buffer: *mut uint8_t,
                           mut buffer_length: uint16_t) -> uint16_t {
    let mut crc_hi: uint8_t = 0xff as libc::c_int as uint8_t;
    let mut crc_lo: uint8_t = 0xff as libc::c_int as uint8_t;
    let mut i: libc::c_uint = 0;
    loop 
         /* pass through message buffer */
         {
        let fresh0 = buffer_length; /* calculate the CRC  */
        buffer_length = buffer_length.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        let fresh1 = buffer;
        buffer = buffer.offset(1);
        i = (crc_hi as libc::c_int ^ *fresh1 as libc::c_int) as libc::c_uint;
        crc_hi =
            (crc_lo as libc::c_int ^ table_crc_hi[i as usize] as libc::c_int)
                as uint8_t;
        crc_lo = table_crc_lo[i as usize]
    }
    return ((crc_hi as libc::c_int) << 8 as libc::c_int |
                crc_lo as libc::c_int) as uint16_t;
}
unsafe extern "C" fn _modbus_rtu_prepare_response_tid(mut req: *const uint8_t,
                                                      mut req_length:
                                                          *mut libc::c_int)
 -> libc::c_int {
    *req_length -= 2 as libc::c_int;
    /* No TID */
    return 0 as libc::c_int;
}
unsafe extern "C" fn _modbus_rtu_send_msg_pre(mut req: *mut uint8_t,
                                              mut req_length: libc::c_int)
 -> libc::c_int {
    let mut crc: uint16_t = crc16(req, req_length as uint16_t);
    let fresh2 = req_length;
    req_length = req_length + 1;
    *req.offset(fresh2 as isize) =
        (crc as libc::c_int >> 8 as libc::c_int) as uint8_t;
    let fresh3 = req_length;
    req_length = req_length + 1;
    *req.offset(fresh3 as isize) =
        (crc as libc::c_int & 0xff as libc::c_int) as uint8_t;
    return req_length;
}
unsafe extern "C" fn _modbus_rtu_ioctl_rts(mut ctx: *mut modbus_t,
                                           mut on: libc::c_int) {
    let mut fd: libc::c_int = (*ctx).s;
    let mut flags: libc::c_int = 0;
    ioctl(fd, 0x5415 as libc::c_int as libc::c_ulong,
          &mut flags as *mut libc::c_int);
    if on != 0 {
        flags |= 0x4 as libc::c_int
    } else { flags &= !(0x4 as libc::c_int) }
    ioctl(fd, 0x5418 as libc::c_int as libc::c_ulong,
          &mut flags as *mut libc::c_int);
}
unsafe extern "C" fn _modbus_rtu_send(mut ctx: *mut modbus_t,
                                      mut req: *const uint8_t,
                                      mut req_length: libc::c_int)
 -> ssize_t {
    let mut ctx_rtu: *mut modbus_rtu_t =
        (*ctx).backend_data as *mut modbus_rtu_t;
    if (*ctx_rtu).rts != 0 as libc::c_int {
        let mut size: ssize_t = 0;
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Sending request using RTS signal\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        (*ctx_rtu).set_rts.expect("non-null function pointer")(ctx,
                                                               ((*ctx_rtu).rts
                                                                    ==
                                                                    1 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_int);
        usleep((*ctx_rtu).rts_delay as __useconds_t);
        size =
            write((*ctx).s, req as *const libc::c_void, req_length as size_t);
        usleep(((*ctx_rtu).onebyte_time * req_length + (*ctx_rtu).rts_delay)
                   as __useconds_t);
        (*ctx_rtu).set_rts.expect("non-null function pointer")(ctx,
                                                               ((*ctx_rtu).rts
                                                                    !=
                                                                    1 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_int);
        return size
    } else {
        return write((*ctx).s, req as *const libc::c_void,
                     req_length as size_t)
    };
}
unsafe extern "C" fn _modbus_rtu_receive(mut ctx: *mut modbus_t,
                                         mut req: *mut uint8_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ctx_rtu: *mut modbus_rtu_t =
        (*ctx).backend_data as *mut modbus_rtu_t;
    if (*ctx_rtu).confirmation_to_ignore != 0 {
        _modbus_receive_msg(ctx, req, MSG_CONFIRMATION);
        /* Ignore errors and reset the flag */
        (*ctx_rtu).confirmation_to_ignore = 0 as libc::c_int;
        rc = 0 as libc::c_int;
        if (*ctx).debug != 0 {
            printf(b"Confirmation to ignore\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    } else {
        rc = _modbus_receive_msg(ctx, req, MSG_INDICATION);
        if rc == 0 as libc::c_int {
            /* The next expected message is a confirmation to ignore */
            (*ctx_rtu).confirmation_to_ignore = 1 as libc::c_int
        }
    }
    return rc;
}
unsafe extern "C" fn _modbus_rtu_recv(mut ctx: *mut modbus_t,
                                      mut rsp: *mut uint8_t,
                                      mut rsp_length: libc::c_int)
 -> ssize_t {
    return read((*ctx).s, rsp as *mut libc::c_void, rsp_length as size_t);
}
unsafe extern "C" fn _modbus_rtu_pre_check_confirmation(mut ctx:
                                                            *mut modbus_t,
                                                        mut req:
                                                            *const uint8_t,
                                                        mut rsp:
                                                            *const uint8_t,
                                                        mut rsp_length:
                                                            libc::c_int)
 -> libc::c_int {
    /* Check responding slave is the slave we requested (except for broacast
     * request) */
    if *req.offset(0 as libc::c_int as isize) as libc::c_int !=
           *rsp.offset(0 as libc::c_int as isize) as libc::c_int &&
           *req.offset(0 as libc::c_int as isize) as libc::c_int !=
               0 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"The responding slave %d isn\'t the requested slave %d\n\x00"
                        as *const u8 as *const libc::c_char,
                    *rsp.offset(0 as libc::c_int as isize) as libc::c_int,
                    *req.offset(0 as libc::c_int as isize) as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                6 as libc::c_int;
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
/* The check_crc16 function shall return 0 if the message is ignored and the
   message length if the CRC is valid. Otherwise it shall return -1 and set
   errno to EMBBADCRC. */
unsafe extern "C" fn _modbus_rtu_check_integrity(mut ctx: *mut modbus_t,
                                                 mut msg: *mut uint8_t,
                                                 msg_length: libc::c_int)
 -> libc::c_int {
    let mut crc_calculated: uint16_t = 0;
    let mut crc_received: uint16_t = 0;
    let mut slave: libc::c_int =
        *msg.offset(0 as libc::c_int as isize) as libc::c_int;
    /* Filter on the Modbus unit identifier (slave) in RTU mode to avoid useless
     * CRC computing. */
    if slave != (*ctx).slave && slave != 0 as libc::c_int {
        if (*ctx).debug != 0 {
            printf(b"Request for slave %d ignored (not %d)\n\x00" as *const u8
                       as *const libc::c_char, slave, (*ctx).slave);
        }
        /* Following call to check_confirmation handles this error */
        return 0 as libc::c_int
    }
    crc_calculated = crc16(msg, (msg_length - 2 as libc::c_int) as uint16_t);
    crc_received =
        ((*msg.offset((msg_length - 2 as libc::c_int) as isize) as
              libc::c_int) << 8 as libc::c_int |
             *msg.offset((msg_length - 1 as libc::c_int) as isize) as
                 libc::c_int) as uint16_t;
    /* Check CRC of msg */
    if crc_calculated as libc::c_int == crc_received as libc::c_int {
        return msg_length
    } else {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR CRC received 0x%0X != CRC calculated 0x%0X\n\x00"
                        as *const u8 as *const libc::c_char,
                    crc_received as libc::c_int,
                    crc_calculated as libc::c_int);
        }
        if (*ctx).error_recovery &
               MODBUS_ERROR_RECOVERY_PROTOCOL as libc::c_int != 0 {
            _modbus_rtu_flush(ctx);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                1 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
/* Sets up a serial port for RTU communications */
unsafe extern "C" fn _modbus_rtu_connect(mut ctx: *mut modbus_t)
 -> libc::c_int {
    let mut tios: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut speed: speed_t = 0;
    let mut flags: libc::c_int = 0;
    let mut ctx_rtu: *mut modbus_rtu_t =
        (*ctx).backend_data as *mut modbus_rtu_t;
    if (*ctx).debug != 0 {
        printf(b"Opening %s at %d bauds (%c, %d, %d)\n\x00" as *const u8 as
                   *const libc::c_char, (*ctx_rtu).device, (*ctx_rtu).baud,
               (*ctx_rtu).parity as libc::c_int,
               (*ctx_rtu).data_bit as libc::c_int,
               (*ctx_rtu).stop_bit as libc::c_int);
    }
    /* The O_NOCTTY flag tells UNIX that this program doesn't want
       to be the "controlling terminal" for that port. If you
       don't specify this then any input (such as keyboard abort
       signals and so forth) will affect your process

       Timeouts are ignored in canonical input mode or when the
       NDELAY option is set on the file via open or fcntl */
    flags =
        0o2 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int |
            0o200 as libc::c_int;
    flags |= 0o2000000 as libc::c_int;
    (*ctx).s = open((*ctx_rtu).device, flags);
    if (*ctx).s == -(1 as libc::c_int) {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Can\'t open the device %s (%s)\n\x00" as *const u8
                        as *const libc::c_char, (*ctx_rtu).device,
                    strerror(*__errno_location()));
        }
        return -(1 as libc::c_int)
    }
    /* Save */
    tcgetattr((*ctx).s, &mut (*ctx_rtu).old_tios);
    memset(&mut tios as *mut termios as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<termios>() as libc::c_ulong);
    /* C_ISPEED     Input baud (new interface)
       C_OSPEED     Output baud (new interface)
    */
    match (*ctx_rtu).baud {
        110 => { speed = 0o3 as libc::c_int as speed_t }
        300 => { speed = 0o7 as libc::c_int as speed_t }
        600 => { speed = 0o10 as libc::c_int as speed_t }
        1200 => { speed = 0o11 as libc::c_int as speed_t }
        2400 => { speed = 0o13 as libc::c_int as speed_t }
        4800 => { speed = 0o14 as libc::c_int as speed_t }
        9600 => { speed = 0o15 as libc::c_int as speed_t }
        19200 => { speed = 0o16 as libc::c_int as speed_t }
        38400 => { speed = 0o17 as libc::c_int as speed_t }
        57600 => { speed = 0o10001 as libc::c_int as speed_t }
        115200 => { speed = 0o10002 as libc::c_int as speed_t }
        230400 => { speed = 0o10003 as libc::c_int as speed_t }
        460800 => { speed = 0o10004 as libc::c_int as speed_t }
        500000 => { speed = 0o10005 as libc::c_int as speed_t }
        576000 => { speed = 0o10006 as libc::c_int as speed_t }
        921600 => { speed = 0o10007 as libc::c_int as speed_t }
        1000000 => { speed = 0o10010 as libc::c_int as speed_t }
        1152000 => { speed = 0o10011 as libc::c_int as speed_t }
        1500000 => { speed = 0o10012 as libc::c_int as speed_t }
        2500000 => { speed = 0o10014 as libc::c_int as speed_t }
        3000000 => { speed = 0o10015 as libc::c_int as speed_t }
        3500000 => { speed = 0o10016 as libc::c_int as speed_t }
        4000000 => { speed = 0o10017 as libc::c_int as speed_t }
        _ => {
            speed = 0o15 as libc::c_int as speed_t;
            if (*ctx).debug != 0 {
                fprintf(stderr,
                        b"WARNING Unknown baud rate %d for %s (B9600 used)\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*ctx_rtu).baud, (*ctx_rtu).device);
            }
        }
    }
    /* Set the baud rate */
    if cfsetispeed(&mut tios, speed) < 0 as libc::c_int ||
           cfsetospeed(&mut tios, speed) < 0 as libc::c_int {
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    /* C_CFLAG      Control options
       CLOCAL       Local line - do not change "owner" of port
       CREAD        Enable receiver
    */
    tios.c_cflag |=
        (0o200 as libc::c_int | 0o4000 as libc::c_int) as libc::c_uint;
    /* CSIZE, HUPCL, CRTSCTS (hardware flow control) */
    /* Set data bits (5, 6, 7, 8 bits)
       CSIZE        Bit mask for data bits
    */
    tios.c_cflag &= !(0o60 as libc::c_int) as libc::c_uint;
    match (*ctx_rtu).data_bit as libc::c_int {
        5 => { tios.c_cflag |= 0 as libc::c_int as libc::c_uint }
        6 => { tios.c_cflag |= 0o20 as libc::c_int as libc::c_uint }
        7 => { tios.c_cflag |= 0o40 as libc::c_int as libc::c_uint }
        8 | _ => { tios.c_cflag |= 0o60 as libc::c_int as libc::c_uint }
    }
    /* Stop bit (1 or 2) */
    if (*ctx_rtu).stop_bit as libc::c_int == 1 as libc::c_int {
        tios.c_cflag &= !(0o100 as libc::c_int) as libc::c_uint
    } else {
        /* 2 */
        tios.c_cflag |= 0o100 as libc::c_int as libc::c_uint
    }
    /* PARENB       Enable parity bit
       PARODD       Use odd parity instead of even */
    if (*ctx_rtu).parity as libc::c_int == 'N' as i32 {
        /* None */
        tios.c_cflag &= !(0o400 as libc::c_int) as libc::c_uint
    } else if (*ctx_rtu).parity as libc::c_int == 'E' as i32 {
        /* Even */
        tios.c_cflag |= 0o400 as libc::c_int as libc::c_uint;
        tios.c_cflag &= !(0o1000 as libc::c_int) as libc::c_uint
    } else {
        /* Odd */
        tios.c_cflag |= 0o400 as libc::c_int as libc::c_uint;
        tios.c_cflag |= 0o1000 as libc::c_int as libc::c_uint
    }
    /* Read the man page of termios if you need more information. */
    /* This field isn't used on POSIX systems
       tios.c_line = 0;
    */
    /* C_LFLAG      Line options

       ISIG Enable SIGINTR, SIGSUSP, SIGDSUSP, and SIGQUIT signals
       ICANON       Enable canonical input (else raw)
       XCASE        Map uppercase \lowercase (obsolete)
       ECHO Enable echoing of input characters
       ECHOE        Echo erase character as BS-SP-BS
       ECHOK        Echo NL after kill character
       ECHONL       Echo NL
       NOFLSH       Disable flushing of input buffers after
       interrupt or quit characters
       IEXTEN       Enable extended functions
       ECHOCTL      Echo control characters as ^char and delete as ~?
       ECHOPRT      Echo erased character as character erased
       ECHOKE       BS-SP-BS entire line on line kill
       FLUSHO       Output being flushed
       PENDIN       Retype pending input at next read or input char
       TOSTOP       Send SIGTTOU for background output

       Canonical input is line-oriented. Input characters are put
       into a buffer which can be edited interactively by the user
       until a CR (carriage return) or LF (line feed) character is
       received.

       Raw input is unprocessed. Input characters are passed
       through exactly as they are received, when they are
       received. Generally you'll deselect the ICANON, ECHO,
       ECHOE, and ISIG options when using raw input
    */
    /* Raw input */
    tios.c_lflag &=
        !(0o2 as libc::c_int | 0o10 as libc::c_int | 0o20 as libc::c_int |
              0o1 as libc::c_int) as libc::c_uint;
    /* C_IFLAG      Input options

       Constant     Description
       INPCK        Enable parity check
       IGNPAR       Ignore parity errors
       PARMRK       Mark parity errors
       ISTRIP       Strip parity bits
       IXON Enable software flow control (outgoing)
       IXOFF        Enable software flow control (incoming)
       IXANY        Allow any character to start flow again
       IGNBRK       Ignore break condition
       BRKINT       Send a SIGINT when a break condition is detected
       INLCR        Map NL to CR
       IGNCR        Ignore CR
       ICRNL        Map CR to NL
       IUCLC        Map uppercase to lowercase
       IMAXBEL      Echo BEL on input line too long
    */
    if (*ctx_rtu).parity as libc::c_int == 'N' as i32 {
        /* None */
        tios.c_iflag &= !(0o20 as libc::c_int) as libc::c_uint
    } else { tios.c_iflag |= 0o20 as libc::c_int as libc::c_uint }
    /* Software flow control is disabled */
    tios.c_iflag &=
        !(0o2000 as libc::c_int | 0o10000 as libc::c_int |
              0o4000 as libc::c_int) as libc::c_uint;
    /* C_OFLAG      Output options
       OPOST        Postprocess output (not set = raw output)
       ONLCR        Map NL to CR-NL

       ONCLR ant others needs OPOST to be enabled
    */
    /* Raw output */
    tios.c_oflag &= !(0o1 as libc::c_int) as libc::c_uint;
    /* C_CC         Control characters
       VMIN         Minimum number of characters to read
       VTIME        Time to wait for data (tenths of seconds)

       UNIX serial interface drivers provide the ability to
       specify character and packet timeouts. Two elements of the
       c_cc array are used for timeouts: VMIN and VTIME. Timeouts
       are ignored in canonical input mode or when the NDELAY
       option is set on the file via open or fcntl.

       VMIN specifies the minimum number of characters to read. If
       it is set to 0, then the VTIME value specifies the time to
       wait for every character read. Note that this does not mean
       that a read call for N bytes will wait for N characters to
       come in. Rather, the timeout will apply to the first
       character and the read call will return the number of
       characters immediately available (up to the number you
       request).

       If VMIN is non-zero, VTIME specifies the time to wait for
       the first character read. If a character is read within the
       time given, any read will block (wait) until all VMIN
       characters are read. That is, once the first character is
       read, the serial interface driver expects to receive an
       entire packet of characters (VMIN bytes total). If no
       character is read within the time allowed, then the call to
       read returns 0. This method allows you to tell the serial
       driver you need exactly N bytes and any read call will
       return 0 or N bytes. However, the timeout only applies to
       the first character read, so if for some reason the driver
       misses one character inside the N byte packet then the read
       call could block forever waiting for additional input
       characters.

       VTIME specifies the amount of time to wait for incoming
       characters in tenths of seconds. If VTIME is set to 0 (the
       default), reads will block (wait) indefinitely unless the
       NDELAY option is set on the port with open or fcntl.
    */
    /* Unused because we use open with the NDELAY option */
    tios.c_cc[6 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    tios.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    if tcsetattr((*ctx).s, 0 as libc::c_int, &mut tios) < 0 as libc::c_int {
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_set_serial_mode(mut ctx: *mut modbus_t,
                                                    mut mode: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t =
            (*ctx).backend_data as *mut modbus_rtu_t;
        let mut rs485conf: serial_rs485 =
            serial_rs485{flags: 0,
                         delay_rts_before_send: 0,
                         delay_rts_after_send: 0,
                         padding: [0; 5],};
        if mode == 1 as libc::c_int {
            // Get
            if ioctl((*ctx).s, 0x542e as libc::c_int as libc::c_ulong,
                     &mut rs485conf as *mut serial_rs485) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            // Set
            rs485conf.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if ioctl((*ctx).s, 0x542f as libc::c_int as libc::c_ulong,
                     &mut rs485conf as *mut serial_rs485) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            (*ctx_rtu).serial_mode = 1 as libc::c_int;
            return 0 as libc::c_int
        } else {
            if mode == 0 as libc::c_int {
                /* Turn off RS485 mode only if required */
                if (*ctx_rtu).serial_mode == 1 as libc::c_int {
                    /* The ioctl call is avoided because it can fail on some RS232 ports */
                    if ioctl((*ctx).s, 0x542e as libc::c_int as libc::c_ulong,
                             &mut rs485conf as *mut serial_rs485) <
                           0 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    rs485conf.flags &=
                        !((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint;
                    if ioctl((*ctx).s, 0x542f as libc::c_int as libc::c_ulong,
                             &mut rs485conf as *mut serial_rs485) <
                           0 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                }
                (*ctx_rtu).serial_mode = 0 as libc::c_int;
                return 0 as libc::c_int
            }
        }
    }
    /* Wrong backend and invalid mode specified */
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_get_serial_mode(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t =
            (*ctx).backend_data as *mut modbus_rtu_t;
        return (*ctx_rtu).serial_mode
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_get_rts(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t =
            (*ctx).backend_data as *mut modbus_rtu_t;
        return (*ctx_rtu).rts
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_set_rts(mut ctx: *mut modbus_t,
                                            mut mode: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t =
            (*ctx).backend_data as *mut modbus_rtu_t;
        if mode == 0 as libc::c_int || mode == 1 as libc::c_int ||
               mode == 2 as libc::c_int {
            (*ctx_rtu).rts = mode;
            /* Set the RTS bit in order to not reserve the RS485 bus */
            (*ctx_rtu).set_rts.expect("non-null function pointer")(ctx,
                                                                   ((*ctx_rtu).rts
                                                                        !=
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_int);
            return 0 as libc::c_int
        } else {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    /* Wrong backend or invalid mode specified */
    *__errno_location() = 22 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_set_custom_rts(mut ctx: *mut modbus_t,
                                                   mut set_rts:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       *mut modbus_t,
                                                                                   _:
                                                                                       libc::c_int)
                                                                  -> ()>)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t =
            (*ctx).backend_data as *mut modbus_rtu_t;
        (*ctx_rtu).set_rts = set_rts;
        return 0 as libc::c_int
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_get_rts_delay(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t = 0 as *mut modbus_rtu_t;
        ctx_rtu = (*ctx).backend_data as *mut modbus_rtu_t;
        return (*ctx_rtu).rts_delay
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_rtu_set_rts_delay(mut ctx: *mut modbus_t,
                                                  mut us: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() || us < 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if (*(*ctx).backend).backend_type ==
           _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint {
        let mut ctx_rtu: *mut modbus_rtu_t = 0 as *mut modbus_rtu_t;
        ctx_rtu = (*ctx).backend_data as *mut modbus_rtu_t;
        (*ctx_rtu).rts_delay = us;
        return 0 as libc::c_int
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn _modbus_rtu_close(mut ctx: *mut modbus_t) {
    /* Restore line settings and close file descriptor in RTU mode */
    let mut ctx_rtu: *mut modbus_rtu_t =
        (*ctx).backend_data as *mut modbus_rtu_t;
    if (*ctx).s != -(1 as libc::c_int) {
        tcsetattr((*ctx).s, 0 as libc::c_int, &mut (*ctx_rtu).old_tios);
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int)
    };
}
unsafe extern "C" fn _modbus_rtu_flush(mut ctx: *mut modbus_t)
 -> libc::c_int {
    return tcflush((*ctx).s, 2 as libc::c_int);
}
unsafe extern "C" fn _modbus_rtu_select(mut ctx: *mut modbus_t,
                                        mut rset: *mut fd_set,
                                        mut tv: *mut timeval,
                                        mut length_to_read: libc::c_int)
 -> libc::c_int {
    let mut s_rc: libc::c_int = 0;
    loop  {
        s_rc =
            select((*ctx).s + 1 as libc::c_int, rset, 0 as *mut fd_set,
                   0 as *mut fd_set, tv);
        if !(s_rc == -(1 as libc::c_int)) { break ; }
        if *__errno_location() == 4 as libc::c_int {
            if (*ctx).debug != 0 {
                fprintf(stderr,
                        b"A non blocked signal was caught\n\x00" as *const u8
                            as *const libc::c_char);
            }
            /* Necessary after an error */
            let mut __i: libc::c_uint = 0;
            let mut __arr: *mut fd_set = rset;
            __i = 0 as libc::c_int as libc::c_uint;
            while (__i as libc::c_ulong) <
                      (::std::mem::size_of::<fd_set>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                           as libc::c_ulong) {
                (*__arr).__fds_bits[__i as usize] =
                    0 as libc::c_int as __fd_mask;
                __i = __i.wrapping_add(1)
            }
            (*rset).__fds_bits[((*ctx).s /
                                    (8 as libc::c_int *
                                         ::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong as libc::c_int)) as
                                   usize] |=
                ((1 as libc::c_ulong) <<
                     (*ctx).s %
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as __fd_mask
        } else { return -(1 as libc::c_int) }
    }
    if s_rc == 0 as libc::c_int {
        /* Timeout */
        *__errno_location() = 110 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return s_rc;
}
unsafe extern "C" fn _modbus_rtu_free(mut ctx: *mut modbus_t) {
    if !(*ctx).backend_data.is_null() {
        free((*((*ctx).backend_data as *mut modbus_rtu_t)).device as
                 *mut libc::c_void);
        free((*ctx).backend_data);
    }
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub static mut _modbus_rtu_backend: modbus_backend_t =
    unsafe {
        {
            let mut init =
                _modbus_backend{backend_type:
                                    _MODBUS_BACKEND_TYPE_RTU as libc::c_int as
                                        libc::c_uint,
                                header_length:
                                    1 as libc::c_int as libc::c_uint,
                                checksum_length:
                                    2 as libc::c_int as libc::c_uint,
                                max_adu_length:
                                    256 as libc::c_int as libc::c_uint,
                                set_slave:
                                    Some(_modbus_set_slave as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                build_request_basis:
                                    Some(_modbus_rtu_build_request_basis as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                build_response_basis:
                                    Some(_modbus_rtu_build_response_basis as
                                             unsafe extern "C" fn(_:
                                                                      *mut sft_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                prepare_response_tid:
                                    Some(_modbus_rtu_prepare_response_tid as
                                             unsafe extern "C" fn(_:
                                                                      *const uint8_t,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> libc::c_int),
                                send_msg_pre:
                                    Some(_modbus_rtu_send_msg_pre as
                                             unsafe extern "C" fn(_:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                send:
                                    Some(_modbus_rtu_send as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *const uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                receive:
                                    Some(_modbus_rtu_receive as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                recv:
                                    Some(_modbus_rtu_recv as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                check_integrity:
                                    Some(_modbus_rtu_check_integrity as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                pre_check_confirmation:
                                    Some(_modbus_rtu_pre_check_confirmation as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *const uint8_t,
                                                                  _:
                                                                      *const uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                connect:
                                    Some(_modbus_rtu_connect as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                close:
                                    Some(_modbus_rtu_close as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),
                                flush:
                                    Some(_modbus_rtu_flush as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                select:
                                    Some(_modbus_rtu_select as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut fd_set,
                                                                  _:
                                                                      *mut timeval,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                free:
                                    Some(_modbus_rtu_free as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn modbus_new_rtu(mut device: *const libc::c_char,
                                        mut baud: libc::c_int,
                                        mut parity: libc::c_char,
                                        mut data_bit: libc::c_int,
                                        mut stop_bit: libc::c_int)
 -> *mut modbus_t {
    let mut ctx: *mut modbus_t = 0 as *mut modbus_t;
    let mut ctx_rtu: *mut modbus_rtu_t = 0 as *mut modbus_rtu_t;
    /* Check device argument */
    if device.is_null() || *device as libc::c_int == 0 as libc::c_int {
        fprintf(stderr,
                b"The device string is empty\n\x00" as *const u8 as
                    *const libc::c_char);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut modbus_t
    }
    /* Check baud argument */
    if baud == 0 as libc::c_int {
        fprintf(stderr,
                b"The baud rate value must not be zero\n\x00" as *const u8 as
                    *const libc::c_char);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut modbus_t
    }
    ctx =
        malloc(::std::mem::size_of::<modbus_t>() as libc::c_ulong) as
            *mut modbus_t;
    if ctx.is_null() { return 0 as *mut modbus_t }
    _modbus_init_common(ctx);
    (*ctx).backend = &_modbus_rtu_backend;
    (*ctx).backend_data =
        malloc(::std::mem::size_of::<modbus_rtu_t>() as libc::c_ulong) as
            *mut modbus_rtu_t as *mut libc::c_void;
    if (*ctx).backend_data.is_null() {
        modbus_free(ctx);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut modbus_t
    }
    ctx_rtu = (*ctx).backend_data as *mut modbus_rtu_t;
    /* Device name and \0 */
    (*ctx_rtu).device =
        malloc(strlen(device).wrapping_add(1 as libc::c_int as
                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                               as
                                                                               libc::c_ulong))
            as *mut libc::c_char;
    if (*ctx_rtu).device.is_null() {
        modbus_free(ctx);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut modbus_t
    }
    strcpy((*ctx_rtu).device, device);
    (*ctx_rtu).baud = baud;
    if parity as libc::c_int == 'N' as i32 ||
           parity as libc::c_int == 'E' as i32 ||
           parity as libc::c_int == 'O' as i32 {
        (*ctx_rtu).parity = parity
    } else {
        modbus_free(ctx);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut modbus_t
    }
    (*ctx_rtu).data_bit = data_bit as uint8_t;
    (*ctx_rtu).stop_bit = stop_bit as uint8_t;
    /* The RS232 mode has been set by default */
    (*ctx_rtu).serial_mode = 0 as libc::c_int;
    /* The RTS use has been set by default */
    (*ctx_rtu).rts = 0 as libc::c_int;
    /* Calculate estimated time in micro second to send one byte */
    (*ctx_rtu).onebyte_time =
        1000000 as libc::c_int *
            (1 as libc::c_int + data_bit +
                 (if parity as libc::c_int == 'N' as i32 {
                      0 as libc::c_int
                  } else { 1 as libc::c_int }) + stop_bit) / baud;
    /* The internal function is used by default to set RTS */
    (*ctx_rtu).set_rts =
        Some(_modbus_rtu_ioctl_rts as
                 unsafe extern "C" fn(_: *mut modbus_t, _: libc::c_int)
                     -> ());
    /* The delay before and after transmission when toggling the RTS pin */
    (*ctx_rtu).rts_delay = (*ctx_rtu).onebyte_time;
    (*ctx_rtu).confirmation_to_ignore = 0 as libc::c_int;
    return ctx;
}
