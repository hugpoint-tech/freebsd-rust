use crate::types::Request;
use crate::types::Object;
use crate::types::MessageHeader;
use crate::connection::WaylandConnection;
use crate::types::Request::*;

impl WaylandConnection {

pub fn enqueue(&self, req: Request) -> u32 {
        let data =  unsafe { &mut *self.data.get() };
        // Used as a return value, not all requests generate a new_id,
        // according to the spec 0 ID is reserved to represent a null or non-existent object.
        let mut new_id: u32 = 0;

        match req {
            // zwp_linux_dmabuf_v1:destroy
            ZwpLinuxDmabufV1Destroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_dmabuf_v1:create_params
            ZwpLinuxDmabufV1CreateParams{sendto} => {
                new_id = data.allocate_id(Object::ZwpLinuxBufferParamsV1);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_dmabuf_v1:get_default_feedback
            ZwpLinuxDmabufV1GetDefaultFeedback{sendto} => {
                new_id = data.allocate_id(Object::ZwpLinuxDmabufFeedbackV1);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_dmabuf_v1:get_surface_feedback
            ZwpLinuxDmabufV1GetSurfaceFeedback{sendto,surface} => {
                new_id = data.allocate_id(Object::ZwpLinuxDmabufFeedbackV1);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(surface);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_buffer_params_v1:destroy
            ZwpLinuxBufferParamsV1Destroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_buffer_params_v1:add
            ZwpLinuxBufferParamsV1Add{sendto,fd,plane_idx,offset,stride,modifier_hi,modifier_lo} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_fd(fd);
                data.write_uint(plane_idx);
                data.write_uint(offset);
                data.write_uint(stride);
                data.write_uint(modifier_hi);
                data.write_uint(modifier_lo);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_buffer_params_v1:create
            ZwpLinuxBufferParamsV1Create{sendto,width,height,format,flags} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(width);
                data.write_int(height);
                data.write_uint(format);
                data.write_uint(flags);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_buffer_params_v1:create_immed
            ZwpLinuxBufferParamsV1CreateImmed{sendto,width,height,format,flags} => {
                new_id = data.allocate_id(Object::WlBuffer);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_int(width);
                data.write_int(height);
                data.write_uint(format);
                data.write_uint(flags);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // zwp_linux_dmabuf_feedback_v1:destroy
            ZwpLinuxDmabufFeedbackV1Destroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_wm_base:destroy
            XdgWmBaseDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_wm_base:create_positioner
            XdgWmBaseCreatePositioner{sendto} => {
                new_id = data.allocate_id(Object::XdgPositioner);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_wm_base:get_xdg_surface
            XdgWmBaseGetXdgSurface{sendto,surface} => {
                new_id = data.allocate_id(Object::XdgSurface);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(surface);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_wm_base:pong
            XdgWmBasePong{sendto,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:destroy
            XdgPositionerDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_size
            XdgPositionerSetSize{sendto,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_anchor_rect
            XdgPositionerSetAnchorRect{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_anchor
            XdgPositionerSetAnchor{sendto,anchor} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(anchor);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_gravity
            XdgPositionerSetGravity{sendto,gravity} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(gravity);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_constraint_adjustment
            XdgPositionerSetConstraintAdjustment{sendto,constraint_adjustment} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(constraint_adjustment);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 5u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_offset
            XdgPositionerSetOffset{sendto,x,y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 6u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_reactive
            XdgPositionerSetReactive{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 7u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_parent_size
            XdgPositionerSetParentSize{sendto,parent_width,parent_height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(parent_width);
                data.write_int(parent_height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 8u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_positioner:set_parent_configure
            XdgPositionerSetParentConfigure{sendto,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 9u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_surface:destroy
            XdgSurfaceDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_surface:get_toplevel
            XdgSurfaceGetToplevel{sendto} => {
                new_id = data.allocate_id(Object::XdgToplevel);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_surface:get_popup
            XdgSurfaceGetPopup{sendto,parent,positioner} => {
                new_id = data.allocate_id(Object::XdgPopup);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(parent);
                data.write_uint(positioner);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_surface:set_window_geometry
            XdgSurfaceSetWindowGeometry{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_surface:ack_configure
            XdgSurfaceAckConfigure{sendto,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:destroy
            XdgToplevelDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_parent
            XdgToplevelSetParent{sendto,parent} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(parent);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_title
            XdgToplevelSetTitle{sendto,title} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(title);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_app_id
            XdgToplevelSetAppId{sendto,app_id} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(app_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:show_window_menu
            XdgToplevelShowWindowMenu{sendto,seat,serial,x,y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                data.write_int(x);
                data.write_int(y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:move
            XdgToplevelMove{sendto,seat,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 5u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:resize
            XdgToplevelResize{sendto,seat,serial,edges} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                data.write_uint(edges);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 6u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_max_size
            XdgToplevelSetMaxSize{sendto,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 7u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_min_size
            XdgToplevelSetMinSize{sendto,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 8u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_maximized
            XdgToplevelSetMaximized{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 9u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:unset_maximized
            XdgToplevelUnsetMaximized{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 10u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_fullscreen
            XdgToplevelSetFullscreen{sendto,output} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(output);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 11u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:unset_fullscreen
            XdgToplevelUnsetFullscreen{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 12u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_toplevel:set_minimized
            XdgToplevelSetMinimized{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 13u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_popup:destroy
            XdgPopupDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_popup:grab
            XdgPopupGrab{sendto,seat,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // xdg_popup:reposition
            XdgPopupReposition{sendto,positioner,token} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(positioner);
                data.write_uint(token);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_display:sync
            WlDisplaySync{sendto} => {
                new_id = data.allocate_id(Object::WlDisplaySyncCallback);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_display:get_registry
            WlDisplayGetRegistry{sendto} => {
                new_id = data.allocate_id(Object::WlRegistry);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_registry:bind
            WlRegistryBind{sendto,name,if_name,if_version} => {
                let kind = Object::from_str(&if_name).unwrap();
                new_id = data.allocate_id(kind);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(name);
                data.write_string(if_name);
                data.write_uint(if_version);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_compositor:create_surface
            WlCompositorCreateSurface{sendto} => {
                new_id = data.allocate_id(Object::WlSurface);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_compositor:create_region
            WlCompositorCreateRegion{sendto} => {
                new_id = data.allocate_id(Object::WlRegion);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shm_pool:create_buffer
            WlShmPoolCreateBuffer{sendto,offset,width,height,stride,format} => {
                new_id = data.allocate_id(Object::WlBuffer);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_int(offset);
                data.write_int(width);
                data.write_int(height);
                data.write_int(stride);
                data.write_uint(format);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shm_pool:destroy
            WlShmPoolDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shm_pool:resize
            WlShmPoolResize{sendto,size} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(size);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shm:create_pool
            WlShmCreatePool{sendto,fd,size} => {
                new_id = data.allocate_id(Object::WlShmPool);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_fd(fd);
                data.write_int(size);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_buffer:destroy
            WlBufferDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_offer:accept
            WlDataOfferAccept{sendto,serial,mime_type} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                data.write_string(mime_type);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_offer:receive
            WlDataOfferReceive{sendto,mime_type,fd} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(mime_type);
                data.write_fd(fd);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_offer:destroy
            WlDataOfferDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_offer:finish
            WlDataOfferFinish{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_offer:set_actions
            WlDataOfferSetActions{sendto,dnd_actions,preferred_action} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(dnd_actions);
                data.write_uint(preferred_action);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_source:offer
            WlDataSourceOffer{sendto,mime_type} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(mime_type);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_source:destroy
            WlDataSourceDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_source:set_actions
            WlDataSourceSetActions{sendto,dnd_actions} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(dnd_actions);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_device:start_drag
            WlDataDeviceStartDrag{sendto,source,origin,icon,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(source);
                data.write_uint(origin);
                data.write_uint(icon);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_device:set_selection
            WlDataDeviceSetSelection{sendto,source,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(source);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_device:release
            WlDataDeviceRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_device_manager:create_data_source
            WlDataDeviceManagerCreateDataSource{sendto} => {
                new_id = data.allocate_id(Object::WlDataSource);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_data_device_manager:get_data_device
            WlDataDeviceManagerGetDataDevice{sendto,seat} => {
                new_id = data.allocate_id(Object::WlDataDevice);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(seat);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell:get_shell_surface
            WlShellGetShellSurface{sendto,surface} => {
                new_id = data.allocate_id(Object::WlShellSurface);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(surface);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:pong
            WlShellSurfacePong{sendto,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:move
            WlShellSurfaceMove{sendto,seat,serial} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:resize
            WlShellSurfaceResize{sendto,seat,serial,edges} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                data.write_uint(edges);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_toplevel
            WlShellSurfaceSetToplevel{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_transient
            WlShellSurfaceSetTransient{sendto,parent,x,y,flags} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(parent);
                data.write_int(x);
                data.write_int(y);
                data.write_uint(flags);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_fullscreen
            WlShellSurfaceSetFullscreen{sendto,method,framerate,output} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(method);
                data.write_uint(framerate);
                data.write_uint(output);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 5u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_popup
            WlShellSurfaceSetPopup{sendto,seat,serial,parent,x,y,flags} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(seat);
                data.write_uint(serial);
                data.write_uint(parent);
                data.write_int(x);
                data.write_int(y);
                data.write_uint(flags);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 6u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_maximized
            WlShellSurfaceSetMaximized{sendto,output} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(output);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 7u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_title
            WlShellSurfaceSetTitle{sendto,title} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(title);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 8u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_shell_surface:set_class
            WlShellSurfaceSetClass{sendto,class_} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_string(class_);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 9u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:destroy
            WlSurfaceDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:attach
            WlSurfaceAttach{sendto,buffer,x,y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(buffer);
                data.write_int(x);
                data.write_int(y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:damage
            WlSurfaceDamage{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:frame
            WlSurfaceFrame{sendto} => {
                new_id = data.allocate_id(Object::WlSurfaceFrameCallback);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:set_opaque_region
            WlSurfaceSetOpaqueRegion{sendto,region} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(region);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:set_input_region
            WlSurfaceSetInputRegion{sendto,region} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(region);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 5u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:commit
            WlSurfaceCommit{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 6u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:set_buffer_transform
            WlSurfaceSetBufferTransform{sendto,transform} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(transform);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 7u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:set_buffer_scale
            WlSurfaceSetBufferScale{sendto,scale} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(scale);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 8u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:damage_buffer
            WlSurfaceDamageBuffer{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 9u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_surface:offset
            WlSurfaceOffset{sendto,x,y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 10u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_seat:get_pointer
            WlSeatGetPointer{sendto} => {
                new_id = data.allocate_id(Object::WlPointer);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_seat:get_keyboard
            WlSeatGetKeyboard{sendto} => {
                new_id = data.allocate_id(Object::WlKeyboard);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_seat:get_touch
            WlSeatGetTouch{sendto} => {
                new_id = data.allocate_id(Object::WlTouch);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_seat:release
            WlSeatRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_pointer:set_cursor
            WlPointerSetCursor{sendto,serial,surface,hotspot_x,hotspot_y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(serial);
                data.write_uint(surface);
                data.write_int(hotspot_x);
                data.write_int(hotspot_y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_pointer:release
            WlPointerRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_keyboard:release
            WlKeyboardRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_touch:release
            WlTouchRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_output:release
            WlOutputRelease{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_region:destroy
            WlRegionDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_region:add
            WlRegionAdd{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_region:subtract
            WlRegionSubtract{sendto,x,y,width,height} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                data.write_int(width);
                data.write_int(height);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subcompositor:destroy
            WlSubcompositorDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subcompositor:get_subsurface
            WlSubcompositorGetSubsurface{sendto,surface,parent} => {
                new_id = data.allocate_id(Object::WlSubsurface);
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(new_id);
                data.write_uint(surface);
                data.write_uint(parent);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:destroy
            WlSubsurfaceDestroy{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 0u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:set_position
            WlSubsurfaceSetPosition{sendto,x,y} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_int(x);
                data.write_int(y);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 1u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:place_above
            WlSubsurfacePlaceAbove{sendto,sibling} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(sibling);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 2u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:place_below
            WlSubsurfacePlaceBelow{sendto,sibling} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                data.write_uint(sibling);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 3u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:set_sync
            WlSubsurfaceSetSync{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 4u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
            // wl_subsurface:set_desync
            WlSubsurfaceSetDesync{sendto} => {
                let hdr_pos = data.send_buf.len();
                data.send_buf.set_len(hdr_pos + 8);
                let hdr = MessageHeader {
                    obj_id: sendto,
                    opcode: 5u16,
                    len: (data.send_buf.len() - hdr_pos) as u16
                };
                data.write_header(hdr, hdr_pos);
            },
        }
        return new_id
    }

}
