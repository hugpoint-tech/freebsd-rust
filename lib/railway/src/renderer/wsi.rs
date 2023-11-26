// https://gitlab.freedesktop.org/mesa/vulkan-wsi-layer/-/blob/main/wsi/README.md#helpers


// TODO check smithay
struct drm_format_pair  {
    fourcc: u32,
    modifier: u64
}



struct Surface {}

impl Surface {


    // * @brief Allocates and initializes a surface
    // *
    // * @param allocator An allocator to use for host allocations needed for the surface.
    // * @param display   The Wayland display used to create the VkSurface
    // * @param surf      The Wayland surface used to create the VkSurface
    // *
    fn new () {}

    fn get_properties() {}

    fn allocate_swapchain(){}

    /** Returns the Wayland display */
    // wl_display *
    fn get_wl_display() {}


    /**
     * @brief Returns a pointer to the Wayland zwp_linux_dmabuf_v1 interface.
     *
     * The raw pointer is valid throughout the lifetime of this surface.
     * returns: zwp_linux_dmabuf_v1 *
     */
    fn get_dmabuf_interface() {}

    /**
     * @brief Returns a pointer to the Wayland zwp_linux_surface_synchronization_v1 interface obtained for the wayland
     *        surface.
     *
     * The raw pointer is valid for the lifetime of the surface.
     * returns: zwp_linux_surface_synchronization_v1*
     */
    fn get_surface_sync_interface() {}

    /**
     * @brief Returns a reference to a list of DRM formats supported by the Wayland surface.
     *
     * The reference is valid throughout the lifetime of this surface.
     * returns: &[drm_format_pair] return supported_formats;
     */
    fn  get_formats(){}

    /**
     * @brief Set the next frame callback.
     *
     * Make a frame request on the compositor which will be applied in the next
     * wl_surface::commit. It overwrites previous requested frame events.
     *
     * @return true for success, false otherwise.
     */
    fn set_frame_callback() -> bool {false}

    /**
     * @brief Wait for the compositor's last requested frame event.
     *
     * @return true for success, false otherwise.
     */
    fn wait_next_frame_event() -> bool {false}



    fn get_supported_formats_and_modifiers() {}
//     static VkResult get_supported_formats_and_modifiers(wl_display *display, wl_event_queue *queue,
//                                                     zwp_linux_dmabuf_v1 *dmabuf_interface,
//                                                     util::vector<drm_format_pair> &supported_formats)
// {
//    formats_vector drm_supported_formats;
//    drm_supported_formats.formats = &supported_formats;
//
//    const zwp_linux_dmabuf_v1_listener dma_buf_listener = {
//       .format = zwp_linux_dmabuf_v1_format_impl,
//       .modifier = zwp_linux_dmabuf_v1_modifier_impl,
//    };
//    int res = zwp_linux_dmabuf_v1_add_listener(dmabuf_interface, &dma_buf_listener, &drm_supported_formats);
//    if (res < 0)
//    {
//       WSI_LOG_ERROR("Failed to add zwp_linux_dmabuf_v1 listener.");
//       return VK_ERROR_UNKNOWN;
//    }
//
//    /* Get all modifier events. */
//    res = wl_display_roundtrip_queue(display, queue);
//    if (res < 0)
//    {
//       WSI_LOG_ERROR("Roundtrip failed.");
//       return VK_ERROR_UNKNOWN;
//    }
//
//    if (drm_supported_formats.is_out_of_memory)
//    {
//       WSI_LOG_ERROR("Host got out of memory.");
//       return VK_ERROR_OUT_OF_HOST_MEMORY;
//    }
//
//    return VK_SUCCESS;
// }

}


struct SurfaceProperties{}


struct Swapchain {}