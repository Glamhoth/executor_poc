use core::cell::UnsafeCell;
use core::cmp::Ordering;
use core::fmt::{Debug, Error, Formatter};

use cortex_m::interrupt;

#[repr(transparent)]
pub struct CritCell<T>(UnsafeCell<T>);

impl<T> CritCell<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        CritCell(UnsafeCell::new(value))
    }

    #[inline(always)]
    pub unsafe fn as_ptr_mut(&self) -> *mut T {
        self.0.get()
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const T {
        self.0.get()
    }

    #[inline(always)]
    pub unsafe fn as_ref(&self) -> &T {
        (&*self.0.get())
    }

    #[inline(always)]
    pub unsafe fn as_ref_mut(&self) -> &mut T {
        (&mut *self.0.get())
    }

    #[inline(always)]
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { interrupt::free(|_| f(self.as_ref_mut())) }
    }
}

unsafe impl<T> Sync for CritCell<T> where T: Send {}

impl<T: Debug> Debug for CritCell<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe { self.as_ref().fmt(fmt) }
    }
}
