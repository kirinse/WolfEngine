use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod chrono;
pub mod compression;
pub mod net;
pub mod os;
pub mod script;
pub mod serialization;
