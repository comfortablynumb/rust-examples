//! Build script demonstrating custom build logic
//!
//! Build scripts run before the package is compiled and can:
//! - Generate code
//! - Compile C/C++ dependencies
//! - Set environment variables
//! - Communicate with Cargo via special println! instructions

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Declare custom cfg for conditional compilation
    println!("cargo::rustc-check-cfg=cfg(release_build)");

    // Get build information
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target = env::var("TARGET").unwrap();

    println!("cargo:warning=Running build script...");
    println!("cargo:warning=  Profile: {}", profile);
    println!("cargo:warning=  Target: {}", target);
    println!("cargo:warning=  Out dir: {}", out_dir);

    // Generate a Rust source file with build information
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let build_info = format!(
        r#"
/// Build profile (dev, release, etc.)
pub const BUILD_PROFILE: &str = "{}";

/// Target triple
pub const BUILD_TARGET: &str = "{}";

/// Build timestamp
pub const BUILD_TIMESTAMP: &str = "{}";

/// Get build information as a formatted string
pub fn get_build_info() -> String {{
    format!(
        "Built with profile '{{}}' for target '{{}}' at {{}}",
        BUILD_PROFILE,
        BUILD_TARGET,
        BUILD_TIMESTAMP
    )
}}
"#,
        profile,
        target,
        chrono::Utc::now().to_rfc3339()
    );

    fs::write(&dest_path, build_info).expect("Failed to write build_info.rs");

    // Set custom environment variables for the compiled crate
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    println!("cargo:rustc-env=BUILD_TARGET={}", target);

    // Example: Set a custom configuration flag
    if profile == "release" {
        println!("cargo:rustc-cfg=release_build");
    }

    // Link to a library (example - not actually linking anything)
    // println!("cargo:rustc-link-lib=static=mylib");
    // println!("cargo:rustc-link-search=native=/path/to/lib");
}
