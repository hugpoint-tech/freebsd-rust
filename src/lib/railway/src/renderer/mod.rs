// PLAN OF ACTIONS:
// 1. Create Vulkan Instance
// 1.  - enumerate instance extensions
// 1.  - check WaylandSurface extension is supported -> guaratees dmabuf_fd
// 2. Enumerate physical devices
// 2. Select Physical device by querying properies
//   - memory props
//   - queue props
// 3. Create logical device
// 4. Create queues
//  - command queue
//  - ? queue
// 5. Create command pool
//
// 6. Create Buffers from VkDeviceMemory and export them as fds
// X. Cleanly Shitdown

use ash::vk;
use ash::Entry;
use ash::Device;
use ash::Instance;

use std::ffi::{CString, CStr};
use ash::extensions::khr::Swapchain;
use ash::vk::{PhysicalDevice, QueueFamilyProperties};


pub struct Renderer {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
}

impl Renderer {
    pub fn new() -> Renderer {
        let entry = unsafe {Entry::load()}
            .expect("Failed to load vulkan library");

        let app_name =  CString::new("railway").unwrap();

        let appinfo = vk::ApplicationInfo::default()
            .application_name(app_name.as_c_str())
            .application_version(0)
            .engine_name(app_name.as_c_str())
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        // TODO enable validation layers
        // let layer_names = [CStr::from_bytes_with_nul_unchecked(
        //     b"VK_LAYER_KHRONOS_validation\0",
        // )];
        // let layers_names_raw: Vec<*const c_char> = layer_names
        //     .iter()
        //     .map(|raw_name| raw_name.as_ptr())
        //     .collect();

        // TODO check extensions: memfd, wayland surface


        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&appinfo)
            .enabled_layer_names(&[])
            .enabled_extension_names(&[])
            .flags(vk::InstanceCreateFlags::default());

        let instance: Instance = unsafe {entry.create_instance(&create_info, None)}
            .expect("Instance creation error");

        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            );

        let physical_devices = unsafe {instance.enumerate_physical_devices()}
            .expect("Physical device enumeration error");

        let (pdevice, queue_family_index) = Self::select_pdevice(&instance, physical_devices)
            .expect("No graphics devices were found");

        // let priorities = [1.0];
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index as u32);
            //.queue_priorities(&priorities);

        let device_extension_names_raw = [
            Swapchain::NAME.as_ptr(),
        ];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw);
            //.enabled_features(&features);

        let device: Device = unsafe {instance.create_device(pdevice, &device_create_info, None)}
            .expect("Logical device creation failed");

        return Self {
            entry,
            instance,
            device
        }


    }

    /// Find appropriate physical device by inspecting memory and queue properties.
    /// First good device is selected.
    fn select_pdevice(instance: &Instance, pdevices: Vec<PhysicalDevice>) -> Option<(PhysicalDevice, usize)> {
        for pdevice in pdevices.into_iter() {
            let qprops = unsafe {instance.get_physical_device_queue_family_properties(pdevice)};
            if let Some(idx) = Self::check_queue_props(qprops) {
                return (pdevice, idx).into();
            }
        }
        return None;
    }

    /// Check if device contains necessary queue family properties.
    /// Currently checks if device has a graphics queue
    fn check_queue_props(qprops: Vec<QueueFamilyProperties>) -> Option<usize> {
        for (index, prop) in qprops.into_iter().enumerate() {
            let supports_graphic_and_surface =
                prop.queue_flags.contains(vk::QueueFlags::GRAPHICS);
            // && surface_loader
            //     .get_physical_device_surface_support(
            //         *pdevice,
            //         index as u32,
            //         surface,
            //     )
            //     .unwrap();
            if supports_graphic_and_surface {
                return index.into()
            }
        }
        return None
    }



}

