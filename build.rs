/*
MIT License

Copyright (c) 2019-2024 Michael Lass

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use e2p_sys::CONSTANTS;

fn main() {
    // Available features
    println!("cargo::rustc-check-cfg=cfg(INLINE_DATA)");
    println!("cargo::rustc-check-cfg=cfg(PROJINHERIT)");
    println!("cargo::rustc-check-cfg=cfg(ENCRYPT)");
    println!("cargo::rustc-check-cfg=cfg(VERITY)");
    println!("cargo::rustc-check-cfg=cfg(CASEFOLD)");
    println!("cargo::rustc-check-cfg=cfg(DAX)");

    // Introduced in v1.43
    if CONSTANTS.iter().any(|&s| s == "EXT4_INLINE_DATA_FL") {
        println!("cargo::rustc-cfg=INLINE_DATA");
    }
    if CONSTANTS.iter().any(|&s| s == "EXT4_PROJINHERIT_FL") {
        println!("cargo::rustc-cfg=PROJINHERIT");
    }
    if CONSTANTS.iter().any(|&s| s == "EXT4_ENCRYPT_FL") {
        println!("cargo::rustc-cfg=ENCRYPT");
    }

    // Introduced in v1.44.4
    if CONSTANTS.iter().any(|&s| s == "EXT4_VERITY_FL") {
        println!("cargo::rustc-cfg=VERITY");
    }

    // Introduced in v1.45.0
    if CONSTANTS.iter().any(|&s| s == "EXT4_CASEFOLD_FL") {
        println!("cargo::rustc-cfg=CASEFOLD");
    }

    // Introduced in v1.45.7
    if CONSTANTS.iter().any(|&s| s == "FS_DAX_FL") {
        println!("cargo::rustc-cfg=DAX");
    }
}
