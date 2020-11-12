use std::path::PathBuf;
use std::env;
use build_utils::build::LibraryType;

fn main() {
    /*
    if let Ok(library) = pkg_config::Config::new()
        .atleast_version("1.0.0")
        .probe("usrsctp") {
        println!("Found usrsctp {} via pkgconfig.", library.version);
        return;
    }
    */

    let source = build_utils::source::BuildSourceGit::builder("https://github.com/sctplab/usrsctp.git".to_owned())
        /* We've to use a slightly older version since the newer version using the __NR_getrandom syscall which isn't supported on older kernels */
        .revision(Some("d6acf1e".to_owned()))
        .build();

    let meson = build_utils::build::MesonBuild::builder()
        .meson_option("sctp_build_programs", "false")
        .build();

    let mut build_builder = build_utils::Build::builder()
        .name("usrsctp")
        .source(Box::new(source))
        .add_step(Box::new(meson));

    match build_builder.build().expect("failed to generate build").execute() {
        Ok(result) => {
            result.emit_cargo();

            /* TODO: Generate bindings */
            /*
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
            */
        },
        Err(error) => {
            println!("{}", error.pretty_format());
            panic!("failed to execute usrsctp build");
        }
    }
}