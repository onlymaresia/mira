use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=vulkan");

    cc::Build::new()
            .flag("-std=c++14")
            .cpp_link_stdlib("stdc++")
            .cpp(true)
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .define("VMA_DYNAMIC_VULKAN_FUNCTIONS", "1")
            .include("extra/VulkanMemoryAllocator/include")
            .file("wrapper_build.cpp")
            .compile("mira_vma");

    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .clang_arg("-Iextra/VulkanMemoryAllocator/include")
        .header("wrapper_build.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from("src/vulkan_memory_allocator.rs"))
        .expect("Couldn't write bindings!");
}
