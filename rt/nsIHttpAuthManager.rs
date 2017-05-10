//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthManager.idl
//


#[repr(C)]
pub struct nsIHttpAuthManager {
    vtable: *const nsIHttpAuthManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpAuthManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x54f90444, 0xc52b, 0x4d2d,
            [0x89, 0x16, 0xc5, 0x9a, 0x2b, 0xb2, 0x59, 0x38])
    }
}

unsafe impl RefCounted for nsIHttpAuthManager {
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
pub trait nsIHttpAuthManagerCoerce {
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self;
}

impl nsIHttpAuthManagerCoerce for nsIHttpAuthManager {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self {
        v
    }
}

impl nsIHttpAuthManager {
    #[inline]
    pub fn coerce<T: nsIHttpAuthManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpAuthManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpAuthManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpAuthManagerVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    pub getAuthIdentity: unsafe extern "C" fn (this: *const nsIHttpAuthManager, aScheme: *const nsACString, aHost: *const nsACString, aPort: int32_t, aAuthType: *const nsACString, aRealm: *const nsACString, aPath: *const nsACString, aUserDomain: *mut nsAString, aUserName: *mut nsAString, aUserPassword: *mut nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* [must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    pub setAuthIdentity: unsafe extern "C" fn (this: *const nsIHttpAuthManager, aScheme: *const nsACString, aHost: *const nsACString, aPort: int32_t, aAuthType: *const nsACString, aRealm: *const nsACString, aPath: *const nsACString, aUserDomain: *const nsAString, aUserName: *const nsAString, aUserPassword: *const nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* [must_use] void clearAll (); */
    pub clearAll: unsafe extern "C" fn (this: *const nsIHttpAuthManager) -> nsresult,

}


impl nsIHttpAuthManager {
    /* [must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn getAuthIdentity(&self, aScheme: &[u8], aHost: &[u8], aPort: int32_t, aAuthType: &[u8], aRealm: &[u8], aPath: &[u8], aIsPrivate: bool, aPrincipal: Option<&nsIPrincipal>) -> Result<(nsString, nsString, nsString), nsresult> {
        let aScheme = nsCString::from(aScheme);
        let aHost = nsCString::from(aHost);
        let aAuthType = nsCString::from(aAuthType);
        let aRealm = nsCString::from(aRealm);
        let aPath = nsCString::from(aPath);
        let mut aUserDomain = nsString::new();
        let mut aUserName = nsString::new();
        let mut aUserPassword = nsString::new();
        match ((*self.vtable).getAuthIdentity)(self as *const _, &*aScheme, &*aHost, aPort, &*aAuthType, &*aRealm, &*aPath, &mut *aUserDomain, &mut *aUserName, &mut *aUserPassword, aIsPrivate, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aUserDomain, aUserName, aUserPassword))
    }

    /* [must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn setAuthIdentity(&self, aScheme: &[u8], aHost: &[u8], aPort: int32_t, aAuthType: &[u8], aRealm: &[u8], aPath: &[u8], aUserDomain: &[u16], aUserName: &[u16], aUserPassword: &[u16], aIsPrivate: bool, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let aScheme = nsCString::from(aScheme);
        let aHost = nsCString::from(aHost);
        let aAuthType = nsCString::from(aAuthType);
        let aRealm = nsCString::from(aRealm);
        let aPath = nsCString::from(aPath);
        let aUserDomain = nsString::from(aUserDomain);
        let aUserName = nsString::from(aUserName);
        let aUserPassword = nsString::from(aUserPassword);
        match ((*self.vtable).setAuthIdentity)(self as *const _, &*aScheme, &*aHost, aPort, &*aAuthType, &*aRealm, &*aPath, &*aUserDomain, &*aUserName, &*aUserPassword, aIsPrivate, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void clearAll (); */
    #[inline]
    pub unsafe fn clearAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


