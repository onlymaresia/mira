use mira::error::MiraError;
use mira::loader;
use mira::mem::{from_cstring, zeroed_vec};
use mira::vulkan::*;

fn main() -> Result<(), MiraError> {
    let enumerate_instance_extensions: PFN_vkEnumerateInstanceExtensionProperties;
    enumerate_instance_extensions = unsafe {
        loader::instance(
            std::ptr::null_mut(),
            c"vkEnumerateInstanceExtensionProperties",
        )?
    };

    let mut count: u32 = 0;
    unsafe {
        enumerate_instance_extensions(std::ptr::null_mut(), &mut count, std::ptr::null_mut())
    };
    let mut extensions = unsafe { zeroed_vec::<VkExtensionProperties>(count as usize) };

    unsafe {
        enumerate_instance_extensions(std::ptr::null_mut(), &mut count, extensions.as_mut_ptr())
    };

    println!("Instance extensions");
    for extension in extensions.iter().enumerate() {
        let str = match unsafe { from_cstring(extension.1.extensionName.as_ptr()) } {
            Ok(str) => str,
            Err(_) => continue,
        };

        println!("extension #{} - {}", extension.0, str);
    }

    Ok(())
}
