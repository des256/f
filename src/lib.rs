#[cfg(build="debug")]
#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => { println!("DEBUG {}:{}:{}: {}",file!(),line!(),column!(),std::format_args!($($arg)*)) };
}

#[cfg(build="release")]
#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => { };
}

#[doc(hidden)]
mod sys;

mod base;
pub use base::*;

mod system;
pub use system::*;

mod gpu;
pub use gpu::*;
