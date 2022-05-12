#![allow(unused_crate_dependencies)]

#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub mod ffi;
#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub mod render;
#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub mod stream;
#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub mod system;

// sample for exposing as c ABI
// #[no_mangle]
// extern "C" fn wolf_init() {
//     println!("Wolf initialized!");
// }

// #[no_mangle]
// pub extern "C" fn wolf_fini() {
//     println!("Wolf released!");
// }
