use crate::connection::c_ffi::*;
use core::mem::size_of;

use core::marker::*;
use core::ffi::c_void;

pub const fn align32(len: usize) -> u16 {
    (len as u16 + 3) & 0xFFFC
}

pub const fn alignptr(len: usize) -> usize {
    len + size_of::<usize>() - 1 & !(size_of::<usize>() - 1)
}


/// Stack allocated buffer that handles u32 alignment
/// Any write operation resets the read pos.
#[derive(Debug)]
pub struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {

    pub fn new() -> Self {
        Buffer {
            data: [0; N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, item: u8) {
        debug_assert!(self.len < self.data.len(), "buffer size exceeded!");
        self.data[self.len] = item;
        self.len += 1;
    }

    // pub fn pop(&mut self, item: u8) -> Option<u8> {
    //     if self.write_pos == 0 {
    //         return None;
    //     }
    //     self.write_pos -= 1;
    //     self.read_pos = std::cmp::min(self.read_pos, self.write_pos);
    //     Some(self.data[self.write_pos])
    // }

    pub fn extend(&mut self, another: &[u8]) {
        for val in another {
            self.push(*val);
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Returns mutable slice the the entire storage
    /// This is useful when reading into buffer from a socket
    pub fn as_storage(&mut self) -> &mut[u8] {
        &mut self.data[..self.len]
    }

    pub fn set_len(&mut self, l: usize) {
        debug_assert!(l <= self.data.len(), "Length exceeds allocated size");
        self.len = l;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Represents send buffer as iovec, exposing only the written portion of
    /// the storage
    pub fn as_send_iovec<'a>(&'a self) -> IOVec<'a> {
        let ptr = if self.len() == 0 {
            std::ptr::null_mut()
        } else {
            self.data.as_ptr() as *mut c_void
        };

        IOVec {
            iov_base: ptr,
            iov_len: self.len(),
            _phantom: PhantomData,
        }
    }

    /// Represents recv buffer as iovec, exposing the entire storage for
    /// receiving data from the socket.
    pub fn as_recv_iovec<'a>(&'a mut self) -> IOVec<'a> {
        IOVec {
            iov_base: self.data.as_ptr() as *mut c_void,
            iov_len: self.capacity(),
            _phantom: PhantomData,
        }
    }
}

