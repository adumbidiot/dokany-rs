use std::path::PathBuf;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap();
    let debug = std::env::var("DEBUG").unwrap().parse::<bool>().unwrap();

    let mut lib_path = manifest_dir.join("dokan");

    match target_arch.as_str() {
        "arm" => {
            lib_path.push("ARM");
        }
        "aarch64" => {
            lib_path.push("ARM64");
        }
        "x86" => {
            lib_path.push("Win32");
        }
        "x86_64" => {
            lib_path.push("x64");
        }
        _ => panic!("target arch {target_arch} is unsupported"),
    }

    if debug {
        lib_path.push("Debug");
    } else {
        lib_path.push("Release");
    }

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=dokan2");
}
