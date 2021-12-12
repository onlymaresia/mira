# mira
[![crates.io](https://img.shields.io/crates/v/mira.svg)](https://crates.io/crates/mira)
[![docs.rs](https://docs.rs/mira/badge.svg)](https://docs.rs/mira)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

Pure and simple Vulkan bindings generated from Vulkan-Headers!

Mira provides a simple and straightforward way to interact with Vulkan.
Everything was generated by bindgen and uses the original API names.
This crate provides:
* 👀 Function pointers!
* 💯 Dynamic loader of instance and device commands!
* ✍️  Original names of commands, structures and macros!
* 💪 Extra useful features to help with development!
* 😀 Simple examples!


## Code
Enumerate all instance extensions.

```rust
use mira::error::MiraError;
use mira::loader;
use mira::mem::{zeroed_vec, from_cstring};
use mira::vulkan::*;
use const_cstr::*;

fn main() -> Result<(), MiraError> {
    let enumerate_instance_extensions:PFN_vkEnumerateInstanceExtensionProperties;
    enumerate_instance_extensions = unsafe {
        loader::instance(std::ptr::null_mut(), const_cstr!("vkEnumerateInstanceExtensionProperties"))?
    };

    let mut count:u32 = 0;
    unsafe { enumerate_instance_extensions(std::ptr::null_mut(), &mut count, std::ptr::null_mut()) };
    let mut extensions = unsafe { zeroed_vec::<VkExtensionProperties>(count as usize) };

    unsafe { enumerate_instance_extensions(std::ptr::null_mut(), &mut count, extensions.as_mut_ptr()) };

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
```

## Examples
Successfully tested on Linux(Lubuntu 20.04) with Intel(R) HD Graphics 620 (KBL GT2).

### Color
Displays a window with a purple background.
>cargo run --example color

![screenshot](examples/mira_color.png)

## Vulkan version
1.2.202

[Vulkan Changelog](https://github.com/KhronosGroup/Vulkan-Docs/blob/main/ChangeLog.txt)

## License
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
