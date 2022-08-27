use std::os::raw;
use ash::vk;

pub fn c_str_ptr(val: &str) -> *const raw::c_char {
    val.as_ptr() as *const raw::c_char
}

pub fn vulkan_api_version_decode(version: u32) -> (u32, u32, u32, u32) {
    let variant = vk::api_version_variant(version);
    let major = vk::api_version_major(version);
    let minor = vk::api_version_minor(version);
    let patch = vk::api_version_patch(version);
    (variant, major, minor, patch)
}