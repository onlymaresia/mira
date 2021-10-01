use mira::vulkan::*;
use mira::loader;
use const_cstr::*;
use mira::mem::{zeroed_vec, from_cstring};

fn main() {
    let layers = vec![
        const_cstr!("VK_LAYER_KHRONOS_validation").as_ptr()
    ];

    let mut instance_info:VkInstanceCreateInfo = unsafe { std::mem::zeroed() };
    instance_info.sType = VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    instance_info.ppEnabledLayerNames = layers.as_ptr();
    instance_info.enabledLayerCount = layers.len() as u32;

    let mut instance:VkInstance = unsafe { std::mem::zeroed() };

    let ci:PFN_vkCreateInstance;
    ci = loader::instance(std::ptr::null_mut(), const_cstr!("vkCreateInstance"));

    unsafe { ci(&instance_info, std::ptr::null_mut(), &mut instance) };

    let mut counter:u32 = 0;
    let epd:PFN_vkEnumeratePhysicalDevices;
    epd = loader::instance(instance, const_cstr!("vkEnumeratePhysicalDevices"));

    unsafe { epd(instance, &mut counter, std::ptr::null_mut()) };

    if counter == 0 {
        println!("this host has 0 vulkan adapters");
        return;
    }

    let mut devices = unsafe { zeroed_vec::<VkPhysicalDevice>(counter as usize) };

    unsafe { epd(instance, &mut counter, devices.as_mut_ptr()) };

    let ede:PFN_vkEnumerateDeviceExtensionProperties;
    ede = loader::instance(instance, const_cstr!("vkEnumerateDeviceExtensionProperties"));

    for device in devices.into_iter().enumerate() {
        println!("Device {}", device.0);
        println!("Extensions");

        unsafe { ede(device.1, std::ptr::null(), &mut counter, std::ptr::null_mut()) };
        if counter == 0 {
            continue;
        }

        let mut extensions;
        extensions = unsafe { zeroed_vec::<VkExtensionProperties>(counter as usize) };
        unsafe { ede(device.1, std::ptr::null(), &mut counter, extensions.as_mut_ptr()) };

        for extension in extensions.into_iter().enumerate() {
            let str = match unsafe { from_cstring(extension.1.extensionName.as_ptr()) } {
                Ok(str) => str,
                Err(_) => continue,
            };

            println!("#{} - {}", extension.0, str);
        }

        println!("\n");
    }
}