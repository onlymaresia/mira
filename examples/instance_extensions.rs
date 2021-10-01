use mira::loader;
use mira::mem::{zeroed_vec, from_cstring};
use mira::vulkan::*;
use const_cstr::*;

fn main() {
    let eip:PFN_vkEnumerateInstanceExtensionProperties = loader::instance(
        std::ptr::null_mut(), const_cstr!("vkEnumerateInstanceExtensionProperties")
    );

    let mut count = 0 as u32;

    unsafe { (eip)(std::ptr::null_mut(), &mut count, std::ptr::null_mut()); }
    let mut extensions = unsafe { zeroed_vec(count as usize) };

    unsafe { (eip)(std::ptr::null_mut(), &mut count, extensions.as_mut_ptr()); }

    println!("Instance extensions");
    for extension in extensions.iter().enumerate() {
        let str = match unsafe { from_cstring(extension.1.extensionName.as_ptr()) } {
            Ok(str) => str,
            Err(_) => continue,
        };

        println!("extension #{} - {}", extension.0, str);
    }
}
