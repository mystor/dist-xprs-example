//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketTransportService.idl
//


#[repr(C)]
pub struct nsISocketTransportService {
    vtable: *const nsISocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketTransportService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xad56b25f, 0xe6bb, 0x4db3,
            [0x9f, 0x7b, 0x5b, 0x7d, 0xb3, 0x3f, 0xd2, 0xb1])
    }
}

unsafe impl RefCounted for nsISocketTransportService {
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
pub trait nsISocketTransportServiceCoerce {
    fn coerce_from(v: &nsISocketTransportService) -> &Self;
}

impl nsISocketTransportServiceCoerce for nsISocketTransportService {
    #[inline]
    fn coerce_from(v: &nsISocketTransportService) -> &Self {
        v
    }
}

impl nsISocketTransportService {
    #[inline]
    pub fn coerce<T: nsISocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketTransportService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketTransportServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsISocketTransport createTransport ([array, size_is (aTypeCount)] in string aSocketTypes, in unsigned long aTypeCount, in AUTF8String aHost, in long aPort, in nsIProxyInfo aProxyInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub createTransport: *const ::libc::c_void,

    /* nsISocketTransport createUnixDomainTransport (in nsIFile aPath); */
    pub createUnixDomainTransport: unsafe extern "C" fn (this: *const nsISocketTransportService, aPath: *const nsIFile, _retval: *mut *const nsISocketTransport) -> nsresult,

    /* [noscript] void attachSocket (in PRFileDescPtr aFd, in nsASocketHandlerPtr aHandler); */
    /// Unable to call function as its signature contains a non-rust type
    pub attachSocket: *const ::libc::c_void,

    /* [noscript] void notifyWhenCanAttachSocket (in nsIRunnable aEvent); */
    pub notifyWhenCanAttachSocket: unsafe extern "C" fn (this: *const nsISocketTransportService, aEvent: *const nsIRunnable) -> nsresult,

}


impl nsISocketTransportService {
    /* nsISocketTransport createTransport ([array, size_is (aTypeCount)] in string aSocketTypes, in unsigned long aTypeCount, in AUTF8String aHost, in long aPort, in nsIProxyInfo aProxyInfo); */


    /* nsISocketTransport createUnixDomainTransport (in nsIFile aPath); */
    #[inline]
    pub unsafe fn createUnixDomainTransport(&self, aPath: Option<&nsIFile>) -> Result<Option<RefPtr<nsISocketTransport>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createUnixDomainTransport)(self as *const _, aPath.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void attachSocket (in PRFileDescPtr aFd, in nsASocketHandlerPtr aHandler); */


    /* [noscript] void notifyWhenCanAttachSocket (in nsIRunnable aEvent); */
    #[inline]
    pub unsafe fn notifyWhenCanAttachSocket(&self, aEvent: Option<&nsIRunnable>) -> Result<(), nsresult> {

        match ((*self.vtable).notifyWhenCanAttachSocket)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIRoutedSocketTransportService {
    vtable: *const nsIRoutedSocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRoutedSocketTransportService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc5204623, 0x5b58, 0x4a16,
            [0x8b, 0x2e, 0x67, 0xc3, 0x4d, 0xd0, 0x2e, 0x3f])
    }
}

unsafe impl RefCounted for nsIRoutedSocketTransportService {
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
pub trait nsIRoutedSocketTransportServiceCoerce {
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self;
}

impl nsIRoutedSocketTransportServiceCoerce for nsIRoutedSocketTransportService {
    #[inline]
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self {
        v
    }
}

impl nsIRoutedSocketTransportService {
    #[inline]
    pub fn coerce<T: nsIRoutedSocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRoutedSocketTransportService {
    type Target = nsISocketTransportService;
    #[inline]
    fn deref(&self) -> &nsISocketTransportService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISocketTransportServiceCoerce> nsIRoutedSocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRoutedSocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRoutedSocketTransportServiceVTable {
    pub __base: nsISocketTransportServiceVTable,

    /* nsISocketTransport createRoutedTransport ([array, size_is (aTypeCount)] in string aSocketTypes, in unsigned long aTypeCount, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub createRoutedTransport: *const ::libc::c_void,

}


impl nsIRoutedSocketTransportService {
    /* nsISocketTransport createRoutedTransport ([array, size_is (aTypeCount)] in string aSocketTypes, in unsigned long aTypeCount, in AUTF8String aHost, in long aPort, in AUTF8String aHostRoute, in long aPortRoute, in nsIProxyInfo aProxyInfo); */


}


