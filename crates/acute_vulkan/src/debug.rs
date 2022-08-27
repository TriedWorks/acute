use std::borrow::Cow;
use std::os::raw;
use std::ffi;
use ash::vk;
use acute_tracing::Level;
use crate::VkSettings;

pub struct VkDebug {
    loader: ash::extensions::ext::DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT,
}

pub struct VkDebugCallback(VkDebugFn);

pub type VkDebugFn = unsafe extern "system" fn(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut raw::c_void,
) -> vk::Bool32;

impl Default for VkDebugCallback {
    fn default() -> Self {
        Self(default_vulkan_debug_callback)
    }
}

impl VkDebug {
    pub fn init(
        entry: &ash::Entry,
        settings: &VkSettings,
        instance: &ash::Instance,
        callback: VkDebugCallback
    ) -> Result<Self, vk::Result> {
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            )
            .pfn_user_callback(Some(callback.0));

        let loader = ash::extensions::ext::DebugUtils::new(entry, instance);

        let messenger = unsafe {
            loader.create_debug_utils_messenger(&debug_info, None)?
        };

        return Ok(Self { loader: loader, messenger: messenger })
    }
}

unsafe extern "system" fn default_vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut ffi::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    if std::thread::panicking() {
        return vk::FALSE;
    }


    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            acute_tracing::event!(
                Level::DEBUG,
                "VULKAN: {:?}, {:?}, {}, {}", message_type,
                message_id_name,
                &message_id_number.to_string()
                ,message
            );
        },
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            acute_tracing::event!(
                Level::INFO,
                "VULKAN: {:?}, {:?}, {}, {}", message_type,
                message_id_name,
                &message_id_number.to_string()
                ,message
            );
        },
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            acute_tracing::event!(
                Level::WARN,
                "VULKAN: {:?}, {:?}, {}, {}", message_type,
                message_id_name,
                &message_id_number.to_string()
                ,message
            );
        },
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            acute_tracing::event!(
                Level::ERROR,
                "VULKAN: {:?}, {:?}, {}, {}", message_type,
                message_id_name,
                &message_id_number.to_string()
                ,message
            );
        },
        _ => {
            acute_tracing::event!(
                Level::WARN,
                "VULKAN: {:?}, {:?}, {}, {}", message_type,
                message_id_name,
                &message_id_number.to_string()
                ,message
            );
        },
    };


    vk::FALSE
}

impl Drop for VkDebug {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}