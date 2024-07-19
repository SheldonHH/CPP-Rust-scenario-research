use cxx_build::CFG;
use std::path::Path;

fn main() {
    CFG.exported_header_dirs.push(Path::new("cpp"));
    cxx_build::bridge("src/cxxbridge/ffi.rs")
        .file("cpp/Operations.cpp")
        .file("cpp/bridge.cpp")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/cxxbridge/ffi.rs");
    println!("cargo:rerun-if-changed=cpp/Operations.h");
    println!("cargo:rerun-if-changed=cpp/Operations.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
}
