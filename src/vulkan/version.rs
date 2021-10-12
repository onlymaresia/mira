use std::fmt::{Display, Formatter};

/// Vulkan version (variant, major, minor, patch)
/// Variant indicates the variant of the Vulkan API supported by the implementation.
/// Variant is always 0 for the Vulkan API.
///
/// From <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#VK_MAKE_API_VERSION>
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major:u8,
    pub minor:u8,
    pub patch:u8,
    pub variant:u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, self.variant)
    }
}

/// Constructs a Vulkan API version number
#[allow(non_snake_case)]
pub const fn VK_MAKE_API_VERSION(variant: u8, major:u8, minor:u8, patch:u8) -> u32 {
    ((variant as u32) << 29) | ((major as u32) << 22) | ((minor as u32) << 12) | (patch as u32)
}

/// Converts a Vulkan version number to a Version
pub const fn from_vulkan_version(version: u32) -> Version {
    Version {
        variant: (version >> 29) as u8,
        patch: (version & 0xfff) as u8,
        minor: ((version >> 12) & 0x3ff) as u8,
        major: ((version >> 22) & 0x7f) as u8,
    }
}

