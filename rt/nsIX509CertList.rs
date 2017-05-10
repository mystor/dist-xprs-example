//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509CertList.idl
//


#[repr(C)]
pub struct nsIX509CertList {
    vtable: *const nsIX509CertListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIX509CertList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae74cda5, 0xcd2f, 0x473f,
            [0x96, 0xf5, 0xf0, 0xb7, 0xff, 0xf6, 0x2c, 0x68])
    }
}

unsafe impl RefCounted for nsIX509CertList {
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
pub trait nsIX509CertListCoerce {
    fn coerce_from(v: &nsIX509CertList) -> &Self;
}

impl nsIX509CertListCoerce for nsIX509CertList {
    #[inline]
    fn coerce_from(v: &nsIX509CertList) -> &Self {
        v
    }
}

impl nsIX509CertList {
    #[inline]
    pub fn coerce<T: nsIX509CertListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIX509CertList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIX509CertListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509CertList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIX509CertListVTable {
    pub __base: nsISupportsVTable,

    /* void addCert (in nsIX509Cert cert); */
    pub addCert: unsafe extern "C" fn (this: *const nsIX509CertList, cert: *const nsIX509Cert) -> nsresult,

    /* void deleteCert (in nsIX509Cert cert); */
    pub deleteCert: unsafe extern "C" fn (this: *const nsIX509CertList, cert: *const nsIX509Cert) -> nsresult,

    /* nsISimpleEnumerator getEnumerator (); */
    pub getEnumerator: unsafe extern "C" fn (this: *const nsIX509CertList, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* [noscript,notxpcom] CERTCertListPtr getRawCertList (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getRawCertList: *const ::libc::c_void,

    /* boolean equals (in nsIX509CertList other); */
    pub equals: unsafe extern "C" fn (this: *const nsIX509CertList, other: *const nsIX509CertList, _retval: *mut bool) -> nsresult,

}


impl nsIX509CertList {
    /* void addCert (in nsIX509Cert cert); */
    #[inline]
    pub unsafe fn addCert(&self, cert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).addCert)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteCert (in nsIX509Cert cert); */
    #[inline]
    pub unsafe fn deleteCert(&self, cert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).deleteCert)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getEnumerator (); */
    #[inline]
    pub unsafe fn getEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,notxpcom] CERTCertListPtr getRawCertList (); */


    /* boolean equals (in nsIX509CertList other); */
    #[inline]
    pub unsafe fn equals(&self, other: Option<&nsIX509CertList>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


