use libc::{
    size_t,
    timeval,
};

#[cfg(unix)]
use libc::{
    in_addr,
    sockaddr,
    socklen_t,
};
#[cfg(windows)]
use winapi::{
    in_addr,
    SOCKADDR,
    socklen_t,
};

#[cfg(windows)]
use winapi::winsock2::fd_set;
#[cfg(unix)]
use libc::fd_set;

#[cfg(unix)]
pub type Struct_sockaddr = sockaddr;
#[cfg(windows)]
pub type Struct_sockaddr = SOCKADDR;

pub type Struct_in_addr = in_addr;
pub type Struct_timeval = timeval;

/* automatically generated by rust-bindgen */

pub type ares_socklen_t = socklen_t;
pub type __cares_rule_02__ = [::libc::c_char; 1usize];
pub type __cares_rule_03__ = [::libc::c_char; 1usize];
#[cfg(windows)]
pub type ares_socket_t = ::std::os::windows::io::RawSocket;
#[cfg(unix)]
pub type ares_socket_t = ::std::os::unix::io::RawFd;
pub type ares_sock_state_cb =
    ::std::option::Option<unsafe extern "C" fn(data: *mut ::libc::c_void,
                                               socket_fd: ares_socket_t,
                                               readable: ::libc::c_int,
                                               writable: ::libc::c_int)
                              -> ()>;
pub enum Struct_apattern { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_options {
    pub flags: ::libc::c_int,
    pub timeout: ::libc::c_int,
    pub tries: ::libc::c_int,
    pub ndots: ::libc::c_int,
    pub udp_port: ::libc::c_ushort,
    pub tcp_port: ::libc::c_ushort,
    pub socket_send_buffer_size: ::libc::c_int,
    pub socket_receive_buffer_size: ::libc::c_int,
    pub servers: *mut Struct_in_addr,
    pub nservers: ::libc::c_int,
    pub domains: *mut *mut ::libc::c_char,
    pub ndomains: ::libc::c_int,
    pub lookups: *mut ::libc::c_char,
    pub sock_state_cb: ares_sock_state_cb,
    pub sock_state_cb_data: *mut ::libc::c_void,
    pub sortlist: *mut Struct_apattern,
    pub nsort: ::libc::c_int,
    pub ednspsz: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ares_options {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_options {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_hostent { }
pub enum Struct_ares_channeldata { }
pub type ares_channel = *mut Struct_ares_channeldata;
pub type ares_callback =
    ::std::option::Option<unsafe extern "C" fn(arg: *mut ::libc::c_void,
                                               status: ::libc::c_int,
                                               timeouts: ::libc::c_int,
                                               abuf: *mut ::libc::c_uchar,
                                               alen: ::libc::c_int) -> ()>;
pub type ares_host_callback =
    ::std::option::Option<unsafe extern "C" fn(arg: *mut ::libc::c_void,
                                               status: ::libc::c_int,
                                               timeouts: ::libc::c_int,
                                               hostent: *mut Struct_hostent)
                              -> ()>;
pub type ares_nameinfo_callback =
    ::std::option::Option<unsafe extern "C" fn(arg: *mut ::libc::c_void,
                                               status: ::libc::c_int,
                                               timeouts: ::libc::c_int,
                                               node: *mut ::libc::c_char,
                                               service: *mut ::libc::c_char)
                              -> ()>;
pub type ares_sock_create_callback =
    ::std::option::Option<unsafe extern "C" fn(socket_fd: ares_socket_t,
                                               _type: ::libc::c_int,
                                               data: *mut ::libc::c_void)
                              -> ::libc::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_in6_addr {
    pub _S6_un: Union_Unnamed1,
}
impl ::std::clone::Clone for Struct_ares_in6_addr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_in6_addr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u8; 16usize],
}
impl Union_Unnamed1 {
    pub unsafe fn _S6_u8(&mut self) -> *mut [::libc::c_uchar; 16usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_addrttl {
    pub ipaddr: Struct_in_addr,
    pub ttl: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ares_addrttl {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_addrttl {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_addr6ttl {
    pub ip6addr: Struct_ares_in6_addr,
    pub ttl: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ares_addr6ttl {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_addr6ttl {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_srv_reply {
    pub next: *mut Struct_ares_srv_reply,
    pub host: *mut ::libc::c_char,
    pub priority: ::libc::c_ushort,
    pub weight: ::libc::c_ushort,
    pub port: ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct_ares_srv_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_srv_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_mx_reply {
    pub next: *mut Struct_ares_mx_reply,
    pub host: *mut ::libc::c_char,
    pub priority: ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct_ares_mx_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_mx_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_txt_reply {
    pub next: *mut Struct_ares_txt_reply,
    pub txt: *mut ::libc::c_uchar,
    pub length: size_t,
}
impl ::std::clone::Clone for Struct_ares_txt_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_txt_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_naptr_reply {
    pub next: *mut Struct_ares_naptr_reply,
    pub flags: *mut ::libc::c_uchar,
    pub service: *mut ::libc::c_uchar,
    pub regexp: *mut ::libc::c_uchar,
    pub replacement: *mut ::libc::c_char,
    pub order: ::libc::c_ushort,
    pub preference: ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct_ares_naptr_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_naptr_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_soa_reply {
    pub nsname: *mut ::libc::c_char,
    pub hostmaster: *mut ::libc::c_char,
    pub serial: ::libc::c_uint,
    pub refresh: ::libc::c_uint,
    pub retry: ::libc::c_uint,
    pub expire: ::libc::c_uint,
    pub minttl: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_ares_soa_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_soa_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ares_addr_node {
    pub next: *mut Struct_ares_addr_node,
    pub family: ::libc::c_int,
    pub addr: Union_Unnamed2,
}
impl ::std::clone::Clone for Struct_ares_addr_node {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ares_addr_node {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed2 {
    pub _bindgen_data_: [u32; 4usize],
}
impl Union_Unnamed2 {
    pub unsafe fn addr4(&mut self) -> *mut Struct_in_addr {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn addr6(&mut self) -> *mut Struct_ares_in6_addr {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn ares_library_init(flags: ::libc::c_int) -> ::libc::c_int;
    pub fn ares_library_cleanup() -> ();
    pub fn ares_version(version: *mut ::libc::c_int) -> *const ::libc::c_char;
    pub fn ares_init(channelptr: *mut ares_channel) -> ::libc::c_int;
    pub fn ares_init_options(channelptr: *mut ares_channel,
                             options: *mut Struct_ares_options,
                             optmask: ::libc::c_int) -> ::libc::c_int;
    pub fn ares_save_options(channel: ares_channel,
                             options: *mut Struct_ares_options,
                             optmask: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ares_destroy_options(options: *mut Struct_ares_options) -> ();
    pub fn ares_dup(dest: *mut ares_channel, src: ares_channel)
     -> ::libc::c_int;
    pub fn ares_destroy(channel: ares_channel) -> ();
    pub fn ares_cancel(channel: ares_channel) -> ();
    pub fn ares_set_local_ip4(channel: ares_channel, local_ip: ::libc::c_uint)
     -> ();
    pub fn ares_set_local_ip6(channel: ares_channel,
                              local_ip6: *const ::libc::c_uchar) -> ();
    pub fn ares_set_local_dev(channel: ares_channel,
                              local_dev_name: *const ::libc::c_char) -> ();
    pub fn ares_set_socket_callback(channel: ares_channel,
                                    callback: ares_sock_create_callback,
                                    user_data: *mut ::libc::c_void) -> ();
    pub fn ares_send(channel: ares_channel, qbuf: *const ::libc::c_uchar,
                     qlen: ::libc::c_int, callback: ares_callback,
                     arg: *mut ::libc::c_void) -> ();
    pub fn ares_query(channel: ares_channel, name: *const ::libc::c_char,
                      dnsclass: ::libc::c_int, _type: ::libc::c_int,
                      callback: ares_callback, arg: *mut ::libc::c_void)
     -> ();
    pub fn ares_search(channel: ares_channel, name: *const ::libc::c_char,
                       dnsclass: ::libc::c_int, _type: ::libc::c_int,
                       callback: ares_callback, arg: *mut ::libc::c_void)
     -> ();
    pub fn ares_gethostbyname(channel: ares_channel,
                              name: *const ::libc::c_char,
                              family: ::libc::c_int,
                              callback: ares_host_callback,
                              arg: *mut ::libc::c_void) -> ();
    pub fn ares_gethostbyname_file(channel: ares_channel,
                                   name: *const ::libc::c_char,
                                   family: ::libc::c_int,
                                   host: *mut *mut Struct_hostent)
     -> ::libc::c_int;
    pub fn ares_gethostbyaddr(channel: ares_channel,
                              addr: *const ::libc::c_void,
                              addrlen: ::libc::c_int, family: ::libc::c_int,
                              callback: ares_host_callback,
                              arg: *mut ::libc::c_void) -> ();
    pub fn ares_getnameinfo(channel: ares_channel, sa: *const Struct_sockaddr,
                            salen: ares_socklen_t, flags: ::libc::c_int,
                            callback: ares_nameinfo_callback,
                            arg: *mut ::libc::c_void) -> ();
    pub fn ares_fds(channel: *const Struct_ares_channeldata,
                    read_fds: *mut fd_set, write_fds: *mut fd_set)
     -> ::libc::c_int;
    pub fn ares_getsock(channel: *const Struct_ares_channeldata,
                        socks: *mut ares_socket_t, numsocks: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ares_timeout(channel: ares_channel, maxtv: *mut Struct_timeval,
                        tv: *mut Struct_timeval) -> *mut Struct_timeval;
    pub fn ares_process(channel: ares_channel, read_fds: *mut fd_set,
                        write_fds: *mut fd_set) -> ();
    pub fn ares_process_fd(channel: ares_channel, read_fd: ares_socket_t,
                           write_fd: ares_socket_t) -> ();
    pub fn ares_create_query(name: *const ::libc::c_char,
                             dnsclass: ::libc::c_int, _type: ::libc::c_int,
                             id: ::libc::c_ushort, rd: ::libc::c_int,
                             buf: *mut *mut ::libc::c_uchar,
                             buflen: *mut ::libc::c_int,
                             max_udp_size: ::libc::c_int) -> ::libc::c_int;
    pub fn ares_mkquery(name: *const ::libc::c_char, dnsclass: ::libc::c_int,
                        _type: ::libc::c_int, id: ::libc::c_ushort,
                        rd: ::libc::c_int, buf: *mut *mut ::libc::c_uchar,
                        buflen: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ares_expand_name(encoded: *const ::libc::c_uchar,
                            abuf: *const ::libc::c_uchar, alen: ::libc::c_int,
                            s: *mut *mut ::libc::c_char,
                            enclen: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn ares_expand_string(encoded: *const ::libc::c_uchar,
                              abuf: *const ::libc::c_uchar,
                              alen: ::libc::c_int,
                              s: *mut *mut ::libc::c_uchar,
                              enclen: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn ares_parse_a_reply(abuf: *const ::libc::c_uchar,
                              alen: ::libc::c_int,
                              host: *mut *mut Struct_hostent,
                              addrttls: *mut Struct_ares_addrttl,
                              naddrttls: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ares_parse_aaaa_reply(abuf: *const ::libc::c_uchar,
                                 alen: ::libc::c_int,
                                 host: *mut *mut Struct_hostent,
                                 addrttls: *mut Struct_ares_addr6ttl,
                                 naddrttls: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn ares_parse_ptr_reply(abuf: *const ::libc::c_uchar,
                                alen: ::libc::c_int,
                                addr: *const ::libc::c_void,
                                addrlen: ::libc::c_int, family: ::libc::c_int,
                                host: *mut *mut Struct_hostent)
     -> ::libc::c_int;
    pub fn ares_parse_ns_reply(abuf: *const ::libc::c_uchar,
                               alen: ::libc::c_int,
                               host: *mut *mut Struct_hostent)
     -> ::libc::c_int;
    pub fn ares_parse_srv_reply(abuf: *const ::libc::c_uchar,
                                alen: ::libc::c_int,
                                srv_out: *mut *mut Struct_ares_srv_reply)
     -> ::libc::c_int;
    pub fn ares_parse_mx_reply(abuf: *const ::libc::c_uchar,
                               alen: ::libc::c_int,
                               mx_out: *mut *mut Struct_ares_mx_reply)
     -> ::libc::c_int;
    pub fn ares_parse_txt_reply(abuf: *const ::libc::c_uchar,
                                alen: ::libc::c_int,
                                txt_out: *mut *mut Struct_ares_txt_reply)
     -> ::libc::c_int;
    pub fn ares_parse_naptr_reply(abuf: *const ::libc::c_uchar,
                                  alen: ::libc::c_int,
                                  naptr_out:
                                      *mut *mut Struct_ares_naptr_reply)
     -> ::libc::c_int;
    pub fn ares_parse_soa_reply(abuf: *const ::libc::c_uchar,
                                alen: ::libc::c_int,
                                soa_out: *mut *mut Struct_ares_soa_reply)
     -> ::libc::c_int;
    pub fn ares_free_string(str: *mut ::libc::c_void) -> ();
    pub fn ares_free_hostent(host: *mut Struct_hostent) -> ();
    pub fn ares_free_data(dataptr: *mut ::libc::c_void) -> ();
    pub fn ares_strerror(code: ::libc::c_int) -> *const ::libc::c_char;
    pub fn ares_set_servers(channel: ares_channel,
                            servers: *mut Struct_ares_addr_node)
     -> ::libc::c_int;
    pub fn ares_set_servers_csv(channel: ares_channel,
                                servers: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn ares_get_servers(channel: ares_channel,
                            servers: *mut *mut Struct_ares_addr_node)
     -> ::libc::c_int;
    pub fn ares_inet_ntop(af: ::libc::c_int, src: *const ::libc::c_void,
                          dst: *mut ::libc::c_char, size: ares_socklen_t)
     -> *const ::libc::c_char;
    pub fn ares_inet_pton(af: ::libc::c_int, src: *const ::libc::c_char,
                          dst: *mut ::libc::c_void) -> ::libc::c_int;
}
