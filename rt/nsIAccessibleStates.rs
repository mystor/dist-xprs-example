//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleStates.idl
//


pub mod nsIAccessibleStates_consts {
    pub const STATE_UNAVAILABLE: i64 = 1;
    pub const STATE_SELECTED: i64 = 2;
    pub const STATE_FOCUSED: i64 = 4;
    pub const STATE_PRESSED: i64 = 8;
    pub const STATE_CHECKED: i64 = 16;
    pub const STATE_MIXED: i64 = 32;
    pub const STATE_READONLY: i64 = 64;
    pub const STATE_HOTTRACKED: i64 = 128;
    pub const STATE_DEFAULT: i64 = 256;
    pub const STATE_EXPANDED: i64 = 512;
    pub const STATE_COLLAPSED: i64 = 1024;
    pub const STATE_BUSY: i64 = 2048;
    pub const STATE_FLOATING: i64 = 4096;
    pub const STATE_MARQUEED: i64 = 8192;
    pub const STATE_ANIMATED: i64 = 16384;
    pub const STATE_INVISIBLE: i64 = 32768;
    pub const STATE_OFFSCREEN: i64 = 65536;
    pub const STATE_SIZEABLE: i64 = 131072;
    pub const STATE_MOVEABLE: i64 = 262144;
    pub const STATE_SELFVOICING: i64 = 524288;
    pub const STATE_FOCUSABLE: i64 = 1048576;
    pub const STATE_SELECTABLE: i64 = 2097152;
    pub const STATE_LINKED: i64 = 4194304;
    pub const STATE_TRAVERSED: i64 = 8388608;
    pub const STATE_MULTISELECTABLE: i64 = 16777216;
    pub const STATE_EXTSELECTABLE: i64 = 33554432;
    pub const STATE_ALERT_LOW: i64 = 67108864;
    pub const STATE_ALERT_MEDIUM: i64 = 134217728;
    pub const STATE_ALERT_HIGH: i64 = 268435456;
    pub const STATE_PROTECTED: i64 = 536870912;
    pub const STATE_HASPOPUP: i64 = 1073741824;
    pub const STATE_REQUIRED: i64 = 67108864;
    pub const STATE_IMPORTANT: i64 = 134217728;
    pub const STATE_INVALID: i64 = 268435456;
    pub const STATE_CHECKABLE: i64 = 8192;
    pub const EXT_STATE_SUPPORTS_AUTOCOMPLETION: i64 = 1;
    pub const EXT_STATE_DEFUNCT: i64 = 2;
    pub const EXT_STATE_SELECTABLE_TEXT: i64 = 4;
    pub const EXT_STATE_EDITABLE: i64 = 8;
    pub const EXT_STATE_ACTIVE: i64 = 16;
    pub const EXT_STATE_MODAL: i64 = 32;
    pub const EXT_STATE_MULTI_LINE: i64 = 64;
    pub const EXT_STATE_HORIZONTAL: i64 = 128;
    pub const EXT_STATE_OPAQUE: i64 = 256;
    pub const EXT_STATE_SINGLE_LINE: i64 = 512;
    pub const EXT_STATE_TRANSIENT: i64 = 1024;
    pub const EXT_STATE_VERTICAL: i64 = 2048;
    pub const EXT_STATE_STALE: i64 = 4096;
    pub const EXT_STATE_ENABLED: i64 = 8192;
    pub const EXT_STATE_SENSITIVE: i64 = 16384;
    pub const EXT_STATE_EXPANDABLE: i64 = 32768;
    pub const EXT_STATE_PINNED: i64 = 65536;
}


#[repr(C)]
pub struct nsIAccessibleStates {
    vtable: *const nsIAccessibleStatesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleStates {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf1e0fbb7, 0xfde4, 0x4519,
            [0x93, 0x83, 0x2b, 0xcb, 0xee, 0x42, 0x85, 0x13])
    }
}

unsafe impl RefCounted for nsIAccessibleStates {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// Enable coercing to ourselves
pub trait nsIAccessibleStatesCoerce {
    fn coerce_from(v: &nsIAccessibleStates) -> &Self;
}

impl nsIAccessibleStatesCoerce for nsIAccessibleStates {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStates) -> &Self {
        v
    }
}

impl nsIAccessibleStates {
    #[inline]
    pub fn coerce<T: nsIAccessibleStatesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleStates {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleStatesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleStates) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleStatesVTable {
    pub __base: nsISupportsVTable,

}


impl nsIAccessibleStates {
}


