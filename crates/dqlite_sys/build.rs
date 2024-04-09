use std::path::PathBuf;

fn main() {
    let dqlite = pkg_config::Config::new().atleast_version("1.16.0").probe("dqlite").unwrap();
    let includes = dqlite.include_paths.iter().map(|path| format!("-I{}", path.to_str().unwrap()));

    let bindings = bindgen::builder()
        .clang_args(includes)
        .header_contents("shim.h", "#include <dqlite.h>")
        .allowlist_function("dqlite_.+")
        .blocklist_function("dqlite_vfs_.+")
        .blocklist_function("dqlite_node_recover")
        .blocklist_function("dqlite_node_set_network_latency")
        .allowlist_type("dqlite_.+")
        .blocklist_type("dqlite_vfs_.+")
        .blocklist_type("dqlite_buffer")
        .blocklist_type("dqlite_node_info")
        .blocklist_type("sqlite3_.+")
        .blocklist_type("sqlite_.+")
        .generate()
        .unwrap();

    bindings.write_to_file(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs")).unwrap();
}
