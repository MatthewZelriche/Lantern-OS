fn main() {
   // Pass in our custom linker script
   println!("cargo:rustc-link-arg=-Tbootloaders/raspi/linker.ld");
}