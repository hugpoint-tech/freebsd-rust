#[allow(dead_code)]

use crate::types::Fixed;
use crate::types::enums;
/// The ping event asks the client if it's still alive. Pass the
/// serial specified in the event back to the compositor by sending
/// a "pong" request back with the specified serial. See xdg_wm_base.pong.
///
/// Compositors can use this to determine if the client is still
/// alive. It's unspecified what will happen if the client doesn't
/// respond to the ping request, or in what timeframe. Clients should
/// try to respond in a reasonable amount of time. The “unresponsive”
/// error is provided for compositors that wish to disconnect unresponsive
/// clients.
///
/// A compositor is free to ping in any way it wants, but a client must
/// always respond to any xdg_wm_base object it created.
#[derive(Debug)]
pub struct XdgWmBasePingEvent {
    /// xdg_wm_base:ping event
    /// id of the object the event came from
    pub source_id: u32,
    /// pass this to the pong request
    pub serial: u32,
}

/// The configure event marks the end of a configure sequence. A configure
/// sequence is a set of one or more events configuring the state of the
/// xdg_surface, including the final xdg_surface.configure event.
///
/// Where applicable, xdg_surface surface roles will during a configure
/// sequence extend this event as a latched state sent as events before the
/// xdg_surface.configure event. Such events should be considered to make up
/// a set of atomically applied configuration states, where the
/// xdg_surface.configure commits the accumulated state.
///
/// Clients should arrange their surface for the new states, and then send
/// an ack_configure request with the serial sent in this configure event at
/// some point before committing the new surface.
///
/// If the client receives multiple configure events before it can respond
/// to one, it is free to discard all but the last event it received.
#[derive(Debug)]
pub struct XdgSurfaceConfigureEvent {
    /// xdg_surface:configure event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial of the configure event
    pub serial: u32,
}

/// This configure event asks the client to resize its toplevel surface or
/// to change its state. The configured state should not be applied
/// immediately. See xdg_surface.configure for details.
///
/// The width and height arguments specify a hint to the window
/// about how its surface should be resized in window geometry
/// coordinates. See set_window_geometry.
///
/// If the width or height arguments are zero, it means the client
/// should decide its own window dimension. This may happen when the
/// compositor needs to configure the state of the surface but doesn't
/// have any information about any previous or expected dimension.
///
/// The states listed in the event specify how the width/height
/// arguments should be interpreted, and possibly how it should be
/// drawn.
///
/// Clients must send an ack_configure in response to this event. See
/// xdg_surface.configure and xdg_surface.ack_configure for details.
#[derive(Debug)]
pub struct XdgToplevelConfigureEvent {
    /// xdg_toplevel:configure event
    /// id of the object the event came from
    pub source_id: u32,
    pub width: i32,
    pub height: i32,
    pub states: Vec<u8>,
}

/// The close event is sent by the compositor when the user
/// wants the surface to be closed. This should be equivalent to
/// the user clicking the close button in client-side decorations,
/// if your application has any.
///
/// This is only a request that the user intends to close the
/// window. The client may choose to ignore this request, or show
/// a dialog to ask the user to save their data, etc.
#[derive(Debug)]
pub struct XdgToplevelCloseEvent {
    /// xdg_toplevel:close event
    /// id of the object the event came from
    pub source_id: u32,
}

/// The configure_bounds event may be sent prior to a xdg_toplevel.configure
/// event to communicate the bounds a window geometry size is recommended
/// to constrain to.
///
/// The passed width and height are in surface coordinate space. If width
/// and height are 0, it means bounds is unknown and equivalent to as if no
/// configure_bounds event was ever sent for this surface.
///
/// The bounds can for example correspond to the size of a monitor excluding
/// any panels or other shell components, so that a surface isn't created in
/// a way that it cannot fit.
///
/// The bounds may change at any point, and in such a case, a new
/// xdg_toplevel.configure_bounds will be sent, followed by
/// xdg_toplevel.configure and xdg_surface.configure.
#[derive(Debug)]
pub struct XdgToplevelConfigureBoundsEvent {
    /// xdg_toplevel:configure_bounds event
    /// id of the object the event came from
    pub source_id: u32,
    pub width: i32,
    pub height: i32,
}

/// This event advertises the capabilities supported by the compositor. If
/// a capability isn't supported, clients should hide or disable the UI
/// elements that expose this functionality. For instance, if the
/// compositor doesn't advertise support for minimized toplevels, a button
/// triggering the set_minimized request should not be displayed.
///
/// The compositor will ignore requests it doesn't support. For instance,
/// a compositor which doesn't advertise support for minimized will ignore
/// set_minimized requests.
///
/// Compositors must send this event once before the first
/// xdg_surface.configure event. When the capabilities change, compositors
/// must send this event again and then send an xdg_surface.configure
/// event.
///
/// The configured state should not be applied immediately. See
/// xdg_surface.configure for details.
///
/// The capabilities are sent as an array of 32-bit unsigned integers in
/// native endianness.
#[derive(Debug)]
pub struct XdgToplevelWmCapabilitiesEvent {
    /// xdg_toplevel:wm_capabilities event
    /// id of the object the event came from
    pub source_id: u32,
    /// array of 32-bit capabilities
    pub capabilities: Vec<u8>,
}

/// This event asks the popup surface to configure itself given the
/// configuration. The configured state should not be applied immediately.
/// See xdg_surface.configure for details.
///
/// The x and y arguments represent the position the popup was placed at
/// given the xdg_positioner rule, relative to the upper left corner of the
/// window geometry of the parent surface.
///
/// For version 2 or older, the configure event for an xdg_popup is only
/// ever sent once for the initial configuration. Starting with version 3,
/// it may be sent again if the popup is setup with an xdg_positioner with
/// set_reactive requested, or in response to xdg_popup.reposition requests.
#[derive(Debug)]
pub struct XdgPopupConfigureEvent {
    /// xdg_popup:configure event
    /// id of the object the event came from
    pub source_id: u32,
    /// x position relative to parent surface window geometry
    pub x: i32,
    /// y position relative to parent surface window geometry
    pub y: i32,
    /// window geometry width
    pub width: i32,
    /// window geometry height
    pub height: i32,
}

/// The popup_done event is sent out when a popup is dismissed by the
/// compositor. The client should destroy the xdg_popup object at this
/// point.
#[derive(Debug)]
pub struct XdgPopupPopupDoneEvent {
    /// xdg_popup:popup_done event
    /// id of the object the event came from
    pub source_id: u32,
}

/// The repositioned event is sent as part of a popup configuration
/// sequence, together with xdg_popup.configure and lastly
/// xdg_surface.configure to notify the completion of a reposition request.
///
/// The repositioned event is to notify about the completion of a
/// xdg_popup.reposition request. The token argument is the token passed
/// in the xdg_popup.reposition request.
///
/// Immediately after this event is emitted, xdg_popup.configure and
/// xdg_surface.configure will be sent with the updated size and position,
/// as well as a new configure serial.
///
/// The client should optionally update the content of the popup, but must
/// acknowledge the new popup configuration for the new position to take
/// effect. See xdg_surface.ack_configure for details.
#[derive(Debug)]
pub struct XdgPopupRepositionedEvent {
    /// xdg_popup:repositioned event
    /// id of the object the event came from
    pub source_id: u32,
    /// reposition request token
    pub token: u32,
}

/// Callback event
#[derive(Debug)]
pub struct WlDisplaySyncDoneEvent{
    /// id of the object the event came from
    pub source_id: u32,
    /// event specific data
    pub data: u32,
}

/// The error event is sent out when a fatal (non-recoverable)
/// error has occurred.  The object_id argument is the object
/// where the error occurred, most often in response to a request
/// to that object.  The code identifies the error and is defined
/// by the object interface.  As such, each interface defines its
/// own set of error codes.  The message is a brief description
/// of the error, for (debugging) convenience.
#[derive(Debug)]
pub struct WlDisplayErrorEvent {
    /// wl_display:error event
    /// id of the object the event came from
    pub source_id: u32,
    /// object where the error occurred
    pub object_id: u32,
    /// error code
    pub code: u32,
    /// error description
    pub message: String,
}

/// This event is used internally by the object ID management
/// logic. When a client deletes an object that it had created,
/// the server will send this event to acknowledge that it has
/// seen the delete request. When the client receives this event,
/// it will know that it can safely reuse the object ID.
#[derive(Debug)]
pub struct WlDisplayDeleteIdEvent {
    /// wl_display:delete_id event
    /// id of the object the event came from
    pub source_id: u32,
    /// deleted object ID
    pub id: u32,
}

/// Notify the client of global objects.
///
/// The event notifies the client that a global object with
/// the given name is now available, and it implements the
/// given version of the given interface.
#[derive(Debug)]
pub struct WlRegistryGlobalEvent {
    /// wl_registry:global event
    /// id of the object the event came from
    pub source_id: u32,
    /// numeric name of the global object
    pub name: u32,
    /// interface implemented by the object
    pub interface: String,
    /// interface version
    pub version: u32,
}

/// Notify the client of removed global objects.
///
/// This event notifies the client that the global identified
/// by name is no longer available.  If the client bound to
/// the global using the bind request, the client should now
/// destroy that object.
///
/// The object remains valid and requests to the object will be
/// ignored until the client destroys it, to avoid races between
/// the global going away and a client sending a request to it.
#[derive(Debug)]
pub struct WlRegistryGlobalRemoveEvent {
    /// wl_registry:global_remove event
    /// id of the object the event came from
    pub source_id: u32,
    /// numeric name of the global object
    pub name: u32,
}

/// Notify the client when the related request is done.
#[derive(Debug)]
pub struct WlCallbackDoneEvent {
    /// wl_callback:done event
    /// id of the object the event came from
    pub source_id: u32,
    /// request-specific data for the callback
    pub callback_data: u32,
}

/// Informs the client about a valid pixel format that
/// can be used for buffers. Known formats include
/// argb8888 and xrgb8888.
#[derive(Debug)]
pub struct WlShmFormatEvent {
    /// wl_shm:format event
    /// id of the object the event came from
    pub source_id: u32,
    /// buffer pixel format
    pub format: enums::WlShmFormat,
}

/// Sent when this wl_buffer is no longer used by the compositor.
/// The client is now free to reuse or destroy this buffer and its
/// backing storage.
///
/// If a client receives a release event before the frame callback
/// requested in the same wl_surface.commit that attaches this
/// wl_buffer to a surface, then the client is immediately free to
/// reuse the buffer and its backing storage, and does not need a
/// second buffer for the next surface content update. Typically
/// this is possible, when the compositor maintains a copy of the
/// wl_surface contents, e.g. as a GL texture. This is an important
/// optimization for GL(ES) compositors with wl_shm clients.
#[derive(Debug)]
pub struct WlBufferReleaseEvent {
    /// wl_buffer:release event
    /// id of the object the event came from
    pub source_id: u32,
}

/// Sent immediately after creating the wl_data_offer object.  One
/// event per offered mime type.
#[derive(Debug)]
pub struct WlDataOfferOfferEvent {
    /// wl_data_offer:offer event
    /// id of the object the event came from
    pub source_id: u32,
    /// offered mime type
    pub mime_type: String,
}

/// This event indicates the actions offered by the data source. It
/// will be sent immediately after creating the wl_data_offer object,
/// or anytime the source side changes its offered actions through
/// wl_data_source.set_actions.
#[derive(Debug)]
pub struct WlDataOfferSourceActionsEvent {
    /// wl_data_offer:source_actions event
    /// id of the object the event came from
    pub source_id: u32,
    /// actions offered by the data source
    pub source_actions: enums::WlDataDeviceManagerDndAction,
}

/// This event indicates the action selected by the compositor after
/// matching the source/destination side actions. Only one action (or
/// none) will be offered here.
///
/// This event can be emitted multiple times during the drag-and-drop
/// operation in response to destination side action changes through
/// wl_data_offer.set_actions.
///
/// This event will no longer be emitted after wl_data_device.drop
/// happened on the drag-and-drop destination, the client must
/// honor the last action received, or the last preferred one set
/// through wl_data_offer.set_actions when handling an "ask" action.
///
/// Compositors may also change the selected action on the fly, mainly
/// in response to keyboard modifier changes during the drag-and-drop
/// operation.
///
/// The most recent action received is always the valid one. Prior to
/// receiving wl_data_device.drop, the chosen action may change (e.g.
/// due to keyboard modifiers being pressed). At the time of receiving
/// wl_data_device.drop the drag-and-drop destination must honor the
/// last action received.
///
/// Action changes may still happen after wl_data_device.drop,
/// especially on "ask" actions, where the drag-and-drop destination
/// may choose another action afterwards. Action changes happening
/// at this stage are always the result of inter-client negotiation, the
/// compositor shall no longer be able to induce a different action.
///
/// Upon "ask" actions, it is expected that the drag-and-drop destination
/// may potentially choose a different action and/or mime type,
/// based on wl_data_offer.source_actions and finally chosen by the
/// user (e.g. popping up a menu with the available options). The
/// final wl_data_offer.set_actions and wl_data_offer.accept requests
/// must happen before the call to wl_data_offer.finish.
#[derive(Debug)]
pub struct WlDataOfferActionEvent {
    /// wl_data_offer:action event
    /// id of the object the event came from
    pub source_id: u32,
    /// action selected by the compositor
    pub dnd_action: enums::WlDataDeviceManagerDndAction,
}

/// Sent when a target accepts pointer_focus or motion events.  If
/// a target does not accept any of the offered types, type is NULL.
///
/// Used for feedback during drag-and-drop.
#[derive(Debug)]
pub struct WlDataSourceTargetEvent {
    /// wl_data_source:target event
    /// id of the object the event came from
    pub source_id: u32,
    /// mime type accepted by the target
    pub mime_type: String,
}

/// Request for data from the client.  Send the data as the
/// specified mime type over the passed file descriptor, then
/// close it.
#[derive(Debug)]
pub struct WlDataSourceSendEvent {
    /// wl_data_source:send event
    /// id of the object the event came from
    pub source_id: u32,
    /// mime type for the data
    pub mime_type: String,
    /// file descriptor for the data
    pub fd: std::os::fd::RawFd,
}

/// This data source is no longer valid. There are several reasons why
/// this could happen:
///
/// - The data source has been replaced by another data source.
/// - The drag-and-drop operation was performed, but the drop destination
/// did not accept any of the mime types offered through
/// wl_data_source.target.
/// - The drag-and-drop operation was performed, but the drop destination
/// did not select any of the actions present in the mask offered through
/// wl_data_source.action.
/// - The drag-and-drop operation was performed but didn't happen over a
/// surface.
/// - The compositor cancelled the drag-and-drop operation (e.g. compositor
/// dependent timeouts to avoid stale drag-and-drop transfers).
///
/// The client should clean up and destroy this data source.
///
/// For objects of version 2 or older, wl_data_source.cancelled will
/// only be emitted if the data source was replaced by another data
/// source.
#[derive(Debug)]
pub struct WlDataSourceCancelledEvent {
    /// wl_data_source:cancelled event
    /// id of the object the event came from
    pub source_id: u32,
}

/// The user performed the drop action. This event does not indicate
/// acceptance, wl_data_source.cancelled may still be emitted afterwards
/// if the drop destination does not accept any mime type.
///
/// However, this event might however not be received if the compositor
/// cancelled the drag-and-drop operation before this event could happen.
///
/// Note that the data_source may still be used in the future and should
/// not be destroyed here.
#[derive(Debug)]
pub struct WlDataSourceDndDropPerformedEvent {
    /// wl_data_source:dnd_drop_performed event
    /// id of the object the event came from
    pub source_id: u32,
}

/// The drop destination finished interoperating with this data
/// source, so the client is now free to destroy this data source and
/// free all associated data.
///
/// If the action used to perform the operation was "move", the
/// source can now delete the transferred data.
#[derive(Debug)]
pub struct WlDataSourceDndFinishedEvent {
    /// wl_data_source:dnd_finished event
    /// id of the object the event came from
    pub source_id: u32,
}

/// This event indicates the action selected by the compositor after
/// matching the source/destination side actions. Only one action (or
/// none) will be offered here.
///
/// This event can be emitted multiple times during the drag-and-drop
/// operation, mainly in response to destination side changes through
/// wl_data_offer.set_actions, and as the data device enters/leaves
/// surfaces.
///
/// It is only possible to receive this event after
/// wl_data_source.dnd_drop_performed if the drag-and-drop operation
/// ended in an "ask" action, in which case the final wl_data_source.action
/// event will happen immediately before wl_data_source.dnd_finished.
///
/// Compositors may also change the selected action on the fly, mainly
/// in response to keyboard modifier changes during the drag-and-drop
/// operation.
///
/// The most recent action received is always the valid one. The chosen
/// action may change alongside negotiation (e.g. an "ask" action can turn
/// into a "move" operation), so the effects of the final action must
/// always be applied in wl_data_offer.dnd_finished.
///
/// Clients can trigger cursor surface changes from this point, so
/// they reflect the current action.
#[derive(Debug)]
pub struct WlDataSourceActionEvent {
    /// wl_data_source:action event
    /// id of the object the event came from
    pub source_id: u32,
    /// action selected by the compositor
    pub dnd_action: enums::WlDataDeviceManagerDndAction,
}

/// The data_offer event introduces a new wl_data_offer object,
/// which will subsequently be used in either the
/// data_device.enter event (for drag-and-drop) or the
/// data_device.selection event (for selections).  Immediately
/// following the data_device.data_offer event, the new data_offer
/// object will send out data_offer.offer events to describe the
/// mime types it offers.
#[derive(Debug)]
pub struct WlDataDeviceDataOfferEvent {
    /// wl_data_device:data_offer event
    /// id of the object the event came from
    pub source_id: u32,
    /// the new data_offer object
    // new_id
    pub id: u32,
}

/// This event is sent when an active drag-and-drop pointer enters
/// a surface owned by the client.  The position of the pointer at
/// enter time is provided by the x and y arguments, in surface-local
/// coordinates.
#[derive(Debug)]
pub struct WlDataDeviceEnterEvent {
    /// wl_data_device:enter event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the enter event
    pub serial: u32,
    /// client surface entered
    pub surface: u32,
    /// surface-local x coordinate
    pub x: Fixed,
    /// surface-local y coordinate
    pub y: Fixed,
    /// source data_offer object
    pub id: u32,
}

/// This event is sent when the drag-and-drop pointer leaves the
/// surface and the session ends.  The client must destroy the
/// wl_data_offer introduced at enter time at this point.
#[derive(Debug)]
pub struct WlDataDeviceLeaveEvent {
    /// wl_data_device:leave event
    /// id of the object the event came from
    pub source_id: u32,
}

/// This event is sent when the drag-and-drop pointer moves within
/// the currently focused surface. The new position of the pointer
/// is provided by the x and y arguments, in surface-local
/// coordinates.
#[derive(Debug)]
pub struct WlDataDeviceMotionEvent {
    /// wl_data_device:motion event
    /// id of the object the event came from
    pub source_id: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// surface-local x coordinate
    pub x: Fixed,
    /// surface-local y coordinate
    pub y: Fixed,
}

/// The event is sent when a drag-and-drop operation is ended
/// because the implicit grab is removed.
///
/// The drag-and-drop destination is expected to honor the last action
/// received through wl_data_offer.action, if the resulting action is
/// "copy" or "move", the destination can still perform
/// wl_data_offer.receive requests, and is expected to end all
/// transfers with a wl_data_offer.finish request.
///
/// If the resulting action is "ask", the action will not be considered
/// final. The drag-and-drop destination is expected to perform one last
/// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
/// to cancel the operation.
#[derive(Debug)]
pub struct WlDataDeviceDropEvent {
    /// wl_data_device:drop event
    /// id of the object the event came from
    pub source_id: u32,
}

/// The selection event is sent out to notify the client of a new
/// wl_data_offer for the selection for this device.  The
/// data_device.data_offer and the data_offer.offer events are
/// sent out immediately before this event to introduce the data
/// offer object.  The selection event is sent to a client
/// immediately before receiving keyboard focus and when a new
/// selection is set while the client has keyboard focus.  The
/// data_offer is valid until a new data_offer or NULL is received
/// or until the client loses keyboard focus.  Switching surface with
/// keyboard focus within the same client doesn't mean a new selection
/// will be sent.  The client must destroy the previous selection
/// data_offer, if any, upon receiving this event.
#[derive(Debug)]
pub struct WlDataDeviceSelectionEvent {
    /// wl_data_device:selection event
    /// id of the object the event came from
    pub source_id: u32,
    /// selection data_offer object
    pub id: u32,
}

/// Ping a client to check if it is receiving events and sending
/// requests. A client is expected to reply with a pong request.
#[derive(Debug)]
pub struct WlShellSurfacePingEvent {
    /// wl_shell_surface:ping event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the ping
    pub serial: u32,
}

/// The configure event asks the client to resize its surface.
///
/// The size is a hint, in the sense that the client is free to
/// ignore it if it doesn't resize, pick a smaller size (to
/// satisfy aspect ratio or resize in steps of NxM pixels).
///
/// The edges parameter provides a hint about how the surface
/// was resized. The client may use this information to decide
/// how to adjust its content to the new size (e.g. a scrolling
/// area might adjust its content position to leave the viewable
/// content unmoved).
///
/// The client is free to dismiss all but the last configure
/// event it received.
///
/// The width and height arguments specify the size of the window
/// in surface-local coordinates.
#[derive(Debug)]
pub struct WlShellSurfaceConfigureEvent {
    /// wl_shell_surface:configure event
    /// id of the object the event came from
    pub source_id: u32,
    /// how the surface was resized
    pub edges: enums::WlShellSurfaceResize,
    /// new width of the surface
    pub width: i32,
    /// new height of the surface
    pub height: i32,
}

/// The popup_done event is sent out when a popup grab is broken,
/// that is, when the user clicks a surface that doesn't belong
/// to the client owning the popup surface.
#[derive(Debug)]
pub struct WlShellSurfacePopupDoneEvent {
    /// wl_shell_surface:popup_done event
    /// id of the object the event came from
    pub source_id: u32,
}

/// Callback event
#[derive(Debug)]
pub struct WlSurfaceFrameDoneEvent{
    /// id of the object the event came from
    pub source_id: u32,
    /// event specific data
    pub data: u32,
}

/// This is emitted whenever a surface's creation, movement, or resizing
/// results in some part of it being within the scanout region of an
/// output.
///
/// Note that a surface may be overlapping with zero or more outputs.
#[derive(Debug)]
pub struct WlSurfaceEnterEvent {
    /// wl_surface:enter event
    /// id of the object the event came from
    pub source_id: u32,
    /// output entered by the surface
    pub output: u32,
}

/// This is emitted whenever a surface's creation, movement, or resizing
/// results in it no longer having any part of it within the scanout region
/// of an output.
///
/// Clients should not use the number of outputs the surface is on for frame
/// throttling purposes. The surface might be hidden even if no leave event
/// has been sent, and the compositor might expect new surface content
/// updates even if no enter event has been sent. The frame event should be
/// used instead.
#[derive(Debug)]
pub struct WlSurfaceLeaveEvent {
    /// wl_surface:leave event
    /// id of the object the event came from
    pub source_id: u32,
    /// output left by the surface
    pub output: u32,
}

/// This event indicates the preferred buffer scale for this surface. It is
/// sent whenever the compositor's preference changes.
///
/// It is intended that scaling aware clients use this event to scale their
/// content and use wl_surface.set_buffer_scale to indicate the scale they
/// have rendered with. This allows clients to supply a higher detail
/// buffer.
#[derive(Debug)]
pub struct WlSurfacePreferredBufferScaleEvent {
    /// wl_surface:preferred_buffer_scale event
    /// id of the object the event came from
    pub source_id: u32,
    /// preferred scaling factor
    pub factor: i32,
}

/// This event indicates the preferred buffer transform for this surface.
/// It is sent whenever the compositor's preference changes.
///
/// It is intended that transform aware clients use this event to apply the
/// transform to their content and use wl_surface.set_buffer_transform to
/// indicate the transform they have rendered with.
#[derive(Debug)]
pub struct WlSurfacePreferredBufferTransformEvent {
    /// wl_surface:preferred_buffer_transform event
    /// id of the object the event came from
    pub source_id: u32,
    /// preferred transform
    pub transform: enums::WlOutputTransform,
}

/// This is emitted whenever a seat gains or loses the pointer,
/// keyboard or touch capabilities.  The argument is a capability
/// enum containing the complete set of capabilities this seat has.
///
/// When the pointer capability is added, a client may create a
/// wl_pointer object using the wl_seat.get_pointer request. This object
/// will receive pointer events until the capability is removed in the
/// future.
///
/// When the pointer capability is removed, a client should destroy the
/// wl_pointer objects associated with the seat where the capability was
/// removed, using the wl_pointer.release request. No further pointer
/// events will be received on these objects.
///
/// In some compositors, if a seat regains the pointer capability and a
/// client has a previously obtained wl_pointer object of version 4 or
/// less, that object may start sending pointer events again. This
/// behavior is considered a misinterpretation of the intended behavior
/// and must not be relied upon by the client. wl_pointer objects of
/// version 5 or later must not send events if created before the most
/// recent event notifying the client of an added pointer capability.
///
/// The above behavior also applies to wl_keyboard and wl_touch with the
/// keyboard and touch capabilities, respectively.
#[derive(Debug)]
pub struct WlSeatCapabilitiesEvent {
    /// wl_seat:capabilities event
    /// id of the object the event came from
    pub source_id: u32,
    /// capabilities of the seat
    pub capabilities: enums::WlSeatCapability,
}

/// In a multi-seat configuration the seat name can be used by clients to
/// help identify which physical devices the seat represents.
///
/// The seat name is a UTF-8 string with no convention defined for its
/// contents. Each name is unique among all wl_seat globals. The name is
/// only guaranteed to be unique for the current compositor instance.
///
/// The same seat names are used for all clients. Thus, the name can be
/// shared across processes to refer to a specific wl_seat global.
///
/// The name event is sent after binding to the seat global. This event is
/// only sent once per seat object, and the name does not change over the
/// lifetime of the wl_seat global.
///
/// Compositors may re-use the same seat name if the wl_seat global is
/// destroyed and re-created later.
#[derive(Debug)]
pub struct WlSeatNameEvent {
    /// wl_seat:name event
    /// id of the object the event came from
    pub source_id: u32,
    /// seat identifier
    pub name: String,
}

/// Notification that this seat's pointer is focused on a certain
/// surface.
///
/// When a seat's focus enters a surface, the pointer image
/// is undefined and a client should respond to this event by setting
/// an appropriate pointer image with the set_cursor request.
#[derive(Debug)]
pub struct WlPointerEnterEvent {
    /// wl_pointer:enter event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the enter event
    pub serial: u32,
    /// surface entered by the pointer
    pub surface: u32,
    /// surface-local x coordinate
    pub surface_x: Fixed,
    /// surface-local y coordinate
    pub surface_y: Fixed,
}

/// Notification that this seat's pointer is no longer focused on
/// a certain surface.
///
/// The leave notification is sent before the enter notification
/// for the new focus.
#[derive(Debug)]
pub struct WlPointerLeaveEvent {
    /// wl_pointer:leave event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the leave event
    pub serial: u32,
    /// surface left by the pointer
    pub surface: u32,
}

/// Notification of pointer location change. The arguments
/// surface_x and surface_y are the location relative to the
/// focused surface.
#[derive(Debug)]
pub struct WlPointerMotionEvent {
    /// wl_pointer:motion event
    /// id of the object the event came from
    pub source_id: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// surface-local x coordinate
    pub surface_x: Fixed,
    /// surface-local y coordinate
    pub surface_y: Fixed,
}

/// Mouse button click and release notifications.
///
/// The location of the click is given by the last motion or
/// enter event.
/// The time argument is a timestamp with millisecond
/// granularity, with an undefined base.
///
/// The button is a button code as defined in the Linux kernel's
/// linux/input-event-codes.h header file, e.g. BTN_LEFT.
///
/// Any 16-bit button code value is reserved for future additions to the
/// kernel's event code list. All other button codes above 0xFFFF are
/// currently undefined but may be used in future versions of this
/// protocol.
#[derive(Debug)]
pub struct WlPointerButtonEvent {
    /// wl_pointer:button event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the button event
    pub serial: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// button that produced the event
    pub button: u32,
    /// physical state of the button
    pub state: enums::WlPointerButtonState,
}

/// Scroll and other axis notifications.
///
/// For scroll events (vertical and horizontal scroll axes), the
/// value parameter is the length of a vector along the specified
/// axis in a coordinate space identical to those of motion events,
/// representing a relative movement along the specified axis.
///
/// For devices that support movements non-parallel to axes multiple
/// axis events will be emitted.
///
/// When applicable, for example for touch pads, the server can
/// choose to emit scroll events where the motion vector is
/// equivalent to a motion event vector.
///
/// When applicable, a client can transform its content relative to the
/// scroll distance.
#[derive(Debug)]
pub struct WlPointerAxisEvent {
    /// wl_pointer:axis event
    /// id of the object the event came from
    pub source_id: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// axis type
    pub axis: enums::WlPointerAxis,
    /// length of vector in surface-local coordinate space
    pub value: Fixed,
}

/// Indicates the end of a set of events that logically belong together.
/// A client is expected to accumulate the data in all events within the
/// frame before proceeding.
///
/// All wl_pointer events before a wl_pointer.frame event belong
/// logically together. For example, in a diagonal scroll motion the
/// compositor will send an optional wl_pointer.axis_source event, two
/// wl_pointer.axis events (horizontal and vertical) and finally a
/// wl_pointer.frame event. The client may use this information to
/// calculate a diagonal vector for scrolling.
///
/// When multiple wl_pointer.axis events occur within the same frame,
/// the motion vector is the combined motion of all events.
/// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
/// the same frame, this indicates that axis movement in one axis has
/// stopped but continues in the other axis.
/// When multiple wl_pointer.axis_stop events occur within the same
/// frame, this indicates that these axes stopped in the same instance.
///
/// A wl_pointer.frame event is sent for every logical event group,
/// even if the group only contains a single wl_pointer event.
/// Specifically, a client may get a sequence: motion, frame, button,
/// frame, axis, frame, axis_stop, frame.
///
/// The wl_pointer.enter and wl_pointer.leave events are logical events
/// generated by the compositor and not the hardware. These events are
/// also grouped by a wl_pointer.frame. When a pointer moves from one
/// surface to another, a compositor should group the
/// wl_pointer.leave event within the same wl_pointer.frame.
/// However, a client must not rely on wl_pointer.leave and
/// wl_pointer.enter being in the same wl_pointer.frame.
/// Compositor-specific policies may require the wl_pointer.leave and
/// wl_pointer.enter event being split across multiple wl_pointer.frame
/// groups.
#[derive(Debug)]
pub struct WlPointerFrameEvent {
    /// wl_pointer:frame event
    /// id of the object the event came from
    pub source_id: u32,
}

/// Source information for scroll and other axes.
///
/// This event does not occur on its own. It is sent before a
/// wl_pointer.frame event and carries the source information for
/// all events within that frame.
///
/// The source specifies how this event was generated. If the source is
/// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
/// sent when the user lifts the finger off the device.
///
/// If the source is wl_pointer.axis_source.wheel,
/// wl_pointer.axis_source.wheel_tilt or
/// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
/// or may not be sent. Whether a compositor sends an axis_stop event
/// for these sources is hardware-specific and implementation-dependent;
/// clients must not rely on receiving an axis_stop event for these
/// scroll sources and should treat scroll sequences from these scroll
/// sources as unterminated by default.
///
/// This event is optional. If the source is unknown for a particular
/// axis event sequence, no event is sent.
/// Only one wl_pointer.axis_source event is permitted per frame.
///
/// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
/// not guaranteed.
#[derive(Debug)]
pub struct WlPointerAxisSourceEvent {
    /// wl_pointer:axis_source event
    /// id of the object the event came from
    pub source_id: u32,
    /// source of the axis event
    pub axis_source: enums::WlPointerAxisSource,
}

/// Stop notification for scroll and other axes.
///
/// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
/// is sent to notify a client that the axis sequence has terminated.
/// This enables the client to implement kinetic scrolling.
/// See the wl_pointer.axis_source documentation for information on when
/// this event may be generated.
///
/// Any wl_pointer.axis events with the same axis_source after this
/// event should be considered as the start of a new axis motion.
///
/// The timestamp is to be interpreted identical to the timestamp in the
/// wl_pointer.axis event. The timestamp value may be the same as a
/// preceding wl_pointer.axis event.
#[derive(Debug)]
pub struct WlPointerAxisStopEvent {
    /// wl_pointer:axis_stop event
    /// id of the object the event came from
    pub source_id: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// the axis stopped with this event
    pub axis: enums::WlPointerAxis,
}

/// Discrete step information for scroll and other axes.
///
/// This event carries the axis value of the wl_pointer.axis event in
/// discrete steps (e.g. mouse wheel clicks).
///
/// This event is deprecated with wl_pointer version 8 - this event is not
/// sent to clients supporting version 8 or later.
///
/// This event does not occur on its own, it is coupled with a
/// wl_pointer.axis event that represents this axis value on a
/// continuous scale. The protocol guarantees that each axis_discrete
/// event is always followed by exactly one axis event with the same
/// axis number within the same wl_pointer.frame. Note that the protocol
/// allows for other events to occur between the axis_discrete and
/// its coupled axis event, including other axis_discrete or axis
/// events. A wl_pointer.frame must not contain more than one axis_discrete
/// event per axis type.
///
/// This event is optional; continuous scrolling devices
/// like two-finger scrolling on touchpads do not have discrete
/// steps and do not generate this event.
///
/// The discrete value carries the directional information. e.g. a value
/// of -2 is two steps towards the negative direction of this axis.
///
/// The axis number is identical to the axis number in the associated
/// axis event.
///
/// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
/// not guaranteed.
#[derive(Debug)]
pub struct WlPointerAxisDiscreteEvent {
    /// wl_pointer:axis_discrete event
    /// id of the object the event came from
    pub source_id: u32,
    /// axis type
    pub axis: enums::WlPointerAxis,
    /// number of steps
    pub discrete: i32,
}

/// Discrete high-resolution scroll information.
///
/// This event carries high-resolution wheel scroll information,
/// with each multiple of 120 representing one logical scroll step
/// (a wheel detent). For example, an axis_value120 of 30 is one quarter of
/// a logical scroll step in the positive direction, a value120 of
/// -240 are two logical scroll steps in the negative direction within the
/// same hardware event.
/// Clients that rely on discrete scrolling should accumulate the
/// value120 to multiples of 120 before processing the event.
///
/// The value120 must not be zero.
///
/// This event replaces the wl_pointer.axis_discrete event in clients
/// supporting wl_pointer version 8 or later.
///
/// Where a wl_pointer.axis_source event occurs in the same
/// wl_pointer.frame, the axis source applies to this event.
///
/// The order of wl_pointer.axis_value120 and wl_pointer.axis_source is
/// not guaranteed.
#[derive(Debug)]
pub struct WlPointerAxisValue120Event {
    /// wl_pointer:axis_value120 event
    /// id of the object the event came from
    pub source_id: u32,
    /// axis type
    pub axis: enums::WlPointerAxis,
    /// scroll distance as fraction of 120
    pub value120: i32,
}

/// Relative directional information of the entity causing the axis
/// motion.
///
/// For a wl_pointer.axis event, the wl_pointer.axis_relative_direction
/// event specifies the movement direction of the entity causing the
/// wl_pointer.axis event. For example:
/// - if a user's fingers on a touchpad move down and this
/// causes a wl_pointer.axis vertical_scroll down event, the physical
/// direction is 'identical'
/// - if a user's fingers on a touchpad move down and this causes a
/// wl_pointer.axis vertical_scroll up scroll up event ('natural
/// scrolling'), the physical direction is 'inverted'.
///
/// A client may use this information to adjust scroll motion of
/// components. Specifically, enabling natural scrolling causes the
/// content to change direction compared to traditional scrolling.
/// Some widgets like volume control sliders should usually match the
/// physical direction regardless of whether natural scrolling is
/// active. This event enables clients to match the scroll direction of
/// a widget to the physical direction.
///
/// This event does not occur on its own, it is coupled with a
/// wl_pointer.axis event that represents this axis value.
/// The protocol guarantees that each axis_relative_direction event is
/// always followed by exactly one axis event with the same
/// axis number within the same wl_pointer.frame. Note that the protocol
/// allows for other events to occur between the axis_relative_direction
/// and its coupled axis event.
///
/// The axis number is identical to the axis number in the associated
/// axis event.
///
/// The order of wl_pointer.axis_relative_direction,
/// wl_pointer.axis_discrete and wl_pointer.axis_source is not
/// guaranteed.
#[derive(Debug)]
pub struct WlPointerAxisRelativeDirectionEvent {
    /// wl_pointer:axis_relative_direction event
    /// id of the object the event came from
    pub source_id: u32,
    /// axis type
    pub axis: enums::WlPointerAxis,
    /// physical direction relative to axis motion
    pub direction: enums::WlPointerAxisRelativeDirection,
}

/// This event provides a file descriptor to the client which can be
/// memory-mapped in read-only mode to provide a keyboard mapping
/// description.
///
/// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
/// the recipient, as MAP_SHARED may fail.
#[derive(Debug)]
pub struct WlKeyboardKeymapEvent {
    /// wl_keyboard:keymap event
    /// id of the object the event came from
    pub source_id: u32,
    /// keymap format
    pub format: enums::WlKeyboardKeymapFormat,
    /// keymap file descriptor
    pub fd: std::os::fd::RawFd,
    /// keymap size, in bytes
    pub size: u32,
}

/// Notification that this seat's keyboard focus is on a certain
/// surface.
///
/// The compositor must send the wl_keyboard.modifiers event after this
/// event.
#[derive(Debug)]
pub struct WlKeyboardEnterEvent {
    /// wl_keyboard:enter event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the enter event
    pub serial: u32,
    /// surface gaining keyboard focus
    pub surface: u32,
    /// the currently pressed keys
    pub keys: Vec<u8>,
}

/// Notification that this seat's keyboard focus is no longer on
/// a certain surface.
///
/// The leave notification is sent before the enter notification
/// for the new focus.
///
/// After this event client must assume that no keys are pressed,
/// it must stop key repeating if there's some going on and until
/// it receives the next wl_keyboard.modifiers event, the client
/// must also assume no modifiers are active.
#[derive(Debug)]
pub struct WlKeyboardLeaveEvent {
    /// wl_keyboard:leave event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the leave event
    pub serial: u32,
    /// surface that lost keyboard focus
    pub surface: u32,
}

/// A key was pressed or released.
/// The time argument is a timestamp with millisecond
/// granularity, with an undefined base.
///
/// The key is a platform-specific key code that can be interpreted
/// by feeding it to the keyboard mapping (see the keymap event).
///
/// If this event produces a change in modifiers, then the resulting
/// wl_keyboard.modifiers event must be sent after this event.
///
/// The compositor must not send this event without a surface of the client
/// having keyboard focus.
#[derive(Debug)]
pub struct WlKeyboardKeyEvent {
    /// wl_keyboard:key event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the key event
    pub serial: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// key that produced the event
    pub key: u32,
    /// physical state of the key
    pub state: enums::WlKeyboardKeyState,
}

/// Notifies clients that the modifier and/or group state has
/// changed, and it should update its local state.
///
/// The compositor may send this event without a surface of the client
/// having keyboard focus, for example to tie modifier information to
/// pointer focus instead. If a modifier event with pressed modifiers is sent
/// without a prior enter event, the client can assume the modifier state is
/// valid until it receives the next wl_keyboard.modifiers event. In order to
/// reset the modifier state again, the compositor can send a
/// wl_keyboard.modifiers event with no pressed modifiers.
#[derive(Debug)]
pub struct WlKeyboardModifiersEvent {
    /// wl_keyboard:modifiers event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the modifiers event
    pub serial: u32,
    /// depressed modifiers
    pub mods_depressed: u32,
    /// latched modifiers
    pub mods_latched: u32,
    /// locked modifiers
    pub mods_locked: u32,
    /// keyboard layout
    pub group: u32,
}

/// Informs the client about the keyboard's repeat rate and delay.
///
/// This event is sent as soon as the wl_keyboard object has been created,
/// and is guaranteed to be received by the client before any key press
/// event.
///
/// Negative values for either rate or delay are illegal. A rate of zero
/// will disable any repeating (regardless of the value of delay).
///
/// This event can be sent later on as well with a new value if necessary,
/// so clients should continue listening for the event past the creation
/// of wl_keyboard.
#[derive(Debug)]
pub struct WlKeyboardRepeatInfoEvent {
    /// wl_keyboard:repeat_info event
    /// id of the object the event came from
    pub source_id: u32,
    /// the rate of repeating keys in characters per second
    pub rate: i32,
    /// delay in milliseconds since key down until repeating starts
    pub delay: i32,
}

/// A new touch point has appeared on the surface. This touch point is
/// assigned a unique ID. Future events from this touch point reference
/// this ID. The ID ceases to be valid after a touch up event and may be
/// reused in the future.
#[derive(Debug)]
pub struct WlTouchDownEvent {
    /// wl_touch:down event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the touch down event
    pub serial: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// surface touched
    pub surface: u32,
    /// the unique ID of this touch point
    pub id: i32,
    /// surface-local x coordinate
    pub x: Fixed,
    /// surface-local y coordinate
    pub y: Fixed,
}

/// The touch point has disappeared. No further events will be sent for
/// this touch point and the touch point's ID is released and may be
/// reused in a future touch down event.
#[derive(Debug)]
pub struct WlTouchUpEvent {
    /// wl_touch:up event
    /// id of the object the event came from
    pub source_id: u32,
    /// serial number of the touch up event
    pub serial: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// the unique ID of this touch point
    pub id: i32,
}

/// A touch point has changed coordinates.
#[derive(Debug)]
pub struct WlTouchMotionEvent {
    /// wl_touch:motion event
    /// id of the object the event came from
    pub source_id: u32,
    /// timestamp with millisecond granularity
    pub time: u32,
    /// the unique ID of this touch point
    pub id: i32,
    /// surface-local x coordinate
    pub x: Fixed,
    /// surface-local y coordinate
    pub y: Fixed,
}

/// Indicates the end of a set of events that logically belong together.
/// A client is expected to accumulate the data in all events within the
/// frame before proceeding.
///
/// A wl_touch.frame terminates at least one event but otherwise no
/// guarantee is provided about the set of events within a frame. A client
/// must assume that any state not updated in a frame is unchanged from the
/// previously known state.
#[derive(Debug)]
pub struct WlTouchFrameEvent {
    /// wl_touch:frame event
    /// id of the object the event came from
    pub source_id: u32,
}

/// Sent if the compositor decides the touch stream is a global
/// gesture. No further events are sent to the clients from that
/// particular gesture. Touch cancellation applies to all touch points
/// currently active on this client's surface. The client is
/// responsible for finalizing the touch points, future touch points on
/// this surface may reuse the touch point ID.
#[derive(Debug)]
pub struct WlTouchCancelEvent {
    /// wl_touch:cancel event
    /// id of the object the event came from
    pub source_id: u32,
}

/// Sent when a touchpoint has changed its shape.
///
/// This event does not occur on its own. It is sent before a
/// wl_touch.frame event and carries the new shape information for
/// any previously reported, or new touch points of that frame.
///
/// Other events describing the touch point such as wl_touch.down,
/// wl_touch.motion or wl_touch.orientation may be sent within the
/// same wl_touch.frame. A client should treat these events as a single
/// logical touch point update. The order of wl_touch.shape,
/// wl_touch.orientation and wl_touch.motion is not guaranteed.
/// A wl_touch.down event is guaranteed to occur before the first
/// wl_touch.shape event for this touch ID but both events may occur within
/// the same wl_touch.frame.
///
/// A touchpoint shape is approximated by an ellipse through the major and
/// minor axis length. The major axis length describes the longer diameter
/// of the ellipse, while the minor axis length describes the shorter
/// diameter. Major and minor are orthogonal and both are specified in
/// surface-local coordinates. The center of the ellipse is always at the
/// touchpoint location as reported by wl_touch.down or wl_touch.move.
///
/// This event is only sent by the compositor if the touch device supports
/// shape reports. The client has to make reasonable assumptions about the
/// shape if it did not receive this event.
#[derive(Debug)]
pub struct WlTouchShapeEvent {
    /// wl_touch:shape event
    /// id of the object the event came from
    pub source_id: u32,
    /// the unique ID of this touch point
    pub id: i32,
    /// length of the major axis in surface-local coordinates
    pub major: Fixed,
    /// length of the minor axis in surface-local coordinates
    pub minor: Fixed,
}

/// Sent when a touchpoint has changed its orientation.
///
/// This event does not occur on its own. It is sent before a
/// wl_touch.frame event and carries the new shape information for
/// any previously reported, or new touch points of that frame.
///
/// Other events describing the touch point such as wl_touch.down,
/// wl_touch.motion or wl_touch.shape may be sent within the
/// same wl_touch.frame. A client should treat these events as a single
/// logical touch point update. The order of wl_touch.shape,
/// wl_touch.orientation and wl_touch.motion is not guaranteed.
/// A wl_touch.down event is guaranteed to occur before the first
/// wl_touch.orientation event for this touch ID but both events may occur
/// within the same wl_touch.frame.
///
/// The orientation describes the clockwise angle of a touchpoint's major
/// axis to the positive surface y-axis and is normalized to the -180 to
/// +180 degree range. The granularity of orientation depends on the touch
/// device, some devices only support binary rotation values between 0 and
/// 90 degrees.
///
/// This event is only sent by the compositor if the touch device supports
/// orientation reports.
#[derive(Debug)]
pub struct WlTouchOrientationEvent {
    /// wl_touch:orientation event
    /// id of the object the event came from
    pub source_id: u32,
    /// the unique ID of this touch point
    pub id: i32,
    /// angle between major axis and positive surface y-axis in degrees
    pub orientation: Fixed,
}

/// The geometry event describes geometric properties of the output.
/// The event is sent when binding to the output object and whenever
/// any of the properties change.
///
/// The physical size can be set to zero if it doesn't make sense for this
/// output (e.g. for projectors or virtual outputs).
///
/// The geometry event will be followed by a done event (starting from
/// version 2).
///
/// Note: wl_output only advertises partial information about the output
/// position and identification. Some compositors, for instance those not
/// implementing a desktop-style output layout or those exposing virtual
/// outputs, might fake this information. Instead of using x and y, clients
/// should use xdg_output.logical_position. Instead of using make and model,
/// clients should use name and description.
#[derive(Debug)]
pub struct WlOutputGeometryEvent {
    /// wl_output:geometry event
    /// id of the object the event came from
    pub source_id: u32,
    /// x position within the global compositor space
    pub x: i32,
    /// y position within the global compositor space
    pub y: i32,
    /// width in millimeters of the output
    pub physical_width: i32,
    /// height in millimeters of the output
    pub physical_height: i32,
    /// subpixel orientation of the output
    pub subpixel: enums::WlOutputSubpixel,
    /// textual description of the manufacturer
    pub make: String,
    /// textual description of the model
    pub model: String,
    /// transform that maps framebuffer to output
    pub transform: enums::WlOutputTransform,
}

/// The mode event describes an available mode for the output.
///
/// The event is sent when binding to the output object and there
/// will always be one mode, the current mode.  The event is sent
/// again if an output changes mode, for the mode that is now
/// current.  In other words, the current mode is always the last
/// mode that was received with the current flag set.
///
/// Non-current modes are deprecated. A compositor can decide to only
/// advertise the current mode and never send other modes. Clients
/// should not rely on non-current modes.
///
/// The size of a mode is given in physical hardware units of
/// the output device. This is not necessarily the same as
/// the output size in the global compositor space. For instance,
/// the output may be scaled, as described in wl_output.scale,
/// or transformed, as described in wl_output.transform. Clients
/// willing to retrieve the output size in the global compositor
/// space should use xdg_output.logical_size instead.
///
/// The vertical refresh rate can be set to zero if it doesn't make
/// sense for this output (e.g. for virtual outputs).
///
/// The mode event will be followed by a done event (starting from
/// version 2).
///
/// Clients should not use the refresh rate to schedule frames. Instead,
/// they should use the wl_surface.frame event or the presentation-time
/// protocol.
///
/// Note: this information is not always meaningful for all outputs. Some
/// compositors, such as those exposing virtual outputs, might fake the
/// refresh rate or the size.
#[derive(Debug)]
pub struct WlOutputModeEvent {
    /// wl_output:mode event
    /// id of the object the event came from
    pub source_id: u32,
    /// bitfield of mode flags
    pub flags: enums::WlOutputMode,
    /// width of the mode in hardware units
    pub width: i32,
    /// height of the mode in hardware units
    pub height: i32,
    /// vertical refresh rate in mHz
    pub refresh: i32,
}

/// This event is sent after all other properties have been
/// sent after binding to the output object and after any
/// other property changes done after that. This allows
/// changes to the output properties to be seen as
/// atomic, even if they happen via multiple events.
#[derive(Debug)]
pub struct WlOutputDoneEvent {
    /// wl_output:done event
    /// id of the object the event came from
    pub source_id: u32,
}

/// This event contains scaling geometry information
/// that is not in the geometry event. It may be sent after
/// binding the output object or if the output scale changes
/// later. If it is not sent, the client should assume a
/// scale of 1.
///
/// A scale larger than 1 means that the compositor will
/// automatically scale surface buffers by this amount
/// when rendering. This is used for very high resolution
/// displays where applications rendering at the native
/// resolution would be too small to be legible.
///
/// It is intended that scaling aware clients track the
/// current output of a surface, and if it is on a scaled
/// output it should use wl_surface.set_buffer_scale with
/// the scale of the output. That way the compositor can
/// avoid scaling the surface, and the client can supply
/// a higher detail image.
///
/// The scale event will be followed by a done event.
#[derive(Debug)]
pub struct WlOutputScaleEvent {
    /// wl_output:scale event
    /// id of the object the event came from
    pub source_id: u32,
    /// scaling factor of output
    pub factor: i32,
}

/// Many compositors will assign user-friendly names to their outputs, show
/// them to the user, allow the user to refer to an output, etc. The client
/// may wish to know this name as well to offer the user similar behaviors.
///
/// The name is a UTF-8 string with no convention defined for its contents.
/// Each name is unique among all wl_output globals. The name is only
/// guaranteed to be unique for the compositor instance.
///
/// The same output name is used for all clients for a given wl_output
/// global. Thus, the name can be shared across processes to refer to a
/// specific wl_output global.
///
/// The name is not guaranteed to be persistent across sessions, thus cannot
/// be used to reliably identify an output in e.g. configuration files.
///
/// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
/// not assume that the name is a reflection of an underlying DRM connector,
/// X11 connection, etc.
///
/// The name event is sent after binding the output object. This event is
/// only sent once per output object, and the name does not change over the
/// lifetime of the wl_output global.
///
/// Compositors may re-use the same output name if the wl_output global is
/// destroyed and re-created later. Compositors should avoid re-using the
/// same name if possible.
///
/// The name event will be followed by a done event.
#[derive(Debug)]
pub struct WlOutputNameEvent {
    /// wl_output:name event
    /// id of the object the event came from
    pub source_id: u32,
    /// output name
    pub name: String,
}

/// Many compositors can produce human-readable descriptions of their
/// outputs. The client may wish to know this description as well, e.g. for
/// output selection purposes.
///
/// The description is a UTF-8 string with no convention defined for its
/// contents. The description is not guaranteed to be unique among all
/// wl_output globals. Examples might include 'Foocorp 11" Display' or
/// 'Virtual X11 output via :1'.
///
/// The description event is sent after binding the output object and
/// whenever the description changes. The description is optional, and may
/// not be sent at all.
///
/// The description event will be followed by a done event.
#[derive(Debug)]
pub struct WlOutputDescriptionEvent {
    /// wl_output:description event
    /// id of the object the event came from
    pub source_id: u32,
    /// output description
    pub description: String,
}

