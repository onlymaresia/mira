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
}
