use std::os::fd::RawFd;
use crate::connection::c_ffi::errno;
use core::ffi::c_void;

// Supported page sizes:
// 4096 bytes
// 2097152 bytes
// 1073741824 bytes

pub const SHM_ANON: *const u8 = 1 as  *const u8;

pub const SHM_LARGEPAGE_ALLOC_DEFAULT: i32 = 0;
pub const SHM_LARGEPAGE_ALLOC_NOWAIT: i32 = 1;
pub const SHM_LARGEPAGE_ALLOC_HARD: i32 = 2;

pub const PROT_READ: i32 = 1;
pub const PROT_WRITE: i32 = 2;

pub const MAP_SHARED: i32 = 1;

pub const MAP_FAILED: *const c_void = !0 as *const c_void;

pub const O_RDWR: i32 = 2;
pub const O_CREAT: i32 = 512;
pub const O_EXCL: i32 = 2048;



fn div_ceil(x: i32, y: i32) -> i32{
    (x + y - 1) / y
}

pub struct SharedMemory {
    pub width: i32,
    pub height: i32,
    pub stride: i32,

    pub buffer1_offset: i32,
    pub buffer2_offset: i32,
    pub fd: RawFd,
    pub size: i32,
    pub data: *const c_void,

}

impl SharedMemory {

    pub const PIXELSIZE:i32 =  4;
    const BUFFER_COUNT: i32 = 2;

    pub fn new(width: i32, height: i32) -> Self {

        let stride = width * Self::PIXELSIZE;

        let offset1 = 0;
        let offset2 = height * stride;

        let len = height * width * Self::PIXELSIZE * Self::BUFFER_COUNT;

        unsafe {
            let fd = shm_create_largepage(
                SHM_ANON,
                O_RDWR | O_CREAT | O_EXCL,
                1,
                SHM_LARGEPAGE_ALLOC_HARD,
                0600
            );
            println!("mempool fd: {}", fd);
            println!("mempool: errno {}" , errno());

            let pages = div_ceil(len, 2097152);
            let size = 2097152 * pages;

            println!("mempool: pages {}" , pages);

            println!("mempool: size {}" ,size );
            // let trunct = ftruncate(fd, s as i64);
            let res = ftruncate(fd, size as i64);
            println!("mempool: size set restult :{}", res);
            println!("mempool: errno {}" , errno());
            let data = mmap(
                std::ptr::null(),
                size as usize,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                0
            );

 // uint32_t *data = mmap(NULL, size,
 //            PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
 //    if (data == MAP_FAILED) {
 //        close(fd);
 //        return NULL;
 //    }

            return Self {
                width,
                height,
                stride,

                buffer1_offset: offset1,
                buffer2_offset: offset2,
                fd,
                size: size.try_into().unwrap(),
                data
            }
        }
    }
}


extern "C" {

    pub fn shm_create_largepage(
        path: *const u8,
        flags: i32,
        psind: i32,
        alloc_policy: i32,
        mode: u16,
    ) -> i32;

    /// RETURN VALUES
    /// Upon successful completion, the value 0 is returned; otherwise the value -1 is returned and
    /// the global variable errno is set to indicate the error.  If the file to be modified is not
    /// a directory or a regular file, the truncate() call has no effect and returns the value 0.
    pub fn ftruncate(
        fd: i32,
        length: i64
    ) -> i32;

    pub fn mmap (
        addr: *const c_void,
        len: usize,
        prot: i32,
        flags: i32,
        fd: RawFd,
        offset: i64
    ) -> *const c_void;

}

