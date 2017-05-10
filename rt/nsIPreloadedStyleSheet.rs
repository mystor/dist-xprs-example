//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPreloadedStyleSheet.idl
//


#[repr(C)]
pub struct nsIPreloadedStyleSheet {
    vtable: *const nsIPreloadedStyleSheetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPreloadedStyleSheet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2e2a84d0, 0x2102, 0x4b9e,
            [0x9b, 0x78, 0x16, 0x70, 0x62, 0x3a, 0x58, 0x2d])
    }
}

unsafe impl RefCounted for nsIPreloadedStyleSheet {
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
pub trait nsIPreloadedStyleSheetCoerce {
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self;
}

impl nsIPreloadedStyleSheetCoerce for nsIPreloadedStyleSheet {
    #[inline]
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self {
        v
    }
}

impl nsIPreloadedStyleSheet {
    #[inline]
    pub fn coerce<T: nsIPreloadedStyleSheetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPreloadedStyleSheet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPreloadedStyleSheetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPreloadedStyleSheet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPreloadedStyleSheetVTable {
    pub __base: nsISupportsVTable,

}


impl nsIPreloadedStyleSheet {
}


