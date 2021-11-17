use bindgen;
use std::path::PathBuf;
use std::process::Command;
use std::io;
use std::io::Write;

/*
use std::io::{BufReader, BufRead, BufWriter};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use regex::Regex;

pub fn enum_generator(source: &str, dest: &str,
                      name: &str, new_name: &str, guard: &str,
                      ends: Option<Vec<&str>>) {
    let input = BufReader::new(File::open(source).unwrap());
    let lines = input.lines();

    let mut output = BufWriter::new(OpenOptions::new().append(true).open(dest).unwrap());

    let case = format!("pub const {}_VK_{}_", name, guard);

    output.write(format!("\
    #[non_exhaustive]\n\
    #[derive(Debug, Eq, PartialEq, num_enum::TryFromPrimitive)]\n\
    #[repr(u64)]\n\
    pub enum {} {{\n", new_name).as_bytes()).unwrap();

    let mut hit = HashMap::<u64, bool>::new();
    let cap = Regex::new(r" ([0-9]+);").unwrap();

    for line in lines {
        if line.is_ok() {
            let mut text = line.unwrap();

            if text.starts_with(&case) {
                match cap.captures(&text) {
                    Some(c) => {
                        if let Some(value) = c.get(1) {
                            let v = value.as_str().parse::<u64>().unwrap();

                            if hit.contains_key(&v) {
                                continue;
                            }

                            hit.insert(v, true);
                        }
                    }

                    _ => {}
                }

                text = text.replace(&case, "\t");
                let r = format!(": {} ", name);
                text = text.replace(&r, " ");

                if let Some(list) = ends.as_ref() {
                    for end in list {
                        let e = format!("{} =", end);
                        if text.contains(&e) {
                            text = text.replace(&e, " =");
                            break;
                        }
                    }
                    text = text.replace(";", ",\n");
                } else {
                    text = text.replace(";", ",\n");
                }

                output.write(text.as_bytes()).unwrap();
            }
        }
    }

    output.write("}\n\n".as_bytes()).unwrap();
    output.flush().unwrap();
}

fn enum_all(src: &str, enum_path: &str) {
    let mut output = BufWriter::new(File::create(&enum_path).unwrap());
    output.write(
        "#![allow(non_upper_case_globals)]\n\
        #![allow(non_camel_case_types)]\n\
        #![allow(non_snake_case)]\n\n".as_bytes()
    ).unwrap();
    output.flush().unwrap();

    enum_generator(src, enum_path, "VkFormat", "VkFormat", "FORMAT", Some(vec!["_EXT", "_KHR"]));
    enum_generator(src, enum_path, "VkColorSpaceKHR", "VkColorSpace", "COLOR_SPACE", Some(vec!["_KHR", "_EXT", "_AMD"]));
    enum_generator(src, enum_path, "VkPhysicalDeviceType", "VkPhysicalDeviceType", "PHYSICAL_DEVICE_TYPE", None);
    enum_generator(src, enum_path, "VkQueueFlagBits", "VkQueueFlagBits", "QUEUE", Some(vec!["_BIT", "_BIT_KHR"]));
    enum_generator(src, enum_path, "VkPresentModeKHR", "VkPresentMode", "PRESENT_MODE", Some(vec!["_KHR"]));
}
*/

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .clang_arg("-I./Vulkan-Headers/include/")
        /*.clang_arg("-I/usr/i686-w64-mingw32/include/")
        .clang_arg("-DVK_ENABLE_BETA_EXTENSIONS")
        .clang_arg("-DVK_USE_PLATFORM_ANDROID_KHR")
        //.clang_arg("-DVK_USE_PLATFORM_FUCHSIA")
        .clang_arg("-DVK_USE_PLATFORM_IOS_MVK")
        .clang_arg("-DVK_USE_PLATFORM_MACOS_MVK")
        .clang_arg("-DVK_USE_PLATFORM_METAL_EXT")
        .clang_arg("-DVK_USE_PLATFORM_VI_NN")
        .clang_arg("-DVK_USE_PLATFORM_WAYLAND_KHR")
        .clang_arg("-DVK_USE_PLATFORM_WIN32_KHR")
        .clang_arg("-DVK_USE_PLATFORM_XCB_KHR")
        .clang_arg("-DVK_USE_PLATFORM_XLIB_KHR")
        .clang_arg("-DVK_USE_PLATFORM_DIRECTFB_EXT")
        .clang_arg("-DVK_USE_PLATFORM_XLIB_XRANDR_EXT")
        //.clang_arg("-DVK_USE_PLATFORM_GGP")
        //.clang_arg("-DVK_USE_PLATFORM_SCREEN_QNX")*/
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out = "../src/vulkan/";
    let out_path = PathBuf::from(out);

    if std::fs::read_dir(out).is_err() {
        if std::fs::create_dir(out).is_err()  {
            println!("Wrong error!");
            return;
        }
    }

    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");

    let mut commands = Command::new("sh");
    commands.arg("-c");
    commands.arg(format!("cat {} | ./vulkan.sh && mv vulkan_generated.rs {} && rm {}",
                         out_path.join("bindings.rs").to_str().unwrap(),
                         out_path.to_str().unwrap(),
                         out_path.join("bindings.rs").to_str().unwrap()
    ));

    let output = commands.output().expect("failed to execute stream");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    //let src = out_path.join("vulkan_generated.rs").to_path_buf();
    //enum_all(src.to_str().unwrap(), out_path.join("vulkan_enum.rs").to_str().unwrap());
}
