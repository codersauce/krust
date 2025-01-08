#[cfg(feature = "dz60")]
pub mod dz60;
#[cfg(feature = "dz60")]
pub use dz60::dz60 as current_board;

pub mod macros;
