use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DoubleBuffer<T> {
    buffers: [UnsafeCell<T>; 2],
    active: AtomicUsize,
}

unsafe impl<T: Send> Send for DoubleBuffer<T> {}
unsafe impl<T: Sync> Sync for DoubleBuffer<T> {}

impl<T> DoubleBuffer<T> {
    pub fn new(a: T, b: T) -> Self {
        Self {
            buffers: [UnsafeCell::new(a), UnsafeCell::new(b)],
            active: AtomicUsize::new(0),
        }
    }

    /// Emulator thread
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let cur = self.active.load(Ordering::Relaxed);
        let next = cur ^ 1;

        unsafe {
            f(&mut *self.buffers[next].get());
        }

        self.active.store(next, Ordering::Release);
    }

    /// Render thread
    pub fn read(&self) -> &T {
        let idx = self.active.load(Ordering::Acquire);
        unsafe { &*self.buffers[idx].get() }
    }
}
