pub mod events;
pub mod requests;
pub mod c_ffi;

use crate::connection::c_ffi::*;

use core::cell::UnsafeCell;
use crate::types::*;
use crate::types::buffer::*;
use crate::types::handler::EventHandler;

use core::ffi::CStr;

use std::os::unix::net::UnixStream;
use std::os::fd::RawFd;
use std::mem::size_of;
use std::env;

use std:: {
    os::unix::prelude::AsRawFd,
    path::PathBuf,
};


#[derive(Debug)]
/// Wayland connection.
/// This type presents an abstraction over the raw socket and allows exchanging
/// protocol defined messages: sending requests and receiving events
pub struct WaylandConnection {
    data: UnsafeCell<WaylandConnectionPrivate>
}

impl WaylandConnection {

    pub fn new() -> Self {
        let socket_name = env::var_os("WAYLAND_DISPLAY")
            .map(Into::<PathBuf>::into)
            .unwrap();

        let socket_path = if socket_name.is_absolute() {
            socket_name
        } else {
            let mut socket_path = env::var_os("XDG_RUNTIME_DIR")
                .map(Into::<PathBuf>::into)
                .unwrap();

            if !socket_path.is_absolute() {
                std::process::exit(1);
            }
            socket_path.push(socket_name);
            socket_path
        };

        let socket = UnixStream::connect(socket_path).unwrap();
        // socket.set_read_timeout(Some(Duration::new(7, 0))).expect("set timeout on socket failed");
        let mut objects = Vec::with_capacity(1024);
        objects.insert(0, Object::Null);
        objects.insert(1, Object::WlDisplay);
        Self {
            data: WaylandConnectionPrivate {
                recv_buf: Buffer::new(),
                send_buf: Buffer::new(),
                fd_buf: Buffer::new(),
                cmsg_recv_buf: Buffer::new(),
                cmsg_send_buf: Buffer::new(),
                objects,
                used_ids: Vec::with_capacity(10),
                socket,

                recv_pos: 0,
                fd_pos: 0,

                _recv_avg: 0,
                _recv_max: 0,
                _send_avg: 0,
                _send_max: 0,
            }.into()
        }
    }

    pub fn dispatch_events<T: EventHandler>(&self, state: &mut T) {
        let data = unsafe {&mut *self.data.get()};
        while data.recv_pos < data.recv_buf.len() {
            data.dispatch_event(self,state);
        }
    }

    pub fn send(&self) {
        let data = unsafe {&mut *self.data.get()};


        if data.send_buf.len() == 0 {
            return;
        }
        // println!("send: {} norm bytes", data.send_buf.len());
        // println!("send: {} cmsg bytes", data.cmsg_send_buf.len());

        let mut send_iov = data.send_buf.as_send_iovec();
        let cmsg_iov = data.cmsg_send_buf.as_send_iovec();

        // println!("send: send_iov len {}", send_iov.iov_len);
        // println!("send: cmsg_iov len {}", cmsg_iov.iov_len);

        let msg = MsgHdr {
            msg_name: std::ptr::null_mut(),
            msg_namelen: 0,
            msg_iov: &mut send_iov as *mut IOVec,
            msg_iovlen: 1,
            msg_control: cmsg_iov.iov_base,
            msg_controllen: cmsg_iov.iov_len as u32,
            msg_flags: 0
        };
        let sent = unsafe {
            sendmsg(data.socket.as_raw_fd(), &msg as *const MsgHdr, 0)
        };
        if sent < 0 {
            panic!("send: errno {}" , errno());
        } else {
            debug_assert_eq!(sent as usize, data.send_buf.len());
        }

        data.send_buf.clear();
        data.cmsg_send_buf.clear();
    }

    pub fn recv(&self) {
        let data = unsafe {&mut *self.data.get()};

        data.recv_buf.clear();
        data.fd_buf.clear();
        data.cmsg_recv_buf.clear();
        data.recv_pos = 0;
        data.fd_pos = 0;

        let mut recv_iov = data.recv_buf.as_recv_iovec();
        let cmgs_iov = data.cmsg_recv_buf.as_recv_iovec();

        let mut msg = MsgHdr {
            msg_name: std::ptr::null_mut(),
            msg_namelen: 0,
            msg_iov: &mut recv_iov as *mut IOVec,
            msg_iovlen: 1,
            msg_control: cmgs_iov.iov_base,
            msg_controllen: cmgs_iov.iov_len as u32,
            msg_flags: 0
            // msg_flags: MSG_DONTWAIT + MSG_CMSG_CLOEXEC,
            // msg_flags: MSG_TRUNC | MSG_PEEK,
        };
        let bytes_read = unsafe { recvmsg(data.socket.as_raw_fd(), &mut msg, 0 )};
        if bytes_read < 0 {
            panic!("recv: errno {}" , errno());
        }

        // println!("recv: bytes read:    {} bytes", bytes_read );
        // println!("recv: msg flags {}", &msg.msg_flags);

        if bytes_read as usize == recv_iov.iov_len {
            panic!("DATA WAS TRUNCATED!")
        }
        if msg.msg_flags & MSG_CTRUNC != 0 {
            panic!("CONTROL DATA  WAS TRUNCATED!")
        }

        debug_assert!(bytes_read % 4 == 0, "Word padding is broken");

        data.recv_buf.set_len(bytes_read as usize);
        data.process_cmsgs();
        // println!("recv descriptors: {} bytes",data.fd_buf.len() );
    }

    pub fn get_display(&self) -> WlDisplay {
        WlDisplay {
            id: 1
        }
    }

    pub fn update_object(&self, id: u32, obj: Object) {
        let data = unsafe {&mut *self.data.get()};

        // TODO check backfil queue for reused indices
        let oldobj = data.objects.get(id as usize)
            .expect("failed to retrieve object with given id: no such id");
        let t1 = std::mem::discriminant(&obj);
        let t2 = std::mem::discriminant(oldobj);
        if t1 != t2 {
            panic!("object update failed, store has object of a different type");
        }
        data.objects[id as usize] = obj
    }

    pub fn delete_object(&self, id: u32) {
        let data = unsafe {&mut *self.data.get()};
        data.objects[id as usize] = Object::Null;
        data.used_ids.push(id);
    }
}




#[derive(Debug)]
struct WaylandConnectionPrivate {
    objects: Vec<Object>,
    used_ids: Vec<u32>,

    recv_buf: Buffer<16_384>,
    send_buf: Buffer<16_384>,
    fd_buf: Buffer<256>,

    cmsg_recv_buf: Buffer<512>,
    cmsg_send_buf: Buffer<512>,

    recv_pos: usize,
    fd_pos: usize,

    socket: UnixStream,

    // stats
    _recv_avg: usize,
    _recv_max: usize,
    _send_avg: usize,
    _send_max: usize,
}

impl WaylandConnectionPrivate {

    fn allocate_id(&mut self, obj: Object) -> u32 {
        if let Some(id) = self.used_ids.pop() {
            self.objects[id as usize] = obj;
            id
        } else {
            self.objects.push(obj);
            (self.objects.len() - 1) as u32
            //(self.objects.len() - 1).try_into().unwrap()
        }
    }

    /// Write wayland message header into the buffer
    fn write_header(&mut self, hdr: MessageHeader, pos: usize) {
        let hdr_bytes = hdr.into_bytes();
        let data = self.send_buf.as_storage();
        data[pos..pos+hdr_bytes.len()].copy_from_slice(&hdr_bytes);
        self.send_buf.set_len(std::cmp::max(pos+8, self.send_buf.len()));
    }

    /// Write wayland string into the buffer
    fn write_string(&mut self, s: String) {
        let mut len_bytes = s.len();
        // add 1 byte for null-byte terminator
        len_bytes += 1;
        let padded_len:u32 = align32(len_bytes) as u32;

        // First word of the string is it's size
        self.send_buf.extend(&padded_len.to_ne_bytes());
        self.send_buf.extend(&s.into_bytes());

        // fill the remaining bytes with nulls
        while len_bytes <= padded_len as usize {
            self.send_buf.push(0u8);
            len_bytes += 1;
        }
        debug_assert_eq!(self.send_buf.len() % 4, 0);
    }

    /// Write wayland array into the buffer
    /// Starts with 32-bit array size in bytes,
    /// followed by the array contents verbatim,
    /// and finally padding to a 32-bit boundary.
    fn write_array(&mut self, v: Vec<u8>) {
        let mut len = v.len();
        let padded_len:u32 = align32(len) as u32;

        // First word of the string is it's size
        self.send_buf.extend(&padded_len.to_ne_bytes());
        self.send_buf.extend(&v);

        // fill the remaining bytes with nulls
        while len <= padded_len as usize {
            self.send_buf.push(0u8);
            len += 1;
        }
        debug_assert_eq!(self.send_buf.len() % 4, 0);
    }

    fn write_uint(&mut self, u: u32) {
        self.send_buf.extend(&u.to_ne_bytes());
    }

    fn write_int(&mut self, i: i32) {
        self.send_buf.extend(&i.to_ne_bytes());
    }

    fn write_fd(&mut self, fd: RawFd) {

        let data = self.cmsg_send_buf.as_storage();
        let hdrlen = alignptr(size_of::<CmsgHdr>());
        let payloadlen = alignptr(size_of::<i32>());

        unsafe {
            let mut ptr = data.as_mut_ptr().add(self.cmsg_send_buf.len());
            let hdr = ptr as *mut CmsgHdr;
            (*hdr).cmsg_len = (hdrlen + payloadlen) as u32;
            (*hdr).cmsg_type = SCM_RIGHTS;
            (*hdr).cmsg_level = SOL_SOCKET;
            ptr = ptr.add(hdrlen);
            let payload = ptr as *mut RawFd;
            (*payload) = fd;
        }
        self.cmsg_send_buf.set_len(self.cmsg_send_buf.len() + hdrlen + payloadlen);

    }


    fn get_uint(&mut self) -> u32 {
        // debug_assert!(self.recv_buf.len() + size_of::<u32>() >= self.recv_pos);
        let data = &self.recv_buf.as_slice()[self.recv_pos..self.recv_pos + size_of::<u32>()];

        unsafe {
            let ptr = data.as_ptr() as *const u32;
            self.recv_pos += size_of::<u32>();
            return *ptr;
        }
    }

    fn get_fd(&mut self) -> RawFd {
        let data = &self.recv_buf.as_slice()[self.fd_pos..self.fd_pos + size_of::<i32>()];
        unsafe {
            let ptr = data.as_ptr() as *const i32;
            self.recv_pos += 4;
            return *ptr;
        }
    }

    fn get_int(&mut self) -> i32 {
        // debug_assert!(self.recv_buf.len() + size_of::<i32>() > self.recv_pos);
        let data = &self.recv_buf.as_slice()[self.recv_pos..self.recv_pos + size_of::<i32>()];
        unsafe {
            let ptr = data.as_ptr() as *const i32;
            self.recv_pos += size_of::<i32>();
            return *ptr;
        }
    }

    fn get_str(&mut self) -> String {
        // debug_assert!(self.recv_buf.len() > self.recv_pos);
        let len = self.get_uint() as usize;
        let data = &self.recv_buf.as_slice()[self.recv_pos..self.recv_pos + len];

        if len == 0 {
            return String::new();
        }

        let len_padded = align32(len) as usize;
        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(data)};

        self.recv_pos += len_padded;
        return cstr.to_str().expect("invalid UTF").to_owned()
    }

    fn get_vec(&mut self) -> Vec<u8> {
        // debug_assert!(self.recv_buf.len() > self.recv_pos);
        let len = self.get_uint() as usize;
        let data = &self.recv_buf.as_slice()[self.recv_pos..self.recv_pos + len];
        if len == 0 {
            return Vec::new();
        }
        let len_padded = align32(len) as usize;

        let result:Vec<u8> = data.into();

        self.recv_pos += len_padded;

        result
    }

    fn get_header(&mut self) -> MessageHeader {
        let word1 = self.get_uint();
        let word2 = self.get_uint();
        MessageHeader::from_words(word1, word2)
    }

    fn process_cmsgs(&mut self) {
        let data = self.recv_buf.as_storage();
        let mut pos = 0;

        let hdr_size = alignptr(size_of::<CmsgHdr>());

        loop {
            // recv() function guarantees that non-truncated data was receieved
            // We assume that the payload is >= 4 bytes
            if pos + hdr_size + size_of::<i32>() > self.cmsg_recv_buf.len() {
                break;
            }
            let hdr = unsafe { &*(data.as_ptr().add(pos) as *mut CmsgHdr) };

            if hdr.cmsg_level == SOL_SOCKET && hdr.cmsg_type == SCM_RIGHTS {
                let begin = pos + hdr_size;
                let end = pos + hdr.cmsg_len as usize;
                self.fd_buf.extend(&data[begin..end]);

                pos += alignptr(hdr.cmsg_len as usize);
            } else {
                break
            }
        }
    }

}


// Fixed to Double
// fn f2d(f: Fixed) -> f64 {
//     let int_part = f >> 8;
//     let frac_part = (f & 0xFF) as u32;
//     let frac_as_double = frac_part as f64 / 256.0;
//     let result = int_part as f64 + frac_as_double;
//
//     result
// }
