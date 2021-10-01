use libloading::*;
use const_cstr::*;
use once_cell::sync::Lazy;
use crate::vulkan::{PFN_vkGetInstanceProcAddr, PFN_vkGetDeviceProcAddr};
use crate::vulkan::{VkInstance, VkDevice};
use std::os::raw::c_char;

#[cfg(target_os = "linux")]
const LIB_NAME:&str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_NAME:&str = "libvulkan.so";

#[cfg(target_os = "macos")]
const LIB_NAME:&str = "libvulkan.1.dylib";

#[cfg(target_os = "ios")]
const LIB_NAME:&str = "libvulkan.1.dylib";

#[cfg(target_os = "windows")]
const LIB_NAME:&str = "vulkan-1.dll";

/// Instance and device command loader provider
pub struct Command {
    library: Library
}

impl Command {
    /// Loads a library that implements the vkGetInstanceProcAddr and vkGetDeviceProcAddr functions
    /// to create a new loader provider
    pub fn new(lib_path:&str) -> Option<Self> {
        unsafe {
            let vulkan = match Library::new(lib_path) {
                Err(_) => return None,
                Ok(lib) => lib
            };

            Some( Self {
                library: vulkan
            })
        }
    }

    /// Get a pointer to the instance command loader
    pub fn instance(&self) -> Option<Symbol<PFN_vkGetInstanceProcAddr>> {
        unsafe {
            let i = match self.library.get::<PFN_vkGetInstanceProcAddr>(
                b"vkGetInstanceProcAddr\0") {
                Ok(sym) => sym,
                Err(_) => return None
            };
            Some(i)
        }
    }

    /// Get a pointer to the device command loader
    pub fn device(&self) -> Option<Symbol<PFN_vkGetDeviceProcAddr>> {
        unsafe {
            let d = match self.library.get::<PFN_vkGetDeviceProcAddr>(
                b"vkGetDeviceProcAddr\0") {
                Ok(sym) => sym,
                Err(_) => return None
            };

            Some(d)
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::new(LIB_NAME).unwrap()
    }
}

static INTERNAL_LOADER:Lazy<Command> = Lazy::new(|| {
    Command::default()
});

/// Get an instance command pointer
///
/// # Arguments
///
/// * `instance` - A Vulkan instance or null
/// * `command` - A command name
///
/// # Examples
/// ```
/// use const_cstr::*;
/// use mira::loader;
/// use mira::vulkan::PFN_vkEnumerateInstanceExtensionProperties;
///
/// let iep:PFN_vkEnumerateInstanceExtensionProperties = loader::instance(
///     std::ptr::null_mut(), const_cstr!("vkEnumerateInstanceExtensionProperties")
/// );
/// ```
pub fn instance<T: Sized>(instance: VkInstance, command: ConstCStr) -> T {
    static I:Lazy<Symbol<'static, PFN_vkGetInstanceProcAddr>> = Lazy::new(|| {
        INTERNAL_LOADER.instance().unwrap()
    });

    unsafe {
        std::mem::transmute_copy(
            &I(instance, command.as_ptr() as *const c_char)
        )
    }
}

/// Get a device command pointer
///
/// # Arguments
///
/// * `device` - A Vulkan device
/// * `command` - A command name
///
///
pub fn device<T: Sized>(device: VkDevice, command: ConstCStr) -> T {
    static I:Lazy<Symbol<'static, PFN_vkGetDeviceProcAddr>> = Lazy::new(|| {
        INTERNAL_LOADER.device().unwrap()
    });

    unsafe {
        std::mem::transmute_copy(
            &I(device, command.as_ptr() as *const c_char)
        )
    }
}