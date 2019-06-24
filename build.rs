use e2p_sys::constants;

fn main() {
    // Introduced in v1.43
    if constants.iter().any(|&s| s == "EXT4_INLINE_DATA_FL") {
        println!("cargo:rustc-cfg=INLINE_DATA");
    }
    if constants.iter().any(|&s| s == "EXT4_PROJINHERIT_FL") {
        println!("cargo:rustc-cfg=PROJINHERIT");
    }
    if constants.iter().any(|&s| s == "EXT4_ENCRYPT_FL") {
        println!("cargo:rustc-cfg=ENCRYPT");
    }

    // Introduced in v1.44.4
    if constants.iter().any(|&s| s == "EXT4_VERITY_FL") {
        println!("cargo:rustc-cfg=VERITY");
    }

    // Introduced in v1.45
    if constants.iter().any(|&s| s == "EXT4_CASEFOLD_FL") {
        println!("cargo:rustc-cfg=CASEFOLD");
    }
}
