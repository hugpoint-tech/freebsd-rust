use railway::connection::WaylandConnection;
use railway::types::enums::*;
use railway::types::events::*;
use railway::types::handler::EventHandler;
use railway::types::mempool::SharedMemory;
use railway::types::object::*;

use railway::renderer::*;

pub struct State {
    // Wayland Globals
    display: WlDisplay,
    registry: WlRegistry,
    compositor: WlCompositor,
    shm: WlShm,
    xdg_wm: XdgWmBase,
    zwp_linux_dmabuf: ZwpLinuxDmabufV1,

    // Wayland Objects
    wl_shm_pool: WlShmPool,
    wl_surface: WlSurface,
    buffer: WlBuffer,
    buffer_locked: bool,
    // buffer_passive: WlBuffer,
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel,

    // Other resources
    mempool: SharedMemory,

    offset: f32,
    last_frame: u32,
}

impl State {
    pub fn new(w: &WaylandConnection) -> Self {
        let display = w.get_display();
        let registry = display.get_registry(&w);

        let mut state = Self {
            display,
            registry,
            compositor: WlCompositor { id: 0 },
            shm: WlShm { id: 0 },

            xdg_wm: XdgWmBase { id: 0 },
            zwp_linux_dmabuf: ZwpLinuxDmabufV1 { id: 0 },

            wl_shm_pool: WlShmPool { id: 0 },
            wl_surface: WlSurface { id: 0 },
            buffer: WlBuffer { id: 0 },
            buffer_locked: false,
            //buffer_passive: WlBuffer { id: 0 },
            xdg_surface: XdgSurface { id: 0 },
            xdg_toplevel: XdgToplevel { id: 0 },

            mempool: SharedMemory::new(1920, 1080),
            offset: 0.0,
            last_frame: 0,
        };

        state.display.sync(w);

        w.send();
        w.recv();
        w.dispatch_events(&mut state);

        if state.compositor.id == 0 {
            panic!("compositor was not initialized")
        }

        if state.shm.id == 0 {
            panic!("shm was not initialized")
        }

        if state.xdg_wm.id == 0 {
            panic!("xdg_wm_base was not initialized")
        }

        if state.zwp_linux_dmabuf.id == 0 {
            panic!("zwp_linux_dmabuf was not initialized")
        }

        let feedback = state.zwp_linux_dmabuf.get_default_feedback(w);

        state.wl_shm_pool = state
            .shm
            .create_pool(w, state.mempool.fd, state.mempool.size);

        state.buffer = state.wl_shm_pool.create_buffer(
            w,
            state.mempool.buffer1_offset,
            state.mempool.width,
            state.mempool.height,
            state.mempool.stride,
            WlShmFormat::XRGB8888 as u32,
        );

        state.wl_surface = state.compositor.create_surface(w);
        state.xdg_surface = state.xdg_wm.get_xdg_surface(w, state.wl_surface);

        state.xdg_toplevel = state.xdg_surface.get_toplevel(w);
        state
            .xdg_toplevel
            .set_title(w, "Example client".to_string());
        state.wl_surface.commit(w);
        state.wl_surface.frame(w);
        //     framecb.set_listener(w, &Self::FrameCallbackListener);

        state
    }

    pub fn draw(&mut self) {
        use std::slice;
        let sharedmemory: &mut [u32] = unsafe {
            slice::from_raw_parts_mut(
                self.mempool.data as *mut u32,
                self.mempool.size as usize / 4,
            )
        };

        let offt: i32 = self.offset as i32 % 8;
        let data = &mut sharedmemory[..self.mempool.width as usize * self.mempool.height as usize];

        for y in 0..self.mempool.height {
            for x in 0..self.mempool.width {
                data[(y * self.mempool.width + x) as usize] =
                    if ((x + offt) + (y + offt) / 8 * 8) % 16 < 8 {
                        0xFF666666
                    } else {
                        0xFFEEEEEE
                    };
            }
        }
    }
}

impl EventHandler for State {
    fn on_wl_display_error(&mut self, event: WlDisplayErrorEvent, _connection: &WaylandConnection) {
        panic!("display error: code {}, {}", event.code, event.message);
    }

    fn on_wl_display_sync_done(
        &mut self,
        event: WlDisplaySyncDoneEvent,
        _connection: &WaylandConnection,
    ) {
        println!("display sync: {}", event.data);
    }

    fn on_wl_display_delete_id(
        &mut self,
        event: WlDisplayDeleteIdEvent,
        connection: &WaylandConnection,
    ) {
        connection.delete_object(event.id);
    }

    fn on_wl_registry_global(&mut self, e: WlRegistryGlobalEvent, c: &WaylandConnection) {
        // println!("registry listener: {:?}", e);
        if let Some(obj) = Object::from_str(&e.interface) {
            match obj {
                Object::WlCompositor => {
                    let id = self.registry.bind(c, e.name, e.interface, e.version);
                    self.compositor = WlCompositor { id: id.id };
                }
                Object::WlShm => {
                    let id = self.registry.bind(c, e.name, e.interface, e.version);
                    self.shm = WlShm { id: id.id };
                }
                Object::XdgWmBase => {
                    let id = self.registry.bind(c, e.name, e.interface, e.version);
                    self.xdg_wm = XdgWmBase { id: id.id };
                }
                Object::ZwpLinuxDmabufV1 => {
                    let id = self.registry.bind(c, e.name, e.interface, e.version);
                    self.zwp_linux_dmabuf = ZwpLinuxDmabufV1 { id: id.id };
                }
                _ => (),
            }
        }
    }

    fn on_wl_buffer_release(&mut self, _e: WlBufferReleaseEvent, _c: &WaylandConnection) {
        self.buffer_locked = false
    }

    fn on_xdg_surface_configure(&mut self, event: XdgSurfaceConfigureEvent, c: &WaylandConnection) {
        // TODO adjust effective window size?
        self.xdg_surface.ack_configure(c, event.serial);
        self.draw();
        self.wl_surface.attach(c, self.buffer, 0, 0);
        self.wl_surface.damage(c, 0, 0, i32::MAX, i32::MAX);
        self.wl_surface.commit(c);
    }

    fn on_xdg_wm_base_ping(&mut self, event: XdgWmBasePingEvent, c: &WaylandConnection) {
        self.xdg_wm.pong(c, event.serial);
    }

    fn on_wl_surface_frame_done(
        &mut self,
        event: WlSurfaceFrameDoneEvent,
        connection: &WaylandConnection,
    ) {
        self.wl_surface.frame(connection);

        if self.last_frame != 0 {
            let elapsed = event.data - self.last_frame;
            self.offset += (elapsed as f32) / 1000.0 * 24.0;
        }
        self.draw();
        if self.buffer_locked {
            panic!("Buffer is still locked!");
        }
        self.wl_surface.attach(connection, self.buffer, 0, 0);
        self.wl_surface.damage(connection, 0, 0, i32::MAX, i32::MAX);
        self.wl_surface.commit(connection);

        self.last_frame = event.data;
    }
}


// [1526437.477] zwp_linux_dmabuf_v1@5.get_default_feedback(new id zwp_linux_dmabuf_feedback_v1@3)
// [1526437.486]  -> zwp_linux_dmabuf_feedback_v1@3.main_device(array[8])
// [1526437.492]  -> zwp_linux_dmabuf_feedback_v1@3.format_table(fd 54, 5216)
// [1526437.495]  -> zwp_linux_dmabuf_feedback_v1@3.tranche_target_device(array[8])
fn main() {
    // let renderer = Renderer::new();

    let c = WaylandConnection::new();
    let mut state = State::new(&c);

    loop {
        c.send();
        c.recv();
        c.dispatch_events(&mut state);
    }
}
