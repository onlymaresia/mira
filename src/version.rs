use std::fmt::{Display, Formatter};

/// Vulkan version (major, minor, patch, variant)
///
/// Variant indicates the variant of the Vulkan API supported by the implementation.
/// Variant is always 0 for the Vulkan API.
///
/// Default version is 1.0.0.0.
///
/// From <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#VK_MAKE_API_VERSION>
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub variant: u16,
}

impl Version {
    /// Constructs a Version using variant, major, minor, patch
    #[deprecated]
    pub fn make(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            variant: 0,
            major,
            minor,
            patch,
        }
    }

    /// Converts a Vulkan version number to a Version
    pub const fn from_vulkan_version(version: u32) -> Self {
        Self {
            variant: VK_API_VERSION_VARIANT(version) as u16,
            patch: VK_API_VERSION_PATCH(version) as u16,
            minor: VK_API_VERSION_MINOR(version) as u16,
            major: VK_API_VERSION_MAJOR(version) as u16,
        }
    }

    /// Converts to a Vulkan version
    pub const fn as_vulkan_version(&self) -> u32 {
        VK_MAKE_API_VERSION(
            self.variant as u32,
            self.major as u32,
            self.minor as u32,
            self.patch as u32,
        )
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
            variant: 0,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.variant
        )
    }
}

/// Constructs a Vulkan API version number
#[allow(non_snake_case)]
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    ((variant as u32) << 29) | ((major as u32) << 22) | ((minor as u32) << 12) | (patch as u32)
}

#[allow(non_snake_case)]
pub const fn VK_API_VERSION_MAJOR(version: u32) -> u32 {
    (version >> 22u32) & 0x7Fu32
}

#[allow(non_snake_case)]
pub const fn VK_API_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12u32) & 0x3FFu32
}

#[allow(non_snake_case)]
pub const fn VK_API_VERSION_PATCH(version: u32) -> u32 {
    version & 0xFFFu32
}

pub const fn VK_API_VERSION_VARIANT(version: u32) -> u32 {
    version >> 29u32
}

// (major, minor, patch, variant)
pub const fn VK_API_VERSION_TUPLE(version: u32) -> (u32, u32, u32, u32) {
    (
        VK_API_VERSION_MAJOR(version),
        VK_API_VERSION_MINOR(version),
        VK_API_VERSION_PATCH(version),
        VK_API_VERSION_VARIANT(version),
    )
}

pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0);
pub const VK_API_VERSION_1_1: u32 = VK_MAKE_API_VERSION(0, 1, 1, 0);
pub const VK_API_VERSION_1_2: u32 = VK_MAKE_API_VERSION(0, 1, 2, 0);
pub const VK_API_VERSION_1_3: u32 = VK_MAKE_API_VERSION(0, 1, 3, 0);
