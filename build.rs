use e2p_sys::CONSTANTS;

fn main() {
    // Introduced in v1.43
    if CONSTANTS.iter().any(|&s| s == "EXT4_INLINE_DATA_FL") {
        println!("cargo:rustc-cfg=INLINE_DATA");
    }
    if CONSTANTS.iter().any(|&s| s == "EXT4_PROJINHERIT_FL") {
        println!("cargo:rustc-cfg=PROJINHERIT");
    }
    if CONSTANTS.iter().any(|&s| s == "EXT4_ENCRYPT_FL") {
        println!("cargo:rustc-cfg=ENCRYPT");
    }

    // Introduced in v1.44.4
    if CONSTANTS.iter().any(|&s| s == "EXT4_VERITY_FL") {
        println!("cargo:rustc-cfg=VERITY");
    }

    // Introduced in v1.45.0
    if CONSTANTS.iter().any(|&s| s == "EXT4_CASEFOLD_FL") {
        println!("cargo:rustc-cfg=CASEFOLD");
    }

    // Introduced in v1.45.7
    if CONSTANTS.iter().any(|&s| s == "FS_DAX_FL") {
        println!("cargo:rustc-cfg=DAX");
    }
}
