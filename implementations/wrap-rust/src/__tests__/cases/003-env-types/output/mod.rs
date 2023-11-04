// Disable unused code warnings for this entire module
#![allow(unused)]

pub mod entry;
pub mod prelude;
pub mod env_object;
pub mod env_enum;
pub mod env;
pub mod module;

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
