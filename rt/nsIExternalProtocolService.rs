//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalProtocolService.idl
//


#[repr(C)]
pub struct nsIExternalProtocolService {
    vtable: *const nsIExternalProtocolServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExternalProtocolService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x70f93b7a, 0x3ec6, 0x4bcb,
            [0xb0, 0x93, 0x92, 0xd9, 0x98, 0x4c, 0x9f, 0x83])
    }
}

unsafe impl RefCounted for nsIExternalProtocolService {
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
pub trait nsIExternalProtocolServiceCoerce {
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self;
}

impl nsIExternalProtocolServiceCoerce for nsIExternalProtocolService {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self {
        v
    }
}

impl nsIExternalProtocolService {
    #[inline]
    pub fn coerce<T: nsIExternalProtocolServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExternalProtocolService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExternalProtocolServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExternalProtocolServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
    pub externalProtocolHandlerExists: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* boolean isExposedProtocol (in string aProtocolScheme); */
    pub isExposedProtocol: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
    pub getProtocolHandlerInfo: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const nsACString, _retval: *mut *const nsIHandlerInfo) -> nsresult,

    /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
    pub getProtocolHandlerInfoFromOS: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const nsACString, aFound: *mut bool, _retval: *mut *const nsIHandlerInfo) -> nsresult,

    /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
    pub setProtocolHandlerDefaults: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aHandlerInfo: *const nsIHandlerInfo, aOSHandlerExists: bool) -> nsresult,

    /* [deprecated] void loadUrl (in nsIURI aURL); */
    pub loadUrl: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aURL: *const nsIURI) -> nsresult,

    /* void loadURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub loadURI: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aURI: *const nsIURI, aWindowContext: *const nsIInterfaceRequestor) -> nsresult,

    /* AString getApplicationDescription (in AUTF8String aScheme); */
    pub getApplicationDescription: unsafe extern "C" fn (this: *const nsIExternalProtocolService, aScheme: *const nsACString, _retval: *mut nsAString) -> nsresult,

}


impl nsIExternalProtocolService {
    /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
    #[inline]
    pub unsafe fn externalProtocolHandlerExists(&self, aProtocolScheme: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).externalProtocolHandlerExists)(self as *const _, aProtocolScheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isExposedProtocol (in string aProtocolScheme); */
    #[inline]
    pub unsafe fn isExposedProtocol(&self, aProtocolScheme: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isExposedProtocol)(self as *const _, aProtocolScheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
    #[inline]
    pub unsafe fn getProtocolHandlerInfo(&self, aProtocolScheme: &[u8]) -> Result<Option<RefPtr<nsIHandlerInfo>>, nsresult> {
        let aProtocolScheme = nsCString::from(aProtocolScheme);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProtocolHandlerInfo)(self as *const _, &*aProtocolScheme, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
    #[inline]
    pub unsafe fn getProtocolHandlerInfoFromOS(&self, aProtocolScheme: &[u8]) -> Result<(bool, Option<RefPtr<nsIHandlerInfo>>), nsresult> {
        let aProtocolScheme = nsCString::from(aProtocolScheme);
        let mut aFound: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProtocolHandlerInfoFromOS)(self as *const _, &*aProtocolScheme, &mut aFound as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFound, _retval.refptr()))
    }

    /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
    #[inline]
    pub unsafe fn setProtocolHandlerDefaults(&self, aHandlerInfo: Option<&nsIHandlerInfo>, aOSHandlerExists: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setProtocolHandlerDefaults)(self as *const _, aHandlerInfo.map_or(::std::ptr::null(), |x| x as *const _), aOSHandlerExists) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void loadUrl (in nsIURI aURL); */
    #[inline]
    pub unsafe fn loadUrl(&self, aURL: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).loadUrl)(self as *const _, aURL.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadURI (in nsIURI aURI, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn loadURI(&self, aURI: Option<&nsIURI>, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).loadURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getApplicationDescription (in AUTF8String aScheme); */
    #[inline]
    pub unsafe fn getApplicationDescription(&self, aScheme: &[u8]) -> Result<nsString, nsresult> {
        let aScheme = nsCString::from(aScheme);
        let mut _retval = nsString::new();
        match ((*self.vtable).getApplicationDescription)(self as *const _, &*aScheme, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


