//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIOSFileConstantsService.idl
//


#[repr(C)]
pub struct nsIOSFileConstantsService {
    vtable: *const nsIOSFileConstantsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOSFileConstantsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd6dd239f, 0x34d6, 0x4b34,
            [0xba, 0xa1, 0xf6, 0x9a, 0xb4, 0xa2, 0x0b, 0xc4])
    }
}

unsafe impl RefCounted for nsIOSFileConstantsService {
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
pub trait nsIOSFileConstantsServiceCoerce {
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self;
}

impl nsIOSFileConstantsServiceCoerce for nsIOSFileConstantsService {
    #[inline]
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self {
        v
    }
}

impl nsIOSFileConstantsService {
    #[inline]
    pub fn coerce<T: nsIOSFileConstantsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOSFileConstantsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOSFileConstantsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOSFileConstantsServiceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void init (); */
    /// Unable to call function as its signature contains a non-rust type
    pub init: *const ::libc::c_void,

}


impl nsIOSFileConstantsService {
    /* [implicit_jscontext] void init (); */


}


