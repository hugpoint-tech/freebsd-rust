# Railway

Small, fast and opinionated wayland client in pure Rust designed for:

- Low overhead ( 0 dependecies)
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
