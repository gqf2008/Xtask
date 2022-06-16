mod fs;

pub use fs::*;

mod fatfs {
    include!("../../vendor/fatfs/binding/binding.rs");
}
