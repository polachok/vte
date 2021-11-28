use ffi;
use glib::translate::*;
use std::mem;

#[repr(transparent)]
pub struct CharAttributes(ffi::VteCharAttributes);

#[doc(hidden)]
impl Uninitialized for CharAttributes {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::VteCharAttributes> for CharAttributes {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::VteCharAttributes, Self> {
        let ptr: *const CharAttributes = &*self;
        Stash(ptr as *const ffi::VteCharAttributes, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::VteCharAttributes> for CharAttributes {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::VteCharAttributes, Self> {
        let ptr: *mut CharAttributes = &mut *self;
        StashMut(ptr as *mut ffi::VteCharAttributes, self)
    }
}
