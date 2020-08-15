use crossbeam_epoch::{Atomic, Owned};
use std::{marker::PhantomData, sync::atomic::Ordering};

const fn drop_inner(flag: usize) -> usize {
    flag & 0b01
}

const fn is_outer_live(flag: usize) -> bool {
    flag & 0b01 > 0
}

#[derive(Debug)]
pub struct Inner<T> {
    flag_ptr: usize,
    data_ptr: usize,
    _type: PhantomData<T>,
}

impl<T> Inner<T> {
    pub(crate) const fn new(flag_ptr: usize, data_ptr: usize) -> Self {
        Self {
            flag_ptr,
            data_ptr,
            _type: PhantomData,
        }
    }
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
