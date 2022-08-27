use crate::driver::command::{CommandEncoder, CommandEncoderInfo};
use crate::driver::instance::VkInstance;
use crate::driver::queue::{QueueCreateInfo, QueueFamily};
use crate::driver::types::{Extensions, Features};
use crate::driver::{Adapter, Queue};
use acute_tracing::info;
use acute_utils::HashMap;
use ash::vk::Handle;
use ash::{
    extensions::{ext, khr},
    vk,
};
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc};
use parking_lot::Mutex;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::os::raw;
use std::sync::Arc;
use std::{error, ffi};

pub(crate) struct VkDevice {
    pub(crate) handle: ash::Device,
    pub(crate) adapter: Arc<Adapter>,
    pub(crate) instance: Arc<VkInstance>,
}

impl Debug for VkDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VkDevice {{ handle: {:?} }}", self.handle.handle())
    }
}

pub struct Device {
    pub(crate) handle: Arc<VkDevice>,
    pub(crate) allocator: Mutex<Allocator>,
    pub(crate) command_encoders: Mutex<HashMap<u64, CommandEncoder>>,
}

impl Device {
    pub fn raw(&self) -> &ash::Device {
        &self.handle.handle
    }
}

impl Adapter {
    pub fn request_device(
        &self,
        info: DeviceCreateInfo,
    ) -> Result<(Device, impl Iterator<Item = Queue>), DeviceError> {
        let DeviceCreateInfo {
            mut extensions,
            features,
            queue_families,
        } = info;

        let instance = self.instance.clone();
        if instance.render {
            extensions.vk_khr_swapchain = true;
            extensions.vk_khr_dynamic_rendering = true;
        }

        extensions.vk_khr_timeline_semaphore = true;

        let extension_names = Vec::<ffi::CString>::from(extensions);

        let layer_names = Vec::<ffi::CString>::from(instance.layers());

        println!("extension_names: {:?}", extension_names);
        let extension_pointers = extension_names
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        let layer_pointers = layer_names
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<*const raw::c_char>>();

        let mut queues_to_get = Vec::with_capacity(queue_families.len());

        let queue_infos = queue_families
            .iter()
            .map(|family| {
                queues_to_get.extend((0..family.priorities.len() as u32).map(move |id| {
                    QueueToGet {
                        family: family.family.family_id,
                        id,
                    }
                }));
                vk::DeviceQueueCreateInfo {
                    flags: vk::DeviceQueueCreateFlags::empty(),
                    queue_family_index: family.family.family_id,
                    queue_count: family.priorities.len() as u32,
                    p_queue_priorities: family.priorities.as_ptr(),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let features = vk::PhysicalDeviceFeatures::from(features);

        let mut device_info = vk::DeviceCreateInfo::builder()
            .flags(vk::DeviceCreateFlags::empty())
            .enabled_layer_names(&layer_pointers)
            .enabled_extension_names(&extension_pointers)
            .enabled_features(&features)
            .queue_create_infos(&queue_infos);

        let mut vulkan12features = vk::PhysicalDeviceVulkan12Features::builder()
            .timeline_semaphore(true)
            .build();

        let mut vulkan_dynamic_rendering = vk::PhysicalDeviceDynamicRenderingFeatures::builder()
            .dynamic_rendering(true)
            .build();

        device_info.p_next = &vulkan_dynamic_rendering as *const _ as *const ffi::c_void;
        vulkan_dynamic_rendering.p_next = &mut vulkan12features as *mut _ as *mut ffi::c_void;

        let vk_handle_device = unsafe {
            instance
                .handle
                .create_device(self.handle, &device_info, None)
                .map_err(|err| DeviceError::Other(err))?
        };

        let vk_device = Arc::new(VkDevice {
            handle: vk_handle_device,
            adapter: Arc::new(self.clone()),
            instance: instance.clone(),
        });

        let queue_iter = {
            let device = vk_device.clone();
            queues_to_get
                .into_iter()
                .map(move |QueueToGet { family, id }| {
                    let vk_queue = unsafe { device.handle.get_device_queue(family, id) };
                    let queue_id = vk_queue.as_raw();
                    let queue = Queue {
                        handle: Mutex::new(vk_queue),
                        device: device.clone(),
                        id: queue_id,
                        family,
                        id_in_family: id,
                    };
                    queue
                })
        };

        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.handle.clone(),
            device: vk_device.handle.clone(),
            physical_device: self.handle.clone(),
            debug_settings: Default::default(),
            buffer_device_address: false,
        })
        .unwrap();

        let device = Device {
            handle: vk_device.clone(),
            allocator: Mutex::new(allocator),
            command_encoders: Mutex::new(HashMap::new()),
        };

        Ok((device, queue_iter))
    }
}

struct QueueToGet {
    family: u32,
    id: u32,
}

#[derive(Debug, Clone)]
pub struct DeviceCreateInfo<'q> {
    pub extensions: Extensions,
    pub features: Features,
    pub queue_families: Vec<QueueCreateInfo<'q>>,
}

impl<'q> Default for DeviceCreateInfo<'q> {
    fn default() -> Self {
        Self {
            extensions: Extensions::none(),
            features: Features::none(),
            queue_families: vec![],
        }
    }
}

#[derive(Debug)]
pub enum DeviceError {
    Other(vk::Result),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo!")?;
        Ok(())
    }
}

impl error::Error for DeviceError {}

impl Drop for VkDevice {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
        info!("Destroyed: Device");
    }
}
