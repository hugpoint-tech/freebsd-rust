# libc-fbsd

FreeBSD specific libc crate.

## Why not default `libc` crate

Default `libc` crate has some disadvantages: 

- no support for sendmsg/recvmsg functionality needed to send descriptors
- too complicated to understand 
- freebsd support is not first-class
- changes are manually added instead of bindgen approach (https://github.com/rust-lang/libc/issues/423#issuecomment-465939354)
- no static linking support for freebsd

libc-fbsd was created to address all of these issues. 

