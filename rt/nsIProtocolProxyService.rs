//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyService.idl
//


pub mod nsIProtocolProxyService_consts {
    pub const RESOLVE_PREFER_SOCKS_PROXY: i64 = 2;
    pub const RESOLVE_IGNORE_URI_SCHEME: i64 = 4;
    pub const RESOLVE_PREFER_HTTPS_PROXY: i64 = 12;
    pub const RESOLVE_ALWAYS_TUNNEL: i64 = 16;
    pub const PROXYCONFIG_DIRECT: i64 = 0;
    pub const PROXYCONFIG_MANUAL: i64 = 1;
    pub const PROXYCONFIG_PAC: i64 = 2;
    pub const PROXYCONFIG_WPAD: i64 = 4;
    pub const PROXYCONFIG_SYSTEM: i64 = 5;
}


#[repr(C)]
pub struct nsIProtocolProxyService {
    vtable: *const nsIProtocolProxyServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolProxyService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xef57c8b6, 0xe09d, 0x4cd4,
            [0x92, 0x22, 0x2a, 0x5d, 0x24, 0x02, 0xe1, 0x5d])
    }
}

unsafe impl RefCounted for nsIProtocolProxyService {
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
pub trait nsIProtocolProxyServiceCoerce {
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self;
}

impl nsIProtocolProxyServiceCoerce for nsIProtocolProxyService {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self {
        v
    }
}

impl nsIProtocolProxyService {
    #[inline]
    pub fn coerce<T: nsIProtocolProxyServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolProxyService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolProxyServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolProxyServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
    pub asyncResolve: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aChannelOrURI: *const nsISupports, aFlags: libc::uint32_t, aCallback: *const nsIProtocolProxyCallback, _retval: *mut *const nsICancelable) -> nsresult,

    /* nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    pub newProxyInfo: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aType: *const nsACString, aHost: *const nsACString, aPort: libc::int32_t, aFlags: libc::uint32_t, aFailoverTimeout: libc::uint32_t, aFailoverProxy: *const nsIProxyInfo, _retval: *mut *const nsIProxyInfo) -> nsresult,

    /* nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    pub newProxyInfoWithAuth: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aType: *const nsACString, aHost: *const nsACString, aPort: libc::int32_t, aUsername: *const nsACString, aPassword: *const nsACString, aFlags: libc::uint32_t, aFailoverTimeout: libc::uint32_t, aFailoverProxy: *const nsIProxyInfo, _retval: *mut *const nsIProxyInfo) -> nsresult,

    /* nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason); */
    pub getFailoverForProxy: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aProxyInfo: *const nsIProxyInfo, aURI: *const nsIURI, aReason: nsresult, _retval: *mut *const nsIProxyInfo) -> nsresult,

    /* void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition); */
    pub registerFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyFilter, aPosition: libc::uint32_t) -> nsresult,

    /* void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition); */
    pub registerChannelFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyChannelFilter, aPosition: libc::uint32_t) -> nsresult,

    /* void unregisterFilter (in nsIProtocolProxyFilter aFilter); */
    pub unregisterFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyFilter) -> nsresult,

    /* void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter); */
    pub unregisterChannelFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aFilter: *const nsIProtocolProxyChannelFilter) -> nsresult,

    /* readonly attribute unsigned long proxyConfigType; */
    pub get_proxyConfigType: unsafe extern "C" fn (this: *const nsIProtocolProxyService, aProxyConfigType: *mut libc::uint32_t) -> nsresult,

}


impl nsIProtocolProxyService {
    /* nsICancelable asyncResolve (in nsISupports aChannelOrURI, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
    #[inline]
    pub unsafe fn asyncResolve(&self, aChannelOrURI: Option<&nsISupports>, aFlags: libc::uint32_t, aCallback: Option<&nsIProtocolProxyCallback>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).asyncResolve)(self as *const _, aChannelOrURI.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIProxyInfo newProxyInfo (in ACString aType, in AUTF8String aHost, in long aPort, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    #[inline]
    pub unsafe fn newProxyInfo(&self, aType: &[u8], aHost: &[u8], aPort: libc::int32_t, aFlags: libc::uint32_t, aFailoverTimeout: libc::uint32_t, aFailoverProxy: Option<&nsIProxyInfo>) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let aType = nsCString::from(aType);
        let aHost = nsCString::from(aHost);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newProxyInfo)(self as *const _, &*aType, &*aHost, aPort, aFlags, aFailoverTimeout, aFailoverProxy.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIProxyInfo newProxyInfoWithAuth (in ACString aType, in AUTF8String aHost, in long aPort, in ACString aUsername, in ACString aPassword, in unsigned long aFlags, in unsigned long aFailoverTimeout, in nsIProxyInfo aFailoverProxy); */
    #[inline]
    pub unsafe fn newProxyInfoWithAuth(&self, aType: &[u8], aHost: &[u8], aPort: libc::int32_t, aUsername: &[u8], aPassword: &[u8], aFlags: libc::uint32_t, aFailoverTimeout: libc::uint32_t, aFailoverProxy: Option<&nsIProxyInfo>) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let aType = nsCString::from(aType);
        let aHost = nsCString::from(aHost);
        let aUsername = nsCString::from(aUsername);
        let aPassword = nsCString::from(aPassword);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newProxyInfoWithAuth)(self as *const _, &*aType, &*aHost, aPort, &*aUsername, &*aPassword, aFlags, aFailoverTimeout, aFailoverProxy.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIProxyInfo getFailoverForProxy (in nsIProxyInfo aProxyInfo, in nsIURI aURI, in nsresult aReason); */
    #[inline]
    pub unsafe fn getFailoverForProxy(&self, aProxyInfo: Option<&nsIProxyInfo>, aURI: Option<&nsIURI>, aReason: nsresult) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFailoverForProxy)(self as *const _, aProxyInfo.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), aReason, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void registerFilter (in nsIProtocolProxyFilter aFilter, in unsigned long aPosition); */
    #[inline]
    pub unsafe fn registerFilter(&self, aFilter: Option<&nsIProtocolProxyFilter>, aPosition: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).registerFilter)(self as *const _, aFilter.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerChannelFilter (in nsIProtocolProxyChannelFilter aFilter, in unsigned long aPosition); */
    #[inline]
    pub unsafe fn registerChannelFilter(&self, aFilter: Option<&nsIProtocolProxyChannelFilter>, aPosition: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).registerChannelFilter)(self as *const _, aFilter.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterFilter (in nsIProtocolProxyFilter aFilter); */
    #[inline]
    pub unsafe fn unregisterFilter(&self, aFilter: Option<&nsIProtocolProxyFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterFilter)(self as *const _, aFilter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterChannelFilter (in nsIProtocolProxyChannelFilter aFilter); */
    #[inline]
    pub unsafe fn unregisterChannelFilter(&self, aFilter: Option<&nsIProtocolProxyChannelFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterChannelFilter)(self as *const _, aFilter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long proxyConfigType; */
    #[inline]
    pub unsafe fn get_proxyConfigType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_proxyConfigType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


