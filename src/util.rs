use std::ops::Deref;

/// Structure that can be used to move non-send objects accross thread. Unsafe.
pub struct Movable<T>(T);
unsafe impl<T> Send for Movable<T> {}

#[allow(dead_code)]
impl<T> Movable<T> {
    /// Safety: This function is unsafe because it turns non Send object into Send.
    pub unsafe fn new(t: T) -> Self {
        Self(t)
    }

    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Deref for Movable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> Clone for Movable<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}