//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannelInternal.idl
//


#[repr(C)]
pub struct nsIHttpUpgradeListener {
    vtable: *const nsIHttpUpgradeListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpUpgradeListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5b515449, 0xab64, 0x4dba,
            [0xb3, 0xcd, 0xda, 0x8f, 0xc2, 0xf8, 0x30, 0x64])
    }
}

unsafe impl RefCounted for nsIHttpUpgradeListener {
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
pub trait nsIHttpUpgradeListenerCoerce {
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self;
}

impl nsIHttpUpgradeListenerCoerce for nsIHttpUpgradeListener {
    #[inline]
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self {
        v
    }
}

impl nsIHttpUpgradeListener {
    #[inline]
    pub fn coerce<T: nsIHttpUpgradeListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpUpgradeListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpUpgradeListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpUpgradeListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpUpgradeListenerVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
    pub onTransportAvailable: unsafe extern "C" fn (this: *const nsIHttpUpgradeListener, aTransport: *const nsISocketTransport, aSocketIn: *const nsIAsyncInputStream, aSocketOut: *const nsIAsyncOutputStream) -> nsresult,

}


impl nsIHttpUpgradeListener {
    /* [must_use] void onTransportAvailable (in nsISocketTransport aTransport, in nsIAsyncInputStream aSocketIn, in nsIAsyncOutputStream aSocketOut); */
    #[inline]
    pub unsafe fn onTransportAvailable(&self, aTransport: Option<&nsISocketTransport>, aSocketIn: Option<&nsIAsyncInputStream>, aSocketOut: Option<&nsIAsyncOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).onTransportAvailable)(self as *const _, aTransport.map_or(::std::ptr::null(), |x| x as *const _), aSocketIn.map_or(::std::ptr::null(), |x| x as *const _), aSocketOut.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIHttpChannelInternal_consts {
    pub const THIRD_PARTY_FORCE_ALLOW: i64 = 1;
    pub const CORS_MODE_SAME_ORIGIN: i64 = 0;
    pub const CORS_MODE_NO_CORS: i64 = 1;
    pub const CORS_MODE_CORS: i64 = 2;
    pub const CORS_MODE_NAVIGATE: i64 = 3;
    pub const REDIRECT_MODE_FOLLOW: i64 = 0;
    pub const REDIRECT_MODE_ERROR: i64 = 1;
    pub const REDIRECT_MODE_MANUAL: i64 = 2;
    pub const FETCH_CACHE_MODE_DEFAULT: i64 = 0;
    pub const FETCH_CACHE_MODE_NO_STORE: i64 = 1;
    pub const FETCH_CACHE_MODE_RELOAD: i64 = 2;
    pub const FETCH_CACHE_MODE_NO_CACHE: i64 = 3;
    pub const FETCH_CACHE_MODE_FORCE_CACHE: i64 = 4;
    pub const FETCH_CACHE_MODE_ONLY_IF_CACHED: i64 = 5;
}


#[repr(C)]
pub struct nsIHttpChannelInternal {
    vtable: *const nsIHttpChannelInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpChannelInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4e28263d, 0x1e03, 0x46f4,
            [0xaa, 0x5c, 0x95, 0x12, 0xf9, 0x19, 0x57, 0xf9])
    }
}

unsafe impl RefCounted for nsIHttpChannelInternal {
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
pub trait nsIHttpChannelInternalCoerce {
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self;
}

impl nsIHttpChannelInternalCoerce for nsIHttpChannelInternal {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self {
        v
    }
}

impl nsIHttpChannelInternal {
    #[inline]
    pub fn coerce<T: nsIHttpChannelInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpChannelInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpChannelInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpChannelInternalVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] attribute nsIURI documentURI; */
    pub get_documentURI: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aDocumentURI: *mut *const nsIURI) -> nsresult,
    pub set_documentURI: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aDocumentURI: *const nsIURI) -> nsresult,

    /* [must_use] void getRequestVersion (out unsigned long major, out unsigned long minor); */
    pub getRequestVersion: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, major: *mut libc::uint32_t, minor: *mut libc::uint32_t) -> nsresult,

    /* [must_use] void getResponseVersion (out unsigned long major, out unsigned long minor); */
    pub getResponseVersion: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, major: *mut libc::uint32_t, minor: *mut libc::uint32_t) -> nsresult,

    /* [must_use,noscript] void takeAllSecurityMessages (in securityMessagesArray aMessages); */
    /// Unable to call function as its signature contains a non-rust type
    pub takeAllSecurityMessages: *const ::libc::c_void,

    /* [must_use] void setCookie (in string aCookieHeader); */
    pub setCookie: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCookieHeader: *const libc::c_char) -> nsresult,

    /* [must_use] void setupFallbackChannel (in string aFallbackKey); */
    pub setupFallbackChannel: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aFallbackKey: *const libc::c_char) -> nsresult,

    /* [must_use] attribute unsigned long thirdPartyFlags; */
    pub get_thirdPartyFlags: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aThirdPartyFlags: *mut libc::uint32_t) -> nsresult,
    pub set_thirdPartyFlags: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aThirdPartyFlags: libc::uint32_t) -> nsresult,

    /* [must_use] attribute boolean forceAllowThirdPartyCookie; */
    pub get_forceAllowThirdPartyCookie: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aForceAllowThirdPartyCookie: *mut bool) -> nsresult,
    pub set_forceAllowThirdPartyCookie: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aForceAllowThirdPartyCookie: bool) -> nsresult,

    /* [must_use] readonly attribute boolean canceled; */
    pub get_canceled: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCanceled: *mut bool) -> nsresult,

    /* [must_use] attribute boolean channelIsForDownload; */
    pub get_channelIsForDownload: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aChannelIsForDownload: *mut bool) -> nsresult,
    pub set_channelIsForDownload: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aChannelIsForDownload: bool) -> nsresult,

    /* [must_use] readonly attribute AUTF8String localAddress; */
    pub get_localAddress: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aLocalAddress: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute int32_t localPort; */
    pub get_localPort: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aLocalPort: *mut int32_t) -> nsresult,

    /* [must_use] readonly attribute AUTF8String remoteAddress; */
    pub get_remoteAddress: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aRemoteAddress: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute int32_t remotePort; */
    pub get_remotePort: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aRemotePort: *mut int32_t) -> nsresult,

    /* [must_use,noscript] void setCacheKeysRedirectChain (in StringArray cacheKeys); */
    /// Unable to call function as its signature contains a non-rust type
    pub setCacheKeysRedirectChain: *const ::libc::c_void,

    /* [must_use] void HTTPUpgrade (in ACString aProtocolName, in nsIHttpUpgradeListener aListener); */
    pub HTTPUpgrade: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aProtocolName: *const nsACString, aListener: *const nsIHttpUpgradeListener) -> nsresult,

    /* [must_use] attribute boolean allowSpdy; */
    pub get_allowSpdy: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aAllowSpdy: *mut bool) -> nsresult,
    pub set_allowSpdy: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aAllowSpdy: bool) -> nsresult,

    /* [must_use] attribute boolean responseTimeoutEnabled; */
    pub get_responseTimeoutEnabled: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aResponseTimeoutEnabled: *mut bool) -> nsresult,
    pub set_responseTimeoutEnabled: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aResponseTimeoutEnabled: bool) -> nsresult,

    /* [must_use] attribute unsigned long initialRwin; */
    pub get_initialRwin: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aInitialRwin: *mut libc::uint32_t) -> nsresult,
    pub set_initialRwin: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aInitialRwin: libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute nsIURI apiRedirectToURI; */
    pub get_apiRedirectToURI: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aApiRedirectToURI: *mut *const nsIURI) -> nsresult,

    /* [must_use] attribute boolean allowAltSvc; */
    pub get_allowAltSvc: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aAllowAltSvc: *mut bool) -> nsresult,
    pub set_allowAltSvc: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aAllowAltSvc: bool) -> nsresult,

    /* [must_use] attribute boolean beConservative; */
    pub get_beConservative: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aBeConservative: *mut bool) -> nsresult,
    pub set_beConservative: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aBeConservative: bool) -> nsresult,

    /* [must_use] readonly attribute PRTime lastModifiedTime; */
    pub get_lastModifiedTime: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aLastModifiedTime: *mut PRTime) -> nsresult,

    /* [must_use] void forceIntercepted (in uint64_t aInterceptionID); */
    pub forceIntercepted: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aInterceptionID: uint64_t) -> nsresult,

    /* [must_use] readonly attribute boolean responseSynthesized; */
    pub get_responseSynthesized: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aResponseSynthesized: *mut bool) -> nsresult,

    /* [must_use] attribute boolean corsIncludeCredentials; */
    pub get_corsIncludeCredentials: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCorsIncludeCredentials: *mut bool) -> nsresult,
    pub set_corsIncludeCredentials: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCorsIncludeCredentials: bool) -> nsresult,

    /* [must_use] attribute unsigned long corsMode; */
    pub get_corsMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCorsMode: *mut libc::uint32_t) -> nsresult,
    pub set_corsMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aCorsMode: libc::uint32_t) -> nsresult,

    /* [must_use] attribute unsigned long redirectMode; */
    pub get_redirectMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aRedirectMode: *mut libc::uint32_t) -> nsresult,
    pub set_redirectMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aRedirectMode: libc::uint32_t) -> nsresult,

    /* [must_use] attribute unsigned long fetchCacheMode; */
    pub get_fetchCacheMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aFetchCacheMode: *mut libc::uint32_t) -> nsresult,
    pub set_fetchCacheMode: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aFetchCacheMode: libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute nsIURI topWindowURI; */
    pub get_topWindowURI: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aTopWindowURI: *mut *const nsIURI) -> nsresult,

    /* [must_use] attribute ACString networkInterfaceId; */
    pub get_networkInterfaceId: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aNetworkInterfaceId: *mut nsACString) -> nsresult,
    pub set_networkInterfaceId: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aNetworkInterfaceId: *const nsACString) -> nsresult,

    /* [must_use] readonly attribute nsIURI proxyURI; */
    pub get_proxyURI: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aProxyURI: *mut *const nsIURI) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void setCorsPreflightParameters (in StringArrayRef unsafeHeaders); */
    /// Unable to call function as its signature contains a non-rust type
    pub setCorsPreflightParameters: *const ::libc::c_void,

    /* [infallible] attribute boolean blockAuthPrompt; */
    pub get_blockAuthPrompt: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aBlockAuthPrompt: *mut bool) -> nsresult,
    pub set_blockAuthPrompt: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aBlockAuthPrompt: bool) -> nsresult,

    /* [must_use] attribute AString integrityMetadata; */
    pub get_integrityMetadata: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aIntegrityMetadata: *mut nsAString) -> nsresult,
    pub set_integrityMetadata: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aIntegrityMetadata: *const nsAString) -> nsresult,

    /* [must_use] readonly attribute ACString connectionInfoHashKey; */
    pub get_connectionInfoHashKey: unsafe extern "C" fn (this: *const nsIHttpChannelInternal, aConnectionInfoHashKey: *mut nsACString) -> nsresult,

}


impl nsIHttpChannelInternal {
    /* [must_use] attribute nsIURI documentURI; */
    #[inline]
    pub unsafe fn get_documentURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_documentURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_documentURI(&self, aDocumentURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_documentURI)(self as *const _, aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void getRequestVersion (out unsigned long major, out unsigned long minor); */
    #[inline]
    pub unsafe fn getRequestVersion(&self, ) -> Result<(libc::uint32_t, libc::uint32_t), nsresult> {
        let mut major: libc::uint32_t = ::std::mem::zeroed();
        let mut minor: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRequestVersion)(self as *const _, &mut major as *mut _, &mut minor as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((major, minor))
    }

    /* [must_use] void getResponseVersion (out unsigned long major, out unsigned long minor); */
    #[inline]
    pub unsafe fn getResponseVersion(&self, ) -> Result<(libc::uint32_t, libc::uint32_t), nsresult> {
        let mut major: libc::uint32_t = ::std::mem::zeroed();
        let mut minor: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getResponseVersion)(self as *const _, &mut major as *mut _, &mut minor as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((major, minor))
    }

    /* [must_use,noscript] void takeAllSecurityMessages (in securityMessagesArray aMessages); */


    /* [must_use] void setCookie (in string aCookieHeader); */
    #[inline]
    pub unsafe fn setCookie(&self, aCookieHeader: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setCookie)(self as *const _, aCookieHeader) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void setupFallbackChannel (in string aFallbackKey); */
    #[inline]
    pub unsafe fn setupFallbackChannel(&self, aFallbackKey: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setupFallbackChannel)(self as *const _, aFallbackKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long thirdPartyFlags; */
    #[inline]
    pub unsafe fn get_thirdPartyFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_thirdPartyFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_thirdPartyFlags(&self, aThirdPartyFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_thirdPartyFlags)(self as *const _, aThirdPartyFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean forceAllowThirdPartyCookie; */
    #[inline]
    pub unsafe fn get_forceAllowThirdPartyCookie(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceAllowThirdPartyCookie)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_forceAllowThirdPartyCookie(&self, aForceAllowThirdPartyCookie: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_forceAllowThirdPartyCookie)(self as *const _, aForceAllowThirdPartyCookie) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute boolean canceled; */
    #[inline]
    pub unsafe fn get_canceled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canceled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] attribute boolean channelIsForDownload; */
    #[inline]
    pub unsafe fn get_channelIsForDownload(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_channelIsForDownload)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_channelIsForDownload(&self, aChannelIsForDownload: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_channelIsForDownload)(self as *const _, aChannelIsForDownload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute AUTF8String localAddress; */
    #[inline]
    pub unsafe fn get_localAddress(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_localAddress)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int32_t localPort; */
    #[inline]
    pub unsafe fn get_localPort(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_localPort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute AUTF8String remoteAddress; */
    #[inline]
    pub unsafe fn get_remoteAddress(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_remoteAddress)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int32_t remotePort; */
    #[inline]
    pub unsafe fn get_remotePort(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_remotePort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use,noscript] void setCacheKeysRedirectChain (in StringArray cacheKeys); */


    /* [must_use] void HTTPUpgrade (in ACString aProtocolName, in nsIHttpUpgradeListener aListener); */
    #[inline]
    pub unsafe fn HTTPUpgrade(&self, aProtocolName: &[u8], aListener: Option<&nsIHttpUpgradeListener>) -> Result<(), nsresult> {
        let aProtocolName = nsCString::from(aProtocolName);
        match ((*self.vtable).HTTPUpgrade)(self as *const _, &*aProtocolName, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean allowSpdy; */
    #[inline]
    pub unsafe fn get_allowSpdy(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowSpdy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowSpdy(&self, aAllowSpdy: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowSpdy)(self as *const _, aAllowSpdy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean responseTimeoutEnabled; */
    #[inline]
    pub unsafe fn get_responseTimeoutEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_responseTimeoutEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_responseTimeoutEnabled(&self, aResponseTimeoutEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_responseTimeoutEnabled)(self as *const _, aResponseTimeoutEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long initialRwin; */
    #[inline]
    pub unsafe fn get_initialRwin(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_initialRwin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_initialRwin(&self, aInitialRwin: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_initialRwin)(self as *const _, aInitialRwin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsIURI apiRedirectToURI; */
    #[inline]
    pub unsafe fn get_apiRedirectToURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_apiRedirectToURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] attribute boolean allowAltSvc; */
    #[inline]
    pub unsafe fn get_allowAltSvc(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowAltSvc)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowAltSvc(&self, aAllowAltSvc: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowAltSvc)(self as *const _, aAllowAltSvc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean beConservative; */
    #[inline]
    pub unsafe fn get_beConservative(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_beConservative)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_beConservative(&self, aBeConservative: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_beConservative)(self as *const _, aBeConservative) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute PRTime lastModifiedTime; */
    #[inline]
    pub unsafe fn get_lastModifiedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModifiedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void forceIntercepted (in uint64_t aInterceptionID); */
    #[inline]
    pub unsafe fn forceIntercepted(&self, aInterceptionID: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).forceIntercepted)(self as *const _, aInterceptionID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute boolean responseSynthesized; */
    #[inline]
    pub unsafe fn get_responseSynthesized(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_responseSynthesized)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] attribute boolean corsIncludeCredentials; */
    #[inline]
    pub unsafe fn get_corsIncludeCredentials(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_corsIncludeCredentials)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_corsIncludeCredentials(&self, aCorsIncludeCredentials: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_corsIncludeCredentials)(self as *const _, aCorsIncludeCredentials) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long corsMode; */
    #[inline]
    pub unsafe fn get_corsMode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_corsMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_corsMode(&self, aCorsMode: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_corsMode)(self as *const _, aCorsMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long redirectMode; */
    #[inline]
    pub unsafe fn get_redirectMode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_redirectMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_redirectMode(&self, aRedirectMode: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_redirectMode)(self as *const _, aRedirectMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long fetchCacheMode; */
    #[inline]
    pub unsafe fn get_fetchCacheMode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fetchCacheMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fetchCacheMode(&self, aFetchCacheMode: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_fetchCacheMode)(self as *const _, aFetchCacheMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsIURI topWindowURI; */
    #[inline]
    pub unsafe fn get_topWindowURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_topWindowURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] attribute ACString networkInterfaceId; */
    #[inline]
    pub unsafe fn get_networkInterfaceId(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_networkInterfaceId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_networkInterfaceId(&self, aNetworkInterfaceId: &[u8]) -> Result<(), nsresult> {
        let aNetworkInterfaceId = nsCString::from(aNetworkInterfaceId);
        match ((*self.vtable).set_networkInterfaceId)(self as *const _, &*aNetworkInterfaceId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsIURI proxyURI; */
    #[inline]
    pub unsafe fn get_proxyURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_proxyURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,nostdcall,notxpcom] void setCorsPreflightParameters (in StringArrayRef unsafeHeaders); */


    /* [infallible] attribute boolean blockAuthPrompt; */
    #[inline]
    pub unsafe fn get_blockAuthPrompt(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_blockAuthPrompt)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_blockAuthPrompt(&self, aBlockAuthPrompt: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_blockAuthPrompt)(self as *const _, aBlockAuthPrompt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute AString integrityMetadata; */
    #[inline]
    pub unsafe fn get_integrityMetadata(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_integrityMetadata)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_integrityMetadata(&self, aIntegrityMetadata: &[u16]) -> Result<(), nsresult> {
        let aIntegrityMetadata = nsString::from(aIntegrityMetadata);
        match ((*self.vtable).set_integrityMetadata)(self as *const _, &*aIntegrityMetadata) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute ACString connectionInfoHashKey; */
    #[inline]
    pub unsafe fn get_connectionInfoHashKey(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_connectionInfoHashKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


