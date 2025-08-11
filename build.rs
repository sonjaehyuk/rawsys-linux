use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let kernel_features = [
        "CARGO_FEATURE_DEFAULT_KERNEL_5_4",
        "CARGO_FEATURE_DEFAULT_KERNEL_5_10",
        "CARGO_FEATURE_DEFAULT_KERNEL_5_15",
        "CARGO_FEATURE_DEFAULT_KERNEL_6_1",
        "CARGO_FEATURE_DEFAULT_KERNEL_6_6",
        "CARGO_FEATURE_DEFAULT_KERNEL_6_10",
        "CARGO_FEATURE_DEFAULT_KERNEL_6_12",
    ];

    let enabled_kernels = kernel_features
        .iter()
        .filter(|name| env::var(name).is_ok())
        .count();

    if enabled_kernels > 1 {
        panic!(
            "💥 Exactly one default_kernel_* feature must be enabled (found {}).\n\
            💡 kernel versions selects the kernel version exposed by default, not the available kernels. All versions are available via modules.",
            enabled_kernels
        );
    } else if enabled_kernels == 0 {
        println!(
            "No default_kernel_* feature explicitly set; build will rely on code-side defaults."
        );
    }

    // Automatically detect if thumb-mode is an available feature by looking at
    // the prefix of the target. Currently, the thumb-mode target feature is
    // only set automatically in nightly builds, so we must do the manual
    // feature detect here.
    //
    // "armv7-linux-androideabi" is a special case that has thumb-mode enabled,
    // but does not start with the "thumb" prefix.
    if env::var("TARGET")
        .is_ok_and(|t| t.starts_with("thumb") || t == "armv7-linux-androideabi")
    {
        println!("cargo:rustc-cfg=feature=\"thumb-mode\"");
    }
}
