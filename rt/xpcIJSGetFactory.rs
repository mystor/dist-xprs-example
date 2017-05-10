//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpcIJSGetFactory.idl
//


#[repr(C)]
pub struct xpcIJSGetFactory {
    vtable: *const xpcIJSGetFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for xpcIJSGetFactory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3fe0c205, 0xd75b, 0x4cac,
            [0x93, 0x47, 0xd2, 0xb8, 0x55, 0x05, 0x01, 0x43])
    }
}

unsafe impl RefCounted for xpcIJSGetFactory {
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
pub trait xpcIJSGetFactoryCoerce {
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self;
}

impl xpcIJSGetFactoryCoerce for xpcIJSGetFactory {
    #[inline]
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self {
        v
    }
}

impl xpcIJSGetFactory {
    #[inline]
    pub fn coerce<T: xpcIJSGetFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for xpcIJSGetFactory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> xpcIJSGetFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct xpcIJSGetFactoryVTable {
    pub __base: nsISupportsVTable,

    /* nsIFactory get (in nsCIDRef aCID); */
    pub get: unsafe extern "C" fn (this: *const xpcIJSGetFactory, aCID: *const nsCID, _retval: *mut *const nsIFactory) -> nsresult,

}


impl xpcIJSGetFactory {
    /* nsIFactory get (in nsCIDRef aCID); */
    #[inline]
    pub unsafe fn get(&self, aCID: *const nsCID) -> Result<Option<RefPtr<nsIFactory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get)(self as *const _, aCID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


