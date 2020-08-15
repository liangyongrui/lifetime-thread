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

use crossbeam_epoch::{Atomic, Owned};
use std::ops::Deref;
use std::{marker::PhantomData, sync::atomic::Ordering, thread};

pub struct Inner<T> {
    flag_ptr: usize,
    data_ptr: usize,
    _type: PhantomData<T>,
}

const fn drop_inner(flag: usize) -> usize {
    flag & 0b01
}

const fn drop_outer(flag: usize) -> usize {
    flag & 0b10
}

const fn is_outer_live(flag: usize) -> bool {
    flag & 0b01 > 0
}

impl<T> Inner<T> {
    #[must_use]
    pub fn get(&self) -> Option<&T> {
        let guard = crossbeam_epoch::pin();
        let flag_ptr = unsafe { &*(self.flag_ptr as *const Atomic<usize>) };
        let flag = flag_ptr.load(Ordering::Acquire, &guard);
        let old = unsafe { *flag.deref() };
        if is_outer_live(old) {
            Some(unsafe { &*(self.data_ptr as *const T) })
        } else {
            None
        }
    }
}
impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        let guard = crossbeam_epoch::pin();
        let flag_ptr = unsafe { &*(self.flag_ptr as *const Atomic<usize>) };
        loop {
            let flag = flag_ptr.load(Ordering::Acquire, &guard);
            let old = unsafe { *flag.deref() };
            let n = drop_inner(old);

            if let Ok(t) = flag_ptr.compare_and_set(flag, Owned::new(n), Ordering::Release, &guard)
            {
                if n == 0 {
                    unsafe {
                        guard.defer_destroy(t);
                        std::ptr::drop_in_place(self.data_ptr as *mut T);
                    }
                }
                break;
            }
        }
    }
}
pub struct Outer<T> {
    flag_ptr: usize,
    data_ptr: usize,
    _type: PhantomData<T>,
}
impl<T> Deref for Outer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.data_ptr as *const T) }
    }
}
impl<T> Drop for Outer<T> {
    fn drop(&mut self) {
        let guard = crossbeam_epoch::pin();
        let flag_ptr = unsafe { &*(self.flag_ptr as *const Atomic<usize>) };
        loop {
            let flag = flag_ptr.load(Ordering::Acquire, &guard);
            let old = unsafe { *flag.deref() };
            let n = drop_outer(old);

            if let Ok(t) = flag_ptr.compare_and_set(flag, Owned::new(n), Ordering::Release, &guard)
            {
                if n == 0 {
                    unsafe {
                        guard.defer_destroy(t);
                        std::ptr::drop_in_place(self.data_ptr as *mut T);
                    }
                }
                break;
            }
        }
    }
}

/// Spawns a new thread, return `data` warpped in `Outer`
pub fn spawn<T, F>(data: T, f: F) -> Outer<T>
where
    T: Send + 'static,
    F: FnOnce(Inner<T>) + Send + 'static,
{
    let flag_ptr = Box::into_raw(Box::new(Atomic::new(0b11))) as usize;
    let data_ptr = Box::into_raw(Box::new(data)) as usize;
    let inner = Inner {
        flag_ptr,
        data_ptr,
        _type: PhantomData,
    };
    thread::spawn(move || f(inner));
    Outer {
        flag_ptr,
        data_ptr,
        _type: PhantomData,
    }
}

/// Spawns a new async task, return `data` warpped in `Outer`
pub fn async_spawn<T, F>(data: T, f: F) -> Outer<T>
where
    T: Send + 'static,
    F: FnOnce(Inner<T>) + Send + 'static,
{
    let flag_ptr = Box::into_raw(Box::new(Atomic::new(0b11))) as usize;
    let data_ptr = Box::into_raw(Box::new(data)) as usize;
    let inner = Inner {
        flag_ptr,
        data_ptr,
        _type: PhantomData,
    };
    async_std::task::spawn(async { f(inner) });
    Outer {
        flag_ptr,
        data_ptr,
        _type: PhantomData,
    }
}
