use std::env;

fn main() {
   // Define some important symbols based on the RPI version we are compiling for
   // Default to RPI4 if we aren't provided with the appropriate ENVVAR, to prevent 
   // rust-analyzer from failing
   match env::var("RPI").map(|x| {x.parse::<u8>().unwrap()}).unwrap_or(4) {
      3 => { 
         println!("cargo:rustc-link-arg=--defsym=__START_ADDR=0x8000");
         println!("cargo:rustc-link-arg=--defsym=__RPI_VER=3");
      }
      4 => { 
         println!("cargo:rustc-link-arg=--defsym=__START_ADDR=0x80000"); 
         println!("cargo:rustc-link-arg=--defsym=__RPI_VER=4");
      }
      _ => panic!("Invalid RPI envvar"),
   }

   // Pass in our custom linker script
   println!("cargo:rustc-link-arg=-Tbootloaders/raspi/linker.ld");
}