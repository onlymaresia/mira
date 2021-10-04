use libloading::*;
use const_cstr::*;
use once_cell::sync::Lazy;
use crate::vulkan::*;
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

/// Instance and device command loader.
pub struct Command {
    library: Library
}

impl Command {
    /// Loads a library with vkGetInstanceProcAddr and vkGetDeviceProcAddr.
    ///
    /// # Safety
    /// When the library is loaded and unloaded unknown functions are executed.
    ///
    /// From [libloading::Library::new].
    ///
    pub unsafe fn new(lib_path:&str) -> Option<Self> {
        let vulkan = match Library::new(lib_path) {
            Err(_) => return None,
            Ok(lib) => lib
        };

        Some( Self {
            library: vulkan
        })
    }

    /// Gets a pointer for vkGetInstanceProcAddr.
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

    /// Gets a pointer for vkGetDeviceProcAddr.
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

/// Internal command loader.
static INTERNAL_LOADER:Lazy<Command> = Lazy::new(|| {
    unsafe { Command::new(LIB_NAME).unwrap() }
});

/// Gets from `instance` an instance command pointer for `command`.
///
/// # Observation
/// `instance` can be a null pointer.
///
/// # Safety
/// If function pointer and T have different size a undefined behavior can occur.
///
pub unsafe fn instance<T: Sized>(instance: VkInstance, command: ConstCStr) -> T {
    static I:Lazy<Symbol<'static, PFN_vkGetInstanceProcAddr>> = Lazy::new(|| {
        INTERNAL_LOADER.instance().unwrap()
    });

    std::mem::transmute_copy(&I(instance, command.as_ptr() as *const c_char))
}

/// Gets from `device` a device command pointer for `command`
///
/// # Safety
/// Using a incorrect type may cause undefined behavior.
///
pub unsafe fn device<T: Sized>(device: VkDevice, command: ConstCStr) -> T {
    static I:Lazy<Symbol<'static, PFN_vkGetDeviceProcAddr>> = Lazy::new(|| {
        INTERNAL_LOADER.device().unwrap()
    });

    std::mem::transmute_copy(&I(device, command.as_ptr() as *const c_char))
}
