//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIReadConfig.idl
//


#[repr(C)]
pub struct nsIReadConfig {
    vtable: *const nsIReadConfigVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIReadConfig {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xba5bc4c6, 0x1dd1, 0x11b2,
            [0xbb, 0x89, 0xb8, 0x44, 0xc6, 0xec, 0x03, 0x39])
    }
}

unsafe impl RefCounted for nsIReadConfig {
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
pub trait nsIReadConfigCoerce {
    fn coerce_from(v: &nsIReadConfig) -> &Self;
}

impl nsIReadConfigCoerce for nsIReadConfig {
    #[inline]
    fn coerce_from(v: &nsIReadConfig) -> &Self {
        v
    }
}

impl nsIReadConfig {
    #[inline]
    pub fn coerce<T: nsIReadConfigCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIReadConfig {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIReadConfigCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIReadConfig) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIReadConfigVTable {
    pub __base: nsISupportsVTable,

}


impl nsIReadConfig {
}


