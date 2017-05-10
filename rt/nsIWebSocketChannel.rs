//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketChannel.idl
//


pub mod nsIWebSocketChannel_consts {
    pub const CLOSE_NORMAL: i64 = 1000;
    pub const CLOSE_GOING_AWAY: i64 = 1001;
    pub const CLOSE_PROTOCOL_ERROR: i64 = 1002;
    pub const CLOSE_UNSUPPORTED_DATATYPE: i64 = 1003;
    pub const CLOSE_NO_STATUS: i64 = 1005;
    pub const CLOSE_ABNORMAL: i64 = 1006;
    pub const CLOSE_INVALID_PAYLOAD: i64 = 1007;
    pub const CLOSE_POLICY_VIOLATION: i64 = 1008;
    pub const CLOSE_TOO_LARGE: i64 = 1009;
    pub const CLOSE_EXTENSION_MISSING: i64 = 1010;
    pub const CLOSE_INTERNAL_ERROR: i64 = 1011;
    pub const CLOSE_TLS_FAILED: i64 = 1015;
}


#[repr(C)]
pub struct nsIWebSocketChannel {
    vtable: *const nsIWebSocketChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebSocketChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xce71d028, 0x322a, 0x4105,
            [0xa9, 0x47, 0xa8, 0x94, 0x68, 0x9b, 0x52, 0xbf])
    }
}

unsafe impl RefCounted for nsIWebSocketChannel {
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
pub trait nsIWebSocketChannelCoerce {
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self;
}

impl nsIWebSocketChannelCoerce for nsIWebSocketChannel {
    #[inline]
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self {
        v
    }
}

impl nsIWebSocketChannel {
    #[inline]
    pub fn coerce<T: nsIWebSocketChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebSocketChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebSocketChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebSocketChannelVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute nsIURI originalURI; */
    pub get_originalURI: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aOriginalURI: *mut *const nsIURI) -> nsresult,

    /* [must_use] readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aURI: *mut *const nsIURI) -> nsresult,

    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
    pub get_notificationCallbacks: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aNotificationCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,
    pub set_notificationCallbacks: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aNotificationCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* [must_use] readonly attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aSecurityInfo: *mut *const nsISupports) -> nsresult,

    /* [must_use] attribute nsILoadGroup loadGroup; */
    pub get_loadGroup: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aLoadGroup: *mut *const nsILoadGroup) -> nsresult,
    pub set_loadGroup: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aLoadGroup: *const nsILoadGroup) -> nsresult,

    /* [must_use] attribute nsILoadInfo loadInfo; */
    pub get_loadInfo: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aLoadInfo: *mut *const nsILoadInfo) -> nsresult,
    pub set_loadInfo: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aLoadInfo: *const nsILoadInfo) -> nsresult,

    /* [must_use] attribute ACString protocol; */
    pub get_protocol: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aProtocol: *mut nsACString) -> nsresult,
    pub set_protocol: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aProtocol: *const nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString extensions; */
    pub get_extensions: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aExtensions: *mut nsACString) -> nsresult,

    /* [must_use] void initLoadInfo (in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    pub initLoadInfo: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aLoadingNode: *const nsIDOMNode, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t) -> nsresult,

    /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
    pub asyncOpen: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aURI: *const nsIURI, aOrigin: *const nsACString, aInnerWindowID: libc::uint64_t, aListener: *const nsIWebSocketListener, aContext: *const nsISupports) -> nsresult,

    /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
    pub close: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aCode: libc::uint16_t, aReason: *const nsACString) -> nsresult,

    /* [must_use] void sendMsg (in AUTF8String aMsg); */
    pub sendMsg: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aMsg: *const nsACString) -> nsresult,

    /* [must_use] void sendBinaryMsg (in ACString aMsg); */
    pub sendBinaryMsg: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aMsg: *const nsACString) -> nsresult,

    /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
    pub sendBinaryStream: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aStream: *const nsIInputStream, length: libc::uint32_t) -> nsresult,

    /* [must_use] attribute unsigned long pingInterval; */
    pub get_pingInterval: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aPingInterval: *mut libc::uint32_t) -> nsresult,
    pub set_pingInterval: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aPingInterval: libc::uint32_t) -> nsresult,

    /* [must_use] attribute unsigned long pingTimeout; */
    pub get_pingTimeout: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aPingTimeout: *mut libc::uint32_t) -> nsresult,
    pub set_pingTimeout: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aPingTimeout: libc::uint32_t) -> nsresult,

    /* [must_use] attribute unsigned long serial; */
    pub get_serial: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aSerial: *mut libc::uint32_t) -> nsresult,
    pub set_serial: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aSerial: libc::uint32_t) -> nsresult,

    /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
    pub setServerParameters: unsafe extern "C" fn (this: *const nsIWebSocketChannel, aProvider: *const nsITransportProvider, aNegotiatedExtensions: *const nsACString) -> nsresult,

}


impl nsIWebSocketChannel {
    /* [must_use] readonly attribute nsIURI originalURI; */
    #[inline]
    pub unsafe fn get_originalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] readonly attribute nsIURI URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_URI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] attribute nsIInterfaceRequestor notificationCallbacks; */
    #[inline]
    pub unsafe fn get_notificationCallbacks(&self, ) -> Result<Option<RefPtr<nsIInterfaceRequestor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_notificationCallbacks)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_notificationCallbacks(&self, aNotificationCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).set_notificationCallbacks)(self as *const _, aNotificationCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] attribute nsILoadGroup loadGroup; */
    #[inline]
    pub unsafe fn get_loadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_loadGroup(&self, aLoadGroup: Option<&nsILoadGroup>) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadGroup)(self as *const _, aLoadGroup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute nsILoadInfo loadInfo; */
    #[inline]
    pub unsafe fn get_loadInfo(&self, ) -> Result<Option<RefPtr<nsILoadInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_loadInfo(&self, aLoadInfo: Option<&nsILoadInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadInfo)(self as *const _, aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute ACString protocol; */
    #[inline]
    pub unsafe fn get_protocol(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_protocol)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_protocol(&self, aProtocol: &[u8]) -> Result<(), nsresult> {
        let aProtocol = nsCString::from(aProtocol);
        match ((*self.vtable).set_protocol)(self as *const _, &*aProtocol) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute ACString extensions; */
    #[inline]
    pub unsafe fn get_extensions(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_extensions)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void initLoadInfo (in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    #[inline]
    pub unsafe fn initLoadInfo(&self, aLoadingNode: Option<&nsIDOMNode>, aLoadingPrincipal: Option<&nsIPrincipal>, aTriggeringPrincipal: Option<&nsIPrincipal>, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).initLoadInfo)(self as *const _, aLoadingNode.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aSecurityFlags, aContentPolicyType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void asyncOpen (in nsIURI aURI, in ACString aOrigin, in unsigned long long aInnerWindowID, in nsIWebSocketListener aListener, in nsISupports aContext); */
    #[inline]
    pub unsafe fn asyncOpen(&self, aURI: Option<&nsIURI>, aOrigin: &[u8], aInnerWindowID: libc::uint64_t, aListener: Option<&nsIWebSocketListener>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {
        let aOrigin = nsCString::from(aOrigin);
        match ((*self.vtable).asyncOpen)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aOrigin, aInnerWindowID, aListener.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void close (in unsigned short aCode, in AUTF8String aReason); */
    #[inline]
    pub unsafe fn close(&self, aCode: libc::uint16_t, aReason: &[u8]) -> Result<(), nsresult> {
        let aReason = nsCString::from(aReason);
        match ((*self.vtable).close)(self as *const _, aCode, &*aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void sendMsg (in AUTF8String aMsg); */
    #[inline]
    pub unsafe fn sendMsg(&self, aMsg: &[u8]) -> Result<(), nsresult> {
        let aMsg = nsCString::from(aMsg);
        match ((*self.vtable).sendMsg)(self as *const _, &*aMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void sendBinaryMsg (in ACString aMsg); */
    #[inline]
    pub unsafe fn sendBinaryMsg(&self, aMsg: &[u8]) -> Result<(), nsresult> {
        let aMsg = nsCString::from(aMsg);
        match ((*self.vtable).sendBinaryMsg)(self as *const _, &*aMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void sendBinaryStream (in nsIInputStream aStream, in unsigned long length); */
    #[inline]
    pub unsafe fn sendBinaryStream(&self, aStream: Option<&nsIInputStream>, length: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).sendBinaryStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), length) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long pingInterval; */
    #[inline]
    pub unsafe fn get_pingInterval(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pingInterval)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pingInterval(&self, aPingInterval: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_pingInterval)(self as *const _, aPingInterval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long pingTimeout; */
    #[inline]
    pub unsafe fn get_pingTimeout(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pingTimeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pingTimeout(&self, aPingTimeout: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_pingTimeout)(self as *const _, aPingTimeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long serial; */
    #[inline]
    pub unsafe fn get_serial(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_serial)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_serial(&self, aSerial: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_serial)(self as *const _, aSerial) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void setServerParameters (in nsITransportProvider aProvider, in ACString aNegotiatedExtensions); */
    #[inline]
    pub unsafe fn setServerParameters(&self, aProvider: Option<&nsITransportProvider>, aNegotiatedExtensions: &[u8]) -> Result<(), nsresult> {
        let aNegotiatedExtensions = nsCString::from(aNegotiatedExtensions);
        match ((*self.vtable).setServerParameters)(self as *const _, aProvider.map_or(::std::ptr::null(), |x| x as *const _), &*aNegotiatedExtensions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


