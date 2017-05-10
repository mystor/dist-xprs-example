//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticator.idl
//


pub mod nsIHttpAuthenticator_consts {
    pub const USING_INTERNAL_IDENTITY: i64 = 1;
    pub const REQUEST_BASED: i64 = 1;
    pub const CONNECTION_BASED: i64 = 2;
    pub const REUSABLE_CREDENTIALS: i64 = 4;
    pub const REUSABLE_CHALLENGE: i64 = 8;
    pub const IDENTITY_IGNORED: i64 = 1024;
    pub const IDENTITY_INCLUDES_DOMAIN: i64 = 2048;
    pub const IDENTITY_ENCRYPTED: i64 = 4096;
}


#[repr(C)]
pub struct nsIHttpAuthenticator {
    vtable: *const nsIHttpAuthenticatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpAuthenticator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfef7db8a, 0xa4e2, 0x49d1,
            [0x96, 0x85, 0x19, 0xed, 0x7e, 0x30, 0x9b, 0x7d])
    }
}

unsafe impl RefCounted for nsIHttpAuthenticator {
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
pub trait nsIHttpAuthenticatorCoerce {
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self;
}

impl nsIHttpAuthenticatorCoerce for nsIHttpAuthenticator {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self {
        v
    }
}

impl nsIHttpAuthenticator {
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpAuthenticator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpAuthenticatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpAuthenticatorVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity); */
    pub challengeReceived: unsafe extern "C" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aInvalidatesIdentity: *mut bool) -> nsresult,

    /* [must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel); */
    pub generateCredentialsAsync: unsafe extern "C" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aCallback: *const nsIHttpAuthenticatorCallback, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const libc::int16_t, aUser: *const libc::int16_t, aPassword: *const libc::int16_t, aSessionState: *const nsISupports, aContinuationState: *const nsISupports, aCancel: *mut *const nsICancelable) -> nsresult,

    /* [must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags); */
    pub generateCredentials: unsafe extern "C" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const libc::int16_t, aUser: *const libc::int16_t, aPassword: *const libc::int16_t, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aFlags: *mut libc::uint32_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* [must_use] readonly attribute unsigned long authFlags; */
    pub get_authFlags: unsafe extern "C" fn (this: *const nsIHttpAuthenticator, aAuthFlags: *mut libc::uint32_t) -> nsresult,

}


impl nsIHttpAuthenticator {
    /* [must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity); */
    #[inline]
    pub unsafe fn challengeReceived(&self, aChannel: Option<&nsIHttpAuthenticableChannel>, aChallenge: *const libc::c_char, aProxyAuth: bool) -> Result<(Option<RefPtr<nsISupports>>, Option<RefPtr<nsISupports>>, bool), nsresult> {
        let mut aSessionState = GetterAddrefs::new();
        let mut aContinuationState = GetterAddrefs::new();
        let mut aInvalidatesIdentity: bool = ::std::mem::zeroed();
        match ((*self.vtable).challengeReceived)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aChallenge, aProxyAuth, aSessionState.ptr(), aContinuationState.ptr(), &mut aInvalidatesIdentity as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aSessionState.refptr(), aContinuationState.refptr(), aInvalidatesIdentity))
    }

    /* [must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel); */
    #[inline]
    pub unsafe fn generateCredentialsAsync(&self, aChannel: Option<&nsIHttpAuthenticableChannel>, aCallback: Option<&nsIHttpAuthenticatorCallback>, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const libc::int16_t, aUser: *const libc::int16_t, aPassword: *const libc::int16_t, aSessionState: Option<&nsISupports>, aContinuationState: Option<&nsISupports>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut aCancel = GetterAddrefs::new();
        match ((*self.vtable).generateCredentialsAsync)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState.map_or(::std::ptr::null(), |x| x as *const _), aContinuationState.map_or(::std::ptr::null(), |x| x as *const _), aCancel.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aCancel.refptr())
    }

    /* [must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags); */
    #[inline]
    pub unsafe fn generateCredentials(&self, aChannel: Option<&nsIHttpAuthenticableChannel>, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const libc::int16_t, aUser: *const libc::int16_t, aPassword: *const libc::int16_t) -> Result<(Option<RefPtr<nsISupports>>, Option<RefPtr<nsISupports>>, libc::uint32_t, *const libc::c_char), nsresult> {
        let mut aSessionState = GetterAddrefs::new();
        let mut aContinuationState = GetterAddrefs::new();
        let mut aFlags: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).generateCredentials)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState.ptr(), aContinuationState.ptr(), &mut aFlags as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aSessionState.refptr(), aContinuationState.refptr(), aFlags, _retval))
    }

    /* [must_use] readonly attribute unsigned long authFlags; */
    #[inline]
    pub unsafe fn get_authFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_authFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


