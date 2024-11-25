use bindgen;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // vulkan
    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .clang_arg("-I./Vulkan-Headers/include/")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("../src/");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut commands = Command::new("sh");
    commands.arg("-c");
    commands.arg(format!(
        "cat {} | ./vulkan.sh && mv vulkan.rs {} && rm {}",
        out_path.join("bindings.rs").to_str().unwrap(),
        out_path.to_str().unwrap(),
        out_path.join("bindings.rs").to_str().unwrap()
    ));

    let output = commands.output().expect("failed to execute stream");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
