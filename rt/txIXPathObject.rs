//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/txIXPathObject.idl
//


#[repr(C)]
pub struct txIXPathObject {
    vtable: *const txIXPathObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for txIXPathObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x67706346, 0xdece, 0x4c9b,
            [0x9f, 0xc2, 0x57, 0xcf, 0x19, 0x07, 0x10, 0x14])
    }
}

unsafe impl RefCounted for txIXPathObject {
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
pub trait txIXPathObjectCoerce {
    fn coerce_from(v: &txIXPathObject) -> &Self;
}

impl txIXPathObjectCoerce for txIXPathObject {
    #[inline]
    fn coerce_from(v: &txIXPathObject) -> &Self {
        v
    }
}

impl txIXPathObject {
    #[inline]
    pub fn coerce<T: txIXPathObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for txIXPathObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> txIXPathObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &txIXPathObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct txIXPathObjectVTable {
    pub __base: nsISupportsVTable,

    /* [noscript,notxpcom] txAExprResultPtr getResult (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getResult: *const ::libc::c_void,

}


impl txIXPathObject {
    /* [noscript,notxpcom] txAExprResultPtr getResult (); */


}


