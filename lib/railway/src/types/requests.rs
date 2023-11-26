#[derive(Debug, Clone)]
pub enum Request {

    /// zwp_linux_dmabuf_v1:destroy request
    /// Objects created through this interface, especially wl_buffers, will
    /// remain valid.
    ZwpLinuxDmabufV1Destroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// zwp_linux_dmabuf_v1:create_params request
    /// This temporary object is used to collect multiple dmabuf handles into
    /// a single batch to create a wl_buffer. It can only be used once and
    /// should be destroyed after a 'created' or 'failed' event has been
    /// received.
    ZwpLinuxDmabufV1CreateParams {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// zwp_linux_dmabuf_v1:get_default_feedback request
    /// This request creates a new wp_linux_dmabuf_feedback object not bound
    /// to a particular surface. This object will deliver feedback about dmabuf
    /// parameters to use if the client doesn't support per-surface feedback
    /// (see get_surface_feedback).
    ZwpLinuxDmabufV1GetDefaultFeedback {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// zwp_linux_dmabuf_v1:get_surface_feedback request
    /// This request creates a new wp_linux_dmabuf_feedback object for the
    /// specified wl_surface. This object will deliver feedback about dmabuf
    /// parameters to use for buffers attached to this surface.
    ///
    /// If the surface is destroyed before the wp_linux_dmabuf_feedback object,
    /// the feedback object becomes inert.
    ZwpLinuxDmabufV1GetSurfaceFeedback {
        /// id of the object to send a request to
        sendto: u32,
        surface: u32,
    },

    /// zwp_linux_buffer_params_v1:destroy request
    /// Cleans up the temporary data sent to the server for dmabuf-based
    /// wl_buffer creation.
    ZwpLinuxBufferParamsV1Destroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// zwp_linux_buffer_params_v1:add request
    /// This request adds one dmabuf to the set in this
    /// zwp_linux_buffer_params_v1.
    ///
    /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
    /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
    /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
    /// This is an opaque token. Drivers use this token to express tiling,
    /// compression, etc. driver-specific modifications to the base format
    /// defined by the DRM fourcc code.
    ///
    /// Starting from version 4, the invalid_format protocol error is sent if
    /// the format + modifier pair was not advertised as supported.
    ///
    /// This request raises the PLANE_IDX error if plane_idx is too large.
    /// The error PLANE_SET is raised if attempting to set a plane that
    /// was already set.
    ZwpLinuxBufferParamsV1Add {
        /// id of the object to send a request to
        sendto: u32,
        /// dmabuf fd
        fd: std::os::fd::RawFd,
        /// plane index
        plane_idx: u32,
        /// offset in bytes
        offset: u32,
        /// stride in bytes
        stride: u32,
        /// high 32 bits of layout modifier
        modifier_hi: u32,
        /// low 32 bits of layout modifier
        modifier_lo: u32,
    },

    /// zwp_linux_buffer_params_v1:create request
    /// This asks for creation of a wl_buffer from the added dmabuf
    /// buffers. The wl_buffer is not created immediately but returned via
    /// the 'created' event if the dmabuf sharing succeeds. The sharing
    /// may fail at runtime for reasons a client cannot predict, in
    /// which case the 'failed' event is triggered.
    ///
    /// The 'format' argument is a DRM_FORMAT code, as defined by the
    /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
    /// authoritative source on how the format codes should work.
    ///
    /// The 'flags' is a bitfield of the flags defined in enum "flags".
    /// 'y_invert' means the that the image needs to be y-flipped.
    ///
    /// Flag 'interlaced' means that the frame in the buffer is not
    /// progressive as usual, but interlaced. An interlaced buffer as
    /// supported here must always contain both top and bottom fields.
    /// The top field always begins on the first pixel row. The temporal
    /// ordering between the two fields is top field first, unless
    /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
    /// is ignored if 'interlaced' is not set.
    ///
    /// This protocol does not convey any information about field rate,
    /// duration, or timing, other than the relative ordering between the
    /// two fields in one buffer. A compositor may have to estimate the
    /// intended field rate from the incoming buffer rate. It is undefined
    /// whether the time of receiving wl_surface.commit with a new buffer
    /// attached, applying the wl_surface state, wl_surface.frame callback
    /// trigger, presentation, or any other point in the compositor cycle
    /// is used to measure the frame or field times. There is no support
    /// for detecting missed or late frames/fields/buffers either, and
    /// there is no support whatsoever for cooperating with interlaced
    /// compositor output.
    ///
    /// The composited image quality resulting from the use of interlaced
    /// buffers is explicitly undefined. A compositor may use elaborate
    /// hardware features or software to deinterlace and create progressive
    /// output frames from a sequence of interlaced input buffers, or it
    /// may produce substandard image quality. However, compositors that
    /// cannot guarantee reasonable image quality in all cases are recommended
    /// to just reject all interlaced buffers.
    ///
    /// Any argument errors, including non-positive width or height,
    /// mismatch between the number of planes and the format, bad
    /// format, bad offset or stride, may be indicated by fatal protocol
    /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
    /// OUT_OF_BOUNDS.
    ///
    /// Dmabuf import errors in the server that are not obvious client
    /// bugs are returned via the 'failed' event as non-fatal. This
    /// allows attempting dmabuf sharing and falling back in the client
    /// if it fails.
    ///
    /// This request can be sent only once in the object's lifetime, after
    /// which the only legal request is destroy. This object should be
    /// destroyed after issuing a 'create' request. Attempting to use this
    /// object after issuing 'create' raises ALREADY_USED protocol error.
    ///
    /// It is not mandatory to issue 'create'. If a client wants to
    /// cancel the buffer creation, it can just destroy this object.
    ZwpLinuxBufferParamsV1Create {
        /// id of the object to send a request to
        sendto: u32,
        /// base plane width in pixels
        width: i32,
        /// base plane height in pixels
        height: i32,
        /// DRM_FORMAT code
        format: u32,
        /// see enum flags
        flags: u32,
    },

    /// zwp_linux_buffer_params_v1:create_immed request
    /// This asks for immediate creation of a wl_buffer by importing the
    /// added dmabufs.
    ///
    /// In case of import success, no event is sent from the server, and the
    /// wl_buffer is ready to be used by the client.
    ///
    /// Upon import failure, either of the following may happen, as seen fit
    /// by the implementation:
    /// - the client is terminated with one of the following fatal protocol
    /// errors:
    /// - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
    /// in case of argument errors such as mismatch between the number
    /// of planes and the format, bad format, non-positive width or
    /// height, or bad offset or stride.
    /// - INVALID_WL_BUFFER, in case the cause for failure is unknown or
    /// plaform specific.
    /// - the server creates an invalid wl_buffer, marks it as failed and
    /// sends a 'failed' event to the client. The result of using this
    /// invalid wl_buffer as an argument in any request by the client is
    /// defined by the compositor implementation.
    ///
    /// This takes the same arguments as a 'create' request, and obeys the
    /// same restrictions.
    ZwpLinuxBufferParamsV1CreateImmed {
        /// id of the object to send a request to
        sendto: u32,
        /// base plane width in pixels
        width: i32,
        /// base plane height in pixels
        height: i32,
        /// DRM_FORMAT code
        format: u32,
        /// see enum flags
        flags: u32,
    },

    /// zwp_linux_dmabuf_feedback_v1:destroy request
    /// Using this request a client can tell the server that it is not going to
    /// use the wp_linux_dmabuf_feedback object anymore.
    ZwpLinuxDmabufFeedbackV1Destroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_wm_base:destroy request
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    XdgWmBaseDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_wm_base:create_positioner request
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    XdgWmBaseCreatePositioner {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_wm_base:get_xdg_surface request
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
    XdgWmBaseGetXdgSurface {
        /// id of the object to send a request to
        sendto: u32,
        surface: u32,
    },

    /// xdg_wm_base:pong request
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    XdgWmBasePong {
        /// id of the object to send a request to
        sendto: u32,
        /// serial of the ping event
        serial: u32,
    },

    /// xdg_positioner:destroy request
    /// Notify the compositor that the xdg_positioner will no longer be used.
    XdgPositionerDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_positioner:set_size request
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    XdgPositionerSetSize {
        /// id of the object to send a request to
        sendto: u32,
        /// width of positioned rectangle
        width: i32,
        /// height of positioned rectangle
        height: i32,
    },

    /// xdg_positioner:set_anchor_rect request
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
    XdgPositionerSetAnchorRect {
        /// id of the object to send a request to
        sendto: u32,
        /// x position of anchor rectangle
        x: i32,
        /// y position of anchor rectangle
        y: i32,
        /// width of anchor rectangle
        width: i32,
        /// height of anchor rectangle
        height: i32,
    },

    /// xdg_positioner:set_anchor request
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    XdgPositionerSetAnchor {
        /// id of the object to send a request to
        sendto: u32,
        /// anchor
        anchor: u32,
    },

    /// xdg_positioner:set_gravity request
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    XdgPositionerSetGravity {
        /// id of the object to send a request to
        sendto: u32,
        /// gravity direction
        gravity: u32,
    },

    /// xdg_positioner:set_constraint_adjustment request
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
    XdgPositionerSetConstraintAdjustment {
        /// id of the object to send a request to
        sendto: u32,
        /// bit mask of constraint adjustments
        constraint_adjustment: u32,
    },

    /// xdg_positioner:set_offset request
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
    XdgPositionerSetOffset {
        /// id of the object to send a request to
        sendto: u32,
        /// surface position x offset
        x: i32,
        /// surface position y offset
        y: i32,
    },

    /// xdg_positioner:set_reactive request
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    XdgPositionerSetReactive {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_positioner:set_parent_size request
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    XdgPositionerSetParentSize {
        /// id of the object to send a request to
        sendto: u32,
        /// future window geometry width of parent
        parent_width: i32,
        /// future window geometry height of parent
        parent_height: i32,
    },

    /// xdg_positioner:set_parent_configure request
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    XdgPositionerSetParentConfigure {
        /// id of the object to send a request to
        sendto: u32,
        /// serial of parent configure event
        serial: u32,
    },

    /// xdg_surface:destroy request
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    XdgSurfaceDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_surface:get_toplevel request
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    XdgSurfaceGetToplevel {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_surface:get_popup request
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    XdgSurfaceGetPopup {
        /// id of the object to send a request to
        sendto: u32,
        parent: u32,
        positioner: u32,
    },

    /// xdg_surface:set_window_geometry request
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
    XdgSurfaceSetWindowGeometry {
        /// id of the object to send a request to
        sendto: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },

    /// xdg_surface:ack_configure request
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
    XdgSurfaceAckConfigure {
        /// id of the object to send a request to
        sendto: u32,
        /// the serial from the configure event
        serial: u32,
    },

    /// xdg_toplevel:destroy request
    /// This request destroys the role surface and unmaps the surface;
    /// see "Unmapping" behavior in interface section for details.
    XdgToplevelDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_toplevel:set_parent request
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
    XdgToplevelSetParent {
        /// id of the object to send a request to
        sendto: u32,
        parent: u32,
    },

    /// xdg_toplevel:set_title request
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    XdgToplevelSetTitle {
        /// id of the object to send a request to
        sendto: u32,
        title: String,
    },

    /// xdg_toplevel:set_app_id request
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
    XdgToplevelSetAppId {
        /// id of the object to send a request to
        sendto: u32,
        app_id: String,
    },

    /// xdg_toplevel:show_window_menu request
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
    XdgToplevelShowWindowMenu {
        /// id of the object to send a request to
        sendto: u32,
        /// the wl_seat of the user event
        seat: u32,
        /// the serial of the user event
        serial: u32,
        /// the x position to pop up the window menu at
        x: i32,
        /// the y position to pop up the window menu at
        y: i32,
    },

    /// xdg_toplevel:move request
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
    XdgToplevelMove {
        /// id of the object to send a request to
        sendto: u32,
        /// the wl_seat of the user event
        seat: u32,
        /// the serial of the user event
        serial: u32,
    },

    /// xdg_toplevel:resize request
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
    XdgToplevelResize {
        /// id of the object to send a request to
        sendto: u32,
        /// the wl_seat of the user event
        seat: u32,
        /// the serial of the user event
        serial: u32,
        /// which edge or corner is being dragged
        edges: u32,
    },

    /// xdg_toplevel:set_max_size request
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
    XdgToplevelSetMaxSize {
        /// id of the object to send a request to
        sendto: u32,
        width: i32,
        height: i32,
    },

    /// xdg_toplevel:set_min_size request
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
    XdgToplevelSetMinSize {
        /// id of the object to send a request to
        sendto: u32,
        width: i32,
        height: i32,
    },

    /// xdg_toplevel:set_maximized request
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
    XdgToplevelSetMaximized {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_toplevel:unset_maximized request
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
    XdgToplevelUnsetMaximized {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_toplevel:set_fullscreen request
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
    XdgToplevelSetFullscreen {
        /// id of the object to send a request to
        sendto: u32,
        output: u32,
    },

    /// xdg_toplevel:unset_fullscreen request
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
    XdgToplevelUnsetFullscreen {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_toplevel:set_minimized request
    /// Request that the compositor minimize your surface. There is no
    /// way to know if the surface is currently minimized, nor is there
    /// any way to unset minimization on this surface.
    ///
    /// If you are looking to throttle redrawing when minimized, please
    /// instead use the wl_surface.frame event for this, as this will
    /// also work with live previews on windows in Alt-Tab, Expose or
    /// similar compositor features.
    XdgToplevelSetMinimized {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_popup:destroy request
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    XdgPopupDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// xdg_popup:grab request
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
    XdgPopupGrab {
        /// id of the object to send a request to
        sendto: u32,
        /// the wl_seat of the user event
        seat: u32,
        /// the serial of the user event
        serial: u32,
    },

    /// xdg_popup:reposition request
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
    XdgPopupReposition {
        /// id of the object to send a request to
        sendto: u32,
        positioner: u32,
        /// reposition request token
        token: u32,
    },

    /// wl_display:sync request
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
    WlDisplaySync {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_display:get_registry request
    /// This request creates a registry object that allows the client
    /// to list and bind the global objects available from the
    /// compositor.
    ///
    /// It should be noted that the server side resources consumed in
    /// response to a get_registry request can only be released when the
    /// client disconnects, not when the client side proxy is destroyed.
    /// Therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    WlDisplayGetRegistry {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_registry:bind request
    /// Binds a new, client-created object to the server using the
    /// specified name as the identifier.
    WlRegistryBind {
        /// id of the object to send a request to
        sendto: u32,
        /// unique numeric name of the object
        name: u32,
        /// new_id value without iterface must be preceeded by a string and a version
        if_name: String,
        /// new_id value without iterface must be preceeded by a string and a version
        if_version: u32
    },

    /// wl_compositor:create_surface request
    /// Ask the compositor to create a new surface.
    WlCompositorCreateSurface {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_compositor:create_region request
    /// Ask the compositor to create a new region.
    WlCompositorCreateRegion {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_shm_pool:create_buffer request
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
    WlShmPoolCreateBuffer {
        /// id of the object to send a request to
        sendto: u32,
        /// buffer byte offset within the pool
        offset: i32,
        /// buffer width, in pixels
        width: i32,
        /// buffer height, in pixels
        height: i32,
        /// number of bytes from the beginning of one row to the beginning of the next row
        stride: i32,
        /// buffer pixel format
        format: u32,
    },

    /// wl_shm_pool:destroy request
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
    WlShmPoolDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_shm_pool:resize request
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
    WlShmPoolResize {
        /// id of the object to send a request to
        sendto: u32,
        /// new size of the pool, in bytes
        size: i32,
    },

    /// wl_shm:create_pool request
    /// Create a new wl_shm_pool object.
    ///
    /// The pool can be used to create shared memory based buffer
    /// objects.  The server will mmap size bytes of the passed file
    /// descriptor, to use as backing memory for the pool.
    WlShmCreatePool {
        /// id of the object to send a request to
        sendto: u32,
        /// file descriptor for the pool
        fd: std::os::fd::RawFd,
        /// pool size, in bytes
        size: i32,
    },

    /// wl_buffer:destroy request
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    WlBufferDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_offer:accept request
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
    WlDataOfferAccept {
        /// id of the object to send a request to
        sendto: u32,
        /// serial number of the accept request
        serial: u32,
        /// mime type accepted by the client
        mime_type: String,
    },

    /// wl_data_offer:receive request
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
    WlDataOfferReceive {
        /// id of the object to send a request to
        sendto: u32,
        /// mime type desired by receiver
        mime_type: String,
        /// file descriptor for data transfer
        fd: std::os::fd::RawFd,
    },

    /// wl_data_offer:destroy request
    /// Destroy the data offer.
    WlDataOfferDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_offer:finish request
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
    WlDataOfferFinish {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_offer:set_actions request
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
    WlDataOfferSetActions {
        /// id of the object to send a request to
        sendto: u32,
        /// actions supported by the destination client
        dnd_actions: u32,
        /// action preferred by the destination client
        preferred_action: u32,
    },

    /// wl_data_source:offer request
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    WlDataSourceOffer {
        /// id of the object to send a request to
        sendto: u32,
        /// mime type offered by the data source
        mime_type: String,
    },

    /// wl_data_source:destroy request
    /// Destroy the data source.
    WlDataSourceDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_source:set_actions request
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
    WlDataSourceSetActions {
        /// id of the object to send a request to
        sendto: u32,
        /// actions supported by the data source
        dnd_actions: u32,
    },

    /// wl_data_device:start_drag request
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
    WlDataDeviceStartDrag {
        /// id of the object to send a request to
        sendto: u32,
        /// data source for the eventual transfer
        source: u32,
        /// surface where the drag originates
        origin: u32,
        /// drag-and-drop icon surface
        icon: u32,
        /// serial number of the implicit grab on the origin
        serial: u32,
    },

    /// wl_data_device:set_selection request
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    WlDataDeviceSetSelection {
        /// id of the object to send a request to
        sendto: u32,
        /// data source for the selection
        source: u32,
        /// serial number of the event that triggered this request
        serial: u32,
    },

    /// wl_data_device:release request
    /// This request destroys the data device.
    WlDataDeviceRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_device_manager:create_data_source request
    /// Create a new data source.
    WlDataDeviceManagerCreateDataSource {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_data_device_manager:get_data_device request
    /// Create a new data device for a given seat.
    WlDataDeviceManagerGetDataDevice {
        /// id of the object to send a request to
        sendto: u32,
        /// seat associated with the data device
        seat: u32,
    },

    /// wl_shell:get_shell_surface request
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    WlShellGetShellSurface {
        /// id of the object to send a request to
        sendto: u32,
        /// surface to be given the shell surface role
        surface: u32,
    },

    /// wl_shell_surface:pong request
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    WlShellSurfacePong {
        /// id of the object to send a request to
        sendto: u32,
        /// serial number of the ping event
        serial: u32,
    },

    /// wl_shell_surface:move request
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    WlShellSurfaceMove {
        /// id of the object to send a request to
        sendto: u32,
        /// seat whose pointer is used
        seat: u32,
        /// serial number of the implicit grab on the pointer
        serial: u32,
    },

    /// wl_shell_surface:resize request
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    WlShellSurfaceResize {
        /// id of the object to send a request to
        sendto: u32,
        /// seat whose pointer is used
        seat: u32,
        /// serial number of the implicit grab on the pointer
        serial: u32,
        /// which edge or corner is being dragged
        edges: u32,
    },

    /// wl_shell_surface:set_toplevel request
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    WlShellSurfaceSetToplevel {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_shell_surface:set_transient request
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    WlShellSurfaceSetTransient {
        /// id of the object to send a request to
        sendto: u32,
        /// parent surface
        parent: u32,
        /// surface-local x coordinate
        x: i32,
        /// surface-local y coordinate
        y: i32,
        /// transient surface behavior
        flags: u32,
    },

    /// wl_shell_surface:set_fullscreen request
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
    WlShellSurfaceSetFullscreen {
        /// id of the object to send a request to
        sendto: u32,
        /// method for resolving size conflict
        method: u32,
        /// framerate in mHz
        framerate: u32,
        /// output on which the surface is to be fullscreen
        output: u32,
    },

    /// wl_shell_surface:set_popup request
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
    WlShellSurfaceSetPopup {
        /// id of the object to send a request to
        sendto: u32,
        /// seat whose pointer is used
        seat: u32,
        /// serial number of the implicit grab on the pointer
        serial: u32,
        /// parent surface
        parent: u32,
        /// surface-local x coordinate
        x: i32,
        /// surface-local y coordinate
        y: i32,
        /// transient surface behavior
        flags: u32,
    },

    /// wl_shell_surface:set_maximized request
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
    WlShellSurfaceSetMaximized {
        /// id of the object to send a request to
        sendto: u32,
        /// output on which the surface is to be maximized
        output: u32,
    },

    /// wl_shell_surface:set_title request
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    WlShellSurfaceSetTitle {
        /// id of the object to send a request to
        sendto: u32,
        /// surface title
        title: String,
    },

    /// wl_shell_surface:set_class request
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    WlShellSurfaceSetClass {
        /// id of the object to send a request to
        sendto: u32,
        /// surface class
        class_: String,
    },

    /// wl_surface:destroy request
    /// Deletes the surface and invalidates its object ID.
    WlSurfaceDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_surface:attach request
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
    WlSurfaceAttach {
        /// id of the object to send a request to
        sendto: u32,
        /// buffer of surface contents
        buffer: u32,
        /// surface-local x coordinate
        x: i32,
        /// surface-local y coordinate
        y: i32,
    },

    /// wl_surface:damage request
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
    WlSurfaceDamage {
        /// id of the object to send a request to
        sendto: u32,
        /// surface-local x coordinate
        x: i32,
        /// surface-local y coordinate
        y: i32,
        /// width of damage rectangle
        width: i32,
        /// height of damage rectangle
        height: i32,
    },

    /// wl_surface:frame request
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
    WlSurfaceFrame {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_surface:set_opaque_region request
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
    WlSurfaceSetOpaqueRegion {
        /// id of the object to send a request to
        sendto: u32,
        /// opaque region of the surface
        region: u32,
    },

    /// wl_surface:set_input_region request
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
    WlSurfaceSetInputRegion {
        /// id of the object to send a request to
        sendto: u32,
        /// input region of the surface
        region: u32,
    },

    /// wl_surface:commit request
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
    WlSurfaceCommit {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_surface:set_buffer_transform request
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
    WlSurfaceSetBufferTransform {
        /// id of the object to send a request to
        sendto: u32,
        /// transform for interpreting buffer contents
        transform: i32,
    },

    /// wl_surface:set_buffer_scale request
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
    WlSurfaceSetBufferScale {
        /// id of the object to send a request to
        sendto: u32,
        /// positive scale for interpreting buffer contents
        scale: i32,
    },

    /// wl_surface:damage_buffer request
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
    WlSurfaceDamageBuffer {
        /// id of the object to send a request to
        sendto: u32,
        /// buffer-local x coordinate
        x: i32,
        /// buffer-local y coordinate
        y: i32,
        /// width of damage rectangle
        width: i32,
        /// height of damage rectangle
        height: i32,
    },

    /// wl_surface:offset request
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
    WlSurfaceOffset {
        /// id of the object to send a request to
        sendto: u32,
        /// surface-local x coordinate
        x: i32,
        /// surface-local y coordinate
        y: i32,
    },

    /// wl_seat:get_pointer request
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    WlSeatGetPointer {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_seat:get_keyboard request
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    WlSeatGetKeyboard {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_seat:get_touch request
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    WlSeatGetTouch {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_seat:release request
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    WlSeatRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_pointer:set_cursor request
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
    WlPointerSetCursor {
        /// id of the object to send a request to
        sendto: u32,
        /// serial number of the enter event
        serial: u32,
        /// pointer surface
        surface: u32,
        /// surface-local x coordinate
        hotspot_x: i32,
        /// surface-local y coordinate
        hotspot_y: i32,
    },

    /// wl_pointer:release request
    /// Using this request a client can tell the server that it is not going to
    /// use the pointer object anymore.
    ///
    /// This request destroys the pointer proxy object, so clients must not call
    /// wl_pointer_destroy() after using this request.
    WlPointerRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_keyboard:release request
    WlKeyboardRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_touch:release request
    WlTouchRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_output:release request
    /// Using this request a client can tell the server that it is not going to
    /// use the output object anymore.
    WlOutputRelease {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_region:destroy request
    /// Destroy the region.  This will invalidate the object ID.
    WlRegionDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_region:add request
    /// Add the specified rectangle to the region.
    WlRegionAdd {
        /// id of the object to send a request to
        sendto: u32,
        /// region-local x coordinate
        x: i32,
        /// region-local y coordinate
        y: i32,
        /// rectangle width
        width: i32,
        /// rectangle height
        height: i32,
    },

    /// wl_region:subtract request
    /// Subtract the specified rectangle from the region.
    WlRegionSubtract {
        /// id of the object to send a request to
        sendto: u32,
        /// region-local x coordinate
        x: i32,
        /// region-local y coordinate
        y: i32,
        /// rectangle width
        width: i32,
        /// rectangle height
        height: i32,
    },

    /// wl_subcompositor:destroy request
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other
    /// objects, wl_subsurface objects included.
    WlSubcompositorDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_subcompositor:get_subsurface request
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
    WlSubcompositorGetSubsurface {
        /// id of the object to send a request to
        sendto: u32,
        /// the surface to be turned into a sub-surface
        surface: u32,
        /// the parent surface
        parent: u32,
    },

    /// wl_subsurface:destroy request
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    WlSubsurfaceDestroy {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_subsurface:set_position request
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
    WlSubsurfaceSetPosition {
        /// id of the object to send a request to
        sendto: u32,
        /// x coordinate in the parent surface
        x: i32,
        /// y coordinate in the parent surface
        y: i32,
    },

    /// wl_subsurface:place_above request
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
    WlSubsurfacePlaceAbove {
        /// id of the object to send a request to
        sendto: u32,
        /// the reference surface
        sibling: u32,
    },

    /// wl_subsurface:place_below request
    /// The sub-surface is placed just below the reference surface.
    /// See wl_subsurface.place_above.
    WlSubsurfacePlaceBelow {
        /// id of the object to send a request to
        sendto: u32,
        /// the reference surface
        sibling: u32,
    },

    /// wl_subsurface:set_sync request
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
    WlSubsurfaceSetSync {
        /// id of the object to send a request to
        sendto: u32,
    },

    /// wl_subsurface:set_desync request
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
    WlSubsurfaceSetDesync {
        /// id of the object to send a request to
        sendto: u32,
    },
}


