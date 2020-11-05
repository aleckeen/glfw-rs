fn main() {
    // download the repository if it isn't already.
    if !std::path::Path::new("glfw/.git").exists() {
        std::process::Command::new("git")
            .args(&["submodule", "update", "--init", "glfw"])
            .status()
            .unwrap();
    }
    // clean the repository.
    std::process::Command::new("git")
        .current_dir("glfw")
        .args(&["clean", "-fd"])
        .status()
        .unwrap();

    // compile the library using cmake.
    let dst = cmake::Config::new("glfw")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .build();

    // generate bindings.
    let bindings = bindgen::builder()
        .header("glfw/include/GLFW/glfw3.h")
        .generate_comments(false)
        .whitelist_var("glfw.*")
        .whitelist_function("glfw.*")
        .whitelist_type("glfw.*")
        .whitelist_var("gl.*")
        .whitelist_function("gl.*")
        .whitelist_type("gl.*")
        .generate()
        .unwrap();

    let out: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().parse().unwrap();
    bindings.write_to_file(out.join("bindings.rs")).unwrap();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=glfw3");
}
