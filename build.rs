use std::path::PathBuf;
use std::env;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_path = out_dir.join("usrsctp_build");
    let output_path = out_dir.join("usrsctp");
    let source_path = env::current_dir().unwrap().join("usrsctp");

    if !output_path.join("bindings.rs").exists() {
        build(&source_path, &build_path, &output_path);

        let bindings = bindgen::Builder::default()
            .header_contents("wrapper.h", &String::from(format!("#include <{}/usrsctp.h>", output_path.join("include").to_string_lossy())))
            .whitelist_type("SCTP.+")
            .whitelist_type("sctp.+")
            .whitelist_function("sctp.+")
            .whitelist_function("usrsctp.+")
            .whitelist_type("sockaddr_conn")
            .whitelist_type("socket")
            .whitelist_recursively(false)
            .generate()
            .expect("failed to generate bindings");

        bindings
            .write_to_file(output_path.join("bindings.rs"))
            .expect("failed to write bindings");
    }

    println!("cargo:rustc-link-lib=usrsctp");
    println!("cargo:rustc-link-search=native={}", output_path.join("lib").to_string_lossy());
}

fn build(source_path: &PathBuf, build_path: &PathBuf, output_path: &PathBuf) {
    /* setup */
    if !build_path.exists() {
        let mut command = Command::new("meson");

        command.arg("setup");
        command.arg("--prefix");
        command.arg(&output_path);
        command.arg("-Dsctp_build_programs=false");
        let value = env::var("usrsctp_build_type").unwrap_or(String::new());
        match value.as_str() {
            "static" => { command.arg("-Ddefault_library=static"); },
            "shared" => { command.arg("-Ddefault_library=shared"); },
            "" => {  },
            _ => panic!("Invalid build type: {:?}", value)
        };
        command.arg(&build_path);
        command.arg(&source_path);

        let result = command.spawn().expect("failed to launch meson command")
            .wait_with_output().expect("failed to await meson output");

        if !result.status.success() {
            panic!("failed to setup usrsctp build");
        }
    }

    /* compile and install */
    {
        let mut compile = Command::new("meson");
        compile.arg("install"); /* implied compile */
        compile.arg("-C");
        compile.arg(&build_path);

        let success = compile.spawn()
            .expect("failed to spawn meson command")
            .wait().expect("failed to await meson output")
            .success();

        if ! success {
            panic!("Failed to compile/install libusrsctp.");
        }
    }
}