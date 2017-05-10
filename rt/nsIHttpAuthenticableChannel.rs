//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticableChannel.idl
//


#[repr(C)]
pub struct nsIHttpAuthenticableChannel {
    vtable: *const nsIHttpAuthenticableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpAuthenticableChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x701093ac, 0x5c7f, 0x429c,
            [0x99, 0xe3, 0x42, 0x3b, 0x04, 0x1f, 0xcc, 0xb4])
    }
}

unsafe impl RefCounted for nsIHttpAuthenticableChannel {
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
pub trait nsIHttpAuthenticableChannelCoerce {
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self;
}

impl nsIHttpAuthenticableChannelCoerce for nsIHttpAuthenticableChannel {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self {
        v
    }
}

impl nsIHttpAuthenticableChannel {
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpAuthenticableChannel {
    type Target = nsIProxiedChannel;
    #[inline]
    fn deref(&self) -> &nsIProxiedChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProxiedChannelCoerce> nsIHttpAuthenticableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticableChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpAuthenticableChannelVTable {
    pub __base: nsIProxiedChannelVTable,

    /* [must_use] readonly attribute boolean isSSL; */
    pub get_isSSL: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aIsSSL: *mut bool) -> nsresult,

    /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
    pub get_proxyMethodIsConnect: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aProxyMethodIsConnect: *mut bool) -> nsresult,

    /* [must_use] void cancel (in nsresult aStatus); */
    pub cancel: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aStatus: nsresult) -> nsresult,

    /* [must_use] readonly attribute nsLoadFlags loadFlags; */
    pub get_loadFlags: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aLoadFlags: *mut nsLoadFlags) -> nsresult,

    /* [must_use] readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aURI: *mut *const nsIURI) -> nsresult,

    /* [must_use] readonly attribute nsILoadGroup loadGroup; */
    pub get_loadGroup: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aLoadGroup: *mut *const nsILoadGroup) -> nsresult,

    /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
    pub get_notificationCallbacks: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aNotificationCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,

    /* [must_use] readonly attribute ACString requestMethod; */
    pub get_requestMethod: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aRequestMethod: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString serverResponseHeader; */
    pub get_serverResponseHeader: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aServerResponseHeader: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString proxyChallenges; */
    pub get_proxyChallenges: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aProxyChallenges: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString WWWChallenges; */
    pub get_WWWChallenges: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, aWWWChallenges: *mut nsACString) -> nsresult,

    /* [must_use] void setProxyCredentials (in ACString credentials); */
    pub setProxyCredentials: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, credentials: *const nsACString) -> nsresult,

    /* [must_use] void setWWWCredentials (in ACString credentials); */
    pub setWWWCredentials: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, credentials: *const nsACString) -> nsresult,

    /* [must_use] void onAuthAvailable (); */
    pub onAuthAvailable: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel) -> nsresult,

    /* [must_use] void onAuthCancelled (in boolean userCancel); */
    pub onAuthCancelled: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, userCancel: bool) -> nsresult,

    /* [must_use] void closeStickyConnection (); */
    pub closeStickyConnection: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel) -> nsresult,

    /* [must_use] void forceNoSpdy (); */
    pub forceNoSpdy: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel) -> nsresult,

    /* void connectionRestartable (in boolean restartable); */
    pub connectionRestartable: unsafe extern "C" fn (this: *const nsIHttpAuthenticableChannel, restartable: bool) -> nsresult,

}


impl nsIHttpAuthenticableChannel {
    /* [must_use] readonly attribute boolean isSSL; */
    #[inline]
    pub unsafe fn get_isSSL(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSSL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
    #[inline]
    pub unsafe fn get_proxyMethodIsConnect(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_proxyMethodIsConnect)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void cancel (in nsresult aStatus); */
    #[inline]
    pub unsafe fn cancel(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute nsLoadFlags loadFlags; */
    #[inline]
    pub unsafe fn get_loadFlags(&self, ) -> Result<nsLoadFlags, nsresult> {
        let mut _retval: nsLoadFlags = ::std::mem::zeroed();
        match ((*self.vtable).get_loadFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* [must_use] readonly attribute nsILoadGroup loadGroup; */
    #[inline]
    pub unsafe fn get_loadGroup(&self, ) -> Result<Option<RefPtr<nsILoadGroup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadGroup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
    #[inline]
    pub unsafe fn get_notificationCallbacks(&self, ) -> Result<Option<RefPtr<nsIInterfaceRequestor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_notificationCallbacks)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] readonly attribute ACString requestMethod; */
    #[inline]
    pub unsafe fn get_requestMethod(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_requestMethod)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString serverResponseHeader; */
    #[inline]
    pub unsafe fn get_serverResponseHeader(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_serverResponseHeader)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString proxyChallenges; */
    #[inline]
    pub unsafe fn get_proxyChallenges(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_proxyChallenges)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString WWWChallenges; */
    #[inline]
    pub unsafe fn get_WWWChallenges(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_WWWChallenges)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setProxyCredentials (in ACString credentials); */
    #[inline]
    pub unsafe fn setProxyCredentials(&self, credentials: &[u8]) -> Result<(), nsresult> {
        let credentials = nsCString::from(credentials);
        match ((*self.vtable).setProxyCredentials)(self as *const _, &*credentials) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void setWWWCredentials (in ACString credentials); */
    #[inline]
    pub unsafe fn setWWWCredentials(&self, credentials: &[u8]) -> Result<(), nsresult> {
        let credentials = nsCString::from(credentials);
        match ((*self.vtable).setWWWCredentials)(self as *const _, &*credentials) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onAuthAvailable (); */
    #[inline]
    pub unsafe fn onAuthAvailable(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onAuthAvailable)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onAuthCancelled (in boolean userCancel); */
    #[inline]
    pub unsafe fn onAuthCancelled(&self, userCancel: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onAuthCancelled)(self as *const _, userCancel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void closeStickyConnection (); */
    #[inline]
    pub unsafe fn closeStickyConnection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeStickyConnection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void forceNoSpdy (); */
    #[inline]
    pub unsafe fn forceNoSpdy(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceNoSpdy)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void connectionRestartable (in boolean restartable); */
    #[inline]
    pub unsafe fn connectionRestartable(&self, restartable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).connectionRestartable)(self as *const _, restartable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


