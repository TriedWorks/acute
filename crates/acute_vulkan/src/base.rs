use ash::{
    vk,
    extensions::{ext, khr},
};
use std::ffi;
use std::mem;
use std::os::raw;
use acute_tracing::{event, info, Level};
use acute_winit::WinitWindow;
use crate::debug::{VkDebug, VkDebugCallback};
use crate::gpu::GPU;
use crate::instance::VkInstance;
use crate::surface::VkSurface;
use crate::VkSettings;

pub struct VkBase {
    pub entry: ash::Entry,
    pub instance: mem::ManuallyDrop<VkInstance>,
    pub debug: mem::ManuallyDrop<Option<VkDebug>>,
    pub gpus: Vec<GPU>,
}

impl VkBase {
    pub fn init(
        settings: &VkSettings,
    ) -> Result<Self, vk::Result> {
        let entry = ash::Entry::linked();
        let instance = VkInstance::init(&entry,settings)?;

        let debug = if settings.debug {
            Some(VkDebug::init(&entry, settings, instance.handle(), VkDebugCallback::default())?)
        } else {
            None
        };

        let gpus = instance.gpus()?;
        for gpu in &gpus {
            info!("found: {}", gpu);
        }

        Ok(Self {
            entry,
            instance: mem::ManuallyDrop::new(instance),
            debug: mem::ManuallyDrop::new(debug),
            gpus
        })
    }

    pub fn create_surface_from_window(&self, window: &WinitWindow) -> Result<VkSurface, vk::Result> {
        VkSurface::init(window, &self.entry, &self.instance)
    }

    pub fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        device_info: &vk::DeviceCreateInfo
    ) -> Result<ash::Device, vk::Result> {
        unsafe { self.instance.handle().create_device(physical_device, device_info, None) }
    }

}

impl Drop for VkBase {
    fn drop(&mut self) {
        event!(Level::INFO, "Dropping: VkBase");
        unsafe {
            mem::ManuallyDrop::drop(&mut self.debug);
            mem::ManuallyDrop::drop(&mut self.instance);
        }
    }
}