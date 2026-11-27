fn main() {
    // libskyrim is pure Rust: the former CommonLibSSE-NG / commonlib_bridge C++
    // build (xmake or prebuilt download) is no longer needed.
    println!("cargo:rerun-if-changed=build.rs");
}
