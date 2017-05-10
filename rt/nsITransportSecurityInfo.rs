//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransportSecurityInfo.idl
//


#[repr(C)]
pub struct nsITransportSecurityInfo {
    vtable: *const nsITransportSecurityInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransportSecurityInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x216112d3, 0x28bc, 0x4671,
            [0xb0, 0x57, 0xf9, 0x8c, 0xc0, 0x9b, 0xa1, 0xea])
    }
}

unsafe impl RefCounted for nsITransportSecurityInfo {
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
pub trait nsITransportSecurityInfoCoerce {
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self;
}

impl nsITransportSecurityInfoCoerce for nsITransportSecurityInfo {
    #[inline]
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self {
        v
    }
}

impl nsITransportSecurityInfo {
    #[inline]
    pub fn coerce<T: nsITransportSecurityInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransportSecurityInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransportSecurityInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportSecurityInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransportSecurityInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long securityState; */
    pub get_securityState: unsafe extern "C" fn (this: *const nsITransportSecurityInfo, aSecurityState: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute wstring errorMessage; */
    pub get_errorMessage: unsafe extern "C" fn (this: *const nsITransportSecurityInfo, aErrorMessage: *mut *const libc::int16_t) -> nsresult,

    /* readonly attribute long errorCode; */
    pub get_errorCode: unsafe extern "C" fn (this: *const nsITransportSecurityInfo, aErrorCode: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIX509CertList failedCertChain; */
    pub get_failedCertChain: unsafe extern "C" fn (this: *const nsITransportSecurityInfo, aFailedCertChain: *mut *const nsIX509CertList) -> nsresult,

}


impl nsITransportSecurityInfo {
    /* readonly attribute unsigned long securityState; */
    #[inline]
    pub unsafe fn get_securityState(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_securityState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute wstring errorMessage; */
    #[inline]
    pub unsafe fn get_errorMessage(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_errorMessage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long errorCode; */
    #[inline]
    pub unsafe fn get_errorCode(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_errorCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIX509CertList failedCertChain; */
    #[inline]
    pub unsafe fn get_failedCertChain(&self, ) -> Result<Option<RefPtr<nsIX509CertList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_failedCertChain)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


