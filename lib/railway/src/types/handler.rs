use crate::types::events::*;
use crate::connection::WaylandConnection;

#[allow(unused)]
pub trait EventHandler{
    fn on_xdg_wm_base_ping(&mut self, event: XdgWmBasePingEvent, connection: &WaylandConnection) {}

    fn on_xdg_surface_configure(&mut self, event: XdgSurfaceConfigureEvent, connection: &WaylandConnection) {}

    fn on_xdg_toplevel_configure(&mut self, event: XdgToplevelConfigureEvent, connection: &WaylandConnection) {}

    fn on_xdg_toplevel_close(&mut self, event: XdgToplevelCloseEvent, connection: &WaylandConnection) {}

    fn on_xdg_toplevel_configure_bounds(&mut self, event: XdgToplevelConfigureBoundsEvent, connection: &WaylandConnection) {}

    fn on_xdg_toplevel_wm_capabilities(&mut self, event: XdgToplevelWmCapabilitiesEvent, connection: &WaylandConnection) {}

    fn on_xdg_popup_configure(&mut self, event: XdgPopupConfigureEvent, connection: &WaylandConnection) {}

    fn on_xdg_popup_popup_done(&mut self, event: XdgPopupPopupDoneEvent, connection: &WaylandConnection) {}

    fn on_xdg_popup_repositioned(&mut self, event: XdgPopupRepositionedEvent, connection: &WaylandConnection) {}

    fn on_wl_display_sync_done(&mut self, event: WlDisplaySyncDoneEvent, connection: &WaylandConnection) {}

    fn on_wl_display_error(&mut self, event: WlDisplayErrorEvent, connection: &WaylandConnection) {}

    fn on_wl_display_delete_id(&mut self, event: WlDisplayDeleteIdEvent, connection: &WaylandConnection) {}

    fn on_wl_registry_global(&mut self, event: WlRegistryGlobalEvent, connection: &WaylandConnection) {}

    fn on_wl_registry_global_remove(&mut self, event: WlRegistryGlobalRemoveEvent, connection: &WaylandConnection) {}

    fn on_wl_shm_format(&mut self, event: WlShmFormatEvent, connection: &WaylandConnection) {}

    fn on_wl_buffer_release(&mut self, event: WlBufferReleaseEvent, connection: &WaylandConnection) {}

    fn on_wl_data_offer_offer(&mut self, event: WlDataOfferOfferEvent, connection: &WaylandConnection) {}

    fn on_wl_data_offer_source_actions(&mut self, event: WlDataOfferSourceActionsEvent, connection: &WaylandConnection) {}

    fn on_wl_data_offer_action(&mut self, event: WlDataOfferActionEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_target(&mut self, event: WlDataSourceTargetEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_send(&mut self, event: WlDataSourceSendEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_cancelled(&mut self, event: WlDataSourceCancelledEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_dnd_drop_performed(&mut self, event: WlDataSourceDndDropPerformedEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_dnd_finished(&mut self, event: WlDataSourceDndFinishedEvent, connection: &WaylandConnection) {}

    fn on_wl_data_source_action(&mut self, event: WlDataSourceActionEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_data_offer(&mut self, event: WlDataDeviceDataOfferEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_enter(&mut self, event: WlDataDeviceEnterEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_leave(&mut self, event: WlDataDeviceLeaveEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_motion(&mut self, event: WlDataDeviceMotionEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_drop(&mut self, event: WlDataDeviceDropEvent, connection: &WaylandConnection) {}

    fn on_wl_data_device_selection(&mut self, event: WlDataDeviceSelectionEvent, connection: &WaylandConnection) {}

    fn on_wl_shell_surface_ping(&mut self, event: WlShellSurfacePingEvent, connection: &WaylandConnection) {}

    fn on_wl_shell_surface_configure(&mut self, event: WlShellSurfaceConfigureEvent, connection: &WaylandConnection) {}

    fn on_wl_shell_surface_popup_done(&mut self, event: WlShellSurfacePopupDoneEvent, connection: &WaylandConnection) {}

    fn on_wl_surface_frame_done(&mut self, event: WlSurfaceFrameDoneEvent, connection: &WaylandConnection) {}

    fn on_wl_surface_enter(&mut self, event: WlSurfaceEnterEvent, connection: &WaylandConnection) {}

    fn on_wl_surface_leave(&mut self, event: WlSurfaceLeaveEvent, connection: &WaylandConnection) {}

    fn on_wl_surface_preferred_buffer_scale(&mut self, event: WlSurfacePreferredBufferScaleEvent, connection: &WaylandConnection) {}

    fn on_wl_surface_preferred_buffer_transform(&mut self, event: WlSurfacePreferredBufferTransformEvent, connection: &WaylandConnection) {}

    fn on_wl_seat_capabilities(&mut self, event: WlSeatCapabilitiesEvent, connection: &WaylandConnection) {}

    fn on_wl_seat_name(&mut self, event: WlSeatNameEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_enter(&mut self, event: WlPointerEnterEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_leave(&mut self, event: WlPointerLeaveEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_motion(&mut self, event: WlPointerMotionEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_button(&mut self, event: WlPointerButtonEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis(&mut self, event: WlPointerAxisEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_frame(&mut self, event: WlPointerFrameEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis_source(&mut self, event: WlPointerAxisSourceEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis_stop(&mut self, event: WlPointerAxisStopEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis_discrete(&mut self, event: WlPointerAxisDiscreteEvent, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis_value120(&mut self, event: WlPointerAxisValue120Event, connection: &WaylandConnection) {}

    fn on_wl_pointer_axis_relative_direction(&mut self, event: WlPointerAxisRelativeDirectionEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_keymap(&mut self, event: WlKeyboardKeymapEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_enter(&mut self, event: WlKeyboardEnterEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_leave(&mut self, event: WlKeyboardLeaveEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_key(&mut self, event: WlKeyboardKeyEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_modifiers(&mut self, event: WlKeyboardModifiersEvent, connection: &WaylandConnection) {}

    fn on_wl_keyboard_repeat_info(&mut self, event: WlKeyboardRepeatInfoEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_down(&mut self, event: WlTouchDownEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_up(&mut self, event: WlTouchUpEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_motion(&mut self, event: WlTouchMotionEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_frame(&mut self, event: WlTouchFrameEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_cancel(&mut self, event: WlTouchCancelEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_shape(&mut self, event: WlTouchShapeEvent, connection: &WaylandConnection) {}

    fn on_wl_touch_orientation(&mut self, event: WlTouchOrientationEvent, connection: &WaylandConnection) {}

    fn on_wl_output_geometry(&mut self, event: WlOutputGeometryEvent, connection: &WaylandConnection) {}

    fn on_wl_output_mode(&mut self, event: WlOutputModeEvent, connection: &WaylandConnection) {}

    fn on_wl_output_done(&mut self, event: WlOutputDoneEvent, connection: &WaylandConnection) {}

    fn on_wl_output_scale(&mut self, event: WlOutputScaleEvent, connection: &WaylandConnection) {}

    fn on_wl_output_name(&mut self, event: WlOutputNameEvent, connection: &WaylandConnection) {}

    fn on_wl_output_description(&mut self, event: WlOutputDescriptionEvent, connection: &WaylandConnection) {}

}
