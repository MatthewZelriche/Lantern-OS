fn main() {
    // Pass in our custom linker script
    println!("cargo:rustc-link-arg=-Tkernel/linker.ld");
}
