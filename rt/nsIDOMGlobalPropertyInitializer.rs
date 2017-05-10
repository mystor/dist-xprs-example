//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMGlobalPropertyInitializer.idl
//


#[repr(C)]
pub struct nsIDOMGlobalPropertyInitializer {
    vtable: *const nsIDOMGlobalPropertyInitializerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMGlobalPropertyInitializer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5842e275, 0x797f, 0x4afb,
            [0xb7, 0xe0, 0xe2, 0x9f, 0x0c, 0xb3, 0x12, 0xae])
    }
}

unsafe impl RefCounted for nsIDOMGlobalPropertyInitializer {
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
pub trait nsIDOMGlobalPropertyInitializerCoerce {
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self;
}

impl nsIDOMGlobalPropertyInitializerCoerce for nsIDOMGlobalPropertyInitializer {
    #[inline]
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self {
        v
    }
}

impl nsIDOMGlobalPropertyInitializer {
    #[inline]
    pub fn coerce<T: nsIDOMGlobalPropertyInitializerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMGlobalPropertyInitializer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMGlobalPropertyInitializerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGlobalPropertyInitializer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMGlobalPropertyInitializerVTable {
    pub __base: nsISupportsVTable,

    /* jsval init (in mozIDOMWindow window); */
    /// Unable to call function as its signature contains a non-rust type
    pub init: *const ::libc::c_void,

}


impl nsIDOMGlobalPropertyInitializer {
    /* jsval init (in mozIDOMWindow window); */


}


