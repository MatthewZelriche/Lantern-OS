use std::env;

fn main() {
   // Inform the linker of which version of the raspberry pi we are building for
   match env::var("RPI").map(|x| {x.parse::<u8>().unwrap()}).unwrap_or(4) {
      3 => { 
         println!("cargo:rustc-link-arg=--defsym=__RPI_VER=3");
      }
      4 => { 
         println!("cargo:rustc-link-arg=--defsym=__RPI_VER=4");
      }
      _ => panic!("Invalid RPI envvar"),
   }

   // Pass in our custom linker script
   println!("cargo:rustc-link-arg=-Tbootloaders/raspi/linker.ld");
}