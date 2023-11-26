/// Wayland connection
pub mod connection;
pub mod types;
pub mod renderer;

// https://wayland.app/protocols/linux-dmabuf-unstable-v1
// TODO support WAYLAND_DEBUG env var
// TODO add server-side object storage
// TODO improve helper objects method signatures (enums)




// TODO zwp_linux_dmabuf handlers
// zwp_linux_dmabuf_v1_format_impl(void *data, struct zwp_linux_dmabuf_v1 *dma_buf, uint32_t drm_format) VWL_API_POST
// {
// }
//
// /* Handler for modifier event of the zwp_linux_dmabuf_v1 interface. */
// VWL_CAPI_CALL(void)
//     zwp_linux_dmabuf_v1_modifier_impl(void *data, struct zwp_linux_dmabuf_v1 *dma_buf, uint32_t drm_format,
//     uint32_t modifier_hi, uint32_t modifier_low) VWL_API_POST
// {
//     auto *drm_supported_formats = reinterpret_cast<formats_vector *>(data);
//
//     drm_format_pair format = {};
//     format.fourcc = drm_format;
//     format.modifier = (static_cast<uint64_t>(modifier_hi) << 32) | modifier_low;
//
//     if (!drm_supported_formats->is_out_of_memory)
//     {
//         drm_supported_formats->is_out_of_memory = !drm_supported_formats->formats->try_push_back(format);
//     }
// }


// VWL_CAPI_CALL(void)
// surface_registry_handler(void *data, struct wl_registry *wl_registry, uint32_t name, const char *interface,
//                          uint32_t version) VWL_API_POST
// {
//    auto wsi_surface = reinterpret_cast<wsi::wayland::surface *>(data);
//
//    if (!strcmp(interface, zwp_linux_dmabuf_v1_interface.name) && version >= ZWP_LINUX_DMABUF_V1_MODIFIER_SINCE_VERSION)
//    {
//       zwp_linux_dmabuf_v1 *dmabuf_interface_obj = reinterpret_cast<zwp_linux_dmabuf_v1 *>(wl_registry_bind(
//          wl_registry, name, &zwp_linux_dmabuf_v1_interface, ZWP_LINUX_DMABUF_V1_MODIFIER_SINCE_VERSION));
//
//       if (dmabuf_interface_obj == nullptr)
//       {
//          WSI_LOG_ERROR("Failed to get zwp_linux_dmabuf_v1 interface.");
//          return;
//       }
//
//       wsi_surface->dmabuf_interface.reset(dmabuf_interface_obj);
//    }
//    else if (!strcmp(interface, zwp_linux_explicit_synchronization_v1_interface.name))
//    {
//       zwp_linux_explicit_synchronization_v1 *explicit_sync_interface_obj =
//          reinterpret_cast<zwp_linux_explicit_synchronization_v1 *>(
//             wl_registry_bind(wl_registry, name, &zwp_linux_explicit_synchronization_v1_interface, 1));
//
//       if (explicit_sync_interface_obj == nullptr)
//       {
//          WSI_LOG_ERROR("Failed to get zwp_linux_explicit_synchronization_v1 interface.");
//          return;
//       }
//
//       wsi_surface->explicit_sync_interface.reset(explicit_sync_interface_obj);
//    }
// }

//--------------------------------------------------------------------------------------------------
// TODO dmabuf init process
// bool surface::init()
// {
//    surface_queue.reset(wl_display_create_queue(wayland_display));
//    if (surface_queue.get() == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to create wl surface queue.");
//       return false;
//    }
//
//    auto display_proxy = make_proxy_with_queue(wayland_display, surface_queue.get());
//    if (display_proxy == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to create wl display proxy.");
//       return false;
//    };
//
//    auto registry = wayland_owner<wl_registry>{ wl_display_get_registry(display_proxy.get()) };
//    if (registry == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to get wl display registry.");
//       return false;
//    }
//
//    const wl_registry_listener registry_listener = { surface_registry_handler };
//    int res = wl_registry_add_listener(registry.get(), &registry_listener, this);
//    if (res < 0)
//    {
//       WSI_LOG_ERROR("Failed to add registry listener.");
//       return false;
//    }
//
//    res = wl_display_roundtrip_queue(wayland_display, surface_queue.get());
//    if (res < 0)
//    {
//       WSI_LOG_ERROR("Roundtrip failed.");
//       return false;
//    }
//
//    if (dmabuf_interface.get() == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to obtain zwp_linux_dma_buf_v1 interface.");
//       return false;
//    }
//
//    if (explicit_sync_interface.get() == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to obtain zwp_linux_explicit_synchronization_v1 interface.");
//       return false;
//    }
//
//    auto surface_sync_obj =
//       zwp_linux_explicit_synchronization_v1_get_synchronization(explicit_sync_interface.get(), wayland_surface);
//    if (surface_sync_obj == nullptr)
//    {
//       WSI_LOG_ERROR("Failed to retrieve surface synchronization interface");
//       return false;
//    }
//
//    surface_sync_interface.reset(surface_sync_obj);
//
//    VkResult vk_res = get_supported_formats_and_modifiers(wayland_display, surface_queue.get(), dmabuf_interface.get(),
//                                                          supported_formats);
//    if (vk_res != VK_SUCCESS)
//    {
//       return false;
//    }
//
//    return true;
// }
