//! A tiny Vec implementation backed by a mutable reference to a slice.

use core::str;

pub struct Vec<'s, T> {
    pub inner: &'s mut [T],
    len: usize,
}

impl<'s, T> Vec<'s, T> {
    pub fn new(inner: &'s mut [T]) -> Self {
        Self { inner, len: 0 }
    }

    pub fn fake_push(&mut self, value: T) {
        assert!(self.len < self.inner.len(), "Vec overflow");
        self.inner[self.len] = value;
    }

    pub fn push(&mut self, value: T) {
        assert!(self.len < self.inner.len(), "Vec overflow");
        self.len += 1;
        self.inner[self.len] = value;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<'s, T> Vec<'s, T>
where
    T: Copy,
{
    pub fn push_slice(&mut self, slice: &[T]) {
        for element in slice.iter() {
            self.push(*element);
        }
    }
}

impl<'s> AsRef<[u8]> for Vec<'s, u8> {
    fn as_ref(&self) -> &[u8] {
        &self.inner[..self.len]
    }
}

impl<'s> Vec<'s, u8> {
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.inner[..self.len])
    }
}
