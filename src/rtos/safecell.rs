use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct SafeCell<T>(UnsafeCell<T>);

impl<T> SafeCell<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        SafeCell(UnsafeCell::new(value))
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    #[inline(always)]
    pub fn as_ref_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl<T> Sync for SafeCell<T> where T: Send {}
