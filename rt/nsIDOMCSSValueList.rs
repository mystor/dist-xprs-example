//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSValueList.idl
//


#[repr(C)]
pub struct nsIDOMCSSValueList {
    vtable: *const nsIDOMCSSValueListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSValueList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92364ed1, 0xe364, 0x4175,
            [0xbc, 0x52, 0xc2, 0xfe, 0x96, 0x71, 0xcb, 0xc7])
    }
}

unsafe impl RefCounted for nsIDOMCSSValueList {
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
pub trait nsIDOMCSSValueListCoerce {
    fn coerce_from(v: &nsIDOMCSSValueList) -> &Self;
}

impl nsIDOMCSSValueListCoerce for nsIDOMCSSValueList {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSValueList) -> &Self {
        v
    }
}

impl nsIDOMCSSValueList {
    #[inline]
    pub fn coerce<T: nsIDOMCSSValueListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSValueList {
    type Target = nsIDOMCSSValue;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSValue {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSValueCoerce> nsIDOMCSSValueListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSValueList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSValueListVTable {
    pub __base: nsIDOMCSSValueVTable,

}


impl nsIDOMCSSValueList {
}


