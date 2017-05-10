//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISSLSocketControl.idl
//


pub mod nsISSLSocketControl_consts {
    pub const KEY_EXCHANGE_UNKNOWN: i64 = -1;
    pub const SSL_VERSION_3: i64 = 768;
    pub const TLS_VERSION_1: i64 = 769;
    pub const TLS_VERSION_1_1: i64 = 770;
    pub const TLS_VERSION_1_2: i64 = 771;
    pub const TLS_VERSION_1_3: i64 = 772;
    pub const SSL_VERSION_UNKNOWN: i64 = -1;
    pub const SSL_MAC_UNKNOWN: i64 = -1;
    pub const SSL_MAC_NULL: i64 = 0;
    pub const SSL_MAC_MD5: i64 = 1;
    pub const SSL_MAC_SHA: i64 = 2;
    pub const SSL_HMAC_MD5: i64 = 3;
    pub const SSL_HMAC_SHA: i64 = 4;
    pub const SSL_HMAC_SHA256: i64 = 5;
    pub const SSL_MAC_AEAD: i64 = 6;
}


#[repr(C)]
pub struct nsISSLSocketControl {
    vtable: *const nsISSLSocketControlVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISSLSocketControl {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x418265c8, 0x654e, 0x4fbb,
            [0xba, 0x62, 0x4e, 0xed, 0x27, 0xde, 0x1f, 0x03])
    }
}

unsafe impl RefCounted for nsISSLSocketControl {
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
pub trait nsISSLSocketControlCoerce {
    fn coerce_from(v: &nsISSLSocketControl) -> &Self;
}

impl nsISSLSocketControlCoerce for nsISSLSocketControl {
    #[inline]
    fn coerce_from(v: &nsISSLSocketControl) -> &Self {
        v
    }
}

impl nsISSLSocketControl {
    #[inline]
    pub fn coerce<T: nsISSLSocketControlCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISSLSocketControl {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISSLSocketControlCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISSLSocketControl) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISSLSocketControlVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub get_notificationCallbacks: unsafe extern "C" fn (this: *const nsISSLSocketControl, aNotificationCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,
    pub set_notificationCallbacks: unsafe extern "C" fn (this: *const nsISSLSocketControl, aNotificationCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* void proxyStartSSL (); */
    pub proxyStartSSL: unsafe extern "C" fn (this: *const nsISSLSocketControl) -> nsresult,

    /* void StartTLS (); */
    pub StartTLS: unsafe extern "C" fn (this: *const nsISSLSocketControl) -> nsresult,

    /* [noscript] void setNPNList (in nsCStringTArrayRef aNPNList); */
    /// Unable to call function as its signature contains a non-rust type
    pub setNPNList: *const ::libc::c_void,

    /* readonly attribute ACString negotiatedNPN; */
    pub get_negotiatedNPN: unsafe extern "C" fn (this: *const nsISSLSocketControl, aNegotiatedNPN: *mut nsACString) -> nsresult,

    /* ACString getAlpnEarlySelection (); */
    pub getAlpnEarlySelection: unsafe extern "C" fn (this: *const nsISSLSocketControl, _retval: *mut nsACString) -> nsresult,

    /* readonly attribute bool earlyDataAccepted; */
    pub get_earlyDataAccepted: unsafe extern "C" fn (this: *const nsISSLSocketControl, aEarlyDataAccepted: *mut bool) -> nsresult,

    /* void driveHandshake (); */
    pub driveHandshake: unsafe extern "C" fn (this: *const nsISSLSocketControl) -> nsresult,

    /* boolean joinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    pub joinConnection: unsafe extern "C" fn (this: *const nsISSLSocketControl, npnProtocol: *const nsACString, hostname: *const nsACString, port: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean testJoinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    pub testJoinConnection: unsafe extern "C" fn (this: *const nsISSLSocketControl, npnProtocol: *const nsACString, hostname: *const nsACString, port: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean isAcceptableForHost (in ACString hostname); */
    pub isAcceptableForHost: unsafe extern "C" fn (this: *const nsISSLSocketControl, hostname: *const nsACString, _retval: *mut bool) -> nsresult,

    /* [infallible] readonly attribute short KEAUsed; */
    pub get_KEAUsed: unsafe extern "C" fn (this: *const nsISSLSocketControl, aKEAUsed: *mut libc::int16_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long KEAKeyBits; */
    pub get_KEAKeyBits: unsafe extern "C" fn (this: *const nsISSLSocketControl, aKEAKeyBits: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute uint32_t providerFlags; */
    pub get_providerFlags: unsafe extern "C" fn (this: *const nsISSLSocketControl, aProviderFlags: *mut uint32_t) -> nsresult,

    /* [infallible] readonly attribute short SSLVersionUsed; */
    pub get_SSLVersionUsed: unsafe extern "C" fn (this: *const nsISSLSocketControl, aSSLVersionUsed: *mut libc::int16_t) -> nsresult,

    /* [infallible] readonly attribute short SSLVersionOffered; */
    pub get_SSLVersionOffered: unsafe extern "C" fn (this: *const nsISSLSocketControl, aSSLVersionOffered: *mut libc::int16_t) -> nsresult,

    /* [infallible] readonly attribute short MACAlgorithmUsed; */
    pub get_MACAlgorithmUsed: unsafe extern "C" fn (this: *const nsISSLSocketControl, aMACAlgorithmUsed: *mut libc::int16_t) -> nsresult,

    /* attribute nsIX509Cert clientCert; */
    pub get_clientCert: unsafe extern "C" fn (this: *const nsISSLSocketControl, aClientCert: *mut *const nsIX509Cert) -> nsresult,
    pub set_clientCert: unsafe extern "C" fn (this: *const nsISSLSocketControl, aClientCert: *const nsIX509Cert) -> nsresult,

    /* [infallible] readonly attribute boolean bypassAuthentication; */
    pub get_bypassAuthentication: unsafe extern "C" fn (this: *const nsISSLSocketControl, aBypassAuthentication: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean failedVerification; */
    pub get_failedVerification: unsafe extern "C" fn (this: *const nsISSLSocketControl, aFailedVerification: *mut bool) -> nsresult,

}


impl nsISSLSocketControl {
    /* attribute nsIInterfaceRequestor notificationCallbacks; */
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

    /* void proxyStartSSL (); */
    #[inline]
    pub unsafe fn proxyStartSSL(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).proxyStartSSL)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void StartTLS (); */
    #[inline]
    pub unsafe fn StartTLS(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).StartTLS)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setNPNList (in nsCStringTArrayRef aNPNList); */


    /* readonly attribute ACString negotiatedNPN; */
    #[inline]
    pub unsafe fn get_negotiatedNPN(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_negotiatedNPN)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getAlpnEarlySelection (); */
    #[inline]
    pub unsafe fn getAlpnEarlySelection(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAlpnEarlySelection)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool earlyDataAccepted; */
    #[inline]
    pub unsafe fn get_earlyDataAccepted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_earlyDataAccepted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void driveHandshake (); */
    #[inline]
    pub unsafe fn driveHandshake(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).driveHandshake)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean joinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    #[inline]
    pub unsafe fn joinConnection(&self, npnProtocol: &[u8], hostname: &[u8], port: libc::int32_t) -> Result<bool, nsresult> {
        let npnProtocol = nsCString::from(npnProtocol);
        let hostname = nsCString::from(hostname);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).joinConnection)(self as *const _, &*npnProtocol, &*hostname, port, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean testJoinConnection (in ACString npnProtocol, in ACString hostname, in long port); */
    #[inline]
    pub unsafe fn testJoinConnection(&self, npnProtocol: &[u8], hostname: &[u8], port: libc::int32_t) -> Result<bool, nsresult> {
        let npnProtocol = nsCString::from(npnProtocol);
        let hostname = nsCString::from(hostname);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).testJoinConnection)(self as *const _, &*npnProtocol, &*hostname, port, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isAcceptableForHost (in ACString hostname); */
    #[inline]
    pub unsafe fn isAcceptableForHost(&self, hostname: &[u8]) -> Result<bool, nsresult> {
        let hostname = nsCString::from(hostname);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAcceptableForHost)(self as *const _, &*hostname, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute short KEAUsed; */
    #[inline]
    pub unsafe fn get_KEAUsed(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_KEAUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long KEAKeyBits; */
    #[inline]
    pub unsafe fn get_KEAKeyBits(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_KEAKeyBits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t providerFlags; */
    #[inline]
    pub unsafe fn get_providerFlags(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_providerFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute short SSLVersionUsed; */
    #[inline]
    pub unsafe fn get_SSLVersionUsed(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_SSLVersionUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute short SSLVersionOffered; */
    #[inline]
    pub unsafe fn get_SSLVersionOffered(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_SSLVersionOffered)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute short MACAlgorithmUsed; */
    #[inline]
    pub unsafe fn get_MACAlgorithmUsed(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_MACAlgorithmUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIX509Cert clientCert; */
    #[inline]
    pub unsafe fn get_clientCert(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_clientCert)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_clientCert(&self, aClientCert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).set_clientCert)(self as *const _, aClientCert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean bypassAuthentication; */
    #[inline]
    pub unsafe fn get_bypassAuthentication(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_bypassAuthentication)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean failedVerification; */
    #[inline]
    pub unsafe fn get_failedVerification(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_failedVerification)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


