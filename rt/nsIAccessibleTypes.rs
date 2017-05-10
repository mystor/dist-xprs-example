//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTypes.idl
//


pub mod nsIAccessibleScrollType_consts {
    pub const SCROLL_TYPE_TOP_LEFT: i64 = 0;
    pub const SCROLL_TYPE_BOTTOM_RIGHT: i64 = 1;
    pub const SCROLL_TYPE_TOP_EDGE: i64 = 2;
    pub const SCROLL_TYPE_BOTTOM_EDGE: i64 = 3;
    pub const SCROLL_TYPE_LEFT_EDGE: i64 = 4;
    pub const SCROLL_TYPE_RIGHT_EDGE: i64 = 5;
    pub const SCROLL_TYPE_ANYWHERE: i64 = 6;
}


#[repr(C)]
pub struct nsIAccessibleScrollType {
    vtable: *const nsIAccessibleScrollTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleScrollType {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x05cd38b1, 0x94b3, 0x4cdf,
            [0x83, 0x71, 0x39, 0x35, 0xa9, 0x61, 0x14, 0x05])
    }
}

unsafe impl RefCounted for nsIAccessibleScrollType {
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
pub trait nsIAccessibleScrollTypeCoerce {
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self;
}

impl nsIAccessibleScrollTypeCoerce for nsIAccessibleScrollType {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self {
        v
    }
}

impl nsIAccessibleScrollType {
    #[inline]
    pub fn coerce<T: nsIAccessibleScrollTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleScrollType {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleScrollTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollType) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleScrollTypeVTable {
    pub __base: nsISupportsVTable,

}


impl nsIAccessibleScrollType {
}


pub mod nsIAccessibleCoordinateType_consts {
    pub const COORDTYPE_SCREEN_RELATIVE: i64 = 0;
    pub const COORDTYPE_WINDOW_RELATIVE: i64 = 1;
    pub const COORDTYPE_PARENT_RELATIVE: i64 = 2;
}


#[repr(C)]
pub struct nsIAccessibleCoordinateType {
    vtable: *const nsIAccessibleCoordinateTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleCoordinateType {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc9fbdf10, 0x619e, 0x436f,
            [0xbf, 0x4b, 0x85, 0x66, 0x68, 0x6f, 0x15, 0x77])
    }
}

unsafe impl RefCounted for nsIAccessibleCoordinateType {
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
pub trait nsIAccessibleCoordinateTypeCoerce {
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self;
}

impl nsIAccessibleCoordinateTypeCoerce for nsIAccessibleCoordinateType {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self {
        v
    }
}

impl nsIAccessibleCoordinateType {
    #[inline]
    pub fn coerce<T: nsIAccessibleCoordinateTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleCoordinateType {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleCoordinateTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCoordinateType) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleCoordinateTypeVTable {
    pub __base: nsISupportsVTable,

}


impl nsIAccessibleCoordinateType {
}


