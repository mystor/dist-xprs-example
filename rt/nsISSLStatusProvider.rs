//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISSLStatusProvider.idl
//


#[repr(C)]
pub struct nsISSLStatusProvider {
    vtable: *const nsISSLStatusProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISSLStatusProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x179b1ab1, 0x0950, 0x4427,
            [0x95, 0x56, 0x6f, 0x49, 0x6d, 0xc4, 0xa2, 0x7f])
    }
}

unsafe impl RefCounted for nsISSLStatusProvider {
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
pub trait nsISSLStatusProviderCoerce {
    fn coerce_from(v: &nsISSLStatusProvider) -> &Self;
}

impl nsISSLStatusProviderCoerce for nsISSLStatusProvider {
    #[inline]
    fn coerce_from(v: &nsISSLStatusProvider) -> &Self {
        v
    }
}

impl nsISSLStatusProvider {
    #[inline]
    pub fn coerce<T: nsISSLStatusProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISSLStatusProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISSLStatusProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISSLStatusProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISSLStatusProviderVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISSLStatus SSLStatus; */
    pub get_SSLStatus: unsafe extern "C" fn (this: *const nsISSLStatusProvider, aSSLStatus: *mut *const nsISSLStatus) -> nsresult,

}


impl nsISSLStatusProvider {
    /* readonly attribute nsISSLStatus SSLStatus; */
    #[inline]
    pub unsafe fn get_SSLStatus(&self, ) -> Result<Option<RefPtr<nsISSLStatus>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_SSLStatus)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


