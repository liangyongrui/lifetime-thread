use crossbeam_epoch::{Atomic, Owned};
use std::ops::Deref;
use std::{marker::PhantomData, sync::atomic::Ordering};

const fn drop_outer(flag: usize) -> usize {
    flag & 0b10
}

#[derive(Debug)]
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
impl<T> Outer<T> {
    pub(crate) const fn new(flag_ptr: usize, data_ptr: usize) -> Self {
        Self {
            flag_ptr,
            data_ptr,
            _type: PhantomData,
        }
    }
}
