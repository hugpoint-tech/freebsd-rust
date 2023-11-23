use std::marker::*;
use std::os::raw::c_void;

//------------------------------------------------------------------------------
// CONSTANTS
//------------------------------------------------------------------------------
pub const SCM_RIGHTS: i32 = 1;

// MSG_
pub const MSG_OOB: i32 = 1;
pub const MSG_PEEK: i32 = 2;
pub const MSG_TRUNC: i32 = 16;
pub const MSG_WAITALL: i32 = 64;
pub const MSG_DONTWAIT: i32 = 128;
pub const MSG_CMSG_CLOEXEC: i32 = 262144;
pub const MSG_CTRUNC: i32 = 32;

pub const SOL_SOCKET: i32 = 65535;

// errors
pub const EAGAIN: i32 = 35;

// pub const MSG_DONTROUTE: i32 = 4;
// pub const MSG_EOR: i32 = 8;
// pub const MSG_EOF: u32 = 256;
// pub const MSG_NOTIFICATION: u32 = 8192;
// pub const MSG_NBIO: u32 = 16384;
// pub const MSG_COMPAT: u32 = 32768;
// pub const MSG_NOSIGNAL: u32 = 131072;
// pub const MSG_WAITFORONE: u32 = 524288;
//------------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// iovec is a byte buffer
pub struct IOVec<'a> {
    /// The pointer to the beginning of the buffer
    pub iov_base: *mut c_void,

    /// Length of the buffer
    pub iov_len: usize,

    /// PhantomData to bind IOVec to the lifetime of Buffer
    pub _phantom: PhantomData<&'a ()>,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CmsgHdr {
    pub cmsg_len: u32,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MsgHdr<'a> {
    /// optional address
    pub msg_name: *mut c_void,

    /// size of address
    pub msg_namelen: u32,

    /// scatter/gather array
    pub msg_iov: *mut IOVec<'a>,

    /// elements in msg_iov
    pub msg_iovlen: i32,

    /// ancillary data, see below
    pub msg_control: *mut c_void,

    /// ancillary data buffer len
    pub msg_controllen: u32,

    /// flags on received message
    pub msg_flags: i32,
}

extern "C" {

    pub fn recvmsg(s: i32, msghdr: *mut MsgHdr, flags: i32) -> isize;

    pub fn sendmsg(s: i32, msghdr: *const MsgHdr, flags: i32) -> isize;

    pub fn close(fd: i32) -> i32;

    #[cfg_attr(
        target_os = "freebsd",
        link_name = "__error"
    )]
    #[cfg_attr(
        any(
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "bitrig",
            target_os = "android"
        ),
        link_name = "__errno"
    )]
    #[cfg_attr(
        any(target_os = "linux", target_os = "redox"),
        link_name = "__errno_location"
    )]
    fn errno_location() -> *mut i32;

}

pub fn errno() -> i32 {
    unsafe {*errno_location()}
}

