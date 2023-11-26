# Railway

Small, fast and opinionated wayland client in pure Rust designed for:

- Low overhead (0 dependecies, except `ash` :D)
- Single threaded event loops ( `WaylandClient` is not thread safe )
- FreeBSD (and maybe Linux)
- Static linking and LTO

Build with [wayland-xml2code](../wayland-xml2code/)


## Example

See [example](examples/basic.rs)

```
cargo run --example basic

```

## Credits

- https://wayland-book.com
- https://gitlab.freedesktop.org/mesa/vulkan-wsi-layer/-/tree/master/wsi/wayland

## Why

All modern GUI toolkits are written in C/C++

GTK is contributor-hostile and too linux-centric.

winit does not support layer shell: 
- https://github.com/rust-windowing/winit/issues/2582
- https://github.com/rust-windowing/winit/issues/2142

smithay toolkit is extremely complicated and macro-heavy

None of the existing solutions today support FreeBSD as a first-class citizenq