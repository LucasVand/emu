use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct SharedBuffer {
    pub buffer: UnsafeCell<Box<[u8]>>,
    id: AtomicUsize,
}
impl SharedBuffer {
    pub fn new(size: usize) -> SharedBuffer {
        let buf: Box<[u8]> = vec![0; size].into_boxed_slice();
        SharedBuffer {
            buffer: buf.into(),
            id: AtomicUsize::new(0),
        }
    }
    pub fn new_with_value(buf: &Box<[u8]>) -> SharedBuffer {
        let mut buf_copy: Box<[u8]> = vec![0; buf.len()].into_boxed_slice();
        buf_copy.copy_from_slice(buf);
        SharedBuffer {
            buffer: buf_copy.into(),
            id: AtomicUsize::new(0),
        }
    }
    pub fn publish(&self) {
        self.id.fetch_add(1, Ordering::Release);
    }
    pub fn update_addr(&self, addr: usize, value: u8) {
        unsafe {
            let buf = &mut *self.buffer.get();
            buf[addr] = value;
        }
    }
    pub fn read(&self) -> &Box<[u8]> {
        unsafe { return &*self.buffer.get() };
    }
}

unsafe impl Sync for SharedBuffer {}
