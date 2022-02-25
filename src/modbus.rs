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
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn modbus_set_bits_from_bytes(dest: *mut uint8_t, idx: libc::c_int,
                                  nb_bits: libc::c_uint,
                                  tab_byte: *const uint8_t);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus_mapping_t {
    pub nb_bits: libc::c_int,
    pub start_bits: libc::c_int,
    pub nb_input_bits: libc::c_int,
    pub start_input_bits: libc::c_int,
    pub nb_input_registers: libc::c_int,
    pub start_input_registers: libc::c_int,
    pub nb_registers: libc::c_int,
    pub start_registers: libc::c_int,
    pub tab_bits: *mut uint8_t,
    pub tab_input_bits: *mut uint8_t,
    pub tab_input_registers: *mut uint16_t,
    pub tab_registers: *mut uint16_t,
}
pub type modbus_mapping_t = _modbus_mapping_t;
pub type modbus_error_recovery_mode = libc::c_uint;
pub const MODBUS_ERROR_RECOVERY_PROTOCOL: modbus_error_recovery_mode = 4;
pub const MODBUS_ERROR_RECOVERY_LINK: modbus_error_recovery_mode = 2;
pub const MODBUS_ERROR_RECOVERY_NONE: modbus_error_recovery_mode = 0;
pub type msg_type_t = libc::c_uint;
pub const MSG_CONFIRMATION: msg_type_t = 1;
pub const MSG_INDICATION: msg_type_t = 0;
pub type _step_t = libc::c_uint;
pub const _STEP_DATA: _step_t = 2;
pub const _STEP_META: _step_t = 1;
pub const _STEP_FUNCTION: _step_t = 0;
pub const _MODBUS_BACKEND_TYPE_RTU: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _MODBUS_BACKEND_TYPE_TCP: C2RustUnnamed_0 = 1;
/* Exported version */
#[no_mangle]
pub static mut libmodbus_version_major: libc::c_uint =
    3 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut libmodbus_version_minor: libc::c_uint =
    1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut libmodbus_version_micro: libc::c_uint =
    7 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn modbus_strerror(mut errnum: libc::c_int)
 -> *const libc::c_char {
    match errnum {
        112345679 => {
            return b"Illegal function\x00" as *const u8 as *const libc::c_char
        }
        112345680 => {
            return b"Illegal data address\x00" as *const u8 as
                       *const libc::c_char
        }
        112345681 => {
            return b"Illegal data value\x00" as *const u8 as
                       *const libc::c_char
        }
        112345682 => {
            return b"Slave device or server failure\x00" as *const u8 as
                       *const libc::c_char
        }
        112345683 => {
            return b"Acknowledge\x00" as *const u8 as *const libc::c_char
        }
        112345684 => {
            return b"Slave device or server is busy\x00" as *const u8 as
                       *const libc::c_char
        }
        112345685 => {
            return b"Negative acknowledge\x00" as *const u8 as
                       *const libc::c_char
        }
        112345686 => {
            return b"Memory parity error\x00" as *const u8 as
                       *const libc::c_char
        }
        112345688 => {
            return b"Gateway path unavailable\x00" as *const u8 as
                       *const libc::c_char
        }
        112345689 => {
            return b"Target device failed to respond\x00" as *const u8 as
                       *const libc::c_char
        }
        112345690 => {
            return b"Invalid CRC\x00" as *const u8 as *const libc::c_char
        }
        112345691 => {
            return b"Invalid data\x00" as *const u8 as *const libc::c_char
        }
        112345692 => {
            return b"Invalid exception code\x00" as *const u8 as
                       *const libc::c_char
        }
        112345694 => {
            return b"Too many data\x00" as *const u8 as *const libc::c_char
        }
        112345695 => {
            return b"Response not from requested slave\x00" as *const u8 as
                       *const libc::c_char
        }
        _ => { return strerror(errnum) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _error_print(mut ctx: *mut modbus_t,
                                      mut context: *const libc::c_char) {
    if (*ctx).debug != 0 {
        fprintf(stderr, b"ERROR %s\x00" as *const u8 as *const libc::c_char,
                modbus_strerror(*__errno_location()));
        if !context.is_null() {
            fprintf(stderr, b": %s\n\x00" as *const u8 as *const libc::c_char,
                    context);
        } else {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn _sleep_response_timeout(mut ctx: *mut modbus_t) {
    /* Response timeout is always positive */
    /* usleep source code */
    let mut request: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    let mut remaining: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    request.tv_sec = (*ctx).response_timeout.tv_sec;
    request.tv_nsec =
        (*ctx).response_timeout.tv_usec * 1000 as libc::c_int as libc::c_long;
    while nanosleep(&mut request, &mut remaining) == -(1 as libc::c_int) &&
              *__errno_location() == 4 as libc::c_int {
        request = remaining
    };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_flush(mut ctx: *mut modbus_t) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    rc = (*(*ctx).backend).flush.expect("non-null function pointer")(ctx);
    if rc != -(1 as libc::c_int) && (*ctx).debug != 0 {
        /* Not all backends are able to return the number of bytes flushed */
        printf(b"Bytes flushed (%d)\n\x00" as *const u8 as
                   *const libc::c_char, rc);
    }
    return rc;
}
/* Computes the length of the expected response */
unsafe extern "C" fn compute_response_length_from_request(mut ctx:
                                                              *mut modbus_t,
                                                          mut req:
                                                              *mut uint8_t)
 -> libc::c_uint {
    let mut length: libc::c_int = 0;
    let offset: libc::c_int = (*(*ctx).backend).header_length as libc::c_int;
    match *req.offset(offset as isize) as libc::c_int {
        1 | 2 => {
            /* Header + nb values (code from write_bits) */
            let mut nb: libc::c_int =
                (*req.offset((offset + 3 as libc::c_int) as isize) as
                     libc::c_int) << 8 as libc::c_int |
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            length =
                2 as libc::c_int + nb / 8 as libc::c_int +
                    (if nb % 8 as libc::c_int != 0 {
                         1 as libc::c_int
                     } else { 0 as libc::c_int })
        }
        23 | 3 | 4 => {
            /* Header + 2 * nb values */
            length =
                2 as libc::c_int +
                    2 as libc::c_int *
                        ((*req.offset((offset + 3 as libc::c_int) as isize) as
                              libc::c_int) << 8 as libc::c_int |
                             *req.offset((offset + 4 as libc::c_int) as isize)
                                 as libc::c_int)
        }
        7 => { length = 3 as libc::c_int }
        17 => {
            /* The response is device specific (the header provides the
           length) */
            return -(1 as libc::c_int) as libc::c_uint
        }
        22 => { length = 7 as libc::c_int }
        _ => { length = 5 as libc::c_int }
    }
    return ((offset + length) as
                libc::c_uint).wrapping_add((*(*ctx).backend).checksum_length);
}
/* Sends a request/response */
unsafe extern "C" fn send_msg(mut ctx: *mut modbus_t, mut msg: *mut uint8_t,
                              mut msg_length: libc::c_int) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    msg_length =
        (*(*ctx).backend).send_msg_pre.expect("non-null function pointer")(msg,
                                                                           msg_length);
    if (*ctx).debug != 0 {
        i = 0 as libc::c_int;
        while i < msg_length {
            printf(b"[%.2X]\x00" as *const u8 as *const libc::c_char,
                   *msg.offset(i as isize) as libc::c_int);
            i += 1
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    loop 
         /* In recovery mode, the write command will be issued until to be
       successful! Disabled by default. */
         {
        rc =
            (*(*ctx).backend).send.expect("non-null function pointer")(ctx,
                                                                       msg,
                                                                       msg_length)
                as libc::c_int;
        if rc == -(1 as libc::c_int) {
            _error_print(ctx, 0 as *const libc::c_char);
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_LINK as libc::c_int != 0 {
                let mut saved_errno: libc::c_int = *__errno_location();
                if *__errno_location() == 9 as libc::c_int ||
                       *__errno_location() == 104 as libc::c_int ||
                       *__errno_location() == 32 as libc::c_int {
                    modbus_close(ctx);
                    _sleep_response_timeout(ctx);
                    modbus_connect(ctx);
                } else { _sleep_response_timeout(ctx); modbus_flush(ctx); }
                *__errno_location() = saved_errno
            }
        }
        if !((*ctx).error_recovery & MODBUS_ERROR_RECOVERY_LINK as libc::c_int
                 != 0 && rc == -(1 as libc::c_int)) {
            break ;
        }
    }
    if rc > 0 as libc::c_int && rc != msg_length {
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_send_raw_request(mut ctx: *mut modbus_t,
                                                 mut raw_req: *const uint8_t,
                                                 mut raw_req_length:
                                                     libc::c_int)
 -> libc::c_int {
    let mut sft: sft_t = sft_t{slave: 0, function: 0, t_id: 0,};
    let mut req: [uint8_t; 260] = [0; 260];
    let mut req_length: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if raw_req_length < 2 as libc::c_int ||
           raw_req_length > 253 as libc::c_int + 1 as libc::c_int {
        /* The raw request must contain function and slave at least and
           must not be longer than the maximum pdu length plus the slave
           address. */
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    sft.slave = *raw_req.offset(0 as libc::c_int as isize) as libc::c_int;
    sft.function = *raw_req.offset(1 as libc::c_int as isize) as libc::c_int;
    /* The t_id is left to zero */
    sft.t_id = 0 as libc::c_int;
    /* This response function only set the header so it's convenient here */
    req_length =
        (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                   req.as_mut_ptr());
    if raw_req_length > 2 as libc::c_int {
        /* Copy data after function code */
        memcpy(req.as_mut_ptr().offset(req_length as isize) as
                   *mut libc::c_void,
               raw_req.offset(2 as libc::c_int as isize) as
                   *const libc::c_void,
               (raw_req_length - 2 as libc::c_int) as libc::c_ulong);
        req_length += raw_req_length - 2 as libc::c_int
    }
    return send_msg(ctx, req.as_mut_ptr(), req_length);
}
/*
 *  ---------- Request     Indication ----------
 *  | Client | ---------------------->| Server |
 *  ---------- Confirmation  Response ----------
 */
/* Computes the length to read after the function received */
unsafe extern "C" fn compute_meta_length_after_function(mut function:
                                                            libc::c_int,
                                                        mut msg_type:
                                                            msg_type_t)
 -> uint8_t {
    let mut length: libc::c_int = 0;
    if msg_type as libc::c_uint ==
           MSG_INDICATION as libc::c_int as libc::c_uint {
        if function <= 0x6 as libc::c_int {
            length = 4 as libc::c_int
        } else if function == 0xf as libc::c_int ||
                      function == 0x10 as libc::c_int {
            length = 5 as libc::c_int
        } else if function == 0x16 as libc::c_int {
            length = 6 as libc::c_int
        } else if function == 0x17 as libc::c_int {
            length = 9 as libc::c_int
        } else {
            /* MODBUS_FC_READ_EXCEPTION_STATUS, MODBUS_FC_REPORT_SLAVE_ID */
            length = 0 as libc::c_int
        }
    } else {
        /* MSG_CONFIRMATION */
        match function {
            5 | 6 | 15 | 16 => { length = 4 as libc::c_int }
            22 => { length = 6 as libc::c_int }
            _ => { length = 1 as libc::c_int }
        }
    }
    return length as uint8_t;
}
/* Computes the length to read after the meta information (address, count, etc) */
unsafe extern "C" fn compute_data_length_after_meta(mut ctx: *mut modbus_t,
                                                    mut msg: *mut uint8_t,
                                                    mut msg_type: msg_type_t)
 -> libc::c_int {
    let mut function: libc::c_int =
        *msg.offset((*(*ctx).backend).header_length as isize) as libc::c_int;
    let mut length: libc::c_int = 0;
    if msg_type as libc::c_uint ==
           MSG_INDICATION as libc::c_int as libc::c_uint {
        match function {
            15 | 16 => {
                length =
                    *msg.offset((*(*ctx).backend).header_length.wrapping_add(5
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                                    as isize) as libc::c_int
            }
            23 => {
                length =
                    *msg.offset((*(*ctx).backend).header_length.wrapping_add(9
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                                    as isize) as libc::c_int
            }
            _ => { length = 0 as libc::c_int }
        }
    } else if function <= 0x4 as libc::c_int ||
                  function == 0x11 as libc::c_int ||
                  function == 0x17 as libc::c_int {
        length =
            *msg.offset((*(*ctx).backend).header_length.wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                            as isize) as libc::c_int
    } else { length = 0 as libc::c_int }
    length =
        (length as
             libc::c_uint).wrapping_add((*(*ctx).backend).checksum_length) as
            libc::c_int as libc::c_int;
    return length;
}
/* MSG_CONFIRMATION */
/* Waits a response from a modbus server or a request from a modbus client.
   This function blocks if there is no replies (3 timeouts).

   The function shall return the number of received characters and the received
   message in an array of uint8_t if successful. Otherwise it shall return -1
   and errno is set to one of the values defined below:
   - ECONNRESET
   - EMBBADDATA
   - ETIMEDOUT
   - read() or recv() error codes
*/
#[no_mangle]
pub unsafe extern "C" fn _modbus_receive_msg(mut ctx: *mut modbus_t,
                                             mut msg: *mut uint8_t,
                                             mut msg_type: msg_type_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut rset: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut p_tv: *mut timeval = 0 as *mut timeval;
    let mut length_to_read: libc::c_int = 0;
    let mut msg_length: libc::c_int = 0 as libc::c_int;
    let mut step: _step_t = _STEP_FUNCTION;
    if (*ctx).debug != 0 {
        if msg_type as libc::c_uint ==
               MSG_INDICATION as libc::c_int as libc::c_uint {
            printf(b"Waiting for an indication...\n\x00" as *const u8 as
                       *const libc::c_char);
        } else {
            printf(b"Waiting for a confirmation...\n\x00" as *const u8 as
                       *const libc::c_char);
        }
    }
    /* Add a file descriptor to the set */
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut rset;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong) <
              (::std::mem::size_of::<fd_set>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                   as libc::c_ulong) {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1)
    }
    rset.__fds_bits[((*ctx).s /
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             (*ctx).s %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    /* We need to analyse the message step by step.  At the first step, we want
     * to reach the function code because all packets contain this
     * information. */
    step = _STEP_FUNCTION;
    length_to_read =
        (*(*ctx).backend).header_length.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
            libc::c_int;
    if msg_type as libc::c_uint ==
           MSG_INDICATION as libc::c_int as libc::c_uint {
        /* Wait for a message, we don't know when the message will be
         * received */
        if (*ctx).indication_timeout.tv_sec ==
               0 as libc::c_int as libc::c_long &&
               (*ctx).indication_timeout.tv_usec ==
                   0 as libc::c_int as libc::c_long {
            /* By default, the indication timeout isn't set */
            p_tv = 0 as *mut timeval
        } else {
            /* Wait for an indication (name of a received request by a server, see schema) */
            tv.tv_sec = (*ctx).indication_timeout.tv_sec;
            tv.tv_usec = (*ctx).indication_timeout.tv_usec;
            p_tv = &mut tv
        }
    } else {
        tv.tv_sec = (*ctx).response_timeout.tv_sec;
        tv.tv_usec = (*ctx).response_timeout.tv_usec;
        p_tv = &mut tv
    }
    while length_to_read != 0 as libc::c_int {
        rc =
            (*(*ctx).backend).select.expect("non-null function pointer")(ctx,
                                                                         &mut rset,
                                                                         p_tv,
                                                                         length_to_read);
        if rc == -(1 as libc::c_int) {
            _error_print(ctx,
                         b"select\x00" as *const u8 as *const libc::c_char);
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_LINK as libc::c_int != 0 {
                let mut saved_errno: libc::c_int = *__errno_location();
                if *__errno_location() == 110 as libc::c_int {
                    _sleep_response_timeout(ctx);
                    modbus_flush(ctx);
                } else if *__errno_location() == 9 as libc::c_int {
                    modbus_close(ctx);
                    modbus_connect(ctx);
                }
                *__errno_location() = saved_errno
            }
            return -(1 as libc::c_int)
        }
        rc =
            (*(*ctx).backend).recv.expect("non-null function pointer")(ctx,
                                                                       msg.offset(msg_length
                                                                                      as
                                                                                      isize),
                                                                       length_to_read)
                as libc::c_int;
        if rc == 0 as libc::c_int {
            *__errno_location() = 104 as libc::c_int;
            rc = -(1 as libc::c_int)
        }
        if rc == -(1 as libc::c_int) {
            _error_print(ctx,
                         b"read\x00" as *const u8 as *const libc::c_char);
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_LINK as libc::c_int != 0 &&
                   (*__errno_location() == 104 as libc::c_int ||
                        *__errno_location() == 111 as libc::c_int ||
                        *__errno_location() == 9 as libc::c_int) {
                let mut saved_errno_0: libc::c_int = *__errno_location();
                modbus_close(ctx);
                modbus_connect(ctx);
                /* else timeout isn't set again, the full response must be read before
           expiration of response timeout (for CONFIRMATION only) */
                /* Could be removed by previous calls */
                *__errno_location() = saved_errno_0
            }
            return -(1 as libc::c_int)
        }
        if (*ctx).debug != 0 {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < rc {
                printf(b"<%.2X>\x00" as *const u8 as *const libc::c_char,
                       *msg.offset((msg_length + i) as isize) as libc::c_int);
                i += 1
            }
        }
        msg_length += rc;
        length_to_read -= rc;
        if length_to_read == 0 as libc::c_int {
            let mut current_block_73: u64;
            match step as libc::c_uint {
                0 => {
                    /* Display the hex code of each character received */
                    /* Sums bytes received */
                    /* Computes remaining bytes */
                    /* Function code position */
                    length_to_read =
                        compute_meta_length_after_function(*msg.offset((*(*ctx).backend).header_length
                                                                           as
                                                                           isize)
                                                               as libc::c_int,
                                                           msg_type) as
                            libc::c_int; /* else switches straight to the next step */
                    if length_to_read != 0 as libc::c_int {
                        step = _STEP_META;
                        current_block_73 = 726525485109251713;
                    } else { current_block_73 = 16830319482262729516; }
                }
                1 => { current_block_73 = 16830319482262729516; }
                _ => { current_block_73 = 726525485109251713; }
            }
            match current_block_73 {
                16830319482262729516 => {
                    length_to_read =
                        compute_data_length_after_meta(ctx, msg, msg_type);
                    if msg_length + length_to_read >
                           (*(*ctx).backend).max_adu_length as libc::c_int {
                        *__errno_location() =
                            112345678 as libc::c_int +
                                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int
                                + 2 as libc::c_int;
                        _error_print(ctx,
                                     b"too many data\x00" as *const u8 as
                                         *const libc::c_char);
                        return -(1 as libc::c_int)
                    }
                    step = _STEP_DATA
                }
                _ => { }
            }
        }
        if length_to_read > 0 as libc::c_int &&
               ((*ctx).byte_timeout.tv_sec > 0 as libc::c_int as libc::c_long
                    ||
                    (*ctx).byte_timeout.tv_usec >
                        0 as libc::c_int as libc::c_long) {
            /* If there is no character in the buffer, the allowed timeout
               interval between two consecutive bytes is defined by
               byte_timeout */
            tv.tv_sec = (*ctx).byte_timeout.tv_sec;
            tv.tv_usec = (*ctx).byte_timeout.tv_usec;
            p_tv = &mut tv
        }
    }
    if (*ctx).debug != 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return (*(*ctx).backend).check_integrity.expect("non-null function pointer")(ctx,
                                                                                 msg,
                                                                                 msg_length);
}
/* Receive the request from a modbus master */
#[no_mangle]
pub unsafe extern "C" fn modbus_receive(mut ctx: *mut modbus_t,
                                        mut req: *mut uint8_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*(*ctx).backend).receive.expect("non-null function pointer")(ctx,
                                                                         req);
}
/* Receives the confirmation.

   The function shall store the read response in rsp and return the number of
   values (bits or words). Otherwise, its shall return -1 and errno is set.

   The function doesn't check the confirmation is the expected response to the
   initial request.
*/
#[no_mangle]
pub unsafe extern "C" fn modbus_receive_confirmation(mut ctx: *mut modbus_t,
                                                     mut rsp: *mut uint8_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return _modbus_receive_msg(ctx, rsp, MSG_CONFIRMATION);
}
unsafe extern "C" fn check_confirmation(mut ctx: *mut modbus_t,
                                        mut req: *mut uint8_t,
                                        mut rsp: *mut uint8_t,
                                        mut rsp_length: libc::c_int)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut rsp_length_computed: libc::c_int = 0;
    let offset: libc::c_int = (*(*ctx).backend).header_length as libc::c_int;
    let function: libc::c_int = *rsp.offset(offset as isize) as libc::c_int;
    if (*(*ctx).backend).pre_check_confirmation.is_some() {
        rc =
            (*(*ctx).backend).pre_check_confirmation.expect("non-null function pointer")(ctx,
                                                                                         req,
                                                                                         rsp,
                                                                                         rsp_length);
        if rc == -(1 as libc::c_int) {
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_PROTOCOL as libc::c_int != 0 {
                _sleep_response_timeout(ctx);
                modbus_flush(ctx);
            }
            return -(1 as libc::c_int)
        }
    }
    rsp_length_computed =
        compute_response_length_from_request(ctx, req) as libc::c_int;
    /* Exception code */
    if function >= 0x80 as libc::c_int {
        if rsp_length ==
               offset + 2 as libc::c_int +
                   (*(*ctx).backend).checksum_length as libc::c_int &&
               *req.offset(offset as isize) as libc::c_int ==
                   *rsp.offset(offset as isize) as libc::c_int -
                       0x80 as libc::c_int {
            /* Valid exception code received */
            let mut exception_code: libc::c_int =
                *rsp.offset((offset + 1 as libc::c_int) as isize) as
                    libc::c_int;
            if exception_code < MODBUS_EXCEPTION_MAX as libc::c_int {
                *__errno_location() =
                    112345678 as libc::c_int + exception_code
            } else {
                *__errno_location() =
                    112345678 as libc::c_int +
                        MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                        3 as libc::c_int
            }
            _error_print(ctx, 0 as *const libc::c_char);
            return -(1 as libc::c_int)
        } else {
            *__errno_location() =
                112345678 as libc::c_int +
                    MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                    3 as libc::c_int;
            _error_print(ctx, 0 as *const libc::c_char);
            return -(1 as libc::c_int)
        }
    }
    /* Check length */
    if (rsp_length == rsp_length_computed ||
            rsp_length_computed == -(1 as libc::c_int)) &&
           function < 0x80 as libc::c_int {
        let mut req_nb_value: libc::c_int = 0;
        let mut rsp_nb_value: libc::c_int = 0;
        /* Check function code */
        if function != *req.offset(offset as isize) as libc::c_int {
            if (*ctx).debug != 0 {
                fprintf(stderr,
                        b"Received function not corresponding to the request (0x%X != 0x%X)\n\x00"
                            as *const u8 as *const libc::c_char, function,
                        *req.offset(offset as isize) as libc::c_int);
            }
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_PROTOCOL as libc::c_int != 0 {
                _sleep_response_timeout(ctx);
                modbus_flush(ctx);
            }
            *__errno_location() =
                112345678 as libc::c_int +
                    MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                    2 as libc::c_int;
            return -(1 as libc::c_int)
        }
        /* Check the number of values is corresponding to the request */
        match function {
            1 | 2 => {
                /* Read functions, 8 values in a byte (nb
             * of values in the request and byte count in
             * the response. */
                req_nb_value =
                    ((*req.offset((offset + 3 as libc::c_int) as isize) as
                          libc::c_int) << 8 as libc::c_int) +
                        *req.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int;
                req_nb_value =
                    req_nb_value / 8 as libc::c_int +
                        (if req_nb_value % 8 as libc::c_int != 0 {
                             1 as libc::c_int
                         } else { 0 as libc::c_int });
                rsp_nb_value =
                    *rsp.offset((offset + 1 as libc::c_int) as isize) as
                        libc::c_int
            }
            23 | 3 | 4 => {
                /* Read functions 1 value = 2 bytes */
                req_nb_value =
                    ((*req.offset((offset + 3 as libc::c_int) as isize) as
                          libc::c_int) << 8 as libc::c_int) +
                        *req.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int;
                rsp_nb_value =
                    *rsp.offset((offset + 1 as libc::c_int) as isize) as
                        libc::c_int / 2 as libc::c_int
            }
            15 | 16 => {
                /* N Write functions */
                req_nb_value =
                    ((*req.offset((offset + 3 as libc::c_int) as isize) as
                          libc::c_int) << 8 as libc::c_int) +
                        *req.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int;
                rsp_nb_value =
                    (*rsp.offset((offset + 3 as libc::c_int) as isize) as
                         libc::c_int) << 8 as libc::c_int |
                        *rsp.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int
            }
            17 => {
                /* Report slave ID (bytes received) */
                rsp_nb_value =
                    *rsp.offset((offset + 1 as libc::c_int) as isize) as
                        libc::c_int;
                req_nb_value = rsp_nb_value
            }
            _ => {
                /* 1 Write functions & others */
                rsp_nb_value = 1 as libc::c_int;
                req_nb_value = rsp_nb_value
            }
        }
        if req_nb_value == rsp_nb_value {
            rc = rsp_nb_value
        } else {
            if (*ctx).debug != 0 {
                fprintf(stderr,
                        b"Quantity not corresponding to the request (%d != %d)\n\x00"
                            as *const u8 as *const libc::c_char, rsp_nb_value,
                        req_nb_value);
            }
            if (*ctx).error_recovery &
                   MODBUS_ERROR_RECOVERY_PROTOCOL as libc::c_int != 0 {
                _sleep_response_timeout(ctx);
                modbus_flush(ctx);
            }
            *__errno_location() =
                112345678 as libc::c_int +
                    MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                    2 as libc::c_int;
            rc = -(1 as libc::c_int)
        }
    } else {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Message length not corresponding to the computed length (%d != %d)\n\x00"
                        as *const u8 as *const libc::c_char, rsp_length,
                    rsp_length_computed);
        }
        if (*ctx).error_recovery &
               MODBUS_ERROR_RECOVERY_PROTOCOL as libc::c_int != 0 {
            _sleep_response_timeout(ctx);
            modbus_flush(ctx);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                2 as libc::c_int;
        rc = -(1 as libc::c_int)
    }
    return rc;
}
unsafe extern "C" fn response_io_status(mut tab_io_status: *mut uint8_t,
                                        mut address: libc::c_int,
                                        mut nb: libc::c_int,
                                        mut rsp: *mut uint8_t,
                                        mut offset: libc::c_int)
 -> libc::c_int {
    let mut shift: libc::c_int = 0 as libc::c_int;
    /* Instead of byte (not allowed in Win32) */
    let mut one_byte: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = address;
    while i < address + nb {
        one_byte |=
            (*tab_io_status.offset(i as isize) as libc::c_int) << shift;
        if shift == 7 as libc::c_int {
            /* Byte is full */
            let fresh0 = offset;
            offset = offset + 1;
            *rsp.offset(fresh0 as isize) = one_byte as uint8_t;
            shift = 0 as libc::c_int;
            one_byte = shift
        } else { shift += 1 }
        i += 1
    }
    if shift != 0 as libc::c_int {
        let fresh1 = offset;
        offset = offset + 1;
        *rsp.offset(fresh1 as isize) = one_byte as uint8_t
    }
    return offset;
}
/* Build the exception response */
unsafe extern "C" fn response_exception(mut ctx: *mut modbus_t,
                                        mut sft: *mut sft_t,
                                        mut exception_code: libc::c_int,
                                        mut rsp: *mut uint8_t,
                                        mut to_flush: libc::c_uint,
                                        mut template: *const libc::c_char,
                                        mut args: ...) -> libc::c_int {
    let mut rsp_length: libc::c_int = 0;
    /* Print debug message */
    if (*ctx).debug != 0 {
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vfprintf(stderr, template, ap.as_va_list());
    }
    /* Flush if required */
    if to_flush != 0 { _sleep_response_timeout(ctx); modbus_flush(ctx); }
    /* Build exception response */
    (*sft).function = (*sft).function + 0x80 as libc::c_int;
    rsp_length =
        (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(sft,
                                                                                   rsp);
    let fresh2 = rsp_length;
    rsp_length = rsp_length + 1;
    *rsp.offset(fresh2 as isize) = exception_code as uint8_t;
    return rsp_length;
}
/* Send a response to the received request.
   Analyses the request and constructs a response.

   If an error occurs, this function construct the response
   accordingly.
*/
#[no_mangle]
pub unsafe extern "C" fn modbus_reply(mut ctx: *mut modbus_t,
                                      mut req: *const uint8_t,
                                      mut req_length: libc::c_int,
                                      mut mb_mapping: *mut modbus_mapping_t)
 -> libc::c_int {
    let mut offset: libc::c_int = 0;
    let mut slave: libc::c_int = 0;
    let mut function: libc::c_int = 0;
    let mut address: uint16_t = 0;
    let mut rsp: [uint8_t; 260] = [0; 260];
    let mut rsp_length: libc::c_int = 0 as libc::c_int;
    let mut sft: sft_t = sft_t{slave: 0, function: 0, t_id: 0,};
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    offset = (*(*ctx).backend).header_length as libc::c_int;
    slave = *req.offset((offset - 1 as libc::c_int) as isize) as libc::c_int;
    function = *req.offset(offset as isize) as libc::c_int;
    address =
        (((*req.offset((offset + 1 as libc::c_int) as isize) as libc::c_int)
              << 8 as libc::c_int) +
             *req.offset((offset + 2 as libc::c_int) as isize) as libc::c_int)
            as uint16_t;
    sft.slave = slave;
    sft.function = function;
    sft.t_id =
        (*(*ctx).backend).prepare_response_tid.expect("non-null function pointer")(req,
                                                                                   &mut req_length);
    /* Data are flushed on illegal number of values errors. */
    match function {
        1 | 2 => {
            let mut is_input: libc::c_uint =
                (function == 0x2 as libc::c_int) as libc::c_int as
                    libc::c_uint;
            let mut start_bits: libc::c_int =
                if is_input != 0 {
                    (*mb_mapping).start_input_bits
                } else { (*mb_mapping).start_bits };
            let mut nb_bits: libc::c_int =
                if is_input != 0 {
                    (*mb_mapping).nb_input_bits
                } else { (*mb_mapping).nb_bits };
            let mut tab_bits: *mut uint8_t =
                if is_input != 0 {
                    (*mb_mapping).tab_input_bits
                } else { (*mb_mapping).tab_bits };
            let name: *const libc::c_char =
                if is_input != 0 {
                    b"read_input_bits\x00" as *const u8 as *const libc::c_char
                } else {
                    b"read_bits\x00" as *const u8 as *const libc::c_char
                };
            let mut nb: libc::c_int =
                ((*req.offset((offset + 3 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            /* The mapping can be shifted to reduce memory consumption and it
           doesn't always start at address zero. */
            let mut mapping_address: libc::c_int =
                address as libc::c_int - start_bits;
            if nb < 1 as libc::c_int || (2000 as libc::c_int) < nb {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE as
                                           libc::c_int, rsp.as_mut_ptr(),
                                       1 as libc::c_int as libc::c_uint,
                                       b"Illegal nb of values %d in %s (max %d)\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, nb, name,
                                       2000 as libc::c_int)
            } else if mapping_address < 0 as libc::c_int ||
                          mapping_address + nb > nb_bits {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in %s\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if mapping_address < 0 as libc::c_int {
                                           address as libc::c_int
                                       } else {
                                           (address as libc::c_int) + nb
                                       }, name)
            } else {
                rsp_length =
                    (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                               rsp.as_mut_ptr());
                let fresh3 = rsp_length;
                rsp_length = rsp_length + 1;
                rsp[fresh3 as usize] =
                    (nb / 8 as libc::c_int +
                         (if nb % 8 as libc::c_int != 0 {
                              1 as libc::c_int
                          } else { 0 as libc::c_int })) as uint8_t;
                rsp_length =
                    response_io_status(tab_bits, mapping_address, nb,
                                       rsp.as_mut_ptr(), rsp_length)
            }
        }
        3 | 4 => {
            let mut is_input_0: libc::c_uint =
                (function == 0x4 as libc::c_int) as libc::c_int as
                    libc::c_uint;
            let mut start_registers: libc::c_int =
                if is_input_0 != 0 {
                    (*mb_mapping).start_input_registers
                } else { (*mb_mapping).start_registers };
            let mut nb_registers: libc::c_int =
                if is_input_0 != 0 {
                    (*mb_mapping).nb_input_registers
                } else { (*mb_mapping).nb_registers };
            let mut tab_registers: *mut uint16_t =
                if is_input_0 != 0 {
                    (*mb_mapping).tab_input_registers
                } else { (*mb_mapping).tab_registers };
            let name_0: *const libc::c_char =
                if is_input_0 != 0 {
                    b"read_input_registers\x00" as *const u8 as
                        *const libc::c_char
                } else {
                    b"read_registers\x00" as *const u8 as *const libc::c_char
                };
            let mut nb_0: libc::c_int =
                ((*req.offset((offset + 3 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            /* The mapping can be shifted to reduce memory consumption and it
           doesn't always start at address zero. */
            let mut mapping_address_0: libc::c_int =
                address as libc::c_int - start_registers;
            if nb_0 < 1 as libc::c_int || (125 as libc::c_int) < nb_0 {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE as
                                           libc::c_int, rsp.as_mut_ptr(),
                                       1 as libc::c_int as libc::c_uint,
                                       b"Illegal nb of values %d in %s (max %d)\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, nb_0, name_0,
                                       125 as libc::c_int)
            } else if mapping_address_0 < 0 as libc::c_int ||
                          mapping_address_0 + nb_0 > nb_registers {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in %s\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if mapping_address_0 < 0 as libc::c_int
                                          {
                                           address as libc::c_int
                                       } else {
                                           (address as libc::c_int) + nb_0
                                       }, name_0)
            } else {
                let mut i: libc::c_int = 0;
                rsp_length =
                    (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                               rsp.as_mut_ptr());
                let fresh4 = rsp_length;
                rsp_length = rsp_length + 1;
                rsp[fresh4 as usize] = (nb_0 << 1 as libc::c_int) as uint8_t;
                i = mapping_address_0;
                while i < mapping_address_0 + nb_0 {
                    let fresh5 = rsp_length;
                    rsp_length = rsp_length + 1;
                    rsp[fresh5 as usize] =
                        (*tab_registers.offset(i as isize) as libc::c_int >>
                             8 as libc::c_int) as uint8_t;
                    let fresh6 = rsp_length;
                    rsp_length = rsp_length + 1;
                    rsp[fresh6 as usize] =
                        (*tab_registers.offset(i as isize) as libc::c_int &
                             0xff as libc::c_int) as uint8_t;
                    i += 1
                }
            }
        }
        5 => {
            let mut mapping_address_1: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_bits;
            if mapping_address_1 < 0 as libc::c_int ||
                   mapping_address_1 >= (*mb_mapping).nb_bits {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in write_bit\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       address as libc::c_int)
            } else {
                let mut data: libc::c_int =
                    ((*req.offset((offset + 3 as libc::c_int) as isize) as
                          libc::c_int) << 8 as libc::c_int) +
                        *req.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int;
                if data == 0xff00 as libc::c_int || data == 0 as libc::c_int {
                    *(*mb_mapping).tab_bits.offset(mapping_address_1 as isize)
                        =
                        if data != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int } as uint8_t;
                    memcpy(rsp.as_mut_ptr() as *mut libc::c_void,
                           req as *const libc::c_void,
                           req_length as libc::c_ulong);
                    rsp_length = req_length
                } else {
                    rsp_length =
                        response_exception(ctx, &mut sft as *mut sft_t,
                                           MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE
                                               as libc::c_int,
                                           rsp.as_mut_ptr(),
                                           0 as libc::c_int as libc::c_uint,
                                           b"Illegal data value 0x%0X in write_bit request at address %0X\n\x00"
                                               as *const u8 as
                                               *const libc::c_char, data,
                                           address as libc::c_int)
                }
            }
        }
        6 => {
            let mut mapping_address_2: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_registers;
            if mapping_address_2 < 0 as libc::c_int ||
                   mapping_address_2 >= (*mb_mapping).nb_registers {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in write_register\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       address as libc::c_int)
            } else {
                let mut data_0: libc::c_int =
                    ((*req.offset((offset + 3 as libc::c_int) as isize) as
                          libc::c_int) << 8 as libc::c_int) +
                        *req.offset((offset + 4 as libc::c_int) as isize) as
                            libc::c_int;
                *(*mb_mapping).tab_registers.offset(mapping_address_2 as
                                                        isize) =
                    data_0 as uint16_t;
                memcpy(rsp.as_mut_ptr() as *mut libc::c_void,
                       req as *const libc::c_void,
                       req_length as libc::c_ulong);
                rsp_length = req_length
            }
        }
        15 => {
            let mut nb_1: libc::c_int =
                ((*req.offset((offset + 3 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            let mut nb_bits_0: libc::c_int =
                *req.offset((offset + 5 as libc::c_int) as isize) as
                    libc::c_int;
            let mut mapping_address_3: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_bits;
            if nb_1 < 1 as libc::c_int || (1968 as libc::c_int) < nb_1 ||
                   (nb_bits_0 * 8 as libc::c_int) < nb_1 {
                /* May be the indication has been truncated on reading because of
             * invalid address (eg. nb is 0 but the request contains values to
             * write) so it's necessary to flush. */
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE as
                                           libc::c_int, rsp.as_mut_ptr(),
                                       1 as libc::c_int as libc::c_uint,
                                       b"Illegal number of values %d in write_bits (max %d)\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, nb_1,
                                       1968 as libc::c_int)
            } else if mapping_address_3 < 0 as libc::c_int ||
                          mapping_address_3 + nb_1 > (*mb_mapping).nb_bits {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in write_bits\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if mapping_address_3 < 0 as libc::c_int
                                          {
                                           address as libc::c_int
                                       } else {
                                           (address as libc::c_int) + nb_1
                                       })
            } else {
                /* 6 = byte count */
                modbus_set_bits_from_bytes((*mb_mapping).tab_bits,
                                           mapping_address_3,
                                           nb_1 as libc::c_uint,
                                           &*req.offset((offset +
                                                             6 as libc::c_int)
                                                            as isize));
                rsp_length =
                    (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                               rsp.as_mut_ptr());
                /* 4 to copy the bit address (2) and the quantity of bits */
                memcpy(rsp.as_mut_ptr().offset(rsp_length as isize) as
                           *mut libc::c_void,
                       req.offset(rsp_length as isize) as *const libc::c_void,
                       4 as libc::c_int as libc::c_ulong);
                rsp_length += 4 as libc::c_int
            }
        }
        16 => {
            let mut nb_2: libc::c_int =
                ((*req.offset((offset + 3 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            let mut nb_bytes: libc::c_int =
                *req.offset((offset + 5 as libc::c_int) as isize) as
                    libc::c_int;
            let mut mapping_address_4: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_registers;
            if nb_2 < 1 as libc::c_int || (123 as libc::c_int) < nb_2 ||
                   nb_bytes != nb_2 * 2 as libc::c_int {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE as
                                           libc::c_int, rsp.as_mut_ptr(),
                                       1 as libc::c_int as libc::c_uint,
                                       b"Illegal number of values %d in write_registers (max %d)\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, nb_2,
                                       123 as libc::c_int)
            } else if mapping_address_4 < 0 as libc::c_int ||
                          mapping_address_4 + nb_2 >
                              (*mb_mapping).nb_registers {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in write_registers\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if mapping_address_4 < 0 as libc::c_int
                                          {
                                           address as libc::c_int
                                       } else {
                                           (address as libc::c_int) + nb_2
                                       })
            } else {
                let mut i_0: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                i_0 = mapping_address_4;
                j = 6 as libc::c_int;
                while i_0 < mapping_address_4 + nb_2 {
                    /* 6 and 7 = first value */
                    *(*mb_mapping).tab_registers.offset(i_0 as isize) =
                        (((*req.offset((offset + j) as isize) as libc::c_int)
                              << 8 as libc::c_int) +
                             *req.offset((offset + j + 1 as libc::c_int) as
                                             isize) as libc::c_int) as
                            uint16_t;
                    i_0 += 1;
                    j += 2 as libc::c_int
                }
                rsp_length =
                    (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                               rsp.as_mut_ptr());
                /* 4 to copy the address (2) and the no. of registers */
                memcpy(rsp.as_mut_ptr().offset(rsp_length as isize) as
                           *mut libc::c_void,
                       req.offset(rsp_length as isize) as *const libc::c_void,
                       4 as libc::c_int as libc::c_ulong);
                rsp_length += 4 as libc::c_int
            }
        }
        17 => {
            let mut str_len: libc::c_int = 0;
            let mut byte_count_pos: libc::c_int = 0;
            rsp_length =
                (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                           rsp.as_mut_ptr());
            /* Skip byte count for now */
            let fresh7 = rsp_length;
            rsp_length = rsp_length + 1;
            byte_count_pos = fresh7;
            let fresh8 = rsp_length;
            rsp_length = rsp_length + 1;
            rsp[fresh8 as usize] = 180 as libc::c_int as uint8_t;
            /* Run indicator status to ON */
            let fresh9 = rsp_length;
            rsp_length = rsp_length + 1;
            rsp[fresh9 as usize] = 0xff as libc::c_int as uint8_t;
            /* LMB + length of LIBMODBUS_VERSION_STRING */
            str_len =
                (3 as libc::c_int as
                     libc::c_ulong).wrapping_add(strlen(b"3.1.7\x00" as
                                                            *const u8 as
                                                            *const libc::c_char))
                    as libc::c_int;
            memcpy(rsp.as_mut_ptr().offset(rsp_length as isize) as
                       *mut libc::c_void,
                   b"LMB3.1.7\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void, str_len as libc::c_ulong);
            rsp_length += str_len;
            rsp[byte_count_pos as usize] =
                (rsp_length - byte_count_pos - 1 as libc::c_int) as uint8_t
        }
        7 => {
            if (*ctx).debug != 0 {
                fprintf(stderr,
                        b"FIXME Not implemented\n\x00" as *const u8 as
                            *const libc::c_char);
            }
            *__errno_location() = 92 as libc::c_int;
            return -(1 as libc::c_int)
        }
        22 => {
            let mut mapping_address_5: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_registers;
            if mapping_address_5 < 0 as libc::c_int ||
                   mapping_address_5 >= (*mb_mapping).nb_registers {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data address 0x%0X in write_register\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       address as libc::c_int)
            } else {
                let mut data_1: uint16_t =
                    *(*mb_mapping).tab_registers.offset(mapping_address_5 as
                                                            isize);
                let mut and: uint16_t =
                    (((*req.offset((offset + 3 as libc::c_int) as isize) as
                           libc::c_int) << 8 as libc::c_int) +
                         *req.offset((offset + 4 as libc::c_int) as isize) as
                             libc::c_int) as uint16_t;
                let mut or: uint16_t =
                    (((*req.offset((offset + 5 as libc::c_int) as isize) as
                           libc::c_int) << 8 as libc::c_int) +
                         *req.offset((offset + 6 as libc::c_int) as isize) as
                             libc::c_int) as uint16_t;
                data_1 =
                    (data_1 as libc::c_int & and as libc::c_int |
                         or as libc::c_int & !(and as libc::c_int)) as
                        uint16_t;
                *(*mb_mapping).tab_registers.offset(mapping_address_5 as
                                                        isize) = data_1;
                memcpy(rsp.as_mut_ptr() as *mut libc::c_void,
                       req as *const libc::c_void,
                       req_length as libc::c_ulong);
                rsp_length = req_length
            }
        }
        23 => {
            let mut nb_3: libc::c_int =
                ((*req.offset((offset + 3 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 4 as libc::c_int) as isize) as
                        libc::c_int;
            let mut address_write: uint16_t =
                (((*req.offset((offset + 5 as libc::c_int) as isize) as
                       libc::c_int) << 8 as libc::c_int) +
                     *req.offset((offset + 6 as libc::c_int) as isize) as
                         libc::c_int) as uint16_t;
            let mut nb_write: libc::c_int =
                ((*req.offset((offset + 7 as libc::c_int) as isize) as
                      libc::c_int) << 8 as libc::c_int) +
                    *req.offset((offset + 8 as libc::c_int) as isize) as
                        libc::c_int;
            let mut nb_write_bytes: libc::c_int =
                *req.offset((offset + 9 as libc::c_int) as isize) as
                    libc::c_int;
            let mut mapping_address_6: libc::c_int =
                address as libc::c_int - (*mb_mapping).start_registers;
            let mut mapping_address_write: libc::c_int =
                address_write as libc::c_int - (*mb_mapping).start_registers;
            if nb_write < 1 as libc::c_int || (121 as libc::c_int) < nb_write
                   || nb_3 < 1 as libc::c_int || (125 as libc::c_int) < nb_3
                   || nb_write_bytes != nb_write * 2 as libc::c_int {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE as
                                           libc::c_int, rsp.as_mut_ptr(),
                                       1 as libc::c_int as libc::c_uint,
                                       b"Illegal nb of values (W%d, R%d) in write_and_read_registers (max W%d, R%d)\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, nb_write,
                                       nb_3, 121 as libc::c_int,
                                       125 as libc::c_int)
            } else if mapping_address_6 < 0 as libc::c_int ||
                          mapping_address_6 + nb_3 >
                              (*mb_mapping).nb_registers ||
                          mapping_address_write < 0 as libc::c_int ||
                          mapping_address_write + nb_write >
                              (*mb_mapping).nb_registers {
                rsp_length =
                    response_exception(ctx, &mut sft as *mut sft_t,
                                       MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS
                                           as libc::c_int, rsp.as_mut_ptr(),
                                       0 as libc::c_int as libc::c_uint,
                                       b"Illegal data read address 0x%0X or write address 0x%0X write_and_read_registers\n\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       if mapping_address_6 < 0 as libc::c_int
                                          {
                                           address as libc::c_int
                                       } else {
                                           (address as libc::c_int) + nb_3
                                       },
                                       if mapping_address_write <
                                              0 as libc::c_int {
                                           address_write as libc::c_int
                                       } else {
                                           (address_write as libc::c_int) +
                                               nb_write
                                       })
            } else {
                let mut i_1: libc::c_int = 0;
                let mut j_0: libc::c_int = 0;
                rsp_length =
                    (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                               rsp.as_mut_ptr());
                let fresh10 = rsp_length;
                rsp_length = rsp_length + 1;
                rsp[fresh10 as usize] = (nb_3 << 1 as libc::c_int) as uint8_t;
                /* Write first.
               10 and 11 are the offset of the first values to write */
                i_1 = mapping_address_write;
                j_0 = 10 as libc::c_int;
                while i_1 < mapping_address_write + nb_write {
                    *(*mb_mapping).tab_registers.offset(i_1 as isize) =
                        (((*req.offset((offset + j_0) as isize) as
                               libc::c_int) << 8 as libc::c_int) +
                             *req.offset((offset + j_0 + 1 as libc::c_int) as
                                             isize) as libc::c_int) as
                            uint16_t;
                    i_1 += 1;
                    j_0 += 2 as libc::c_int
                }
                /* and read the data for the response */
                i_1 = mapping_address_6;
                while i_1 < mapping_address_6 + nb_3 {
                    let fresh11 = rsp_length;
                    rsp_length = rsp_length + 1;
                    rsp[fresh11 as usize] =
                        (*(*mb_mapping).tab_registers.offset(i_1 as isize) as
                             libc::c_int >> 8 as libc::c_int) as uint8_t;
                    let fresh12 = rsp_length;
                    rsp_length = rsp_length + 1;
                    rsp[fresh12 as usize] =
                        (*(*mb_mapping).tab_registers.offset(i_1 as isize) as
                             libc::c_int & 0xff as libc::c_int) as uint8_t;
                    i_1 += 1
                }
            }
        }
        _ => {
            rsp_length =
                response_exception(ctx, &mut sft as *mut sft_t,
                                   MODBUS_EXCEPTION_ILLEGAL_FUNCTION as
                                       libc::c_int, rsp.as_mut_ptr(),
                                   1 as libc::c_int as libc::c_uint,
                                   b"Unknown Modbus function code: 0x%0X\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   function)
        }
    }
    /* Suppress any responses when the request was a broadcast */
    return if (*(*ctx).backend).backend_type ==
                  _MODBUS_BACKEND_TYPE_RTU as libc::c_int as libc::c_uint &&
                  slave == 0 as libc::c_int {
               0 as libc::c_int
           } else { send_msg(ctx, rsp.as_mut_ptr(), rsp_length) };
}
#[no_mangle]
pub unsafe extern "C" fn modbus_reply_exception(mut ctx: *mut modbus_t,
                                                mut req: *const uint8_t,
                                                mut exception_code:
                                                    libc::c_uint)
 -> libc::c_int {
    let mut offset: libc::c_int = 0;
    let mut slave: libc::c_int = 0;
    let mut function: libc::c_int = 0;
    let mut rsp: [uint8_t; 260] = [0; 260];
    let mut rsp_length: libc::c_int = 0;
    let mut dummy_length: libc::c_int = 99 as libc::c_int;
    let mut sft: sft_t = sft_t{slave: 0, function: 0, t_id: 0,};
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    offset = (*(*ctx).backend).header_length as libc::c_int;
    slave = *req.offset((offset - 1 as libc::c_int) as isize) as libc::c_int;
    function = *req.offset(offset as isize) as libc::c_int;
    sft.slave = slave;
    sft.function = function + 0x80 as libc::c_int;
    sft.t_id =
        (*(*ctx).backend).prepare_response_tid.expect("non-null function pointer")(req,
                                                                                   &mut dummy_length);
    rsp_length =
        (*(*ctx).backend).build_response_basis.expect("non-null function pointer")(&mut sft,
                                                                                   rsp.as_mut_ptr());
    /* Positive exception code */
    if exception_code < MODBUS_EXCEPTION_MAX as libc::c_int as libc::c_uint {
        let fresh13 = rsp_length;
        rsp_length = rsp_length + 1;
        rsp[fresh13 as usize] = exception_code as uint8_t;
        return send_msg(ctx, rsp.as_mut_ptr(), rsp_length)
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    };
}
/* Reads IO status */
unsafe extern "C" fn read_io_status(mut ctx: *mut modbus_t,
                                    mut function: libc::c_int,
                                    mut addr: libc::c_int,
                                    mut nb: libc::c_int,
                                    mut dest: *mut uint8_t) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut req: [uint8_t; 12] = [0; 12];
    let mut rsp: [uint8_t; 260] = [0; 260];
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  function,
                                                                                  addr,
                                                                                  nb,
                                                                                  req.as_mut_ptr());
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut temp: libc::c_int = 0;
        let mut bit: libc::c_int = 0;
        let mut pos: libc::c_int = 0 as libc::c_int;
        let mut offset: libc::c_int = 0;
        let mut offset_end: libc::c_int = 0;
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        offset =
            (*(*ctx).backend).header_length.wrapping_add(2 as libc::c_int as
                                                             libc::c_uint) as
                libc::c_int;
        offset_end = offset + rc;
        i = offset;
        while i < offset_end {
            /* Shift reg hi_byte to temp */
            temp = rsp[i as usize] as libc::c_int;
            bit = 0x1 as libc::c_int;
            while bit & 0xff as libc::c_int != 0 && pos < nb {
                let fresh14 = pos;
                pos = pos + 1;
                *dest.offset(fresh14 as isize) =
                    if temp & bit != 0 {
                        1 as libc::c_int
                    } else { 0 as libc::c_int } as uint8_t;
                bit = bit << 1 as libc::c_int
            }
            i += 1
        }
    }
    return rc;
}
/* Reads the boolean status of bits and sets the array elements
   in the destination to TRUE or FALSE (single bits). */
#[no_mangle]
pub unsafe extern "C" fn modbus_read_bits(mut ctx: *mut modbus_t,
                                          mut addr: libc::c_int,
                                          mut nb: libc::c_int,
                                          mut dest: *mut uint8_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 2000 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many bits requested (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, nb,
                    2000 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    rc = read_io_status(ctx, 0x1 as libc::c_int, addr, nb, dest);
    if rc == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else { return nb };
}
/* Same as modbus_read_bits but reads the remote device input table */
#[no_mangle]
pub unsafe extern "C" fn modbus_read_input_bits(mut ctx: *mut modbus_t,
                                                mut addr: libc::c_int,
                                                mut nb: libc::c_int,
                                                mut dest: *mut uint8_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 2000 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many discrete inputs requested (%d > %d)\n\x00"
                        as *const u8 as *const libc::c_char, nb,
                    2000 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    rc = read_io_status(ctx, 0x2 as libc::c_int, addr, nb, dest);
    if rc == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else { return nb };
}
/* Reads the data from a remote device and put that data into an array */
unsafe extern "C" fn read_registers(mut ctx: *mut modbus_t,
                                    mut function: libc::c_int,
                                    mut addr: libc::c_int,
                                    mut nb: libc::c_int,
                                    mut dest: *mut uint16_t) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut req: [uint8_t; 12] = [0; 12];
    let mut rsp: [uint8_t; 260] = [0; 260];
    if nb > 125 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many registers requested (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, nb,
                    125 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  function,
                                                                                  addr,
                                                                                  nb,
                                                                                  req.as_mut_ptr());
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut offset: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        offset = (*(*ctx).backend).header_length as libc::c_int;
        i = 0 as libc::c_int;
        while i < rc {
            /* shift reg hi_byte to temp OR with lo_byte */
            *dest.offset(i as isize) =
                ((rsp[(offset + 2 as libc::c_int + (i << 1 as libc::c_int)) as
                          usize] as libc::c_int) << 8 as libc::c_int |
                     rsp[(offset + 3 as libc::c_int + (i << 1 as libc::c_int))
                             as usize] as libc::c_int) as uint16_t;
            i += 1
        }
    }
    return rc;
}
/* Reads the holding registers of remote device and put the data into an
   array */
#[no_mangle]
pub unsafe extern "C" fn modbus_read_registers(mut ctx: *mut modbus_t,
                                               mut addr: libc::c_int,
                                               mut nb: libc::c_int,
                                               mut dest: *mut uint16_t)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 125 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many registers requested (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, nb,
                    125 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    status = read_registers(ctx, 0x3 as libc::c_int, addr, nb, dest);
    return status;
}
/* Reads the input registers of remote device and put the data into an array */
#[no_mangle]
pub unsafe extern "C" fn modbus_read_input_registers(mut ctx: *mut modbus_t,
                                                     mut addr: libc::c_int,
                                                     mut nb: libc::c_int,
                                                     mut dest: *mut uint16_t)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 125 as libc::c_int {
        fprintf(stderr,
                b"ERROR Too many input registers requested (%d > %d)\n\x00" as
                    *const u8 as *const libc::c_char, nb, 125 as libc::c_int);
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    status = read_registers(ctx, 0x4 as libc::c_int, addr, nb, dest);
    return status;
}
/* Write a value to the specified register of the remote device.
   Used by write_bit and write_register */
unsafe extern "C" fn write_single(mut ctx: *mut modbus_t,
                                  mut function: libc::c_int,
                                  mut addr: libc::c_int, value: uint16_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut req: [uint8_t; 12] = [0; 12];
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  function,
                                                                                  addr,
                                                                                  value
                                                                                      as
                                                                                      libc::c_int,
                                                                                  req.as_mut_ptr());
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        /* Used by write_bit and write_register */
        let mut rsp: [uint8_t; 260] = [0; 260];
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc)
    }
    return rc;
}
/* Turns ON or OFF a single bit of the remote device */
#[no_mangle]
pub unsafe extern "C" fn modbus_write_bit(mut ctx: *mut modbus_t,
                                          mut addr: libc::c_int,
                                          mut status: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return write_single(ctx, 0x5 as libc::c_int, addr,
                        if status != 0 {
                            0xff00 as libc::c_int
                        } else { 0 as libc::c_int } as uint16_t);
}
/* Writes a value in one register of the remote device */
#[no_mangle]
pub unsafe extern "C" fn modbus_write_register(mut ctx: *mut modbus_t,
                                               mut addr: libc::c_int,
                                               value: uint16_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return write_single(ctx, 0x6 as libc::c_int, addr, value);
}
/* Write the bits of the array in the remote device */
#[no_mangle]
pub unsafe extern "C" fn modbus_write_bits(mut ctx: *mut modbus_t,
                                           mut addr: libc::c_int,
                                           mut nb: libc::c_int,
                                           mut src: *const uint8_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut byte_count: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut bit_check: libc::c_int = 0 as libc::c_int;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut req: [uint8_t; 260] = [0; 260];
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 1968 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Writing too many bits (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, nb,
                    1968 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  0xf
                                                                                      as
                                                                                      libc::c_int,
                                                                                  addr,
                                                                                  nb,
                                                                                  req.as_mut_ptr());
    byte_count =
        nb / 8 as libc::c_int +
            (if nb % 8 as libc::c_int != 0 {
                 1 as libc::c_int
             } else { 0 as libc::c_int });
    let fresh15 = req_length;
    req_length = req_length + 1;
    req[fresh15 as usize] = byte_count as uint8_t;
    i = 0 as libc::c_int;
    while i < byte_count {
        let mut bit: libc::c_int = 0;
        bit = 0x1 as libc::c_int;
        req[req_length as usize] = 0 as libc::c_int as uint8_t;
        while bit & 0xff as libc::c_int != 0 &&
                  {
                      let fresh16 = bit_check;
                      bit_check = bit_check + 1;
                      (fresh16) < nb
                  } {
            let fresh17 = pos;
            pos = pos + 1;
            if *src.offset(fresh17 as isize) != 0 {
                req[req_length as usize] =
                    (req[req_length as usize] as libc::c_int | bit) as uint8_t
            } else {
                req[req_length as usize] =
                    (req[req_length as usize] as libc::c_int & !bit) as
                        uint8_t
            }
            bit = bit << 1 as libc::c_int
        }
        req_length += 1;
        i += 1
    }
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut rsp: [uint8_t; 260] = [0; 260];
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc)
    }
    return rc;
}
/* Write the values from the array to the registers of the remote device */
#[no_mangle]
pub unsafe extern "C" fn modbus_write_registers(mut ctx: *mut modbus_t,
                                                mut addr: libc::c_int,
                                                mut nb: libc::c_int,
                                                mut src: *const uint16_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut byte_count: libc::c_int = 0;
    let mut req: [uint8_t; 260] = [0; 260];
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if nb > 123 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Trying to write to too many registers (%d > %d)\n\x00"
                        as *const u8 as *const libc::c_char, nb,
                    123 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  0x10
                                                                                      as
                                                                                      libc::c_int,
                                                                                  addr,
                                                                                  nb,
                                                                                  req.as_mut_ptr());
    byte_count = nb * 2 as libc::c_int;
    let fresh18 = req_length;
    req_length = req_length + 1;
    req[fresh18 as usize] = byte_count as uint8_t;
    i = 0 as libc::c_int;
    while i < nb {
        let fresh19 = req_length;
        req_length = req_length + 1;
        req[fresh19 as usize] =
            (*src.offset(i as isize) as libc::c_int >> 8 as libc::c_int) as
                uint8_t;
        let fresh20 = req_length;
        req_length = req_length + 1;
        req[fresh20 as usize] =
            (*src.offset(i as isize) as libc::c_int & 0xff as libc::c_int) as
                uint8_t;
        i += 1
    }
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut rsp: [uint8_t; 260] = [0; 260];
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc)
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_mask_write_register(mut ctx: *mut modbus_t,
                                                    mut addr: libc::c_int,
                                                    mut and_mask: uint16_t,
                                                    mut or_mask: uint16_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    /* The request length can not exceed _MIN_REQ_LENGTH - 2 and 4 bytes to
     * store the masks. The ugly subtraction is there to remove the 'nb' value
     * (2 bytes) which is not used. */
    let mut req: [uint8_t; 14] = [0; 14];
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  0x16
                                                                                      as
                                                                                      libc::c_int,
                                                                                  addr,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int,
                                                                                  req.as_mut_ptr());
    /* HACKISH, count is not used */
    req_length -= 2 as libc::c_int;
    let fresh21 = req_length;
    req_length = req_length + 1;
    req[fresh21 as usize] =
        (and_mask as libc::c_int >> 8 as libc::c_int) as uint8_t;
    let fresh22 = req_length;
    req_length = req_length + 1;
    req[fresh22 as usize] =
        (and_mask as libc::c_int & 0xff as libc::c_int) as uint8_t;
    let fresh23 = req_length;
    req_length = req_length + 1;
    req[fresh23 as usize] =
        (or_mask as libc::c_int >> 8 as libc::c_int) as uint8_t;
    let fresh24 = req_length;
    req_length = req_length + 1;
    req[fresh24 as usize] =
        (or_mask as libc::c_int & 0xff as libc::c_int) as uint8_t;
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        /* Used by write_bit and write_register */
        let mut rsp: [uint8_t; 260] = [0; 260];
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc)
    }
    return rc;
}
/* Write multiple registers from src array to remote device and read multiple
   registers from remote device to dest array. */
#[no_mangle]
pub unsafe extern "C" fn modbus_write_and_read_registers(mut ctx:
                                                             *mut modbus_t,
                                                         mut write_addr:
                                                             libc::c_int,
                                                         mut write_nb:
                                                             libc::c_int,
                                                         mut src:
                                                             *const uint16_t,
                                                         mut read_addr:
                                                             libc::c_int,
                                                         mut read_nb:
                                                             libc::c_int,
                                                         mut dest:
                                                             *mut uint16_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut byte_count: libc::c_int = 0;
    let mut req: [uint8_t; 260] = [0; 260];
    let mut rsp: [uint8_t; 260] = [0; 260];
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if write_nb > 121 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many registers to write (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, write_nb,
                    121 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if read_nb > 125 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"ERROR Too many registers requested (%d > %d)\n\x00" as
                        *const u8 as *const libc::c_char, read_nb,
                    125 as libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                5 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  0x17
                                                                                      as
                                                                                      libc::c_int,
                                                                                  read_addr,
                                                                                  read_nb,
                                                                                  req.as_mut_ptr());
    let fresh25 = req_length;
    req_length = req_length + 1;
    req[fresh25 as usize] = (write_addr >> 8 as libc::c_int) as uint8_t;
    let fresh26 = req_length;
    req_length = req_length + 1;
    req[fresh26 as usize] = (write_addr & 0xff as libc::c_int) as uint8_t;
    let fresh27 = req_length;
    req_length = req_length + 1;
    req[fresh27 as usize] = (write_nb >> 8 as libc::c_int) as uint8_t;
    let fresh28 = req_length;
    req_length = req_length + 1;
    req[fresh28 as usize] = (write_nb & 0xff as libc::c_int) as uint8_t;
    byte_count = write_nb * 2 as libc::c_int;
    let fresh29 = req_length;
    req_length = req_length + 1;
    req[fresh29 as usize] = byte_count as uint8_t;
    i = 0 as libc::c_int;
    while i < write_nb {
        let fresh30 = req_length;
        req_length = req_length + 1;
        req[fresh30 as usize] =
            (*src.offset(i as isize) as libc::c_int >> 8 as libc::c_int) as
                uint8_t;
        let fresh31 = req_length;
        req_length = req_length + 1;
        req[fresh31 as usize] =
            (*src.offset(i as isize) as libc::c_int & 0xff as libc::c_int) as
                uint8_t;
        i += 1
    }
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut offset: libc::c_int = 0;
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        offset = (*(*ctx).backend).header_length as libc::c_int;
        i = 0 as libc::c_int;
        while i < rc {
            /* shift reg hi_byte to temp OR with lo_byte */
            *dest.offset(i as isize) =
                ((rsp[(offset + 2 as libc::c_int + (i << 1 as libc::c_int)) as
                          usize] as libc::c_int) << 8 as libc::c_int |
                     rsp[(offset + 3 as libc::c_int + (i << 1 as libc::c_int))
                             as usize] as libc::c_int) as uint16_t;
            i += 1
        }
    }
    return rc;
}
/* Send a request to get the slave ID of the device (only available in serial
   communication). */
#[no_mangle]
pub unsafe extern "C" fn modbus_report_slave_id(mut ctx: *mut modbus_t,
                                                mut max_dest: libc::c_int,
                                                mut dest: *mut uint8_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut req_length: libc::c_int = 0;
    let mut req: [uint8_t; 12] = [0; 12];
    if ctx.is_null() || max_dest <= 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_length =
        (*(*ctx).backend).build_request_basis.expect("non-null function pointer")(ctx,
                                                                                  0x11
                                                                                      as
                                                                                      libc::c_int,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int,
                                                                                  req.as_mut_ptr());
    /* HACKISH, addr and count are not used */
    req_length -= 4 as libc::c_int;
    rc = send_msg(ctx, req.as_mut_ptr(), req_length);
    if rc > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut offset: libc::c_int = 0;
        let mut rsp: [uint8_t; 260] = [0; 260];
        rc = _modbus_receive_msg(ctx, rsp.as_mut_ptr(), MSG_CONFIRMATION);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        rc = check_confirmation(ctx, req.as_mut_ptr(), rsp.as_mut_ptr(), rc);
        if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        offset =
            (*(*ctx).backend).header_length.wrapping_add(2 as libc::c_int as
                                                             libc::c_uint) as
                libc::c_int;
        /* Byte count, slave id, run indicator status and
           additional data. Truncate copy to max_dest. */
        i = 0 as libc::c_int;
        while i < rc && i < max_dest {
            *dest.offset(i as isize) = rsp[(offset + i) as usize];
            i += 1
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn _modbus_init_common(mut ctx: *mut modbus_t) {
    /* Slave and socket are initialized to -1 */
    (*ctx).slave = -(1 as libc::c_int);
    (*ctx).s = -(1 as libc::c_int);
    (*ctx).debug = 0 as libc::c_int;
    (*ctx).error_recovery = MODBUS_ERROR_RECOVERY_NONE as libc::c_int;
    (*ctx).response_timeout.tv_sec = 0 as libc::c_int as __time_t;
    (*ctx).response_timeout.tv_usec = 500000 as libc::c_int as __suseconds_t;
    (*ctx).byte_timeout.tv_sec = 0 as libc::c_int as __time_t;
    (*ctx).byte_timeout.tv_usec = 500000 as libc::c_int as __suseconds_t;
    (*ctx).indication_timeout.tv_sec = 0 as libc::c_int as __time_t;
    (*ctx).indication_timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
}
/* Define the slave number */
#[no_mangle]
pub unsafe extern "C" fn modbus_set_slave(mut ctx: *mut modbus_t,
                                          mut slave: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*(*ctx).backend).set_slave.expect("non-null function pointer")(ctx,
                                                                           slave);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_get_slave(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*ctx).slave;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_error_recovery(mut ctx: *mut modbus_t,
                                                   mut error_recovery:
                                                       modbus_error_recovery_mode)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    /* The type of modbus_error_recovery_mode is unsigned enum */
    (*ctx).error_recovery = error_recovery as uint8_t as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_socket(mut ctx: *mut modbus_t,
                                           mut s: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*ctx).s = s;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_get_socket(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*ctx).s;
}
/* Get the timeout interval used to wait for a response */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_response_timeout(mut ctx: *mut modbus_t,
                                                     mut to_sec:
                                                         *mut uint32_t,
                                                     mut to_usec:
                                                         *mut uint32_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    *to_sec = (*ctx).response_timeout.tv_sec as uint32_t;
    *to_usec = (*ctx).response_timeout.tv_usec as uint32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_response_timeout(mut ctx: *mut modbus_t,
                                                     mut to_sec: uint32_t,
                                                     mut to_usec: uint32_t)
 -> libc::c_int {
    if ctx.is_null() ||
           to_sec == 0 as libc::c_int as libc::c_uint &&
               to_usec == 0 as libc::c_int as libc::c_uint ||
           to_usec > 999999 as libc::c_int as libc::c_uint {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*ctx).response_timeout.tv_sec = to_sec as __time_t;
    (*ctx).response_timeout.tv_usec = to_usec as __suseconds_t;
    return 0 as libc::c_int;
}
/* Get the timeout interval between two consecutive bytes of a message */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_byte_timeout(mut ctx: *mut modbus_t,
                                                 mut to_sec: *mut uint32_t,
                                                 mut to_usec: *mut uint32_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    *to_sec = (*ctx).byte_timeout.tv_sec as uint32_t;
    *to_usec = (*ctx).byte_timeout.tv_usec as uint32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_byte_timeout(mut ctx: *mut modbus_t,
                                                 mut to_sec: uint32_t,
                                                 mut to_usec: uint32_t)
 -> libc::c_int {
    /* Byte timeout can be disabled when both values are zero */
    if ctx.is_null() || to_usec > 999999 as libc::c_int as libc::c_uint {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*ctx).byte_timeout.tv_sec = to_sec as __time_t;
    (*ctx).byte_timeout.tv_usec = to_usec as __suseconds_t;
    return 0 as libc::c_int;
}
/* Get the timeout interval used by the server to wait for an indication from a client */
#[no_mangle]
pub unsafe extern "C" fn modbus_get_indication_timeout(mut ctx: *mut modbus_t,
                                                       mut to_sec:
                                                           *mut uint32_t,
                                                       mut to_usec:
                                                           *mut uint32_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    *to_sec = (*ctx).indication_timeout.tv_sec as uint32_t;
    *to_usec = (*ctx).indication_timeout.tv_usec as uint32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_indication_timeout(mut ctx: *mut modbus_t,
                                                       mut to_sec: uint32_t,
                                                       mut to_usec: uint32_t)
 -> libc::c_int {
    /* Indication timeout can be disabled when both values are zero */
    if ctx.is_null() || to_usec > 999999 as libc::c_int as libc::c_uint {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*ctx).indication_timeout.tv_sec = to_sec as __time_t;
    (*ctx).indication_timeout.tv_usec = to_usec as __suseconds_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_get_header_length(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*(*ctx).backend).header_length as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_connect(mut ctx: *mut modbus_t)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return (*(*ctx).backend).connect.expect("non-null function pointer")(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_close(mut ctx: *mut modbus_t) {
    if ctx.is_null() { return }
    (*(*ctx).backend).close.expect("non-null function pointer")(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_free(mut ctx: *mut modbus_t) {
    if ctx.is_null() { return }
    (*(*ctx).backend).free.expect("non-null function pointer")(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn modbus_set_debug(mut ctx: *mut modbus_t,
                                          mut flag: libc::c_int)
 -> libc::c_int {
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    (*ctx).debug = flag;
    return 0 as libc::c_int;
}
/* Allocates 4 arrays to store bits, input bits, registers and inputs
   registers. The pointers are stored in modbus_mapping structure.

   The modbus_mapping_new_start_address() function shall return the new allocated
   structure if successful. Otherwise it shall return NULL and set errno to
   ENOMEM. */
#[no_mangle]
pub unsafe extern "C" fn modbus_mapping_new_start_address(mut start_bits:
                                                              libc::c_uint,
                                                          mut nb_bits:
                                                              libc::c_uint,
                                                          mut start_input_bits:
                                                              libc::c_uint,
                                                          mut nb_input_bits:
                                                              libc::c_uint,
                                                          mut start_registers:
                                                              libc::c_uint,
                                                          mut nb_registers:
                                                              libc::c_uint,
                                                          mut start_input_registers:
                                                              libc::c_uint,
                                                          mut nb_input_registers:
                                                              libc::c_uint)
 -> *mut modbus_mapping_t {
    let mut mb_mapping: *mut modbus_mapping_t = 0 as *mut modbus_mapping_t;
    mb_mapping =
        malloc(::std::mem::size_of::<modbus_mapping_t>() as libc::c_ulong) as
            *mut modbus_mapping_t;
    if mb_mapping.is_null() { return 0 as *mut modbus_mapping_t }
    /* 0X */
    (*mb_mapping).nb_bits = nb_bits as libc::c_int;
    (*mb_mapping).start_bits = start_bits as libc::c_int;
    if nb_bits == 0 as libc::c_int as libc::c_uint {
        (*mb_mapping).tab_bits = 0 as *mut uint8_t
    } else {
        /* Negative number raises a POSIX error */
        (*mb_mapping).tab_bits =
            malloc((nb_bits as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint8_t>()
                                                        as libc::c_ulong)) as
                *mut uint8_t;
        if (*mb_mapping).tab_bits.is_null() {
            free(mb_mapping as *mut libc::c_void);
            return 0 as *mut modbus_mapping_t
        }
        memset((*mb_mapping).tab_bits as *mut libc::c_void, 0 as libc::c_int,
               (nb_bits as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint8_t>()
                                                    as libc::c_ulong));
    }
    /* 1X */
    (*mb_mapping).nb_input_bits = nb_input_bits as libc::c_int;
    (*mb_mapping).start_input_bits = start_input_bits as libc::c_int;
    if nb_input_bits == 0 as libc::c_int as libc::c_uint {
        (*mb_mapping).tab_input_bits = 0 as *mut uint8_t
    } else {
        (*mb_mapping).tab_input_bits =
            malloc((nb_input_bits as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint8_t>()
                                                        as libc::c_ulong)) as
                *mut uint8_t;
        if (*mb_mapping).tab_input_bits.is_null() {
            free((*mb_mapping).tab_bits as *mut libc::c_void);
            free(mb_mapping as *mut libc::c_void);
            return 0 as *mut modbus_mapping_t
        }
        memset((*mb_mapping).tab_input_bits as *mut libc::c_void,
               0 as libc::c_int,
               (nb_input_bits as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint8_t>()
                                                    as libc::c_ulong));
    }
    /* 4X */
    (*mb_mapping).nb_registers = nb_registers as libc::c_int;
    (*mb_mapping).start_registers = start_registers as libc::c_int;
    if nb_registers == 0 as libc::c_int as libc::c_uint {
        (*mb_mapping).tab_registers = 0 as *mut uint16_t
    } else {
        (*mb_mapping).tab_registers =
            malloc((nb_registers as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                        as libc::c_ulong)) as
                *mut uint16_t;
        if (*mb_mapping).tab_registers.is_null() {
            free((*mb_mapping).tab_input_bits as *mut libc::c_void);
            free((*mb_mapping).tab_bits as *mut libc::c_void);
            free(mb_mapping as *mut libc::c_void);
            return 0 as *mut modbus_mapping_t
        }
        memset((*mb_mapping).tab_registers as *mut libc::c_void,
               0 as libc::c_int,
               (nb_registers as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                    as libc::c_ulong));
    }
    /* 3X */
    (*mb_mapping).nb_input_registers = nb_input_registers as libc::c_int;
    (*mb_mapping).start_input_registers =
        start_input_registers as libc::c_int;
    if nb_input_registers == 0 as libc::c_int as libc::c_uint {
        (*mb_mapping).tab_input_registers = 0 as *mut uint16_t
    } else {
        (*mb_mapping).tab_input_registers =
            malloc((nb_input_registers as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                        as libc::c_ulong)) as
                *mut uint16_t;
        if (*mb_mapping).tab_input_registers.is_null() {
            free((*mb_mapping).tab_registers as *mut libc::c_void);
            free((*mb_mapping).tab_input_bits as *mut libc::c_void);
            free((*mb_mapping).tab_bits as *mut libc::c_void);
            free(mb_mapping as *mut libc::c_void);
            return 0 as *mut modbus_mapping_t
        }
        memset((*mb_mapping).tab_input_registers as *mut libc::c_void,
               0 as libc::c_int,
               (nb_input_registers as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint16_t>()
                                                    as libc::c_ulong));
    }
    return mb_mapping;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_mapping_new(mut nb_bits: libc::c_int,
                                            mut nb_input_bits: libc::c_int,
                                            mut nb_registers: libc::c_int,
                                            mut nb_input_registers:
                                                libc::c_int)
 -> *mut modbus_mapping_t {
    return modbus_mapping_new_start_address(0 as libc::c_int as libc::c_uint,
                                            nb_bits as libc::c_uint,
                                            0 as libc::c_int as libc::c_uint,
                                            nb_input_bits as libc::c_uint,
                                            0 as libc::c_int as libc::c_uint,
                                            nb_registers as libc::c_uint,
                                            0 as libc::c_int as libc::c_uint,
                                            nb_input_registers as
                                                libc::c_uint);
}
/* Frees the 4 arrays */
#[no_mangle]
pub unsafe extern "C" fn modbus_mapping_free(mut mb_mapping:
                                                 *mut modbus_mapping_t) {
    if mb_mapping.is_null() { return }
    free((*mb_mapping).tab_input_registers as *mut libc::c_void);
    free((*mb_mapping).tab_registers as *mut libc::c_void);
    free((*mb_mapping).tab_input_bits as *mut libc::c_void);
    free((*mb_mapping).tab_bits as *mut libc::c_void);
    free(mb_mapping as *mut libc::c_void);
}
/*
 * Function strlcpy was originally developed by
 * Todd C. Miller <Todd.Miller@courtesan.com> to simplify writing secure code.
 * See ftp://ftp.openbsd.org/pub/OpenBSD/src/lib/libc/string/strlcpy.3
 * for more information.
 *
 * Thank you Ulrich Drepper... not!
 *
 * Copy src to string dest of size dest_size.  At most dest_size-1 characters
 * will be copied.  Always NUL terminates (unless dest_size == 0).  Returns
 * strlen(src); if retval >= dest_size, truncation occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn strlcpy(mut dest: *mut libc::c_char,
                                 mut src: *const libc::c_char,
                                 mut dest_size: size_t) -> size_t {
    let mut d: *mut libc::c_char = dest;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = dest_size;
    /* Copy as many bytes as will fit */
    if n != 0 as libc::c_int as libc::c_ulong &&
           { n = n.wrapping_sub(1); (n) != 0 as libc::c_int as libc::c_ulong }
       {
        loop  {
            let fresh32 = s;
            s = s.offset(1);
            let fresh33 = d;
            d = d.offset(1);
            *fresh33 = *fresh32;
            if *fresh33 as libc::c_int == 0 as libc::c_int { break ; }
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) { break ; }
        }
    }
    /* Not enough room in dest, add NUL and traverse rest of src */
    if n == 0 as libc::c_int as libc::c_ulong {
        if dest_size != 0 as libc::c_int as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char
        } /* NUL-terminate dest */
        loop  {
            let fresh34 = s;
            s = s.offset(1);
            if !(*fresh34 != 0) { break ; }
        }
    }
    return (s.wrapping_offset_from(src) as libc::c_long -
                1 as libc::c_int as libc::c_long) as size_t;
    /* count does not include NUL */
}
