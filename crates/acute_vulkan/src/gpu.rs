use std::{ffi, fmt};
use ash::vk;
use crate::instance::VkInstance;
use crate::utils;

pub struct GPUs {
    gpus: Vec<GPU>,
}

pub struct GPUInfo {

}


#[derive(Debug)]
pub struct GPU {
    handle: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    queue_families: Vec<vk::QueueFamilyProperties>,
}

impl GPU {
    pub fn new(
        handle: vk::PhysicalDevice,
        properties: vk::PhysicalDeviceProperties,
        queue_families: Vec<vk::QueueFamilyProperties>,
    ) -> Self {
        Self {
            handle,
            properties,
            queue_families
        }
    }

}

impl fmt::Display for GPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_raw = unsafe { ffi::CStr::from_ptr(self.properties.device_name.as_ptr()) };
        let api = utils::vulkan_api_version_decode(self.properties.api_version);
        let name = name_raw.to_str().unwrap();
        write!(f, "GPU: {{\n  name: {:?}\n  api: {:?}\n  queue_families:\n", name, api)?;
        for family in &self.queue_families {
            write!(f, "    {:?},\n", family)?;
        }
        Ok(())
    }
}