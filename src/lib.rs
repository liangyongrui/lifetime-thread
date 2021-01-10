#![deny(
    warnings,
    clippy::all,
    clippy::correctness,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
use crossbeam_epoch::Atomic;
use std::{future::Future, thread};
mod inner;
pub use inner::Inner;

mod outer;
pub use outer::Outer;

/// Spawns a new thread, return `data` warpped in `Outer`
pub fn spawn<T, F>(data: T, f: F) -> Outer<T>
where
    T: Send + 'static,
    F: FnOnce(Inner<T>) + Send + 'static,
{
    let flag_ptr = Box::into_raw(Box::new(Atomic::new(0b11_u8))) as usize;
    let data_ptr = Box::into_raw(Box::new(data)) as usize;
    let inner = Inner::new(flag_ptr, data_ptr);
    thread::spawn(move || f(inner));
    Outer::new(flag_ptr, data_ptr)
}

/// Spawns a new async task, return `data` warpped in `Outer`
pub fn async_spawn<T, F, FU>(data: T, f: F) -> Outer<T>
where
    T: Send + 'static,
    FU: Future<Output = ()> + Send + 'static,
    F: FnOnce(Inner<T>) -> FU + Send + 'static,
{
    let flag_ptr = Box::into_raw(Box::new(Atomic::new(0b11_u8))) as usize;
    let data_ptr = Box::into_raw(Box::new(data)) as usize;
    let inner = Inner::new(flag_ptr, data_ptr);
    tokio::spawn(async { f(inner).await });
    Outer::new(flag_ptr, data_ptr)
}
