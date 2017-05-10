//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocCharset.idl
//


#[repr(C)]
pub struct nsIDocCharset {
    vtable: *const nsIDocCharsetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocCharset {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc3faaf6e, 0x40f0, 0x11e1,
            [0x95, 0xfc, 0x6c, 0x62, 0x6d, 0x69, 0x67, 0x5c])
    }
}

unsafe impl RefCounted for nsIDocCharset {
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
pub trait nsIDocCharsetCoerce {
    fn coerce_from(v: &nsIDocCharset) -> &Self;
}

impl nsIDocCharsetCoerce for nsIDocCharset {
    #[inline]
    fn coerce_from(v: &nsIDocCharset) -> &Self {
        v
    }
}

impl nsIDocCharset {
    #[inline]
    pub fn coerce<T: nsIDocCharsetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocCharset {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocCharsetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocCharset) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocCharsetVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDocCharset {
}


