use ash::extensions::khr;
use ash::{vk};
use acute_winit::WinitWindow;
use crate::instance::VkInstance;

pub struct VkSurface {
    handle: vk::SurfaceKHR,
    loader: khr::Surface,
}

impl VkSurface {
    pub fn init(
        window: &WinitWindow,
        entry: &ash::Entry,
        instance: &VkInstance,
    ) -> Result<Self, vk::Result> {
        let handle = unsafe { ash_window::create_surface(entry, instance.handle(), window, None)? };
        let loader = khr::Surface::new(entry, instance.handle());
        Ok(Self {
            handle,
            loader
        })
    }

    #[inline]
    pub fn handle(&self) -> &vk::SurfaceKHR {
        &self.handle
    }

    #[inline]
    pub fn loader(&self) -> &khr::Surface {
        &self.loader
    }

    pub fn surface_formats(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::SurfaceFormatKHR>, vk::Result> {
        unsafe {
            self.loader.get_physical_device_surface_formats(physical_device, self.handle)
        }
    }

    pub fn surface_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<vk::SurfaceCapabilitiesKHR, vk::Result> {
        unsafe {
            self.loader.get_physical_device_surface_capabilities(physical_device, self.handle)
        }
    }

    pub fn present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::PresentModeKHR>, vk::Result> {
        unsafe {
            self.loader.get_physical_device_surface_present_modes(physical_device, self.handle)
        }
    }
}

impl Drop for VkSurface {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_surface(self.handle, None)
        }
    }
}