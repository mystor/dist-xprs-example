//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBadCertListener2.idl
//


#[repr(C)]
pub struct nsIBadCertListener2 {
    vtable: *const nsIBadCertListener2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBadCertListener2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2c3d268c, 0xad82, 0x49f3,
            [0x99, 0xaa, 0xe9, 0xff, 0xdd, 0xd7, 0xa0, 0xdc])
    }
}

unsafe impl RefCounted for nsIBadCertListener2 {
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
pub trait nsIBadCertListener2Coerce {
    fn coerce_from(v: &nsIBadCertListener2) -> &Self;
}

impl nsIBadCertListener2Coerce for nsIBadCertListener2 {
    #[inline]
    fn coerce_from(v: &nsIBadCertListener2) -> &Self {
        v
    }
}

impl nsIBadCertListener2 {
    #[inline]
    pub fn coerce<T: nsIBadCertListener2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBadCertListener2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBadCertListener2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIBadCertListener2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBadCertListener2VTable {
    pub __base: nsISupportsVTable,

    /* boolean notifyCertProblem (in nsIInterfaceRequestor socketInfo, in nsISSLStatus status, in AUTF8String targetSite); */
    pub notifyCertProblem: unsafe extern "C" fn (this: *const nsIBadCertListener2, socketInfo: *const nsIInterfaceRequestor, status: *const nsISSLStatus, targetSite: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl nsIBadCertListener2 {
    /* boolean notifyCertProblem (in nsIInterfaceRequestor socketInfo, in nsISSLStatus status, in AUTF8String targetSite); */
    #[inline]
    pub unsafe fn notifyCertProblem(&self, socketInfo: Option<&nsIInterfaceRequestor>, status: Option<&nsISSLStatus>, targetSite: &[u8]) -> Result<bool, nsresult> {
        let targetSite = nsCString::from(targetSite);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).notifyCertProblem)(self as *const _, socketInfo.map_or(::std::ptr::null(), |x| x as *const _), status.map_or(::std::ptr::null(), |x| x as *const _), &*targetSite, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


