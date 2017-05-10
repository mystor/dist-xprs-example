//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcIJSWeakReference.idl
//


#[repr(C)]
pub struct xpcIJSWeakReference {
    vtable: *const xpcIJSWeakReferenceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for xpcIJSWeakReference {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x75767928, 0xecb1, 0x4e6c,
            [0x9f, 0x55, 0xc1, 0x18, 0xb2, 0x97, 0xfc, 0xef])
    }
}

unsafe impl RefCounted for xpcIJSWeakReference {
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
pub trait xpcIJSWeakReferenceCoerce {
    fn coerce_from(v: &xpcIJSWeakReference) -> &Self;
}

impl xpcIJSWeakReferenceCoerce for xpcIJSWeakReference {
    #[inline]
    fn coerce_from(v: &xpcIJSWeakReference) -> &Self {
        v
    }
}

impl xpcIJSWeakReference {
    #[inline]
    pub fn coerce<T: xpcIJSWeakReferenceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for xpcIJSWeakReference {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> xpcIJSWeakReferenceCoerce for T {
    #[inline]
    fn coerce_from(v: &xpcIJSWeakReference) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct xpcIJSWeakReferenceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval get (); */
    /// Unable to call function as its signature contains a non-rust type
    pub get: *const ::libc::c_void,

}


impl xpcIJSWeakReference {
    /* [implicit_jscontext] jsval get (); */


}


