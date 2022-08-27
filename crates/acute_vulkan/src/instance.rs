use crate::{VkSettings, utils::c_str_ptr};

use ash::{
    extensions::{ext, khr},
    vk,
};
use std::ffi;
use std::os::raw;
use acute_tracing::{event, Level};
use acute_winit::{WinitWindow};
use crate::gpu::GPU;

pub struct VkInstance {
    instance: ash::Instance,
}

impl VkInstance {
    pub fn init(
        entry: &ash::Entry,
        settings: &VkSettings,
    ) -> Result<Self, vk::Result> {
        let layer_names: Vec<ffi::CString> = settings.layers
            .iter()
            .map(|ln| ffi::CString::new(ln.as_str()).unwrap())
            .collect();

        let layers: Vec<*const raw::c_char> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let app_name = unsafe { ffi::CStr::from_ptr(c_str_ptr(&settings.app_name)) };
        let engine_name = unsafe { ffi::CStr::from_ptr(c_str_ptr(&settings.engine_name)) };

        let version = settings.app_version;
        let app_version = vk::make_api_version(version.0, version.1, version.2, version.3);

        let instance_extensions = entry.enumerate_instance_extension_properties(None)?;

        let mut extensions = vec![];

        if settings.render {
            extensions.push(khr::Surface::name());
            if cfg!(all(
                unix,
                not(target_os = "android"),
                not(target_os = "macos")
            )) {
                extensions.push(khr::XlibSurface::name());
                extensions.push(khr::XcbSurface::name());
                extensions.push(khr::WaylandSurface::name());
            }
            if cfg!(target_os = "android") {
                extensions.push(khr::AndroidSurface::name());
            }
            if cfg!(target_os = "windows") {
                extensions.push(khr::Win32Surface::name());
            }
            if cfg!(target_os = "macos") {
                extensions.push(ext::MetalSurface::name());
            }
        }

        if settings.debug {
            extensions.push(ext::DebugUtils::name())
        }

        if let Some(additional_extensions) = &settings.extensions {
            additional_extensions.iter()
                .map(|extension| unsafe { ffi::CStr::from_ptr(extension.as_ptr() as *const raw::c_char) })
                .for_each(|extension| extensions.push(extension))
        }

        extensions.retain(|&ext| {
            if instance_extensions
                .iter()
                .any(|inst_ext| unsafe { ffi::CStr::from_ptr(inst_ext.extension_name.as_ptr()) == ext })
            {
                true
            } else {
                println!("unable to find extension! {}", ext.to_string_lossy());
                false
            }
        });

        event!(Level::INFO, "Vulkan Extensions: {:?}", extensions);

        let extension_ptrs = extensions.iter().map(|ext| ext.as_ptr()).collect::<Vec<*const raw::c_char>>();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(app_version)
            .engine_name(engine_name)
            .engine_version(vk::make_api_version(0, 0, 1, 0))
            .api_version(settings.vulkan_api_version.into());

        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extension_ptrs);

        let instance = unsafe { entry.create_instance(&instance_info, None)? };

        Ok(Self {
            instance
        })
    }

    #[inline]
    pub fn handle(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn gpus(&self) -> Result<Vec<GPU>, vk::Result> {
        unsafe {
            let devices = self.instance.enumerate_physical_devices()?;
            let gpus = devices.iter()
                .map(|&device| {
                    let properties = self.physical_device_properties(device);
                    let queue_families = self.physical_device_queue_families(device);
                    GPU::new(device, properties, queue_families)
                }).collect::<Vec<GPU>>();
            Ok(gpus)
        }
    }

    pub fn physical_device_properties(&self, device: vk::PhysicalDevice) -> vk::PhysicalDeviceProperties {
        unsafe { self.instance.get_physical_device_properties(device) }
    }

    pub fn physical_device_queue_families(&self, device: vk::PhysicalDevice) -> Vec<vk::QueueFamilyProperties> {
        unsafe { self.instance.get_physical_device_queue_family_properties(device) }
    }

}

impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}