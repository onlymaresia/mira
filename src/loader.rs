use crate::error::MiraError;
use crate::error::MiraError::{BackendError, CommandLoadError};
use crate::vulkan::*;
use libloading::*;
use once_cell::sync::Lazy;
use std::ffi::CStr;

#[cfg(target_os = "linux")]
pub const LIB_NAME: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
pub const LIB_NAME: &str = "libvulkan.so";

#[cfg(target_os = "macos")]
pub const LIB_NAME: &str = "libvulkan.1.dylib";

#[cfg(target_os = "ios")]
pub const LIB_NAME: &str = "libvulkan.1.dylib";

#[cfg(target_os = "windows")]
pub const LIB_NAME: &str = "vulkan-1.dll";

/// Command errors
//type CommandError = libloading::Error;

/// Instance and device command loader.
pub struct Command {
    library: Library,
}

impl Command {
    /// Loads a library with vkGetInstanceProcAddr and vkGetDeviceProcAddr.
    ///
    /// # Safety
    /// When the library is loaded and unloaded unknown functions are executed.
    ///
    /// From [libloading::Library::new].
    ///
    pub unsafe fn new(lib_path: &str) -> Result<Self, MiraError> {
        let lib = match Library::new(lib_path) {
            Err(e) => return Err(BackendError(e)),
            Ok(lib) => lib,
        };

        Ok(Self { library: lib })
    }

    /// Gets a pointer for vkGetInstanceProcAddr.
    pub fn instance(&self) -> Result<Symbol<PFN_vkGetInstanceProcAddr>, MiraError> {
        let sym = unsafe {
            match self
                .library
                .get::<PFN_vkGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0")
            {
                Ok(sym) => sym,
                Err(e) => return Err(BackendError(e)),
            }
        };

        Ok(sym)
    }

    /// Gets a pointer for vkGetDeviceProcAddr.
    pub fn device(&self) -> Result<Symbol<PFN_vkGetDeviceProcAddr>, MiraError> {
        let sym = unsafe {
            match self
                .library
                .get::<PFN_vkGetDeviceProcAddr>(b"vkGetDeviceProcAddr\0")
            {
                Ok(sym) => sym,
                Err(e) => return Err(BackendError(e)),
            }
        };

        Ok(sym)
    }
}

/// Internal command loader.
static INTERNAL_LOADER: Lazy<Command> = Lazy::new(|| unsafe { Command::new(LIB_NAME).unwrap() });

/// Gets from `instance` an instance command pointer for `command`.
///
/// # Observation
/// `instance` can be a null pointer.
///
/// # Safety
/// If function pointer and T have different size a undefined behavior can occur.
/// Command must be a null terminated string.
///
pub unsafe fn instance<T: Sized>(instance: VkInstance, command: &'static CStr) -> Result<T, MiraError> {
    static I: Lazy<Symbol<'static, PFN_vkGetInstanceProcAddr>> =
        Lazy::new(|| INTERNAL_LOADER.instance().unwrap());

    let pfn = I(instance, command.as_ptr()) as *const PFN_vkVoidFunction;
    if pfn == std::ptr::null() {
        return Err(CommandLoadError {
            command: command.to_str().unwrap(),
        });
    }

    Ok(std::mem::transmute_copy(&pfn))
}

pub unsafe fn instance_proc_addr<T: Sized>() -> Result<T, MiraError> {
    static I: Lazy<Symbol<'static, PFN_vkGetInstanceProcAddr>> =
        Lazy::new(|| INTERNAL_LOADER.instance().unwrap());

    let get_proc = I.clone().into_raw().into_raw();

    Ok(std::mem::transmute_copy(&get_proc))
}

/// Gets from `device` a device command pointer for `command`
///
/// # Safety
/// If function pointer and T have different size a undefined behavior can occur.
/// Command must be a null terminated string.
///
pub unsafe fn device<T: Sized>(device: VkDevice, command: &'static CStr) -> Result<T, MiraError> {
    static D: Lazy<Symbol<'static, PFN_vkGetDeviceProcAddr>> =
        Lazy::new(|| INTERNAL_LOADER.device().unwrap());

    let pfn = D(device, command.as_ptr()) as *const PFN_vkVoidFunction;
    if pfn == std::ptr::null() {
        return Err(CommandLoadError {
            command: command.to_str().unwrap(),
        });
    }

    Ok(std::mem::transmute_copy(&pfn))
}

pub unsafe fn device_proc_addr<T: Sized>() -> Result<T, MiraError> {
    static D: Lazy<Symbol<'static, PFN_vkGetDeviceProcAddr>> =
        Lazy::new(|| INTERNAL_LOADER.device().unwrap());

    let get_proc = D.clone().into_raw().into_raw();

    Ok(std::mem::transmute_copy(&get_proc))
}
