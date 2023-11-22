use crate::types::Id;
use crate::connection::WaylandConnection;
use crate::types::Request;

use std::os::fd::RawFd;

#[derive(Debug, Clone, Copy)]
/// All wayland interfaces
pub enum Object {

    /// The 0 ID is reserved to represent a null or non-existent object
    Null,
    /// The xdg_wm_base interface is exposed as a global object enabling clients
    /// to turn their wl_surfaces into windows in a desktop environment. It
    /// defines the basic functionality needed for clients and the compositor to
    /// create windows that can be dragged, resized, maximized, etc, as well as
    /// creating transient windows such as popup menus.
    XdgWmBase,

    /// The xdg_positioner provides a collection of rules for the placement of a
    /// child surface relative to a parent surface. Rules can be defined to ensure
    /// the child surface remains within the visible area's borders, and to
    /// specify how the child surface changes its position, such as sliding along
    /// an axis, or flipping around a rectangle. These positioner-created rules are
    /// constrained by the requirement that a child surface must intersect with or
    /// be at least partially adjacent to its parent surface.
    ///
    /// See the various requests for details about possible rules.
    ///
    /// At the time of the request, the compositor makes a copy of the rules
    /// specified by the xdg_positioner. Thus, after the request is complete the
    /// xdg_positioner object can be destroyed or reused; further changes to the
    /// object will have no effect on previous usages.
    ///
    /// For an xdg_positioner object to be considered complete, it must have a
    /// non-zero size set by set_size, and a non-zero anchor rectangle set by
    /// set_anchor_rect. Passing an incomplete xdg_positioner object when
    /// positioning a surface raises an invalid_positioner error.
    XdgPositioner,

    /// An interface that may be implemented by a wl_surface, for
    /// implementations that provide a desktop-style user interface.
    ///
    /// It provides a base set of functionality required to construct user
    /// interface elements requiring management by the compositor, such as
    /// toplevel windows, menus, etc. The types of functionality are split into
    /// xdg_surface roles.
    ///
    /// Creating an xdg_surface does not set the role for a wl_surface. In order
    /// to map an xdg_surface, the client must create a role-specific object
    /// using, e.g., get_toplevel, get_popup. The wl_surface for any given
    /// xdg_surface can have at most one role, and may not be assigned any role
    /// not based on xdg_surface.
    ///
    /// A role must be assigned before any other requests are made to the
    /// xdg_surface object.
    ///
    /// The client must call wl_surface.commit on the corresponding wl_surface
    /// for the xdg_surface state to take effect.
    ///
    /// Creating an xdg_surface from a wl_surface which has a buffer attached or
    /// committed is a client error, and any attempts by a client to attach or
    /// manipulate a buffer prior to the first xdg_surface.configure call must
    /// also be treated as errors.
    ///
    /// After creating a role-specific object and setting it up, the client must
    /// perform an initial commit without any buffer attached. The compositor
    /// will reply with an xdg_surface.configure event. The client must
    /// acknowledge it and is then allowed to attach a buffer to map the surface.
    ///
    /// Mapping an xdg_surface-based role surface is defined as making it
    /// possible for the surface to be shown by the compositor. Note that
    /// a mapped surface is not guaranteed to be visible once it is mapped.
    ///
    /// For an xdg_surface to be mapped by the compositor, the following
    /// conditions must be met:
    /// (1) the client has assigned an xdg_surface-based role to the surface
    /// (2) the client has set and committed the xdg_surface state and the
    /// role-dependent state to the surface
    /// (3) the client has committed a buffer to the surface
    ///
    /// A newly-unmapped surface is considered to have met condition (1) out
    /// of the 3 required conditions for mapping a surface if its role surface
    /// has not been destroyed, i.e. the client must perform the initial commit
    /// again before attaching a buffer.
    XdgSurface,

    /// This interface defines an xdg_surface role which allows a surface to,
    /// among other things, set window-like properties such as maximize,
    /// fullscreen, and minimize, set application-specific metadata like title and
    /// id, and well as trigger user interactive operations such as interactive
    /// resize and move.
    ///
    /// Unmapping an xdg_toplevel means that the surface cannot be shown
    /// by the compositor until it is explicitly mapped again.
    /// All active operations (e.g., move, resize) are canceled and all
    /// attributes (e.g. title, state, stacking, ...) are discarded for
    /// an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to
    /// the state it had right after xdg_surface.get_toplevel. The client
    /// can re-map the toplevel by perfoming a commit without any buffer
    /// attached, waiting for a configure event and handling it as usual (see
    /// xdg_surface description).
    ///
    /// Attaching a null buffer to a toplevel unmaps the surface.
    XdgToplevel,

    /// A popup surface is a short-lived, temporary surface. It can be used to
    /// implement for example menus, popovers, tooltips and other similar user
    /// interface concepts.
    ///
    /// A popup can be made to take an explicit grab. See xdg_popup.grab for
    /// details.
    ///
    /// When the popup is dismissed, a popup_done event will be sent out, and at
    /// the same time the surface will be unmapped. See the xdg_popup.popup_done
    /// event for details.
    ///
    /// Explicitly destroying the xdg_popup object will also dismiss the popup and
    /// unmap the surface. Clients that want to dismiss the popup when another
    /// surface of their own is clicked should dismiss the popup using the destroy
    /// request.
    ///
    /// A newly created xdg_popup will be stacked on top of all previously created
    /// xdg_popup surfaces associated with the same xdg_toplevel.
    ///
    /// The parent of an xdg_popup must be mapped (see the xdg_surface
    /// description) before the xdg_popup itself.
    ///
    /// The client must call wl_surface.commit on the corresponding wl_surface
    /// for the xdg_popup state to take effect.
    XdgPopup,

    /// The core global object.  This is a special singleton object.  It
    /// is used for internal Wayland protocol features.
    WlDisplay,

    /// Typed wl_callback object
    WlDisplaySyncCallback,

    /// The singleton global registry object.  The server has a number of
    /// global objects that are available to all clients.  These objects
    /// typically represent an actual object in the server (for example,
    /// an input device) or they are singleton objects that provide
    /// extension functionality.
    ///
    /// When a client creates a registry object, the registry object
    /// will emit a global event for each global currently in the
    /// registry.  Globals come and go as a result of device or
    /// monitor hotplugs, reconfiguration or other events, and the
    /// registry will send out global and global_remove events to
    /// keep the client up to date with the changes.  To mark the end
    /// of the initial burst of events, the client can use the
    /// wl_display.sync request immediately after calling
    /// wl_display.get_registry.
    ///
    /// A client can bind to a global object by using the bind
    /// request.  This creates a client-side handle that lets the object
    /// emit events to the client and lets the client invoke requests on
    /// the object.
    WlRegistry,

    /// A compositor.  This object is a singleton global.  The
    /// compositor is in charge of combining the contents of multiple
    /// surfaces into one displayable output.
    WlCompositor,

    /// The wl_shm_pool object encapsulates a piece of memory shared
    /// between the compositor and client.  Through the wl_shm_pool
    /// object, the client can allocate shared memory wl_buffer objects.
    /// All objects created through the same pool share the same
    /// underlying mapped memory. Reusing the mapped memory avoids the
    /// setup/teardown overhead and is useful when interactively resizing
    /// a surface or for many small buffers.
    WlShmPool,

    /// A singleton global object that provides support for shared
    /// memory.
    ///
    /// Clients can create wl_shm_pool objects using the create_pool
    /// request.
    ///
    /// On binding the wl_shm object one or more format events
    /// are emitted to inform clients about the valid pixel formats
    /// that can be used for buffers.
    WlShm,

    /// A buffer provides the content for a wl_surface. Buffers are
    /// created through factory interfaces such as wl_shm, wp_linux_buffer_params
    /// (from the linux-dmabuf protocol extension) or similar. It has a width and
    /// a height and can be attached to a wl_surface, but the mechanism by which a
    /// client provides and updates the contents is defined by the buffer factory
    /// interface.
    ///
    /// If the buffer uses a format that has an alpha channel, the alpha channel
    /// is assumed to be premultiplied in the color channels unless otherwise
    /// specified.
    ///
    /// Note, because wl_buffer objects are created from multiple independent
    /// factory interfaces, the wl_buffer interface is frozen at version 1.
    WlBuffer,

    /// A wl_data_offer represents a piece of data offered for transfer
    /// by another client (the source client).  It is used by the
    /// copy-and-paste and drag-and-drop mechanisms.  The offer
    /// describes the different mime types that the data can be
    /// converted to and provides the mechanism for transferring the
    /// data directly from the source client.
    WlDataOffer,

    /// The wl_data_source object is the source side of a wl_data_offer.
    /// It is created by the source client in a data transfer and
    /// provides a way to describe the offered data and a way to respond
    /// to requests to transfer the data.
    WlDataSource,

    /// There is one wl_data_device per seat which can be obtained
    /// from the global wl_data_device_manager singleton.
    ///
    /// A wl_data_device provides access to inter-client data transfer
    /// mechanisms such as copy-and-paste and drag-and-drop.
    WlDataDevice,

    /// The wl_data_device_manager is a singleton global object that
    /// provides access to inter-client data transfer mechanisms such as
    /// copy-and-paste and drag-and-drop.  These mechanisms are tied to
    /// a wl_seat and this interface lets a client get a wl_data_device
    /// corresponding to a wl_seat.
    ///
    /// Depending on the version bound, the objects created from the bound
    /// wl_data_device_manager object will have different requirements for
    /// functioning properly. See wl_data_source.set_actions,
    /// wl_data_offer.accept and wl_data_offer.finish for details.
    WlDataDeviceManager,

    /// This interface is implemented by servers that provide
    /// desktop-style user interfaces.
    ///
    /// It allows clients to associate a wl_shell_surface with
    /// a basic surface.
    ///
    /// Note! This protocol is deprecated and not intended for production use.
    /// For desktop-style user interfaces, use xdg_shell. Compositors and clients
    /// should not implement this interface.
    WlShell,

    /// An interface that may be implemented by a wl_surface, for
    /// implementations that provide a desktop-style user interface.
    ///
    /// It provides requests to treat surfaces like toplevel, fullscreen
    /// or popup windows, move, resize or maximize them, associate
    /// metadata like title and class, etc.
    ///
    /// On the server side the object is automatically destroyed when
    /// the related wl_surface is destroyed. On the client side,
    /// wl_shell_surface_destroy() must be called before destroying
    /// the wl_surface object.
    WlShellSurface,

    /// A surface is a rectangular area that may be displayed on zero
    /// or more outputs, and shown any number of times at the compositor's
    /// discretion. They can present wl_buffers, receive user input, and
    /// define a local coordinate system.
    ///
    /// The size of a surface (and relative positions on it) is described
    /// in surface-local coordinates, which may differ from the buffer
    /// coordinates of the pixel content, in case a buffer_transform
    /// or a buffer_scale is used.
    ///
    /// A surface without a "role" is fairly useless: a compositor does
    /// not know where, when or how to present it. The role is the
    /// purpose of a wl_surface. Examples of roles are a cursor for a
    /// pointer (as set by wl_pointer.set_cursor), a drag icon
    /// (wl_data_device.start_drag), a sub-surface
    /// (wl_subcompositor.get_subsurface), and a window as defined by a
    /// shell protocol (e.g. wl_shell.get_shell_surface).
    ///
    /// A surface can have only one role at a time. Initially a
    /// wl_surface does not have a role. Once a wl_surface is given a
    /// role, it is set permanently for the whole lifetime of the
    /// wl_surface object. Giving the current role again is allowed,
    /// unless explicitly forbidden by the relevant interface
    /// specification.
    ///
    /// Surface roles are given by requests in other interfaces such as
    /// wl_pointer.set_cursor. The request should explicitly mention
    /// that this request gives a role to a wl_surface. Often, this
    /// request also creates a new protocol object that represents the
    /// role and adds additional functionality to wl_surface. When a
    /// client wants to destroy a wl_surface, they must destroy this role
    /// object before the wl_surface, otherwise a defunct_role_object error is
    /// sent.
    ///
    /// Destroying the role object does not remove the role from the
    /// wl_surface, but it may stop the wl_surface from "playing the role".
    /// For instance, if a wl_subsurface object is destroyed, the wl_surface
    /// it was created for will be unmapped and forget its position and
    /// z-order. It is allowed to create a wl_subsurface for the same
    /// wl_surface again, but it is not allowed to use the wl_surface as
    /// a cursor (cursor is a different role than sub-surface, and role
    /// switching is not allowed).
    WlSurface,

    /// Typed wl_callback object
    WlSurfaceFrameCallback,

    /// A seat is a group of keyboards, pointer and touch devices. This
    /// object is published as a global during start up, or when such a
    /// device is hot plugged.  A seat typically has a pointer and
    /// maintains a keyboard focus and a pointer focus.
    WlSeat,

    /// The wl_pointer interface represents one or more input devices,
    /// such as mice, which control the pointer location and pointer_focus
    /// of a seat.
    ///
    /// The wl_pointer interface generates motion, enter and leave
    /// events for the surfaces that the pointer is located over,
    /// and button and axis events for button presses, button releases
    /// and scrolling.
    WlPointer,

    /// The wl_keyboard interface represents one or more keyboards
    /// associated with a seat.
    WlKeyboard,

    /// The wl_touch interface represents a touchscreen
    /// associated with a seat.
    ///
    /// Touch interactions can consist of one or more contacts.
    /// For each contact, a series of events is generated, starting
    /// with a down event, followed by zero or more motion events,
    /// and ending with an up event. Events relating to the same
    /// contact point can be identified by the ID of the sequence.
    WlTouch,

    /// An output describes part of the compositor geometry.  The
    /// compositor works in the 'compositor coordinate system' and an
    /// output corresponds to a rectangular area in that space that is
    /// actually visible.  This typically corresponds to a monitor that
    /// displays part of the compositor space.  This object is published
    /// as global during start up, or when a monitor is hotplugged.
    WlOutput,

    /// A region object describes an area.
    ///
    /// Region objects are used to describe the opaque and input
    /// regions of a surface.
    WlRegion,

    /// The global interface exposing sub-surface compositing capabilities.
    /// A wl_surface, that has sub-surfaces associated, is called the
    /// parent surface. Sub-surfaces can be arbitrarily nested and create
    /// a tree of sub-surfaces.
    ///
    /// The root surface in a tree of sub-surfaces is the main
    /// surface. The main surface cannot be a sub-surface, because
    /// sub-surfaces must always have a parent.
    ///
    /// A main surface with its sub-surfaces forms a (compound) window.
    /// For window management purposes, this set of wl_surface objects is
    /// to be considered as a single window, and it should also behave as
    /// such.
    ///
    /// The aim of sub-surfaces is to offload some of the compositing work
    /// within a window from clients to the compositor. A prime example is
    /// a video player with decorations and video in separate wl_surface
    /// objects. This should allow the compositor to pass YUV video buffer
    /// processing to dedicated overlay hardware when possible.
    WlSubcompositor,

    /// An additional interface to a wl_surface object, which has been
    /// made a sub-surface. A sub-surface has one parent surface. A
    /// sub-surface's size and position are not limited to that of the parent.
    /// Particularly, a sub-surface is not automatically clipped to its
    /// parent's area.
    ///
    /// A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
    /// and the parent surface is mapped. The order of which one happens
    /// first is irrelevant. A sub-surface is hidden if the parent becomes
    /// hidden, or if a NULL wl_buffer is applied. These rules apply
    /// recursively through the tree of surfaces.
    ///
    /// The behaviour of a wl_surface.commit request on a sub-surface
    /// depends on the sub-surface's mode. The possible modes are
    /// synchronized and desynchronized, see methods
    /// wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
    /// mode caches the wl_surface state to be applied when the parent's
    /// state gets applied, and desynchronized mode applies the pending
    /// wl_surface state directly. A sub-surface is initially in the
    /// synchronized mode.
    ///
    /// Sub-surfaces also have another kind of state, which is managed by
    /// wl_subsurface requests, as opposed to wl_surface requests. This
    /// state includes the sub-surface position relative to the parent
    /// surface (wl_subsurface.set_position), and the stacking order of
    /// the parent and its sub-surfaces (wl_subsurface.place_above and
    /// .place_below). This state is applied when the parent surface's
    /// wl_surface state is applied, regardless of the sub-surface's mode.
    /// As the exception, set_sync and set_desync are effective immediately.
    ///
    /// The main surface can be thought to be always in desynchronized mode,
    /// since it does not have a parent in the sub-surfaces sense.
    ///
    /// Even if a sub-surface is in desynchronized mode, it will behave as
    /// in synchronized mode, if its parent surface behaves as in
    /// synchronized mode. This rule is applied recursively throughout the
    /// tree of surfaces. This means, that one can set a sub-surface into
    /// synchronized mode, and then assume that all its child and grand-child
    /// sub-surfaces are synchronized, too, without explicitly setting them.
    ///
    /// Destroying a sub-surface takes effect immediately. If you need to
    /// synchronize the removal of a sub-surface to the parent surface update,
    /// unmap the sub-surface first by attaching a NULL wl_buffer, update parent,
    /// and then destroy the sub-surface.
    ///
    /// If the parent wl_surface object is destroyed, the sub-surface is
    /// unmapped.
    WlSubsurface,

}

impl Object {

    pub fn from_str(s: &str) -> Option<Self> {
    use Object::*;
        match s {
            "xdg_wm_base" => Some(XdgWmBase),
            "xdg_positioner" => Some(XdgPositioner),
            "xdg_surface" => Some(XdgSurface),
            "xdg_toplevel" => Some(XdgToplevel),
            "xdg_popup" => Some(XdgPopup),
            "wl_display" => Some(WlDisplay),
            "wl_registry" => Some(WlRegistry),
            "wl_compositor" => Some(WlCompositor),
            "wl_shm_pool" => Some(WlShmPool),
            "wl_shm" => Some(WlShm),
            "wl_buffer" => Some(WlBuffer),
            "wl_data_offer" => Some(WlDataOffer),
            "wl_data_source" => Some(WlDataSource),
            "wl_data_device" => Some(WlDataDevice),
            "wl_data_device_manager" => Some(WlDataDeviceManager),
            "wl_shell" => Some(WlShell),
            "wl_shell_surface" => Some(WlShellSurface),
            "wl_surface" => Some(WlSurface),
            "wl_seat" => Some(WlSeat),
            "wl_pointer" => Some(WlPointer),
            "wl_keyboard" => Some(WlKeyboard),
            "wl_touch" => Some(WlTouch),
            "wl_output" => Some(WlOutput),
            "wl_region" => Some(WlRegion),
            "wl_subcompositor" => Some(WlSubcompositor),
            "wl_subsurface" => Some(WlSubsurface),
            _ => None
        }
    }
}



/// The xdg_wm_base interface is exposed as a global object enabling clients
/// to turn their wl_surfaces into windows in a desktop environment. It
/// defines the basic functionality needed for clients and the compositor to
/// create windows that can be dragged, resized, maximized, etc, as well as
/// creating transient windows such as popup menus.
#[derive(Clone, Copy)]
pub struct XdgWmBase{
    pub id: u32,
}

impl XdgWmBase {

    /// `xdg_wm_base:destroy` request
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgWmBaseDestroy {
                sendto: self.id,
            }
        );
    }

    /// `xdg_wm_base:create_positioner` request
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    pub fn r#create_positioner(&self, connection: &WaylandConnection) -> XdgPositioner {

        let _enq_id = connection.enqueue(
            Request::XdgWmBaseCreatePositioner {
                sendto: self.id,
            }
        );
        return XdgPositioner{
            id: _enq_id,
        };
    }

    /// `xdg_wm_base:get_xdg_surface` request
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    pub fn r#get_xdg_surface(&self, connection: &WaylandConnection, surface: WlSurface) -> XdgSurface {

        let _enq_id = connection.enqueue(
            Request::XdgWmBaseGetXdgSurface {
                sendto: self.id,
                surface: surface.id,
            }
        );
        return XdgSurface{
            id: _enq_id,
        };
    }

    /// `xdg_wm_base:pong` request
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    pub fn r#pong(&self, connection: &WaylandConnection, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgWmBasePong {
                sendto: self.id,
                serial,
            }
        );
    }
}


/// The xdg_positioner provides a collection of rules for the placement of a
/// child surface relative to a parent surface. Rules can be defined to ensure
/// the child surface remains within the visible area's borders, and to
/// specify how the child surface changes its position, such as sliding along
/// an axis, or flipping around a rectangle. These positioner-created rules are
/// constrained by the requirement that a child surface must intersect with or
/// be at least partially adjacent to its parent surface.
///
/// See the various requests for details about possible rules.
///
/// At the time of the request, the compositor makes a copy of the rules
/// specified by the xdg_positioner. Thus, after the request is complete the
/// xdg_positioner object can be destroyed or reused; further changes to the
/// object will have no effect on previous usages.
///
/// For an xdg_positioner object to be considered complete, it must have a
/// non-zero size set by set_size, and a non-zero anchor rectangle set by
/// set_anchor_rect. Passing an incomplete xdg_positioner object when
/// positioning a surface raises an invalid_positioner error.
#[derive(Clone, Copy)]
pub struct XdgPositioner{
    pub id: u32,
}

impl XdgPositioner {

    /// `xdg_positioner:destroy` request
    /// Notify the compositor that the xdg_positioner will no longer be used.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerDestroy {
                sendto: self.id,
            }
        );
    }

    /// `xdg_positioner:set_size` request
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    pub fn r#set_size(&self, connection: &WaylandConnection, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetSize {
                sendto: self.id,
                width,
                height,
            }
        );
    }

    /// `xdg_positioner:set_anchor_rect` request
    /// Specify the anchor rectangle within the parent surface that the child
    /// surface will be placed relative to. The rectangle is relative to the
    /// window geometry as defined by xdg_surface.set_window_geometry of the
    /// parent surface.
    ///
    /// When the xdg_positioner object is used to position a child surface, the
    /// anchor rectangle may not extend outside the window geometry of the
    /// positioned child's parent surface.
    ///
    /// If a negative size is set the invalid_input error is raised.
    pub fn r#set_anchor_rect(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetAnchorRect {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }

    /// `xdg_positioner:set_anchor` request
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    pub fn r#set_anchor(&self, connection: &WaylandConnection, anchor: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetAnchor {
                sendto: self.id,
                anchor,
            }
        );
    }

    /// `xdg_positioner:set_gravity` request
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    pub fn r#set_gravity(&self, connection: &WaylandConnection, gravity: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetGravity {
                sendto: self.id,
                gravity,
            }
        );
    }

    /// `xdg_positioner:set_constraint_adjustment` request
    /// Specify how the window should be positioned if the originally intended
    /// position caused the surface to be constrained, meaning at least
    /// partially outside positioning boundaries set by the compositor. The
    /// adjustment is set by constructing a bitmask describing the adjustment to
    /// be made when the surface is constrained on that axis.
    ///
    /// If no bit for one axis is set, the compositor will assume that the child
    /// surface should not change its position on that axis when constrained.
    ///
    /// If more than one bit for one axis is set, the order of how adjustments
    /// are applied is specified in the corresponding adjustment descriptions.
    ///
    /// The default adjustment is none.
    pub fn r#set_constraint_adjustment(&self, connection: &WaylandConnection, constraint_adjustment: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetConstraintAdjustment {
                sendto: self.id,
                constraint_adjustment,
            }
        );
    }

    /// `xdg_positioner:set_offset` request
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the surface. For
    /// example if the anchor of the anchor rectangle is at (x, y), the surface
    /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
    /// surface position will be (x + ox, y + oy). The offset position of the
    /// surface is the one used for constraint testing. See
    /// set_constraint_adjustment.
    ///
    /// An example use case is placing a popup menu on top of a user interface
    /// element, while aligning the user interface element of the parent surface
    /// with some user interface element placed somewhere in the popup surface.
    pub fn r#set_offset(&self, connection: &WaylandConnection, x: i32, y: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetOffset {
                sendto: self.id,
                x,
                y,
            }
        );
    }

    /// `xdg_positioner:set_reactive` request
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    pub fn r#set_reactive(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetReactive {
                sendto: self.id,
            }
        );
    }

    /// `xdg_positioner:set_parent_size` request
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    pub fn r#set_parent_size(&self, connection: &WaylandConnection, parent_width: i32, parent_height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetParentSize {
                sendto: self.id,
                parent_width,
                parent_height,
            }
        );
    }

    /// `xdg_positioner:set_parent_configure` request
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    pub fn r#set_parent_configure(&self, connection: &WaylandConnection, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPositionerSetParentConfigure {
                sendto: self.id,
                serial,
            }
        );
    }
}


/// An interface that may be implemented by a wl_surface, for
/// implementations that provide a desktop-style user interface.
///
/// It provides a base set of functionality required to construct user
/// interface elements requiring management by the compositor, such as
/// toplevel windows, menus, etc. The types of functionality are split into
/// xdg_surface roles.
///
/// Creating an xdg_surface does not set the role for a wl_surface. In order
/// to map an xdg_surface, the client must create a role-specific object
/// using, e.g., get_toplevel, get_popup. The wl_surface for any given
/// xdg_surface can have at most one role, and may not be assigned any role
/// not based on xdg_surface.
///
/// A role must be assigned before any other requests are made to the
/// xdg_surface object.
///
/// The client must call wl_surface.commit on the corresponding wl_surface
/// for the xdg_surface state to take effect.
///
/// Creating an xdg_surface from a wl_surface which has a buffer attached or
/// committed is a client error, and any attempts by a client to attach or
/// manipulate a buffer prior to the first xdg_surface.configure call must
/// also be treated as errors.
///
/// After creating a role-specific object and setting it up, the client must
/// perform an initial commit without any buffer attached. The compositor
/// will reply with an xdg_surface.configure event. The client must
/// acknowledge it and is then allowed to attach a buffer to map the surface.
///
/// Mapping an xdg_surface-based role surface is defined as making it
/// possible for the surface to be shown by the compositor. Note that
/// a mapped surface is not guaranteed to be visible once it is mapped.
///
/// For an xdg_surface to be mapped by the compositor, the following
/// conditions must be met:
/// (1) the client has assigned an xdg_surface-based role to the surface
/// (2) the client has set and committed the xdg_surface state and the
/// role-dependent state to the surface
/// (3) the client has committed a buffer to the surface
///
/// A newly-unmapped surface is considered to have met condition (1) out
/// of the 3 required conditions for mapping a surface if its role surface
/// has not been destroyed, i.e. the client must perform the initial commit
/// again before attaching a buffer.
#[derive(Clone, Copy)]
pub struct XdgSurface{
    pub id: u32,
}

impl XdgSurface {

    /// `xdg_surface:destroy` request
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgSurfaceDestroy {
                sendto: self.id,
            }
        );
    }

    /// `xdg_surface:get_toplevel` request
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    pub fn r#get_toplevel(&self, connection: &WaylandConnection) -> XdgToplevel {

        let _enq_id = connection.enqueue(
            Request::XdgSurfaceGetToplevel {
                sendto: self.id,
            }
        );
        return XdgToplevel{
            id: _enq_id,
        };
    }

    /// `xdg_surface:get_popup` request
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    pub fn r#get_popup(&self, connection: &WaylandConnection, parent: XdgSurface, positioner: XdgPositioner) -> XdgPopup {

        let _enq_id = connection.enqueue(
            Request::XdgSurfaceGetPopup {
                sendto: self.id,
                parent: parent.id,
                positioner: positioner.id,
            }
        );
        return XdgPopup{
            id: _enq_id,
        };
    }

    /// `xdg_surface:set_window_geometry` request
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows.
    ///
    /// The window geometry is double buffered, and will be applied at the
    /// time wl_surface.commit of the corresponding wl_surface is called.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    pub fn r#set_window_geometry(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgSurfaceSetWindowGeometry {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }

    /// `xdg_surface:ack_configure` request
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    pub fn r#ack_configure(&self, connection: &WaylandConnection, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgSurfaceAckConfigure {
                sendto: self.id,
                serial,
            }
        );
    }
}


/// This interface defines an xdg_surface role which allows a surface to,
/// among other things, set window-like properties such as maximize,
/// fullscreen, and minimize, set application-specific metadata like title and
/// id, and well as trigger user interactive operations such as interactive
/// resize and move.
///
/// Unmapping an xdg_toplevel means that the surface cannot be shown
/// by the compositor until it is explicitly mapped again.
/// All active operations (e.g., move, resize) are canceled and all
/// attributes (e.g. title, state, stacking, ...) are discarded for
/// an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to
/// the state it had right after xdg_surface.get_toplevel. The client
/// can re-map the toplevel by perfoming a commit without any buffer
/// attached, waiting for a configure event and handling it as usual (see
/// xdg_surface description).
///
/// Attaching a null buffer to a toplevel unmaps the surface.
#[derive(Clone, Copy)]
pub struct XdgToplevel{
    pub id: u32,
}

impl XdgToplevel {

    /// `xdg_toplevel:destroy` request
    /// This request destroys the role surface and unmaps the surface;
    /// see "Unmapping" behavior in interface section for details.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelDestroy {
                sendto: self.id,
            }
        );
    }

    /// `xdg_toplevel:set_parent` request
    /// Set the "parent" of this surface. This surface should be stacked
    /// above the parent surface and all other ancestor surfaces.
    ///
    /// Parent surfaces should be set on dialogs, toolboxes, or other
    /// "auxiliary" surfaces, so that the parent is raised when the dialog
    /// is raised.
    ///
    /// Setting a null parent for a child surface unsets its parent. Setting
    /// a null parent for a surface which currently has no parent is a no-op.
    ///
    /// Only mapped surfaces can have child surfaces. Setting a parent which
    /// is not mapped is equivalent to setting a null parent. If a surface
    /// becomes unmapped, its children's parent is set to the parent of
    /// the now-unmapped surface. If the now-unmapped surface has no parent,
    /// its children's parent is unset. If the now-unmapped surface becomes
    /// mapped again, its parent-child relationship is not restored.
    ///
    /// The parent toplevel must not be one of the child toplevel's
    /// descendants, and the parent must be different from the child toplevel,
    /// otherwise the invalid_parent protocol error is raised.
    pub fn r#set_parent(&self, connection: &WaylandConnection, parent: XdgToplevel)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetParent {
                sendto: self.id,
                parent: parent.id,
            }
        );
    }

    /// `xdg_toplevel:set_title` request
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    pub fn r#set_title(&self, connection: &WaylandConnection, title: String)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetTitle {
                sendto: self.id,
                title,
            }
        );
    }

    /// `xdg_toplevel:set_app_id` request
    /// Set an application identifier for the surface.
    ///
    /// The app ID identifies the general class of applications to which
    /// the surface belongs. The compositor can use this to group multiple
    /// surfaces together, or to determine how to launch a new application.
    ///
    /// For D-Bus activatable applications, the app ID is used as the D-Bus
    /// service name.
    ///
    /// The compositor shell will try to group application surfaces together
    /// by their app ID. As a best practice, it is suggested to select app
    /// ID's that match the basename of the application's .desktop file.
    /// For example, "org.freedesktop.FooViewer" where the .desktop file is
    /// "org.freedesktop.FooViewer.desktop".
    ///
    /// Like other properties, a set_app_id request can be sent after the
    /// xdg_toplevel has been mapped to update the property.
    ///
    /// See the desktop-entry specification [0] for more details on
    /// application identifiers and how they relate to well-known D-Bus
    /// names and .desktop files.
    ///
    /// [0] https://standards.freedesktop.org/desktop-entry-spec/
    pub fn r#set_app_id(&self, connection: &WaylandConnection, app_id: String)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetAppId {
                sendto: self.id,
                app_id,
            }
        );
    }

    /// `xdg_toplevel:show_window_menu` request
    /// Clients implementing client-side decorations might want to show
    /// a context menu when right-clicking on the decorations, giving the
    /// user a menu that they can use to maximize or minimize the window.
    ///
    /// This request asks the compositor to pop up such a window menu at
    /// the given position, relative to the local surface coordinates of
    /// the parent surface. There are no guarantees as to what menu items
    /// the window menu contains, or even if a window menu will be drawn
    /// at all.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event.
    pub fn r#show_window_menu(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32, x: i32, y: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelShowWindowMenu {
                sendto: self.id,
                seat: seat.id,
                serial,
                x,
                y,
            }
        );
    }

    /// `xdg_toplevel:move` request
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized), or if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    pub fn r#move(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelMove {
                sendto: self.id,
                seat: seat.id,
                serial,
            }
        );
    }

    /// `xdg_toplevel:resize` request
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// If triggered, the client will receive configure events with the
    /// "resize" state enum value and the expected sizes. See the "resize"
    /// enum value for more details about what is required. The client
    /// must also acknowledge configure events using "ack_configure". After
    /// the resize is completed, the client will receive another "configure"
    /// event without the resize state.
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    pub fn r#resize(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32, edges: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelResize {
                sendto: self.id,
                seat: seat.id,
                serial,
                edges,
            }
        );
    }

    /// `xdg_toplevel:set_max_size` request
    /// Set a maximum size for the window.
    ///
    /// The client can specify a maximum size so that the compositor does
    /// not try to configure the window beyond this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered. They will get applied
    /// on the next commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the maximum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a larger size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected maximum size in the given dimension.
    /// As a result, a client wishing to reset the maximum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a maximum size to be smaller than the minimum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width or height will result in a
    /// invalid_size error.
    pub fn r#set_max_size(&self, connection: &WaylandConnection, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetMaxSize {
                sendto: self.id,
                width,
                height,
            }
        );
    }

    /// `xdg_toplevel:set_min_size` request
    /// Set a minimum size for the window.
    ///
    /// The client can specify a minimum size so that the compositor does
    /// not try to configure the window below this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered. They will get applied
    /// on the next commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the minimum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a smaller size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected minimum size in the given dimension.
    /// As a result, a client wishing to reset the minimum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a minimum size to be larger than the maximum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width and height will result in a
    /// invalid_size error.
    pub fn r#set_min_size(&self, connection: &WaylandConnection, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetMinSize {
                sendto: self.id,
                width,
                height,
            }
        );
    }

    /// `xdg_toplevel:set_maximized` request
    /// Maximize the surface.
    ///
    /// After requesting that the surface should be maximized, the compositor
    /// will respond by emitting a configure event. Whether this configure
    /// actually sets the window maximized is subject to compositor policies.
    /// The client must then update its content, drawing in the configured
    /// state. The client must also acknowledge the configure when committing
    /// the new content (see ack_configure).
    ///
    /// It is up to the compositor to decide how and where to maximize the
    /// surface, for example which output and what region of the screen should
    /// be used.
    ///
    /// If the surface was already maximized, the compositor will still emit
    /// a configure event with the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    pub fn r#set_maximized(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetMaximized {
                sendto: self.id,
            }
        );
    }

    /// `xdg_toplevel:unset_maximized` request
    /// Unmaximize the surface.
    ///
    /// After requesting that the surface should be unmaximized, the compositor
    /// will respond by emitting a configure event. Whether this actually
    /// un-maximizes the window is subject to compositor policies.
    /// If available and applicable, the compositor will include the window
    /// geometry dimensions the window had prior to being maximized in the
    /// configure event. The client must then update its content, drawing it in
    /// the configured state. The client must also acknowledge the configure
    /// when committing the new content (see ack_configure).
    ///
    /// It is up to the compositor to position the surface after it was
    /// unmaximized; usually the position the surface had before maximizing, if
    /// applicable.
    ///
    /// If the surface was already not maximized, the compositor will still
    /// emit a configure event without the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    pub fn r#unset_maximized(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelUnsetMaximized {
                sendto: self.id,
            }
        );
    }

    /// `xdg_toplevel:set_fullscreen` request
    /// Make the surface fullscreen.
    ///
    /// After requesting that the surface should be fullscreened, the
    /// compositor will respond by emitting a configure event. Whether the
    /// client is actually put into a fullscreen state is subject to compositor
    /// policies. The client must also acknowledge the configure when
    /// committing the new content (see ack_configure).
    ///
    /// The output passed by the request indicates the client's preference as
    /// to which display it should be set fullscreen on. If this value is NULL,
    /// it's up to the compositor to choose which display will be used to map
    /// this surface.
    ///
    /// If the surface doesn't cover the whole output, the compositor will
    /// position the surface in the center of the output and compensate with
    /// with border fill covering the rest of the output. The content of the
    /// border fill is undefined, but should be assumed to be in some way that
    /// attempts to blend into the surrounding area (e.g. solid black).
    ///
    /// If the fullscreened surface is not opaque, the compositor must make
    /// sure that other screen content not part of the same surface tree (made
    /// up of subsurfaces, popups or similarly coupled surfaces) are not
    /// visible below the fullscreened surface.
    pub fn r#set_fullscreen(&self, connection: &WaylandConnection, output: WlOutput)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetFullscreen {
                sendto: self.id,
                output: output.id,
            }
        );
    }

    /// `xdg_toplevel:unset_fullscreen` request
    /// Make the surface no longer fullscreen.
    ///
    /// After requesting that the surface should be unfullscreened, the
    /// compositor will respond by emitting a configure event.
    /// Whether this actually removes the fullscreen state of the client is
    /// subject to compositor policies.
    ///
    /// Making a surface unfullscreen sets states for the surface based on the following:
    /// * the state(s) it may have had before becoming fullscreen
    /// * any state(s) decided by the compositor
    /// * any state(s) requested by the client while the surface was fullscreen
    ///
    /// The compositor may include the previous window geometry dimensions in
    /// the configure event, if applicable.
    ///
    /// The client must also acknowledge the configure when committing the new
    /// content (see ack_configure).
    pub fn r#unset_fullscreen(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelUnsetFullscreen {
                sendto: self.id,
            }
        );
    }

    /// `xdg_toplevel:set_minimized` request
    /// Request that the compositor minimize your surface. There is no
    /// way to know if the surface is currently minimized, nor is there
    /// any way to unset minimization on this surface.
    ///
    /// If you are looking to throttle redrawing when minimized, please
    /// instead use the wl_surface.frame event for this, as this will
    /// also work with live previews on windows in Alt-Tab, Expose or
    /// similar compositor features.
    pub fn r#set_minimized(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgToplevelSetMinimized {
                sendto: self.id,
            }
        );
    }
}


/// A popup surface is a short-lived, temporary surface. It can be used to
/// implement for example menus, popovers, tooltips and other similar user
/// interface concepts.
///
/// A popup can be made to take an explicit grab. See xdg_popup.grab for
/// details.
///
/// When the popup is dismissed, a popup_done event will be sent out, and at
/// the same time the surface will be unmapped. See the xdg_popup.popup_done
/// event for details.
///
/// Explicitly destroying the xdg_popup object will also dismiss the popup and
/// unmap the surface. Clients that want to dismiss the popup when another
/// surface of their own is clicked should dismiss the popup using the destroy
/// request.
///
/// A newly created xdg_popup will be stacked on top of all previously created
/// xdg_popup surfaces associated with the same xdg_toplevel.
///
/// The parent of an xdg_popup must be mapped (see the xdg_surface
/// description) before the xdg_popup itself.
///
/// The client must call wl_surface.commit on the corresponding wl_surface
/// for the xdg_popup state to take effect.
#[derive(Clone, Copy)]
pub struct XdgPopup{
    pub id: u32,
}

impl XdgPopup {

    /// `xdg_popup:destroy` request
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::XdgPopupDestroy {
                sendto: self.id,
            }
        );
    }

    /// `xdg_popup:grab` request
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    pub fn r#grab(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPopupGrab {
                sendto: self.id,
                seat: seat.id,
                serial,
            }
        );
    }

    /// `xdg_popup:reposition` request
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    pub fn r#reposition(&self, connection: &WaylandConnection, positioner: XdgPositioner, token: u32)  {

        let _enq_id = connection.enqueue(
            Request::XdgPopupReposition {
                sendto: self.id,
                positioner: positioner.id,
                token,
            }
        );
    }
}

/// Typed wl_callback object
#[derive(Clone, Copy)]
pub struct WlDisplaySyncCallback {
    pub id: u32,
}


/// The core global object.  This is a special singleton object.  It
/// is used for internal Wayland protocol features.
#[derive(Clone, Copy)]
pub struct WlDisplay{
    pub id: u32,
}

impl WlDisplay {

    /// `wl_display:sync` request
    /// The sync request asks the server to emit the 'done' event
    /// on the returned wl_callback object.  Since requests are
    /// handled in-order and events are delivered in-order, this can
    /// be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the event serial.
    pub fn r#sync(&self, connection: &WaylandConnection) -> WlDisplaySyncCallback {

        let _enq_id = connection.enqueue(
            Request::WlDisplaySync {
                sendto: self.id,
            }
        );
        return WlDisplaySyncCallback{
            id: _enq_id,
        };
    }

    /// `wl_display:get_registry` request
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    pub fn r#get_registry(&self, connection: &WaylandConnection) -> WlRegistry {

        let _enq_id = connection.enqueue(
            Request::WlDisplayGetRegistry {
                sendto: self.id,
            }
        );
        return WlRegistry{
            id: _enq_id,
        };
    }
}


/// The singleton global registry object.  The server has a number of
/// global objects that are available to all clients.  These objects
/// typically represent an actual object in the server (for example,
/// an input device) or they are singleton objects that provide
/// extension functionality.
///
/// When a client creates a registry object, the registry object
/// will emit a global event for each global currently in the
/// registry.  Globals come and go as a result of device or
/// monitor hotplugs, reconfiguration or other events, and the
/// registry will send out global and global_remove events to
/// keep the client up to date with the changes.  To mark the end
/// of the initial burst of events, the client can use the
/// wl_display.sync request immediately after calling
/// wl_display.get_registry.
///
/// A client can bind to a global object by using the bind
/// request.  This creates a client-side handle that lets the object
/// emit events to the client and lets the client invoke requests on
/// the object.
#[derive(Clone, Copy)]
pub struct WlRegistry{
    pub id: u32,
}

impl WlRegistry {

    /// `wl_registry:bind` request
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    pub fn r#bind(&self, connection: &WaylandConnection, name: u32, if_name: String, if_version: u32) -> Id {

        let _enq_id = connection.enqueue(
            Request::WlRegistryBind {
                sendto: self.id,
                name,
                if_name,
                if_version,
            }
        );
        return Id{
            id: _enq_id,
        };
    }
}


/// A compositor.  This object is a singleton global.  The
/// compositor is in charge of combining the contents of multiple
/// surfaces into one displayable output.
#[derive(Clone, Copy)]
pub struct WlCompositor{
    pub id: u32,
}

impl WlCompositor {

    /// `wl_compositor:create_surface` request
    /// Ask the compositor to create a new surface.
    pub fn r#create_surface(&self, connection: &WaylandConnection) -> WlSurface {

        let _enq_id = connection.enqueue(
            Request::WlCompositorCreateSurface {
                sendto: self.id,
            }
        );
        return WlSurface{
            id: _enq_id,
        };
    }

    /// `wl_compositor:create_region` request
    /// Ask the compositor to create a new region.
    pub fn r#create_region(&self, connection: &WaylandConnection) -> WlRegion {

        let _enq_id = connection.enqueue(
            Request::WlCompositorCreateRegion {
                sendto: self.id,
            }
        );
        return WlRegion{
            id: _enq_id,
        };
    }
}


/// The wl_shm_pool object encapsulates a piece of memory shared
/// between the compositor and client.  Through the wl_shm_pool
/// object, the client can allocate shared memory wl_buffer objects.
/// All objects created through the same pool share the same
/// underlying mapped memory. Reusing the mapped memory avoids the
/// setup/teardown overhead and is useful when interactively resizing
/// a surface or for many small buffers.
#[derive(Clone, Copy)]
pub struct WlShmPool{
    pub id: u32,
}

impl WlShmPool {

    /// `wl_shm_pool:create_buffer` request
    /// Create a wl_buffer object from the pool.
    ///
    /// The buffer is created offset bytes into the pool and has
    /// width and height as specified.  The stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning
    /// of the next.  The format is the pixel format of the buffer and
    /// must be one of those advertised through the wl_shm.format event.
    ///
    /// A buffer will keep a reference to the pool it was created from
    /// so it is valid to destroy the pool immediately after creating
    /// a buffer from it.
    pub fn r#create_buffer(&self, connection: &WaylandConnection, offset: i32, width: i32, height: i32, stride: i32, format: u32) -> WlBuffer {

        let _enq_id = connection.enqueue(
            Request::WlShmPoolCreateBuffer {
                sendto: self.id,
                offset,
                width,
                height,
                stride,
                format,
            }
        );
        return WlBuffer{
            id: _enq_id,
        };
    }

    /// `wl_shm_pool:destroy` request
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlShmPoolDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_shm_pool:resize` request
    /// This request will cause the server to remap the backing memory
    /// for the pool from the file descriptor passed when the pool was
    /// created, but using the new size.  This request can only be
    /// used to make the pool bigger.
    ///
    /// This request only changes the amount of bytes that are mmapped
    /// by the server and does not touch the file corresponding to the
    /// file descriptor passed at creation time. It is the client's
    /// responsibility to ensure that the file is at least as big as
    /// the new pool size.
    pub fn r#resize(&self, connection: &WaylandConnection, size: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlShmPoolResize {
                sendto: self.id,
                size,
            }
        );
    }
}


/// A singleton global object that provides support for shared
/// memory.
///
/// Clients can create wl_shm_pool objects using the create_pool
/// request.
///
/// On binding the wl_shm object one or more format events
/// are emitted to inform clients about the valid pixel formats
/// that can be used for buffers.
#[derive(Clone, Copy)]
pub struct WlShm{
    pub id: u32,
}

impl WlShm {

    /// `wl_shm:create_pool` request
    /// Create a new wl_shm_pool object.
    ///
    /// The pool can be used to create shared memory based buffer
    /// objects.  The server will mmap size bytes of the passed file
    /// descriptor, to use as backing memory for the pool.
    pub fn r#create_pool(&self, connection: &WaylandConnection, fd: RawFd, size: i32) -> WlShmPool {

        let _enq_id = connection.enqueue(
            Request::WlShmCreatePool {
                sendto: self.id,
                fd,
                size,
            }
        );
        return WlShmPool{
            id: _enq_id,
        };
    }
}


/// A buffer provides the content for a wl_surface. Buffers are
/// created through factory interfaces such as wl_shm, wp_linux_buffer_params
/// (from the linux-dmabuf protocol extension) or similar. It has a width and
/// a height and can be attached to a wl_surface, but the mechanism by which a
/// client provides and updates the contents is defined by the buffer factory
/// interface.
///
/// If the buffer uses a format that has an alpha channel, the alpha channel
/// is assumed to be premultiplied in the color channels unless otherwise
/// specified.
///
/// Note, because wl_buffer objects are created from multiple independent
/// factory interfaces, the wl_buffer interface is frozen at version 1.
#[derive(Clone, Copy)]
pub struct WlBuffer{
    pub id: u32,
}

impl WlBuffer {

    /// `wl_buffer:destroy` request
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlBufferDestroy {
                sendto: self.id,
            }
        );
    }
}


/// A wl_data_offer represents a piece of data offered for transfer
/// by another client (the source client).  It is used by the
/// copy-and-paste and drag-and-drop mechanisms.  The offer
/// describes the different mime types that the data can be
/// converted to and provides the mechanism for transferring the
/// data directly from the source client.
#[derive(Clone, Copy)]
pub struct WlDataOffer{
    pub id: u32,
}

impl WlDataOffer {

    /// `wl_data_offer:accept` request
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    ///
    /// For objects of version 2 or older, this request is used by the
    /// client to give feedback whether the client can receive the given
    /// mime type, or NULL if none is accepted; the feedback does not
    /// determine whether the drag-and-drop operation succeeds or not.
    ///
    /// For objects of version 3 or newer, this request determines the
    /// final result of the drag-and-drop operation. If the end result
    /// is that no mime types were accepted, the drag-and-drop operation
    /// will be cancelled and the corresponding drag source will receive
    /// wl_data_source.cancelled. Clients may still use this event in
    /// conjunction with wl_data_source.action for feedback.
    pub fn r#accept(&self, connection: &WaylandConnection, serial: u32, mime_type: String)  {

        let _enq_id = connection.enqueue(
            Request::WlDataOfferAccept {
                sendto: self.id,
                serial,
                mime_type,
            }
        );
    }

    /// `wl_data_offer:receive` request
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    ///
    /// This request may happen multiple times for different mime types,
    /// both before and after wl_data_device.drop. Drag-and-drop destination
    /// clients may preemptively fetch data or examine it more closely to
    /// determine acceptance.
    pub fn r#receive(&self, connection: &WaylandConnection, mime_type: String, fd: RawFd)  {

        let _enq_id = connection.enqueue(
            Request::WlDataOfferReceive {
                sendto: self.id,
                mime_type,
                fd,
            }
        );
    }

    /// `wl_data_offer:destroy` request
    /// Destroy the data offer.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlDataOfferDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_data_offer:finish` request
    /// Notifies the compositor that the drag destination successfully
    /// finished the drag-and-drop operation.
    ///
    /// Upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client.
    ///
    /// It is a client error to perform other requests than
    /// wl_data_offer.destroy after this one. It is also an error to perform
    /// this request after a NULL mime type has been set in
    /// wl_data_offer.accept or no action was received through
    /// wl_data_offer.action.
    ///
    /// If wl_data_offer.finish request is received for a non drag and drop
    /// operation, the invalid_finish protocol error is raised.
    pub fn r#finish(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlDataOfferFinish {
                sendto: self.id,
            }
        );
    }

    /// `wl_data_offer:set_actions` request
    /// Sets the actions that the destination side client supports for
    /// this operation. This request may trigger the emission of
    /// wl_data_source.action and wl_data_offer.action events if the compositor
    /// needs to change the selected action.
    ///
    /// This request can be called multiple times throughout the
    /// drag-and-drop operation, typically in response to wl_data_device.enter
    /// or wl_data_device.motion events.
    ///
    /// This request determines the final result of the drag-and-drop
    /// operation. If the end result is that no action is accepted,
    /// the drag source will receive wl_data_source.cancelled.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, and the preferred_action
    /// argument must only contain one of those values set, otherwise it
    /// will result in a protocol error.
    ///
    /// While managing an "ask" action, the destination drag-and-drop client
    /// may perform further wl_data_offer.receive requests, and is expected
    /// to perform one last wl_data_offer.set_actions request with a preferred
    /// action other than "ask" (and optionally wl_data_offer.accept) before
    /// requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. If the preferred action is not in the
    /// wl_data_offer.source_actions mask, an error will be raised.
    ///
    /// If the "ask" action is dismissed (e.g. user cancellation), the client
    /// is expected to perform wl_data_offer.destroy right away.
    ///
    /// This request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    pub fn r#set_actions(&self, connection: &WaylandConnection, dnd_actions: u32, preferred_action: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlDataOfferSetActions {
                sendto: self.id,
                dnd_actions,
                preferred_action,
            }
        );
    }
}


/// The wl_data_source object is the source side of a wl_data_offer.
/// It is created by the source client in a data transfer and
/// provides a way to describe the offered data and a way to respond
/// to requests to transfer the data.
#[derive(Clone, Copy)]
pub struct WlDataSource{
    pub id: u32,
}

impl WlDataSource {

    /// `wl_data_source:offer` request
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    pub fn r#offer(&self, connection: &WaylandConnection, mime_type: String)  {

        let _enq_id = connection.enqueue(
            Request::WlDataSourceOffer {
                sendto: self.id,
                mime_type,
            }
        );
    }

    /// `wl_data_source:destroy` request
    /// Destroy the data source.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlDataSourceDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_data_source:set_actions` request
    /// Sets the actions that the source side client supports for this
    /// operation. This request may trigger wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the
    /// selected action.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, otherwise it will result
    /// in a protocol error.
    ///
    /// This request must be made once only, and can only be made on sources
    /// used in drag-and-drop, so it must be performed before
    /// wl_data_device.start_drag. Attempting to use the source other than
    /// for drag-and-drop will raise a protocol error.
    pub fn r#set_actions(&self, connection: &WaylandConnection, dnd_actions: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlDataSourceSetActions {
                sendto: self.id,
                dnd_actions,
            }
        );
    }
}


/// There is one wl_data_device per seat which can be obtained
/// from the global wl_data_device_manager singleton.
///
/// A wl_data_device provides access to inter-client data transfer
/// mechanisms such as copy-and-paste and drag-and-drop.
#[derive(Clone, Copy)]
pub struct WlDataDevice{
    pub id: u32,
}

impl WlDataDevice {

    /// `wl_data_device:start_drag` request
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    ///
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally. If source is destroyed, the drag-and-drop session will be
    /// cancelled.
    ///
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    ///
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.attach request can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    ///
    /// The input region is ignored for wl_surfaces with the role of a
    /// drag-and-drop icon.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    pub fn r#start_drag(&self, connection: &WaylandConnection, source: WlDataSource, origin: WlSurface, icon: WlSurface, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlDataDeviceStartDrag {
                sendto: self.id,
                source: source.id,
                origin: origin.id,
                icon: icon.id,
                serial,
            }
        );
    }

    /// `wl_data_device:set_selection` request
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    pub fn r#set_selection(&self, connection: &WaylandConnection, source: WlDataSource, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlDataDeviceSetSelection {
                sendto: self.id,
                source: source.id,
                serial,
            }
        );
    }

    /// `wl_data_device:release` request
    /// This request destroys the data device.
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlDataDeviceRelease {
                sendto: self.id,
            }
        );
    }
}


/// The wl_data_device_manager is a singleton global object that
/// provides access to inter-client data transfer mechanisms such as
/// copy-and-paste and drag-and-drop.  These mechanisms are tied to
/// a wl_seat and this interface lets a client get a wl_data_device
/// corresponding to a wl_seat.
///
/// Depending on the version bound, the objects created from the bound
/// wl_data_device_manager object will have different requirements for
/// functioning properly. See wl_data_source.set_actions,
/// wl_data_offer.accept and wl_data_offer.finish for details.
#[derive(Clone, Copy)]
pub struct WlDataDeviceManager{
    pub id: u32,
}

impl WlDataDeviceManager {

    /// `wl_data_device_manager:create_data_source` request
    /// Create a new data source.
    pub fn r#create_data_source(&self, connection: &WaylandConnection) -> WlDataSource {

        let _enq_id = connection.enqueue(
            Request::WlDataDeviceManagerCreateDataSource {
                sendto: self.id,
            }
        );
        return WlDataSource{
            id: _enq_id,
        };
    }

    /// `wl_data_device_manager:get_data_device` request
    /// Create a new data device for a given seat.
    pub fn r#get_data_device(&self, connection: &WaylandConnection, seat: WlSeat) -> WlDataDevice {

        let _enq_id = connection.enqueue(
            Request::WlDataDeviceManagerGetDataDevice {
                sendto: self.id,
                seat: seat.id,
            }
        );
        return WlDataDevice{
            id: _enq_id,
        };
    }
}


/// This interface is implemented by servers that provide
/// desktop-style user interfaces.
///
/// It allows clients to associate a wl_shell_surface with
/// a basic surface.
///
/// Note! This protocol is deprecated and not intended for production use.
/// For desktop-style user interfaces, use xdg_shell. Compositors and clients
/// should not implement this interface.
#[derive(Clone, Copy)]
pub struct WlShell{
    pub id: u32,
}

impl WlShell {

    /// `wl_shell:get_shell_surface` request
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    pub fn r#get_shell_surface(&self, connection: &WaylandConnection, surface: WlSurface) -> WlShellSurface {

        let _enq_id = connection.enqueue(
            Request::WlShellGetShellSurface {
                sendto: self.id,
                surface: surface.id,
            }
        );
        return WlShellSurface{
            id: _enq_id,
        };
    }
}


/// An interface that may be implemented by a wl_surface, for
/// implementations that provide a desktop-style user interface.
///
/// It provides requests to treat surfaces like toplevel, fullscreen
/// or popup windows, move, resize or maximize them, associate
/// metadata like title and class, etc.
///
/// On the server side the object is automatically destroyed when
/// the related wl_surface is destroyed. On the client side,
/// wl_shell_surface_destroy() must be called before destroying
/// the wl_surface object.
#[derive(Clone, Copy)]
pub struct WlShellSurface{
    pub id: u32,
}

impl WlShellSurface {

    /// `wl_shell_surface:pong` request
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    pub fn r#pong(&self, connection: &WaylandConnection, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfacePong {
                sendto: self.id,
                serial,
            }
        );
    }

    /// `wl_shell_surface:move` request
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    pub fn r#move(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceMove {
                sendto: self.id,
                seat: seat.id,
                serial,
            }
        );
    }

    /// `wl_shell_surface:resize` request
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    pub fn r#resize(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32, edges: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceResize {
                sendto: self.id,
                seat: seat.id,
                serial,
                edges,
            }
        );
    }

    /// `wl_shell_surface:set_toplevel` request
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    pub fn r#set_toplevel(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetToplevel {
                sendto: self.id,
            }
        );
    }

    /// `wl_shell_surface:set_transient` request
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    pub fn r#set_transient(&self, connection: &WaylandConnection, parent: WlSurface, x: i32, y: i32, flags: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetTransient {
                sendto: self.id,
                parent: parent.id,
                x,
                y,
                flags,
            }
        );
    }

    /// `wl_shell_surface:set_fullscreen` request
    /// Map the surface as a fullscreen surface.
    ///
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    ///
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the client does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    ///
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    ///
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    ///
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    pub fn r#set_fullscreen(&self, connection: &WaylandConnection, method: u32, framerate: u32, output: WlOutput)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetFullscreen {
                sendto: self.id,
                method,
                framerate,
                output: output.id,
            }
        );
    }

    /// `wl_shell_surface:set_popup` request
    /// Map the surface as a popup.
    ///
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    ///
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    ///
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other client's window. A click
    /// in any of the client's surfaces is reported as normal, however,
    /// clicks in other clients' surfaces will be discarded and trigger
    /// the callback.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    pub fn r#set_popup(&self, connection: &WaylandConnection, seat: WlSeat, serial: u32, parent: WlSurface, x: i32, y: i32, flags: u32)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetPopup {
                sendto: self.id,
                seat: seat.id,
                serial,
                parent: parent.id,
                x,
                y,
                flags,
            }
        );
    }

    /// `wl_shell_surface:set_maximized` request
    /// Map the surface as a maximized surface.
    ///
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    ///
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop elements such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    ///
    /// The details depend on the compositor implementation.
    pub fn r#set_maximized(&self, connection: &WaylandConnection, output: WlOutput)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetMaximized {
                sendto: self.id,
                output: output.id,
            }
        );
    }

    /// `wl_shell_surface:set_title` request
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    pub fn r#set_title(&self, connection: &WaylandConnection, title: String)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetTitle {
                sendto: self.id,
                title,
            }
        );
    }

    /// `wl_shell_surface:set_class` request
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    pub fn r#set_class(&self, connection: &WaylandConnection, class_: String)  {

        let _enq_id = connection.enqueue(
            Request::WlShellSurfaceSetClass {
                sendto: self.id,
                class_,
            }
        );
    }
}

/// Typed wl_callback object
#[derive(Clone, Copy)]
pub struct WlSurfaceFrameCallback {
    pub id: u32,
}


/// A surface is a rectangular area that may be displayed on zero
/// or more outputs, and shown any number of times at the compositor's
/// discretion. They can present wl_buffers, receive user input, and
/// define a local coordinate system.
///
/// The size of a surface (and relative positions on it) is described
/// in surface-local coordinates, which may differ from the buffer
/// coordinates of the pixel content, in case a buffer_transform
/// or a buffer_scale is used.
///
/// A surface without a "role" is fairly useless: a compositor does
/// not know where, when or how to present it. The role is the
/// purpose of a wl_surface. Examples of roles are a cursor for a
/// pointer (as set by wl_pointer.set_cursor), a drag icon
/// (wl_data_device.start_drag), a sub-surface
/// (wl_subcompositor.get_subsurface), and a window as defined by a
/// shell protocol (e.g. wl_shell.get_shell_surface).
///
/// A surface can have only one role at a time. Initially a
/// wl_surface does not have a role. Once a wl_surface is given a
/// role, it is set permanently for the whole lifetime of the
/// wl_surface object. Giving the current role again is allowed,
/// unless explicitly forbidden by the relevant interface
/// specification.
///
/// Surface roles are given by requests in other interfaces such as
/// wl_pointer.set_cursor. The request should explicitly mention
/// that this request gives a role to a wl_surface. Often, this
/// request also creates a new protocol object that represents the
/// role and adds additional functionality to wl_surface. When a
/// client wants to destroy a wl_surface, they must destroy this role
/// object before the wl_surface, otherwise a defunct_role_object error is
/// sent.
///
/// Destroying the role object does not remove the role from the
/// wl_surface, but it may stop the wl_surface from "playing the role".
/// For instance, if a wl_subsurface object is destroyed, the wl_surface
/// it was created for will be unmapped and forget its position and
/// z-order. It is allowed to create a wl_subsurface for the same
/// wl_surface again, but it is not allowed to use the wl_surface as
/// a cursor (cursor is a different role than sub-surface, and role
/// switching is not allowed).
#[derive(Clone, Copy)]
pub struct WlSurface{
    pub id: u32,
}

impl WlSurface {

    /// `wl_surface:destroy` request
    /// Deletes the surface and invalidates its object ID.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_surface:attach` request
    /// Set a buffer as the content of this surface.
    ///
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that at commit time the supplied
    /// buffer size must be an integer multiple of the buffer_scale. If
    /// that's not the case, an invalid_size error is sent.
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes. Setting anything other than 0
    /// as x and y arguments is discouraged, and should instead be replaced
    /// with using the separate wl_surface.offset request.
    ///
    /// When the bound wl_surface version is 5 or higher, passing any
    /// non-zero x or y is a protocol violation, and will result in an
    /// 'invalid_offset' error being raised. The x and y arguments are ignored
    /// and do not change the pending state. To achieve equivalent semantics,
    /// use wl_surface.offset.
    ///
    /// Surface contents are double-buffered state, see wl_surface.commit.
    ///
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    ///
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may reuse the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    ///
    /// If a pending wl_buffer has been committed to more than one wl_surface,
    /// the delivery of wl_buffer.release events becomes undefined. A well
    /// behaved client should not rely on wl_buffer.release events in this
    /// case. Alternatively, a client could create multiple wl_buffer objects
    /// from the same backing storage or use wp_linux_buffer_release.
    ///
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. Destroying the wl_buffer before wl_buffer.release
    /// is allowed as long as the underlying buffer storage isn't re-used (this
    /// can happen e.g. on client process termination). However, if the client
    /// destroys the wl_buffer before receiving the wl_buffer.release event and
    /// mutates the underlying buffer storage, the surface contents become
    /// undefined immediately.
    ///
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    pub fn r#attach(&self, connection: &WaylandConnection, buffer: WlBuffer, x: i32, y: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceAttach {
                sendto: self.id,
                buffer: buffer.id,
                x,
                y,
            }
        );
    }

    /// `wl_surface:damage` request
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in surface-local coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// Note! New clients should not use this request. Instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates
    /// instead of surface coordinates.
    pub fn r#damage(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceDamage {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }

    /// `wl_surface:frame` request
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    pub fn r#frame(&self, connection: &WaylandConnection) -> WlSurfaceFrameCallback {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceFrame {
                sendto: self.id,
            }
        );
        return WlSurfaceFrameCallback{
            id: _enq_id,
        };
    }

    /// `wl_surface:set_opaque_region` request
    /// This request sets the region of the surface that contains
    /// opaque content.
    ///
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize the redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    ///
    /// The opaque region is specified in surface-local coordinates.
    ///
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    ///
    /// Opaque region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    ///
    /// The initial value for an opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    pub fn r#set_opaque_region(&self, connection: &WaylandConnection, region: WlRegion)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceSetOpaqueRegion {
                sendto: self.id,
                region: region.id,
            }
        );
    }

    /// `wl_surface:set_input_region` request
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    ///
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    ///
    /// The input region is specified in surface-local coordinates.
    ///
    /// Input region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    ///
    /// The initial value for an input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    pub fn r#set_input_region(&self, connection: &WaylandConnection, region: WlRegion)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceSetInputRegion {
                sendto: self.id,
                region: region.id,
            }
        );
    }

    /// `wl_surface:commit` request
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending state,
    /// as opposed to the current state in use by the compositor. A commit
    /// request atomically applies all pending state, replacing the current
    /// state. After commit, the new pending state is as documented for each
    /// related request.
    ///
    /// On commit, a pending wl_buffer is applied first, and all other state
    /// second. This means that all coordinates in double-buffered state are
    /// relative to the new wl_buffer coming into use, except for
    /// wl_surface.attach itself. If there is no pending wl_buffer, the
    /// coordinates are relative to the current surface contents.
    ///
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    ///
    /// Other interfaces may add further double-buffered surface state.
    pub fn r#commit(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceCommit {
                sendto: self.id,
            }
        );
    }

    /// `wl_surface:set_buffer_transform` request
    /// This request sets an optional transformation on how the compositor
    /// interprets the contents of the buffer attached to the surface. The
    /// accepted values for the transform parameter are the values for
    /// wl_output.transform.
    ///
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer transformation set to normal.
    ///
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    ///
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    ///
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    ///
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    pub fn r#set_buffer_transform(&self, connection: &WaylandConnection, transform: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceSetBufferTransform {
                sendto: self.id,
                transform,
            }
        );
    }

    /// `wl_surface:set_buffer_scale` request
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    ///
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer scale set to 1.
    ///
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    ///
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. It is
    /// intended that you pick the same buffer scale as the scale of the
    /// output that the surface is displayed on. This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    ///
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    ///
    /// If scale is not positive the invalid_scale protocol error is
    /// raised.
    pub fn r#set_buffer_scale(&self, connection: &WaylandConnection, scale: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceSetBufferScale {
                sendto: self.id,
                scale,
            }
        );
    }

    /// `wl_surface:damage_buffer` request
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in buffer coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage_buffer adds pending damage: the new pending
    /// damage is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// This request differs from wl_surface.damage in only one way - it
    /// takes damage in buffer coordinates instead of surface-local
    /// coordinates. While this generally is more intuitive than surface
    /// coordinates, it is especially desirable when using wp_viewport
    /// or when a drawing library (like EGL) is unaware of buffer scale
    /// and buffer transform.
    ///
    /// Note: Because buffer transformation changes and damage requests may
    /// be interleaved in the protocol stream, it is impossible to determine
    /// the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. Therefore, compositors wishing to take both
    /// kinds of damage into account will have to accumulate damage from the
    /// two requests separately and only transform from one to the other
    /// after receiving the wl_surface.commit.
    pub fn r#damage_buffer(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceDamageBuffer {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }

    /// `wl_surface:offset` request
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    ///
    /// Surface location offset is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// This request is semantically equivalent to and the replaces the x and y
    /// arguments in the wl_surface.attach request in wl_surface versions prior
    /// to 5. See wl_surface.attach for details.
    pub fn r#offset(&self, connection: &WaylandConnection, x: i32, y: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSurfaceOffset {
                sendto: self.id,
                x,
                y,
            }
        );
    }
}


/// A seat is a group of keyboards, pointer and touch devices. This
/// object is published as a global during start up, or when such a
/// device is hot plugged.  A seat typically has a pointer and
/// maintains a keyboard focus and a pointer focus.
#[derive(Clone, Copy)]
pub struct WlSeat{
    pub id: u32,
}

impl WlSeat {

    /// `wl_seat:get_pointer` request
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    pub fn r#get_pointer(&self, connection: &WaylandConnection) -> WlPointer {

        let _enq_id = connection.enqueue(
            Request::WlSeatGetPointer {
                sendto: self.id,
            }
        );
        return WlPointer{
            id: _enq_id,
        };
    }

    /// `wl_seat:get_keyboard` request
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    pub fn r#get_keyboard(&self, connection: &WaylandConnection) -> WlKeyboard {

        let _enq_id = connection.enqueue(
            Request::WlSeatGetKeyboard {
                sendto: self.id,
            }
        );
        return WlKeyboard{
            id: _enq_id,
        };
    }

    /// `wl_seat:get_touch` request
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    pub fn r#get_touch(&self, connection: &WaylandConnection) -> WlTouch {

        let _enq_id = connection.enqueue(
            Request::WlSeatGetTouch {
                sendto: self.id,
            }
        );
        return WlTouch{
            id: _enq_id,
        };
    }

    /// `wl_seat:release` request
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSeatRelease {
                sendto: self.id,
            }
        );
    }
}


/// The wl_pointer interface represents one or more input devices,
/// such as mice, which control the pointer location and pointer_focus
/// of a seat.
///
/// The wl_pointer interface generates motion, enter and leave
/// events for the surfaces that the pointer is located over,
/// and button and axis events for button presses, button releases
/// and scrolling.
#[derive(Clone, Copy)]
pub struct WlPointer{
    pub id: u32,
}

impl WlPointer {

    /// `wl_pointer:set_cursor` request
    /// Set the pointer surface, i.e., the surface that contains the
    /// pointer image (cursor). This request gives the surface the role
    /// of a cursor. If the surface already has another role, it raises
    /// a protocol error.
    ///
    /// The cursor actually changes only if the pointer
    /// focus for this device is one of the requesting client's surfaces
    /// or the surface parameter is the current pointer surface. If
    /// there was a previous surface set with this request it is
    /// replaced. If surface is NULL, the pointer image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of
    /// the pointer surface relative to the pointer location. Its
    /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    /// where (x, y) are the coordinates of the pointer location, in
    /// surface-local coordinates.
    ///
    /// On surface.attach requests to the pointer surface, hotspot_x
    /// and hotspot_y are decremented by the x and y parameters
    /// passed to the request. Attach must be confirmed by
    /// wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set
    /// pointer surface to this request with new values for hotspot_x
    /// and hotspot_y.
    ///
    /// The input region is ignored for wl_surfaces with the role of
    /// a cursor. When the use as a cursor ends, the wl_surface is
    /// unmapped.
    ///
    /// The serial parameter must match the latest wl_pointer.enter
    /// serial number sent to the client. Otherwise the request will be
    /// ignored.
    pub fn r#set_cursor(&self, connection: &WaylandConnection, serial: u32, surface: WlSurface, hotspot_x: i32, hotspot_y: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlPointerSetCursor {
                sendto: self.id,
                serial,
                surface: surface.id,
                hotspot_x,
                hotspot_y,
            }
        );
    }

    /// `wl_pointer:release` request
    /// Using this request a client can tell the server that it is not going to
    /// use the pointer object anymore.
    ///
    /// This request destroys the pointer proxy object, so clients must not call
    /// wl_pointer_destroy() after using this request.
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlPointerRelease {
                sendto: self.id,
            }
        );
    }
}


/// The wl_keyboard interface represents one or more keyboards
/// associated with a seat.
#[derive(Clone, Copy)]
pub struct WlKeyboard{
    pub id: u32,
}

impl WlKeyboard {

    /// `wl_keyboard:release` request
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlKeyboardRelease {
                sendto: self.id,
            }
        );
    }
}


/// The wl_touch interface represents a touchscreen
/// associated with a seat.
///
/// Touch interactions can consist of one or more contacts.
/// For each contact, a series of events is generated, starting
/// with a down event, followed by zero or more motion events,
/// and ending with an up event. Events relating to the same
/// contact point can be identified by the ID of the sequence.
#[derive(Clone, Copy)]
pub struct WlTouch{
    pub id: u32,
}

impl WlTouch {

    /// `wl_touch:release` request
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlTouchRelease {
                sendto: self.id,
            }
        );
    }
}


/// An output describes part of the compositor geometry.  The
/// compositor works in the 'compositor coordinate system' and an
/// output corresponds to a rectangular area in that space that is
/// actually visible.  This typically corresponds to a monitor that
/// displays part of the compositor space.  This object is published
/// as global during start up, or when a monitor is hotplugged.
#[derive(Clone, Copy)]
pub struct WlOutput{
    pub id: u32,
}

impl WlOutput {

    /// `wl_output:release` request
    /// Using this request a client can tell the server that it is not going to
    /// use the output object anymore.
    pub fn r#release(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlOutputRelease {
                sendto: self.id,
            }
        );
    }
}


/// A region object describes an area.
///
/// Region objects are used to describe the opaque and input
/// regions of a surface.
#[derive(Clone, Copy)]
pub struct WlRegion{
    pub id: u32,
}

impl WlRegion {

    /// `wl_region:destroy` request
    /// Destroy the region.  This will invalidate the object ID.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlRegionDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_region:add` request
    /// Add the specified rectangle to the region.
    pub fn r#add(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlRegionAdd {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }

    /// `wl_region:subtract` request
    /// Subtract the specified rectangle from the region.
    pub fn r#subtract(&self, connection: &WaylandConnection, x: i32, y: i32, width: i32, height: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlRegionSubtract {
                sendto: self.id,
                x,
                y,
                width,
                height,
            }
        );
    }
}


/// The global interface exposing sub-surface compositing capabilities.
/// A wl_surface, that has sub-surfaces associated, is called the
/// parent surface. Sub-surfaces can be arbitrarily nested and create
/// a tree of sub-surfaces.
///
/// The root surface in a tree of sub-surfaces is the main
/// surface. The main surface cannot be a sub-surface, because
/// sub-surfaces must always have a parent.
///
/// A main surface with its sub-surfaces forms a (compound) window.
/// For window management purposes, this set of wl_surface objects is
/// to be considered as a single window, and it should also behave as
/// such.
///
/// The aim of sub-surfaces is to offload some of the compositing work
/// within a window from clients to the compositor. A prime example is
/// a video player with decorations and video in separate wl_surface
/// objects. This should allow the compositor to pass YUV video buffer
/// processing to dedicated overlay hardware when possible.
#[derive(Clone, Copy)]
pub struct WlSubcompositor{
    pub id: u32,
}

impl WlSubcompositor {

    /// `wl_subcompositor:destroy` request
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other
    /// objects, wl_subsurface objects included.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSubcompositorDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_subcompositor:get_subsurface` request
    /// Create a sub-surface interface for the given surface, and
    /// associate it with the given parent surface. This turns a
    /// plain wl_surface into a sub-surface.
    ///
    /// The to-be sub-surface must not already have another role, and it
    /// must not have an existing wl_subsurface object. Otherwise the
    /// bad_surface protocol error is raised.
    ///
    /// Adding sub-surfaces to a parent is a double-buffered operation on the
    /// parent (see wl_surface.commit). The effect of adding a sub-surface
    /// becomes visible on the next time the state of the parent surface is
    /// applied.
    ///
    /// The parent surface must not be one of the child surface's descendants,
    /// and the parent must be different from the child surface, otherwise the
    /// bad_parent protocol error is raised.
    ///
    /// This request modifies the behaviour of wl_surface.commit request on
    /// the sub-surface, see the documentation on wl_subsurface interface.
    pub fn r#get_subsurface(&self, connection: &WaylandConnection, surface: WlSurface, parent: WlSurface) -> WlSubsurface {

        let _enq_id = connection.enqueue(
            Request::WlSubcompositorGetSubsurface {
                sendto: self.id,
                surface: surface.id,
                parent: parent.id,
            }
        );
        return WlSubsurface{
            id: _enq_id,
        };
    }
}


/// An additional interface to a wl_surface object, which has been
/// made a sub-surface. A sub-surface has one parent surface. A
/// sub-surface's size and position are not limited to that of the parent.
/// Particularly, a sub-surface is not automatically clipped to its
/// parent's area.
///
/// A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
/// and the parent surface is mapped. The order of which one happens
/// first is irrelevant. A sub-surface is hidden if the parent becomes
/// hidden, or if a NULL wl_buffer is applied. These rules apply
/// recursively through the tree of surfaces.
///
/// The behaviour of a wl_surface.commit request on a sub-surface
/// depends on the sub-surface's mode. The possible modes are
/// synchronized and desynchronized, see methods
/// wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
/// mode caches the wl_surface state to be applied when the parent's
/// state gets applied, and desynchronized mode applies the pending
/// wl_surface state directly. A sub-surface is initially in the
/// synchronized mode.
///
/// Sub-surfaces also have another kind of state, which is managed by
/// wl_subsurface requests, as opposed to wl_surface requests. This
/// state includes the sub-surface position relative to the parent
/// surface (wl_subsurface.set_position), and the stacking order of
/// the parent and its sub-surfaces (wl_subsurface.place_above and
/// .place_below). This state is applied when the parent surface's
/// wl_surface state is applied, regardless of the sub-surface's mode.
/// As the exception, set_sync and set_desync are effective immediately.
///
/// The main surface can be thought to be always in desynchronized mode,
/// since it does not have a parent in the sub-surfaces sense.
///
/// Even if a sub-surface is in desynchronized mode, it will behave as
/// in synchronized mode, if its parent surface behaves as in
/// synchronized mode. This rule is applied recursively throughout the
/// tree of surfaces. This means, that one can set a sub-surface into
/// synchronized mode, and then assume that all its child and grand-child
/// sub-surfaces are synchronized, too, without explicitly setting them.
///
/// Destroying a sub-surface takes effect immediately. If you need to
/// synchronize the removal of a sub-surface to the parent surface update,
/// unmap the sub-surface first by attaching a NULL wl_buffer, update parent,
/// and then destroy the sub-surface.
///
/// If the parent wl_surface object is destroyed, the sub-surface is
/// unmapped.
#[derive(Clone, Copy)]
pub struct WlSubsurface{
    pub id: u32,
}

impl WlSubsurface {

    /// `wl_subsurface:destroy` request
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    pub fn r#destroy(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfaceDestroy {
                sendto: self.id,
            }
        );
    }

    /// `wl_subsurface:set_position` request
    /// This schedules a sub-surface position change.
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The scheduled coordinates will take effect whenever the state of the
    /// parent surface is applied. When this happens depends on whether the
    /// parent surface is in synchronized mode or not. See
    /// wl_subsurface.set_sync and wl_subsurface.set_desync for details.
    ///
    /// If more than one set_position request is invoked by the client before
    /// the commit of the parent surface, the position of a new request always
    /// replaces the scheduled position from any previous request.
    ///
    /// The initial position is 0, 0.
    pub fn r#set_position(&self, connection: &WaylandConnection, x: i32, y: i32)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfaceSetPosition {
                sendto: self.id,
                x,
                y,
            }
        );
    }

    /// `wl_subsurface:place_above` request
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// The z-order is double-buffered. Requests are handled in order and
    /// applied immediately to a pending state. The final pending state is
    /// copied to the active state the next time the state of the parent
    /// surface is applied. When this happens depends on whether the parent
    /// surface is in synchronized mode or not. See wl_subsurface.set_sync and
    /// wl_subsurface.set_desync for details.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    pub fn r#place_above(&self, connection: &WaylandConnection, sibling: WlSurface)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfacePlaceAbove {
                sendto: self.id,
                sibling: sibling.id,
            }
        );
    }

    /// `wl_subsurface:place_below` request
    /// The sub-surface is placed just below the reference surface.
    /// See wl_subsurface.place_above.
    pub fn r#place_below(&self, connection: &WaylandConnection, sibling: WlSurface)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfacePlaceBelow {
                sendto: self.id,
                sibling: sibling.id,
            }
        );
    }

    /// `wl_subsurface:set_sync` request
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode, also described as the parent dependent mode.
    ///
    /// In synchronized mode, wl_surface.commit on a sub-surface will
    /// accumulate the committed state in a cache, but the state will
    /// not be applied and hence will not change the compositor output.
    /// The cached state is applied to the sub-surface immediately after
    /// the parent surface's state is applied. This ensures atomic
    /// updates of the parent and all its synchronized sub-surfaces.
    /// Applying the cached state will invalidate the cache, so further
    /// parent surface commits do not (re-)apply old state.
    ///
    /// See wl_subsurface for the recursive effect of this mode.
    pub fn r#set_sync(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfaceSetSync {
                sendto: self.id,
            }
        );
    }

    /// `wl_subsurface:set_desync` request
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode, also described as independent or freely running mode.
    ///
    /// In desynchronized mode, wl_surface.commit on a sub-surface will
    /// apply the pending state directly, without caching, as happens
    /// normally with a wl_surface. Calling wl_surface.commit on the
    /// parent surface has no effect on the sub-surface's wl_surface
    /// state. This mode allows a sub-surface to be updated on its own.
    ///
    /// If cached state exists when wl_surface.commit is called in
    /// desynchronized mode, the pending state is added to the cached
    /// state, and applied as a whole. This invalidates the cache.
    ///
    /// Note: even if a sub-surface is set to desynchronized, a parent
    /// sub-surface may override it to behave as synchronized. For details,
    /// see wl_subsurface.
    ///
    /// If a surface's parent surface behaves as desynchronized, then
    /// the cached state is applied on set_desync.
    pub fn r#set_desync(&self, connection: &WaylandConnection)  {

        let _enq_id = connection.enqueue(
            Request::WlSubsurfaceSetDesync {
                sendto: self.id,
            }
        );
    }
}

