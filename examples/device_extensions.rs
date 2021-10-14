use mira::error::MiraError;
use mira::vulkan::*;
use mira::loader;
use const_cstr::*;
use mira::mem::{zeroed_vec, from_cstring};

fn main() -> Result<(), MiraError> {
    let layers = vec![
        const_cstr!("VK_LAYER_KHRONOS_validation").as_ptr()
    ];

    let mut instance_info:VkInstanceCreateInfo = unsafe { std::mem::zeroed() };
    instance_info.sType = VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    instance_info.ppEnabledLayerNames = layers.as_ptr();
    instance_info.enabledLayerCount = layers.len() as u32;

    let mut instance:VkInstance = unsafe { std::mem::zeroed() };

    let create_instance:PFN_vkCreateInstance;
    create_instance = unsafe {
        loader::instance(std::ptr::null_mut(), const_cstr!("vkCreateInstance"))?
    };

    unsafe { create_instance(&instance_info, std::ptr::null_mut(), &mut instance) };

    let enumerate_physical_devices:PFN_vkEnumeratePhysicalDevices;
    enumerate_physical_devices = unsafe {
        loader::instance(instance, const_cstr!("vkEnumeratePhysicalDevices"))?
    };

    let mut counter:u32 = 0;
    unsafe { enumerate_physical_devices(instance, &mut counter, std::ptr::null_mut()) };
    let mut devices = unsafe { zeroed_vec::<VkPhysicalDevice>(counter as usize) };

    unsafe { enumerate_physical_devices(instance, &mut counter, devices.as_mut_ptr()) };

    let enumerate_device_extensions:PFN_vkEnumerateDeviceExtensionProperties;
    enumerate_device_extensions = unsafe {
        loader::instance(instance, const_cstr!("vkEnumerateDeviceExtensionProperties"))?
    };

    for device in devices.into_iter().enumerate() {
        println!("Device {}", device.0);
        println!("Extensions");

        unsafe { enumerate_device_extensions(device.1, std::ptr::null(), &mut counter, std::ptr::null_mut()) };

        let mut extensions = unsafe { zeroed_vec::<VkExtensionProperties>(counter as usize) };
        unsafe { enumerate_device_extensions(device.1, std::ptr::null(), &mut counter, extensions.as_mut_ptr()) };

        for extension in extensions.into_iter().enumerate() {
            let str = match unsafe { from_cstring(extension.1.extensionName.as_ptr()) } {
                Ok(str) => str,
                Err(_) => continue,
            };

            println!("#{} - {}", extension.0, str);
        }

        println!("\n");
    }

    Ok(())
}
