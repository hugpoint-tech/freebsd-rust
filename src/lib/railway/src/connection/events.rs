use crate::connection::WaylandConnection;
use crate::connection::WaylandConnectionPrivate;
use crate::types::Fixed;
use crate::types::events::*;
use crate::types::handler::EventHandler;
use crate::types::object::Object;

impl WaylandConnectionPrivate {

    pub(crate) fn dispatch_event<T: EventHandler>(&mut self, c: &WaylandConnection, state: &mut T) {
        if self.recv_pos >= self.recv_buf.len() {
            return
        }

        let hdr = self.get_header();
        let id =  hdr.obj_id;
        let op = hdr.opcode;
        let obj = self.objects[id as usize];

        match obj {
            Object::Null => panic!("object of type null was sent by the server"),
            Object::XdgWmBase => {
                match op {
                    0 =>{
                        let event = XdgWmBasePingEvent {
                            source_id: id,
                            serial: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_wm_base_ping(event, c);
                    },
                    _ => (),
                }
            },
            Object::XdgPositioner => panic!("event from object with no events"),
            Object::XdgSurface => {
                match op {
                    0 =>{
                        let event = XdgSurfaceConfigureEvent {
                            source_id: id,
                            serial: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_surface_configure(event, c);
                    },
                    _ => (),
                }
            },
            Object::XdgToplevel => {
                match op {
                    0 =>{
                        let event = XdgToplevelConfigureEvent {
                            source_id: id,
                            width: self.get_int(),
                            height: self.get_int(),
                            states: self.get_vec(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_toplevel_configure(event, c);
                    },
                    1 =>{
                        let event = XdgToplevelCloseEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_toplevel_close(event, c);
                    },
                    2 =>{
                        let event = XdgToplevelConfigureBoundsEvent {
                            source_id: id,
                            width: self.get_int(),
                            height: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_toplevel_configure_bounds(event, c);
                    },
                    3 =>{
                        let event = XdgToplevelWmCapabilitiesEvent {
                            source_id: id,
                            capabilities: self.get_vec(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_toplevel_wm_capabilities(event, c);
                    },
                    _ => (),
                }
            },
            Object::XdgPopup => {
                match op {
                    0 =>{
                        let event = XdgPopupConfigureEvent {
                            source_id: id,
                            x: self.get_int(),
                            y: self.get_int(),
                            width: self.get_int(),
                            height: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_popup_configure(event, c);
                    },
                    1 =>{
                        let event = XdgPopupPopupDoneEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_popup_popup_done(event, c);
                    },
                    2 =>{
                        let event = XdgPopupRepositionedEvent {
                            source_id: id,
                            token: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_xdg_popup_repositioned(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlDisplaySyncCallback => {
                match op {
                    0 =>{
                        let event = WlDisplaySyncDoneEvent {
                                source_id: id,
                                data: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_display_sync_done(event, c);
                    },
                    _ => ()
                }
            },

            Object::WlDisplay => {
                match op {
                    0 =>{
                        let event = WlDisplayErrorEvent {
                            source_id: id,
                            object_id: self.get_uint(),
                            code: self.get_uint(),
                            message: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_display_error(event, c);
                    },
                    1 =>{
                        let event = WlDisplayDeleteIdEvent {
                            source_id: id,
                            id: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_display_delete_id(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlRegistry => {
                match op {
                    0 =>{
                        let event = WlRegistryGlobalEvent {
                            source_id: id,
                            name: self.get_uint(),
                            interface: self.get_str(),
                            version: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_registry_global(event, c);
                    },
                    1 =>{
                        let event = WlRegistryGlobalRemoveEvent {
                            source_id: id,
                            name: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_registry_global_remove(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlCompositor => panic!("event from object with no events"),
            Object::WlShmPool => panic!("event from object with no events"),
            Object::WlShm => {
                match op {
                    0 =>{
                        let event = WlShmFormatEvent {
                            source_id: id,
                            format: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_shm_format(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlBuffer => {
                match op {
                    0 =>{
                        let event = WlBufferReleaseEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_buffer_release(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlDataOffer => {
                match op {
                    0 =>{
                        let event = WlDataOfferOfferEvent {
                            source_id: id,
                            mime_type: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_offer_offer(event, c);
                    },
                    1 =>{
                        let event = WlDataOfferSourceActionsEvent {
                            source_id: id,
                            source_actions: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_offer_source_actions(event, c);
                    },
                    2 =>{
                        let event = WlDataOfferActionEvent {
                            source_id: id,
                            dnd_action: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_offer_action(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlDataSource => {
                match op {
                    0 =>{
                        let event = WlDataSourceTargetEvent {
                            source_id: id,
                            mime_type: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_target(event, c);
                    },
                    1 =>{
                        let event = WlDataSourceSendEvent {
                            source_id: id,
                            mime_type: self.get_str(),
                            fd: self.get_fd(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_send(event, c);
                    },
                    2 =>{
                        let event = WlDataSourceCancelledEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_cancelled(event, c);
                    },
                    3 =>{
                        let event = WlDataSourceDndDropPerformedEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_dnd_drop_performed(event, c);
                    },
                    4 =>{
                        let event = WlDataSourceDndFinishedEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_dnd_finished(event, c);
                    },
                    5 =>{
                        let event = WlDataSourceActionEvent {
                            source_id: id,
                            dnd_action: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_source_action(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlDataDevice => {
                match op {
                    0 =>{
                        let event = WlDataDeviceDataOfferEvent {
                            source_id: id,
                            // new_id
                            id: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_data_offer(event, c);
                    },
                    1 =>{
                        let event = WlDataDeviceEnterEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            surface: self.get_uint(),
                            x: Fixed::new(self.get_uint()),
                            y: Fixed::new(self.get_uint()),
                            id: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_enter(event, c);
                    },
                    2 =>{
                        let event = WlDataDeviceLeaveEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_leave(event, c);
                    },
                    3 =>{
                        let event = WlDataDeviceMotionEvent {
                            source_id: id,
                            time: self.get_uint(),
                            x: Fixed::new(self.get_uint()),
                            y: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_motion(event, c);
                    },
                    4 =>{
                        let event = WlDataDeviceDropEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_drop(event, c);
                    },
                    5 =>{
                        let event = WlDataDeviceSelectionEvent {
                            source_id: id,
                            id: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_data_device_selection(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlDataDeviceManager => panic!("event from object with no events"),
            Object::WlShell => panic!("event from object with no events"),
            Object::WlShellSurface => {
                match op {
                    0 =>{
                        let event = WlShellSurfacePingEvent {
                            source_id: id,
                            serial: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_shell_surface_ping(event, c);
                    },
                    1 =>{
                        let event = WlShellSurfaceConfigureEvent {
                            source_id: id,
                            edges: self.get_uint().into(),
                            width: self.get_int(),
                            height: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_shell_surface_configure(event, c);
                    },
                    2 =>{
                        let event = WlShellSurfacePopupDoneEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_shell_surface_popup_done(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlSurfaceFrameCallback => {
                match op {
                    0 =>{
                        let event = WlSurfaceFrameDoneEvent {
                                source_id: id,
                                data: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_surface_frame_done(event, c);
                    },
                    _ => ()
                }
            },

            Object::WlSurface => {
                match op {
                    0 =>{
                        let event = WlSurfaceEnterEvent {
                            source_id: id,
                            output: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_surface_enter(event, c);
                    },
                    1 =>{
                        let event = WlSurfaceLeaveEvent {
                            source_id: id,
                            output: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_surface_leave(event, c);
                    },
                    2 =>{
                        let event = WlSurfacePreferredBufferScaleEvent {
                            source_id: id,
                            factor: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_surface_preferred_buffer_scale(event, c);
                    },
                    3 =>{
                        let event = WlSurfacePreferredBufferTransformEvent {
                            source_id: id,
                            transform: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_surface_preferred_buffer_transform(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlSeat => {
                match op {
                    0 =>{
                        let event = WlSeatCapabilitiesEvent {
                            source_id: id,
                            capabilities: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_seat_capabilities(event, c);
                    },
                    1 =>{
                        let event = WlSeatNameEvent {
                            source_id: id,
                            name: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_seat_name(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlPointer => {
                match op {
                    0 =>{
                        let event = WlPointerEnterEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            surface: self.get_uint(),
                            surface_x: Fixed::new(self.get_uint()),
                            surface_y: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_enter(event, c);
                    },
                    1 =>{
                        let event = WlPointerLeaveEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            surface: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_leave(event, c);
                    },
                    2 =>{
                        let event = WlPointerMotionEvent {
                            source_id: id,
                            time: self.get_uint(),
                            surface_x: Fixed::new(self.get_uint()),
                            surface_y: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_motion(event, c);
                    },
                    3 =>{
                        let event = WlPointerButtonEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            time: self.get_uint(),
                            button: self.get_uint(),
                            state: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_button(event, c);
                    },
                    4 =>{
                        let event = WlPointerAxisEvent {
                            source_id: id,
                            time: self.get_uint(),
                            axis: self.get_uint().into(),
                            value: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis(event, c);
                    },
                    5 =>{
                        let event = WlPointerFrameEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_frame(event, c);
                    },
                    6 =>{
                        let event = WlPointerAxisSourceEvent {
                            source_id: id,
                            axis_source: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis_source(event, c);
                    },
                    7 =>{
                        let event = WlPointerAxisStopEvent {
                            source_id: id,
                            time: self.get_uint(),
                            axis: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis_stop(event, c);
                    },
                    8 =>{
                        let event = WlPointerAxisDiscreteEvent {
                            source_id: id,
                            axis: self.get_uint().into(),
                            discrete: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis_discrete(event, c);
                    },
                    9 =>{
                        let event = WlPointerAxisValue120Event {
                            source_id: id,
                            axis: self.get_uint().into(),
                            value120: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis_value120(event, c);
                    },
                    10 =>{
                        let event = WlPointerAxisRelativeDirectionEvent {
                            source_id: id,
                            axis: self.get_uint().into(),
                            direction: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_pointer_axis_relative_direction(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlKeyboard => {
                match op {
                    0 =>{
                        let event = WlKeyboardKeymapEvent {
                            source_id: id,
                            format: self.get_uint().into(),
                            fd: self.get_fd(),
                            size: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_keymap(event, c);
                    },
                    1 =>{
                        let event = WlKeyboardEnterEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            surface: self.get_uint(),
                            keys: self.get_vec(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_enter(event, c);
                    },
                    2 =>{
                        let event = WlKeyboardLeaveEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            surface: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_leave(event, c);
                    },
                    3 =>{
                        let event = WlKeyboardKeyEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            time: self.get_uint(),
                            key: self.get_uint(),
                            state: self.get_uint().into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_key(event, c);
                    },
                    4 =>{
                        let event = WlKeyboardModifiersEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            mods_depressed: self.get_uint(),
                            mods_latched: self.get_uint(),
                            mods_locked: self.get_uint(),
                            group: self.get_uint(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_modifiers(event, c);
                    },
                    5 =>{
                        let event = WlKeyboardRepeatInfoEvent {
                            source_id: id,
                            rate: self.get_int(),
                            delay: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_keyboard_repeat_info(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlTouch => {
                match op {
                    0 =>{
                        let event = WlTouchDownEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            time: self.get_uint(),
                            surface: self.get_uint(),
                            id: self.get_int(),
                            x: Fixed::new(self.get_uint()),
                            y: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_down(event, c);
                    },
                    1 =>{
                        let event = WlTouchUpEvent {
                            source_id: id,
                            serial: self.get_uint(),
                            time: self.get_uint(),
                            id: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_up(event, c);
                    },
                    2 =>{
                        let event = WlTouchMotionEvent {
                            source_id: id,
                            time: self.get_uint(),
                            id: self.get_int(),
                            x: Fixed::new(self.get_uint()),
                            y: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_motion(event, c);
                    },
                    3 =>{
                        let event = WlTouchFrameEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_frame(event, c);
                    },
                    4 =>{
                        let event = WlTouchCancelEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_cancel(event, c);
                    },
                    5 =>{
                        let event = WlTouchShapeEvent {
                            source_id: id,
                            id: self.get_int(),
                            major: Fixed::new(self.get_uint()),
                            minor: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_shape(event, c);
                    },
                    6 =>{
                        let event = WlTouchOrientationEvent {
                            source_id: id,
                            id: self.get_int(),
                            orientation: Fixed::new(self.get_uint()),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_touch_orientation(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlOutput => {
                match op {
                    0 =>{
                        let event = WlOutputGeometryEvent {
                            source_id: id,
                            x: self.get_int(),
                            y: self.get_int(),
                            physical_width: self.get_int(),
                            physical_height: self.get_int(),
                            subpixel: (self.get_int() as u32).into(),
                            make: self.get_str(),
                            model: self.get_str(),
                            transform: (self.get_int() as u32).into(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_geometry(event, c);
                    },
                    1 =>{
                        let event = WlOutputModeEvent {
                            source_id: id,
                            flags: self.get_uint().into(),
                            width: self.get_int(),
                            height: self.get_int(),
                            refresh: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_mode(event, c);
                    },
                    2 =>{
                        let event = WlOutputDoneEvent {
                            source_id: id,
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_done(event, c);
                    },
                    3 =>{
                        let event = WlOutputScaleEvent {
                            source_id: id,
                            factor: self.get_int(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_scale(event, c);
                    },
                    4 =>{
                        let event = WlOutputNameEvent {
                            source_id: id,
                            name: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_name(event, c);
                    },
                    5 =>{
                        let event = WlOutputDescriptionEvent {
                            source_id: id,
                            description: self.get_str(),
                        };
                        println!("dispatch: event {:?}", event);
                        state.on_wl_output_description(event, c);
                    },
                    _ => (),
                }
            },
            Object::WlRegion => panic!("event from object with no events"),
            Object::WlSubcompositor => panic!("event from object with no events"),
            Object::WlSubsurface => panic!("event from object with no events"),
        }
    }
}
