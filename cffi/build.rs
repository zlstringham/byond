use cbindgen::{Config, ExportConfig, Language};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join("include")
        .join("byondrs")
        .join(format!("{}.h", package_name))
        .display()
        .to_string();

    let mut renames = HashMap::new();
    renames.insert("Crc32Struct".to_string(), "crc32_struct".to_string());
    let exports = ExportConfig {
        rename: renames,
        ..Default::default()
    };

    let config = Config {
        autogen_warning: Some("// GENERATED FILE. Do not modify.".to_string()),
        language: Language::C,
        cpp_compat: true,
        namespace: Some("byondrs".to_string()),
        export: exports,
        line_length: 80,
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .join("target")
    }
}
