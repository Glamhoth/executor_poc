use core::cell::UnsafeCell;
use core::cmp::Ordering;
use core::fmt::{Debug, Error, Formatter};

#[repr(transparent)]
pub struct ThinCell<T>(UnsafeCell<T>);

impl<T> ThinCell<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        ThinCell(UnsafeCell::new(value))
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
        (& *self.0.get())
    }

    #[inline(always)]
    pub unsafe fn as_ref_mut(&self) -> &mut T {
        (&mut *self.0.get())
    }
}

unsafe impl<T> Sync for ThinCell<T> where T: Send {}

impl<T: Debug> Debug for ThinCell<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        unsafe { self.as_ref().fmt(fmt) }
    }
}

impl<T: PartialEq> PartialEq for ThinCell<T> {
    #[inline]
    fn eq(&self, other: &ThinCell<T>) -> bool {
        unsafe { self.as_ref() == other.as_ref() }
    }
}

impl<T: Eq> Eq for ThinCell<T> {}

impl<T: PartialOrd> PartialOrd for ThinCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &ThinCell<T>) -> Option<Ordering> {
        unsafe { self.as_ref().partial_cmp(&other.as_ref()) }
    }

    #[inline]
    fn lt(&self, other: &ThinCell<T>) -> bool {
        unsafe { self.as_ref() < other.as_ref() }
    }

    #[inline]
    fn le(&self, other: &ThinCell<T>) -> bool {
        unsafe { self.as_ref() <= other.as_ref() }
    }

    #[inline]
    fn gt(&self, other: &ThinCell<T>) -> bool {
        unsafe { self.as_ref() > other.as_ref() }
    }

    #[inline]
    fn ge(&self, other: &ThinCell<T>) -> bool {
        unsafe { self.as_ref() >= other.as_ref() }
    }
}

impl<T: Ord> Ord for ThinCell<T> {
    #[inline]
    fn cmp(&self, other: &ThinCell<T>) -> Ordering {
        unsafe { self.as_ref().cmp(&other.as_ref()) }
    }
}
