//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityReporter.idl
//


#[repr(C)]
pub struct nsISecurityReporter {
    vtable: *const nsISecurityReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecurityReporter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a997c9a, 0xbea1, 0x11e5,
            [0xa1, 0xfa, 0xbe, 0x6a, 0xbc, 0x8e, 0x7f, 0x8b])
    }
}

unsafe impl RefCounted for nsISecurityReporter {
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
pub trait nsISecurityReporterCoerce {
    fn coerce_from(v: &nsISecurityReporter) -> &Self;
}

impl nsISecurityReporterCoerce for nsISecurityReporter {
    #[inline]
    fn coerce_from(v: &nsISecurityReporter) -> &Self {
        v
    }
}

impl nsISecurityReporter {
    #[inline]
    pub fn coerce<T: nsISecurityReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecurityReporter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecurityReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityReporter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecurityReporterVTable {
    pub __base: nsISupportsVTable,

    /* void reportTLSError (in nsITransportSecurityInfo aSecurityInfo, in AUTF8String aHostname, in long aPort); */
    pub reportTLSError: unsafe extern "C" fn (this: *const nsISecurityReporter, aSecurityInfo: *const nsITransportSecurityInfo, aHostname: *const nsACString, aPort: libc::int32_t) -> nsresult,

}


impl nsISecurityReporter {
    /* void reportTLSError (in nsITransportSecurityInfo aSecurityInfo, in AUTF8String aHostname, in long aPort); */
    #[inline]
    pub unsafe fn reportTLSError(&self, aSecurityInfo: Option<&nsITransportSecurityInfo>, aHostname: &[u8], aPort: libc::int32_t) -> Result<(), nsresult> {
        let aHostname = nsCString::from(aHostname);
        match ((*self.vtable).reportTLSError)(self as *const _, aSecurityInfo.map_or(::std::ptr::null(), |x| x as *const _), &*aHostname, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


