// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "VteFeatureFlags")]
    pub struct FeatureFlags: u32 {
        #[doc(alias = "VTE_FEATURE_FLAG_BIDI")]
        const FLAG_BIDI = ffi::VTE_FEATURE_FLAG_BIDI as u32;
        #[doc(alias = "VTE_FEATURE_FLAG_ICU")]
        const FLAG_ICU = ffi::VTE_FEATURE_FLAG_ICU as u32;
        #[doc(alias = "VTE_FEATURE_FLAG_SYSTEMD")]
        const FLAG_SYSTEMD = ffi::VTE_FEATURE_FLAG_SYSTEMD as u32;
        #[doc(alias = "VTE_FEATURE_FLAG_SIXEL")]
        const FLAG_SIXEL = ffi::VTE_FEATURE_FLAG_SIXEL as u32;
        #[doc(alias = "VTE_FEATURE_FLAGS_MASK")]
        const FLAGS_MASK = ffi::VTE_FEATURE_FLAGS_MASK as u32;
    }
}

impl fmt::Display for FeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FeatureFlags {
    type GlibType = ffi::VteFeatureFlags;

    fn into_glib(self) -> ffi::VteFeatureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VteFeatureFlags> for FeatureFlags {
    unsafe fn from_glib(value: ffi::VteFeatureFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[doc(alias = "VtePtyFlags")]
    pub struct PtyFlags: u32 {
        #[doc(alias = "VTE_PTY_NO_LASTLOG")]
        const NO_LASTLOG = ffi::VTE_PTY_NO_LASTLOG as u32;
        #[doc(alias = "VTE_PTY_NO_UTMP")]
        const NO_UTMP = ffi::VTE_PTY_NO_UTMP as u32;
        #[doc(alias = "VTE_PTY_NO_WTMP")]
        const NO_WTMP = ffi::VTE_PTY_NO_WTMP as u32;
        #[doc(alias = "VTE_PTY_NO_HELPER")]
        const NO_HELPER = ffi::VTE_PTY_NO_HELPER as u32;
        #[doc(alias = "VTE_PTY_NO_FALLBACK")]
        const NO_FALLBACK = ffi::VTE_PTY_NO_FALLBACK as u32;
        #[doc(alias = "VTE_PTY_NO_SESSION")]
        const NO_SESSION = ffi::VTE_PTY_NO_SESSION as u32;
        #[doc(alias = "VTE_PTY_NO_CTTY")]
        const NO_CTTY = ffi::VTE_PTY_NO_CTTY as u32;
        #[doc(alias = "VTE_PTY_DEFAULT")]
        const DEFAULT = ffi::VTE_PTY_DEFAULT as u32;
    }
}

impl fmt::Display for PtyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for PtyFlags {
    type GlibType = ffi::VtePtyFlags;

    fn into_glib(self) -> ffi::VtePtyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VtePtyFlags> for PtyFlags {
    unsafe fn from_glib(value: ffi::VtePtyFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PtyFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_pty_flags_get_type()) }
    }
}

impl glib::value::ValueType for PtyFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PtyFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PtyFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
