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

/// Command errors
type CommandError = libloading::Error;

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
    pub unsafe fn new(lib_path:&str) -> Result<Self, CommandError> {
        let lib = match Library::new(lib_path) {
            Err(e) => return Err(e),
            Ok(lib) => lib
        };

        Ok( Self {
            library: lib
        })
    }

    /// Gets a pointer for vkGetInstanceProcAddr.
    pub fn instance(&self) -> Result<Symbol<PFN_vkGetInstanceProcAddr>, CommandError> {
        let sym = unsafe {
            match self.library.get::<PFN_vkGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0") {
                Ok(sym) => sym,
                Err(e) => return Err(e)
            }
        };

        Ok(sym)
    }

    /// Gets a pointer for vkGetDeviceProcAddr.
    pub fn device(&self) -> Result<Symbol<PFN_vkGetDeviceProcAddr>, CommandError> {
        let sym = unsafe {
            match self.library.get::<PFN_vkGetDeviceProcAddr>(b"vkGetDeviceProcAddr\0") {
                Ok(sym) => sym,
                Err(e) => return Err(e)
            }
        };

        Ok(sym)
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
