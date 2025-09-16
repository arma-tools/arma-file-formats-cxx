fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("include")
        .file("src/reader.cpp")
        .cpp(true)
        .std("c++17")
        .compile("arma_file_formats_cxx");

    println!("cargo:rerun-if-changed=include/reader.h");
}
