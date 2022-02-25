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
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
              __writefds: *mut fd_set, __exceptfds: *mut fd_set,
              __timeout: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *mut libc::c_void,
                  __optlen: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn send(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
            __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    #[no_mangle]
    fn htons(__hostshort: uint16_t) -> uint16_t;
    #[no_mangle]
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    #[no_mangle]
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn getaddrinfo(__name: *const libc::c_char,
                   __service: *const libc::c_char, __req: *const addrinfo,
                   __pai: *mut *mut addrinfo) -> libc::c_int;
    #[no_mangle]
    fn freeaddrinfo(__ai: *mut addrinfo);
    #[no_mangle]
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn modbus_free(ctx: *mut modbus_t);
    #[no_mangle]
    fn _modbus_receive_msg(ctx: *mut modbus_t, msg: *mut uint8_t,
                           msg_type: msg_type_t) -> libc::c_int;
    #[no_mangle]
    fn strlcpy(dest: *mut libc::c_char, src: *const libc::c_char,
               dest_size: size_t) -> size_t;
    #[no_mangle]
    fn _modbus_init_common(ctx: *mut modbus_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_0 = 2;
pub const SHUT_WR: C2RustUnnamed_0 = 1;
pub const SHUT_RD: C2RustUnnamed_0 = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MODBUS_EXCEPTION_MAX: C2RustUnnamed_2 = 12;
pub const MODBUS_EXCEPTION_GATEWAY_TARGET: C2RustUnnamed_2 = 11;
pub const MODBUS_EXCEPTION_GATEWAY_PATH: C2RustUnnamed_2 = 10;
pub const MODBUS_EXCEPTION_NOT_DEFINED: C2RustUnnamed_2 = 9;
pub const MODBUS_EXCEPTION_MEMORY_PARITY: C2RustUnnamed_2 = 8;
pub const MODBUS_EXCEPTION_NEGATIVE_ACKNOWLEDGE: C2RustUnnamed_2 = 7;
pub const MODBUS_EXCEPTION_SLAVE_OR_SERVER_BUSY: C2RustUnnamed_2 = 6;
pub const MODBUS_EXCEPTION_ACKNOWLEDGE: C2RustUnnamed_2 = 5;
pub const MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE: C2RustUnnamed_2 = 4;
pub const MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE: C2RustUnnamed_2 = 3;
pub const MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS: C2RustUnnamed_2 = 2;
pub const MODBUS_EXCEPTION_ILLEGAL_FUNCTION: C2RustUnnamed_2 = 1;
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
pub type modbus_tcp_t = _modbus_tcp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus_tcp {
    pub t_id: uint16_t,
    pub port: libc::c_int,
    pub ip: [libc::c_char; 16],
}
pub type msg_type_t = libc::c_uint;
pub const MSG_CONFIRMATION: msg_type_t = 1;
pub const MSG_INDICATION: msg_type_t = 0;
pub const _MODBUS_BACKEND_TYPE_TCP: C2RustUnnamed_3 = 1;
pub type modbus_tcp_pi_t = _modbus_tcp_pi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _modbus_tcp_pi {
    pub t_id: uint16_t,
    pub port: libc::c_int,
    pub node: [libc::c_char; 1025],
    pub service: [libc::c_char; 32],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _MODBUS_BACKEND_TYPE_RTU: C2RustUnnamed_3 = 0;
/*
 * Copyright © 2001-2013 Stéphane Raimbault <stephane.raimbault@gmail.com>
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 */
unsafe extern "C" fn _modbus_set_slave(mut ctx: *mut modbus_t,
                                       mut slave: libc::c_int)
 -> libc::c_int {
    /* Broadcast address is 0 (MODBUS_BROADCAST_ADDRESS) */
    if slave >= 0 as libc::c_int && slave <= 247 as libc::c_int {
        (*ctx).slave = slave
    } else if slave == 0xff as libc::c_int {
        /* The special value MODBUS_TCP_SLAVE (0xFF) can be used in TCP mode to
         * restore the default value. */
        (*ctx).slave = slave
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Builds a TCP request header */
unsafe extern "C" fn _modbus_tcp_build_request_basis(mut ctx: *mut modbus_t,
                                                     mut function:
                                                         libc::c_int,
                                                     mut addr: libc::c_int,
                                                     mut nb: libc::c_int,
                                                     mut req: *mut uint8_t)
 -> libc::c_int {
    let mut ctx_tcp: *mut modbus_tcp_t =
        (*ctx).backend_data as *mut modbus_tcp_t;
    /* Increase transaction ID */
    if ((*ctx_tcp).t_id as libc::c_int) < 65535 as libc::c_int {
        (*ctx_tcp).t_id = (*ctx_tcp).t_id.wrapping_add(1)
    } else { (*ctx_tcp).t_id = 0 as libc::c_int as uint16_t }
    *req.offset(0 as libc::c_int as isize) =
        ((*ctx_tcp).t_id as libc::c_int >> 8 as libc::c_int) as uint8_t;
    *req.offset(1 as libc::c_int as isize) =
        ((*ctx_tcp).t_id as libc::c_int & 0xff as libc::c_int) as uint8_t;
    /* Protocol Modbus */
    *req.offset(2 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *req.offset(3 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    /* Length will be defined later by set_req_length_tcp at offsets 4
       and 5 */
    *req.offset(6 as libc::c_int as isize) = (*ctx).slave as uint8_t;
    *req.offset(7 as libc::c_int as isize) = function as uint8_t;
    *req.offset(8 as libc::c_int as isize) =
        (addr >> 8 as libc::c_int) as uint8_t;
    *req.offset(9 as libc::c_int as isize) =
        (addr & 0xff as libc::c_int) as uint8_t;
    *req.offset(10 as libc::c_int as isize) =
        (nb >> 8 as libc::c_int) as uint8_t;
    *req.offset(11 as libc::c_int as isize) =
        (nb & 0xff as libc::c_int) as uint8_t;
    return 12 as libc::c_int;
}
/* Builds a TCP response header */
unsafe extern "C" fn _modbus_tcp_build_response_basis(mut sft: *mut sft_t,
                                                      mut rsp: *mut uint8_t)
 -> libc::c_int {
    /* Extract from MODBUS Messaging on TCP/IP Implementation
       Guide V1.0b (page 23/46):
       The transaction identifier is used to associate the future
       response with the request. */
    *rsp.offset(0 as libc::c_int as isize) =
        ((*sft).t_id >> 8 as libc::c_int) as uint8_t;
    *rsp.offset(1 as libc::c_int as isize) =
        ((*sft).t_id & 0xff as libc::c_int) as uint8_t;
    /* Protocol Modbus */
    *rsp.offset(2 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    *rsp.offset(3 as libc::c_int as isize) = 0 as libc::c_int as uint8_t;
    /* Length will be set later by send_msg (4 and 5) */
    /* The slave ID is copied from the indication */
    *rsp.offset(6 as libc::c_int as isize) = (*sft).slave as uint8_t;
    *rsp.offset(7 as libc::c_int as isize) = (*sft).function as uint8_t;
    return 8 as libc::c_int;
}
unsafe extern "C" fn _modbus_tcp_prepare_response_tid(mut req: *const uint8_t,
                                                      mut req_length:
                                                          *mut libc::c_int)
 -> libc::c_int {
    return ((*req.offset(0 as libc::c_int as isize) as libc::c_int) <<
                8 as libc::c_int) +
               *req.offset(1 as libc::c_int as isize) as libc::c_int;
}
unsafe extern "C" fn _modbus_tcp_send_msg_pre(mut req: *mut uint8_t,
                                              mut req_length: libc::c_int)
 -> libc::c_int {
    /* Subtract the header length to the message length */
    let mut mbap_length: libc::c_int = req_length - 6 as libc::c_int;
    *req.offset(4 as libc::c_int as isize) =
        (mbap_length >> 8 as libc::c_int) as uint8_t;
    *req.offset(5 as libc::c_int as isize) =
        (mbap_length & 0xff as libc::c_int) as uint8_t;
    return req_length;
}
unsafe extern "C" fn _modbus_tcp_send(mut ctx: *mut modbus_t,
                                      mut req: *const uint8_t,
                                      mut req_length: libc::c_int)
 -> ssize_t {
    /* MSG_NOSIGNAL
       Requests not to send SIGPIPE on errors on stream oriented
       sockets when the other end breaks the connection.  The EPIPE
       error is still returned. */
    return send((*ctx).s, req as *const libc::c_char as *const libc::c_void,
                req_length as size_t, MSG_NOSIGNAL as libc::c_int);
}
unsafe extern "C" fn _modbus_tcp_receive(mut ctx: *mut modbus_t,
                                         mut req: *mut uint8_t)
 -> libc::c_int {
    return _modbus_receive_msg(ctx, req, MSG_INDICATION);
}
unsafe extern "C" fn _modbus_tcp_recv(mut ctx: *mut modbus_t,
                                      mut rsp: *mut uint8_t,
                                      mut rsp_length: libc::c_int)
 -> ssize_t {
    return recv((*ctx).s, rsp as *mut libc::c_char as *mut libc::c_void,
                rsp_length as size_t, 0 as libc::c_int);
}
unsafe extern "C" fn _modbus_tcp_check_integrity(mut ctx: *mut modbus_t,
                                                 mut msg: *mut uint8_t,
                                                 msg_length: libc::c_int)
 -> libc::c_int {
    return msg_length;
}
unsafe extern "C" fn _modbus_tcp_pre_check_confirmation(mut ctx:
                                                            *mut modbus_t,
                                                        mut req:
                                                            *const uint8_t,
                                                        mut rsp:
                                                            *const uint8_t,
                                                        mut rsp_length:
                                                            libc::c_int)
 -> libc::c_int {
    /* Check transaction ID */
    if *req.offset(0 as libc::c_int as isize) as libc::c_int !=
           *rsp.offset(0 as libc::c_int as isize) as libc::c_int ||
           *req.offset(1 as libc::c_int as isize) as libc::c_int !=
               *rsp.offset(1 as libc::c_int as isize) as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Invalid transaction ID received 0x%X (not 0x%X)\n\x00"
                        as *const u8 as *const libc::c_char,
                    ((*rsp.offset(0 as libc::c_int as isize) as libc::c_int)
                         << 8 as libc::c_int) +
                        *rsp.offset(1 as libc::c_int as isize) as libc::c_int,
                    ((*req.offset(0 as libc::c_int as isize) as libc::c_int)
                         << 8 as libc::c_int) +
                        *req.offset(1 as libc::c_int as isize) as
                            libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    /* Check protocol ID */
    if *rsp.offset(2 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int &&
           *rsp.offset(3 as libc::c_int as isize) as libc::c_int !=
               0 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Invalid protocol ID received 0x%X (not 0x0)\n\x00" as
                        *const u8 as *const libc::c_char,
                    ((*rsp.offset(2 as libc::c_int as isize) as libc::c_int)
                         << 8 as libc::c_int) +
                        *rsp.offset(3 as libc::c_int as isize) as
                            libc::c_int);
        }
        *__errno_location() =
            112345678 as libc::c_int +
                MODBUS_EXCEPTION_GATEWAY_TARGET as libc::c_int +
                2 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _modbus_tcp_set_ipv4_options(mut s: libc::c_int)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut option: libc::c_int = 0;
    /* Set the TCP no delay flag */
    /* SOL_TCP = IPPROTO_TCP */
    option = 1 as libc::c_int;
    rc =
        setsockopt(s, IPPROTO_TCP as libc::c_int, 1 as libc::c_int,
                   &mut option as *mut libc::c_int as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                       socklen_t);
    if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    /* If the OS does not offer SOCK_NONBLOCK, fall back to setting FIONBIO to
     * make sockets non-blocking */
    /* Do not care about the return value, this is optional */
    /* *
     * Cygwin defines IPTOS_LOWDELAY but can't handle that flag so it's
     * necessary to workaround that problem.
     **/
    /* Set the IP low delay option */
    option = 0x10 as libc::c_int;
    rc =
        setsockopt(s, IPPROTO_IP as libc::c_int, 1 as libc::c_int,
                   &mut option as *mut libc::c_int as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                       socklen_t);
    if rc == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _connect(mut sockfd: libc::c_int,
                              mut addr: *const sockaddr,
                              mut addrlen: socklen_t,
                              mut ro_tv: *const timeval) -> libc::c_int {
    let mut rc: libc::c_int = connect(sockfd, addr, addrlen);
    if rc == -(1 as libc::c_int) && *__errno_location() == 115 as libc::c_int
       {
        let mut wset: fd_set = fd_set{__fds_bits: [0; 16],};
        let mut optval: libc::c_int = 0;
        let mut optlen: socklen_t =
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                socklen_t;
        let mut tv: timeval = *ro_tv;
        /* Wait to be available in writing */
        let mut __i: libc::c_uint = 0;
        let mut __arr: *mut fd_set = &mut wset;
        __i = 0 as libc::c_int as libc::c_uint;
        while (__i as libc::c_ulong) <
                  (::std::mem::size_of::<fd_set>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                       as libc::c_ulong) {
            (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
            __i = __i.wrapping_add(1)
        }
        wset.__fds_bits[(sockfd /
                             (8 as libc::c_int *
                                  ::std::mem::size_of::<__fd_mask>() as
                                      libc::c_ulong as libc::c_int)) as usize]
            |=
            ((1 as libc::c_ulong) <<
                 sockfd %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as __fd_mask;
        rc =
            select(sockfd + 1 as libc::c_int, 0 as *mut fd_set, &mut wset,
                   0 as *mut fd_set, &mut tv);
        if rc <= 0 as libc::c_int {
            /* Timeout or fail */
            return -(1 as libc::c_int)
        }
        /* The connection is established if SO_ERROR and optval are set to 0 */
        rc =
            getsockopt(sockfd, 1 as libc::c_int, 4 as libc::c_int,
                       &mut optval as *mut libc::c_int as *mut libc::c_void,
                       &mut optlen);
        if rc == 0 as libc::c_int && optval == 0 as libc::c_int {
            return 0 as libc::c_int
        } else {
            *__errno_location() = 111 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    return rc;
}
/* Establishes a modbus TCP connection with a Modbus server. */
unsafe extern "C" fn _modbus_tcp_connect(mut ctx: *mut modbus_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    /* Specialized version of sockaddr for Internet socket address (same size) */
    let mut addr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut ctx_tcp: *mut modbus_tcp_t =
        (*ctx).backend_data as *mut modbus_tcp_t;
    let mut flags: libc::c_int = SOCK_STREAM as libc::c_int;
    flags |= SOCK_CLOEXEC as libc::c_int;
    flags |= SOCK_NONBLOCK as libc::c_int;
    (*ctx).s = socket(2 as libc::c_int, flags, 0 as libc::c_int);
    if (*ctx).s == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    rc = _modbus_tcp_set_ipv4_options((*ctx).s);
    if rc == -(1 as libc::c_int) {
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    if (*ctx).debug != 0 {
        printf(b"Connecting to %s:%d\n\x00" as *const u8 as
                   *const libc::c_char, (*ctx_tcp).ip.as_mut_ptr(),
               (*ctx_tcp).port);
    }
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr.sin_port = htons((*ctx_tcp).port as uint16_t);
    addr.sin_addr.s_addr = inet_addr((*ctx_tcp).ip.as_mut_ptr());
    rc =
        _connect((*ctx).s, &mut addr as *mut sockaddr_in as *mut sockaddr,
                 ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                     socklen_t, &mut (*ctx).response_timeout);
    if rc == -(1 as libc::c_int) {
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int);
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/* Establishes a modbus TCP PI connection with a Modbus server. */
unsafe extern "C" fn _modbus_tcp_pi_connect(mut ctx: *mut modbus_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ai_list: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai_ptr: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai_hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut ctx_tcp_pi: *mut modbus_tcp_pi_t =
        (*ctx).backend_data as *mut modbus_tcp_pi_t;
    memset(&mut ai_hints as *mut addrinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    ai_hints.ai_flags |= 0x20 as libc::c_int;
    ai_hints.ai_family = 0 as libc::c_int;
    ai_hints.ai_socktype = SOCK_STREAM as libc::c_int;
    ai_hints.ai_addr = 0 as *mut sockaddr;
    ai_hints.ai_canonname = 0 as *mut libc::c_char;
    ai_hints.ai_next = 0 as *mut addrinfo;
    ai_list = 0 as *mut addrinfo;
    rc =
        getaddrinfo((*ctx_tcp_pi).node.as_mut_ptr(),
                    (*ctx_tcp_pi).service.as_mut_ptr(), &mut ai_hints,
                    &mut ai_list);
    if rc != 0 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Error returned by getaddrinfo: %s\n\x00" as *const u8 as
                        *const libc::c_char, gai_strerror(rc));
        }
        *__errno_location() = 111 as libc::c_int;
        return -(1 as libc::c_int)
    }
    ai_ptr = ai_list;
    while !ai_ptr.is_null() {
        let mut flags: libc::c_int = (*ai_ptr).ai_socktype;
        let mut s: libc::c_int = 0;
        flags |= SOCK_CLOEXEC as libc::c_int;
        flags |= SOCK_NONBLOCK as libc::c_int;
        s = socket((*ai_ptr).ai_family, flags, (*ai_ptr).ai_protocol);
        if !(s < 0 as libc::c_int) {
            if (*ai_ptr).ai_family == 2 as libc::c_int {
                _modbus_tcp_set_ipv4_options(s);
            }
            if (*ctx).debug != 0 {
                printf(b"Connecting to [%s]:%s\n\x00" as *const u8 as
                           *const libc::c_char,
                       (*ctx_tcp_pi).node.as_mut_ptr(),
                       (*ctx_tcp_pi).service.as_mut_ptr());
            }
            rc =
                _connect(s, (*ai_ptr).ai_addr, (*ai_ptr).ai_addrlen,
                         &mut (*ctx).response_timeout);
            if rc == -(1 as libc::c_int) {
                close(s);
            } else { (*ctx).s = s; break ; }
        }
        ai_ptr = (*ai_ptr).ai_next
    }
    freeaddrinfo(ai_list);
    if (*ctx).s < 0 as libc::c_int { return -(1 as libc::c_int) }
    return 0 as libc::c_int;
}
/* Closes the network connection and socket in TCP mode */
unsafe extern "C" fn _modbus_tcp_close(mut ctx: *mut modbus_t) {
    if (*ctx).s != -(1 as libc::c_int) {
        shutdown((*ctx).s, SHUT_RDWR as libc::c_int);
        close((*ctx).s);
        (*ctx).s = -(1 as libc::c_int)
    };
}
unsafe extern "C" fn _modbus_tcp_flush(mut ctx: *mut modbus_t)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut rc_sum: libc::c_int = 0 as libc::c_int;
    loop  {
        /* Extract the garbage from the socket */
        let mut devnull: [libc::c_char; 260] = [0; 260];
        rc =
            recv((*ctx).s, devnull.as_mut_ptr() as *mut libc::c_void,
                 260 as libc::c_int as size_t, MSG_DONTWAIT as libc::c_int) as
                libc::c_int;
        if rc > 0 as libc::c_int { rc_sum += rc }
        if !(rc == 260 as libc::c_int) { break ; }
    }
    return rc_sum;
}
/* Listens for any request from one or many modbus masters in TCP */
#[no_mangle]
pub unsafe extern "C" fn modbus_tcp_listen(mut ctx: *mut modbus_t,
                                           mut nb_connection: libc::c_int)
 -> libc::c_int {
    let mut new_s: libc::c_int = 0;
    let mut enable: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut addr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut ctx_tcp: *mut modbus_tcp_t = 0 as *mut modbus_tcp_t;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    ctx_tcp = (*ctx).backend_data as *mut modbus_tcp_t;
    flags = SOCK_STREAM as libc::c_int;
    flags |= SOCK_CLOEXEC as libc::c_int;
    new_s = socket(2 as libc::c_int, flags, IPPROTO_TCP as libc::c_int);
    if new_s == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    enable = 1 as libc::c_int;
    if setsockopt(new_s, 1 as libc::c_int, 2 as libc::c_int,
                  &mut enable as *mut libc::c_int as *mut libc::c_char as
                      *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) == -(1 as libc::c_int) {
        close(new_s);
        return -(1 as libc::c_int)
    }
    memset(&mut addr as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    /* If the modbus port is < to 1024, we need the setuid root. */
    addr.sin_port = htons((*ctx_tcp).port as uint16_t);
    if (*ctx_tcp).ip[0 as libc::c_int as usize] as libc::c_int == '0' as i32 {
        /* Listen any addresses */
        addr.sin_addr.s_addr = htonl(0 as libc::c_int as in_addr_t)
    } else {
        /* Listen only specified IP address */
        addr.sin_addr.s_addr = inet_addr((*ctx_tcp).ip.as_mut_ptr())
    }
    if bind(new_s, &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) == -(1 as libc::c_int) {
        close(new_s);
        return -(1 as libc::c_int)
    }
    if listen(new_s, nb_connection) == -(1 as libc::c_int) {
        close(new_s);
        return -(1 as libc::c_int)
    }
    return new_s;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_tcp_pi_listen(mut ctx: *mut modbus_t,
                                              mut nb_connection: libc::c_int)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ai_list: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai_ptr: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai_hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut node: *const libc::c_char = 0 as *const libc::c_char;
    let mut service: *const libc::c_char = 0 as *const libc::c_char;
    let mut new_s: libc::c_int = 0;
    let mut ctx_tcp_pi: *mut modbus_tcp_pi_t = 0 as *mut modbus_tcp_pi_t;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    ctx_tcp_pi = (*ctx).backend_data as *mut modbus_tcp_pi_t;
    if (*ctx_tcp_pi).node[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        node = 0 as *const libc::c_char
        /* == any */
    } else { node = (*ctx_tcp_pi).node.as_mut_ptr() }
    if (*ctx_tcp_pi).service[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        service = b"502\x00" as *const u8 as *const libc::c_char
    } else { service = (*ctx_tcp_pi).service.as_mut_ptr() }
    memset(&mut ai_hints as *mut addrinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    /* If node is not NULL, than the AI_PASSIVE flag is ignored. */
    ai_hints.ai_flags |= 0x1 as libc::c_int;
    ai_hints.ai_flags |= 0x20 as libc::c_int;
    ai_hints.ai_family = 0 as libc::c_int;
    ai_hints.ai_socktype = SOCK_STREAM as libc::c_int;
    ai_hints.ai_addr = 0 as *mut sockaddr;
    ai_hints.ai_canonname = 0 as *mut libc::c_char;
    ai_hints.ai_next = 0 as *mut addrinfo;
    ai_list = 0 as *mut addrinfo;
    rc = getaddrinfo(node, service, &mut ai_hints, &mut ai_list);
    if rc != 0 as libc::c_int {
        if (*ctx).debug != 0 {
            fprintf(stderr,
                    b"Error returned by getaddrinfo: %s\n\x00" as *const u8 as
                        *const libc::c_char, gai_strerror(rc));
        }
        *__errno_location() = 111 as libc::c_int;
        return -(1 as libc::c_int)
    }
    new_s = -(1 as libc::c_int);
    ai_ptr = ai_list;
    while !ai_ptr.is_null() {
        let mut flags: libc::c_int = (*ai_ptr).ai_socktype;
        let mut s: libc::c_int = 0;
        flags |= SOCK_CLOEXEC as libc::c_int;
        s = socket((*ai_ptr).ai_family, flags, (*ai_ptr).ai_protocol);
        if s < 0 as libc::c_int {
            if (*ctx).debug != 0 {
                perror(b"socket\x00" as *const u8 as *const libc::c_char);
            }
        } else {
            let mut enable: libc::c_int = 1 as libc::c_int;
            rc =
                setsockopt(s, 1 as libc::c_int, 2 as libc::c_int,
                           &mut enable as *mut libc::c_int as
                               *mut libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t);
            if rc != 0 as libc::c_int {
                close(s);
                if (*ctx).debug != 0 {
                    perror(b"setsockopt\x00" as *const u8 as
                               *const libc::c_char);
                }
            } else {
                rc = bind(s, (*ai_ptr).ai_addr, (*ai_ptr).ai_addrlen);
                if rc != 0 as libc::c_int {
                    close(s);
                    if (*ctx).debug != 0 {
                        perror(b"bind\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                } else {
                    rc = listen(s, nb_connection);
                    if rc != 0 as libc::c_int {
                        close(s);
                        if (*ctx).debug != 0 {
                            perror(b"listen\x00" as *const u8 as
                                       *const libc::c_char);
                        }
                    } else { new_s = s; break ; }
                }
            }
        }
        ai_ptr = (*ai_ptr).ai_next
    }
    freeaddrinfo(ai_list);
    if new_s < 0 as libc::c_int { return -(1 as libc::c_int) }
    return new_s;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_tcp_accept(mut ctx: *mut modbus_t,
                                           mut s: *mut libc::c_int)
 -> libc::c_int {
    let mut addr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut addrlen: socklen_t = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    addrlen =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    /* Inherit socket flags and use accept4 call */
    (*ctx).s =
        accept4(*s, &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut addrlen, SOCK_CLOEXEC as libc::c_int);
    if (*ctx).s == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if (*ctx).debug != 0 {
        printf(b"The client connection from %s is accepted\n\x00" as *const u8
                   as *const libc::c_char, inet_ntoa(addr.sin_addr));
    }
    return (*ctx).s;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_tcp_pi_accept(mut ctx: *mut modbus_t,
                                              mut s: *mut libc::c_int)
 -> libc::c_int {
    let mut addr: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut addrlen: socklen_t = 0;
    if ctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    addrlen =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    /* Inherit socket flags and use accept4 call */
    (*ctx).s =
        accept4(*s, &mut addr as *mut sockaddr_storage as *mut sockaddr,
                &mut addrlen, SOCK_CLOEXEC as libc::c_int);
    if (*ctx).s == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if (*ctx).debug != 0 {
        printf(b"The client connection is accepted.\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    return (*ctx).s;
}
unsafe extern "C" fn _modbus_tcp_select(mut ctx: *mut modbus_t,
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
        *__errno_location() = 110 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return s_rc;
}
unsafe extern "C" fn _modbus_tcp_free(mut ctx: *mut modbus_t) {
    free((*ctx).backend_data);
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub static mut _modbus_tcp_backend: modbus_backend_t =
    unsafe {
        {
            let mut init =
                _modbus_backend{backend_type:
                                    _MODBUS_BACKEND_TYPE_TCP as libc::c_int as
                                        libc::c_uint,
                                header_length:
                                    7 as libc::c_int as libc::c_uint,
                                checksum_length:
                                    0 as libc::c_int as libc::c_uint,
                                max_adu_length:
                                    260 as libc::c_int as libc::c_uint,
                                set_slave:
                                    Some(_modbus_set_slave as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                build_request_basis:
                                    Some(_modbus_tcp_build_request_basis as
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
                                    Some(_modbus_tcp_build_response_basis as
                                             unsafe extern "C" fn(_:
                                                                      *mut sft_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                prepare_response_tid:
                                    Some(_modbus_tcp_prepare_response_tid as
                                             unsafe extern "C" fn(_:
                                                                      *const uint8_t,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> libc::c_int),
                                send_msg_pre:
                                    Some(_modbus_tcp_send_msg_pre as
                                             unsafe extern "C" fn(_:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                send:
                                    Some(_modbus_tcp_send as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *const uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                receive:
                                    Some(_modbus_tcp_receive as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                recv:
                                    Some(_modbus_tcp_recv as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                check_integrity:
                                    Some(_modbus_tcp_check_integrity as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                pre_check_confirmation:
                                    Some(_modbus_tcp_pre_check_confirmation as
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
                                    Some(_modbus_tcp_connect as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                close:
                                    Some(_modbus_tcp_close as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),
                                flush:
                                    Some(_modbus_tcp_flush as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                select:
                                    Some(_modbus_tcp_select as
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
                                    Some(_modbus_tcp_free as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),};
            init
        }
    };
#[no_mangle]
pub static mut _modbus_tcp_pi_backend: modbus_backend_t =
    unsafe {
        {
            let mut init =
                _modbus_backend{backend_type:
                                    _MODBUS_BACKEND_TYPE_TCP as libc::c_int as
                                        libc::c_uint,
                                header_length:
                                    7 as libc::c_int as libc::c_uint,
                                checksum_length:
                                    0 as libc::c_int as libc::c_uint,
                                max_adu_length:
                                    260 as libc::c_int as libc::c_uint,
                                set_slave:
                                    Some(_modbus_set_slave as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                build_request_basis:
                                    Some(_modbus_tcp_build_request_basis as
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
                                    Some(_modbus_tcp_build_response_basis as
                                             unsafe extern "C" fn(_:
                                                                      *mut sft_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                prepare_response_tid:
                                    Some(_modbus_tcp_prepare_response_tid as
                                             unsafe extern "C" fn(_:
                                                                      *const uint8_t,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> libc::c_int),
                                send_msg_pre:
                                    Some(_modbus_tcp_send_msg_pre as
                                             unsafe extern "C" fn(_:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                send:
                                    Some(_modbus_tcp_send as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *const uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                receive:
                                    Some(_modbus_tcp_receive as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t)
                                                 -> libc::c_int),
                                recv:
                                    Some(_modbus_tcp_recv as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> ssize_t),
                                check_integrity:
                                    Some(_modbus_tcp_check_integrity as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t,
                                                                  _:
                                                                      *mut uint8_t,
                                                                  _:
                                                                      libc::c_int)
                                                 -> libc::c_int),
                                pre_check_confirmation:
                                    Some(_modbus_tcp_pre_check_confirmation as
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
                                    Some(_modbus_tcp_pi_connect as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                close:
                                    Some(_modbus_tcp_close as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),
                                flush:
                                    Some(_modbus_tcp_flush as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> libc::c_int),
                                select:
                                    Some(_modbus_tcp_select as
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
                                    Some(_modbus_tcp_free as
                                             unsafe extern "C" fn(_:
                                                                      *mut modbus_t)
                                                 -> ()),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn modbus_new_tcp(mut ip: *const libc::c_char,
                                        mut port: libc::c_int)
 -> *mut modbus_t {
    let mut ctx: *mut modbus_t = 0 as *mut modbus_t;
    let mut ctx_tcp: *mut modbus_tcp_t = 0 as *mut modbus_tcp_t;
    let mut dest_size: size_t = 0;
    let mut ret_size: size_t = 0;
    ctx =
        malloc(::std::mem::size_of::<modbus_t>() as libc::c_ulong) as
            *mut modbus_t;
    if ctx.is_null() { return 0 as *mut modbus_t }
    _modbus_init_common(ctx);
    /* Could be changed after to reach a remote serial Modbus device */
    (*ctx).slave = 0xff as libc::c_int;
    (*ctx).backend = &_modbus_tcp_backend;
    (*ctx).backend_data =
        malloc(::std::mem::size_of::<modbus_tcp_t>() as libc::c_ulong) as
            *mut modbus_tcp_t as *mut libc::c_void;
    if (*ctx).backend_data.is_null() {
        modbus_free(ctx);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut modbus_t
    }
    ctx_tcp = (*ctx).backend_data as *mut modbus_tcp_t;
    if !ip.is_null() {
        dest_size =
            (::std::mem::size_of::<libc::c_char>() as
                 libc::c_ulong).wrapping_mul(16 as libc::c_int as
                                                 libc::c_ulong);
        ret_size = strlcpy((*ctx_tcp).ip.as_mut_ptr(), ip, dest_size);
        if ret_size == 0 as libc::c_int as libc::c_ulong {
            fprintf(stderr,
                    b"The IP string is empty\n\x00" as *const u8 as
                        *const libc::c_char);
            modbus_free(ctx);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut modbus_t
        }
        if ret_size >= dest_size {
            fprintf(stderr,
                    b"The IP string has been truncated\n\x00" as *const u8 as
                        *const libc::c_char);
            modbus_free(ctx);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut modbus_t
        }
    } else {
        (*ctx_tcp).ip[0 as libc::c_int as usize] = '0' as i32 as libc::c_char
    }
    (*ctx_tcp).port = port;
    (*ctx_tcp).t_id = 0 as libc::c_int as uint16_t;
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn modbus_new_tcp_pi(mut node: *const libc::c_char,
                                           mut service: *const libc::c_char)
 -> *mut modbus_t {
    let mut ctx: *mut modbus_t = 0 as *mut modbus_t;
    let mut ctx_tcp_pi: *mut modbus_tcp_pi_t = 0 as *mut modbus_tcp_pi_t;
    let mut dest_size: size_t = 0;
    let mut ret_size: size_t = 0;
    ctx =
        malloc(::std::mem::size_of::<modbus_t>() as libc::c_ulong) as
            *mut modbus_t;
    if ctx.is_null() { return 0 as *mut modbus_t }
    _modbus_init_common(ctx);
    /* Could be changed after to reach a remote serial Modbus device */
    (*ctx).slave = 0xff as libc::c_int;
    (*ctx).backend = &_modbus_tcp_pi_backend;
    (*ctx).backend_data =
        malloc(::std::mem::size_of::<modbus_tcp_pi_t>() as libc::c_ulong) as
            *mut modbus_tcp_pi_t as *mut libc::c_void;
    if (*ctx).backend_data.is_null() {
        modbus_free(ctx);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut modbus_t
    }
    ctx_tcp_pi = (*ctx).backend_data as *mut modbus_tcp_pi_t;
    if node.is_null() {
        /* The node argument can be empty to indicate any hosts */
        (*ctx_tcp_pi).node[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    } else {
        dest_size =
            (::std::mem::size_of::<libc::c_char>() as
                 libc::c_ulong).wrapping_mul(1025 as libc::c_int as
                                                 libc::c_ulong);
        ret_size = strlcpy((*ctx_tcp_pi).node.as_mut_ptr(), node, dest_size);
        if ret_size == 0 as libc::c_int as libc::c_ulong {
            fprintf(stderr,
                    b"The node string is empty\n\x00" as *const u8 as
                        *const libc::c_char);
            modbus_free(ctx);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut modbus_t
        }
        if ret_size >= dest_size {
            fprintf(stderr,
                    b"The node string has been truncated\n\x00" as *const u8
                        as *const libc::c_char);
            modbus_free(ctx);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut modbus_t
        }
    }
    if !service.is_null() {
        dest_size =
            (::std::mem::size_of::<libc::c_char>() as
                 libc::c_ulong).wrapping_mul(32 as libc::c_int as
                                                 libc::c_ulong);
        ret_size =
            strlcpy((*ctx_tcp_pi).service.as_mut_ptr(), service, dest_size)
    } else {
        /* Empty service is not allowed, error caught below. */
        ret_size = 0 as libc::c_int as size_t
    }
    if ret_size == 0 as libc::c_int as libc::c_ulong {
        fprintf(stderr,
                b"The service string is empty\n\x00" as *const u8 as
                    *const libc::c_char);
        modbus_free(ctx);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut modbus_t
    }
    if ret_size >= dest_size {
        fprintf(stderr,
                b"The service string has been truncated\n\x00" as *const u8 as
                    *const libc::c_char);
        modbus_free(ctx);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut modbus_t
    }
    (*ctx_tcp_pi).t_id = 0 as libc::c_int as uint16_t;
    return ctx;
}
