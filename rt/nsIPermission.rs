//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPermission.idl
//


#[repr(C)]
pub struct nsIPermission {
    vtable: *const nsIPermissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPermission {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbb409a51, 0x2371, 0x4fea,
            [0x9d, 0xc9, 0xb7, 0x28, 0x6a, 0x45, 0x8b, 0x8c])
    }
}

unsafe impl RefCounted for nsIPermission {
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
pub trait nsIPermissionCoerce {
    fn coerce_from(v: &nsIPermission) -> &Self;
}

impl nsIPermissionCoerce for nsIPermission {
    #[inline]
    fn coerce_from(v: &nsIPermission) -> &Self {
        v
    }
}

impl nsIPermission {
    #[inline]
    pub fn coerce<T: nsIPermissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPermission {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPermissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPermission) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPermissionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIPermission, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute ACString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIPermission, aType: *mut nsACString) -> nsresult,

    /* readonly attribute uint32_t capability; */
    pub get_capability: unsafe extern "C" fn (this: *const nsIPermission, aCapability: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t expireType; */
    pub get_expireType: unsafe extern "C" fn (this: *const nsIPermission, aExpireType: *mut uint32_t) -> nsresult,

    /* readonly attribute int64_t expireTime; */
    pub get_expireTime: unsafe extern "C" fn (this: *const nsIPermission, aExpireTime: *mut int64_t) -> nsresult,

    /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
    pub matches: unsafe extern "C" fn (this: *const nsIPermission, principal: *const nsIPrincipal, exactHost: bool, _retval: *mut bool) -> nsresult,

    /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
    pub matchesURI: unsafe extern "C" fn (this: *const nsIPermission, uri: *const nsIURI, exactHost: bool, _retval: *mut bool) -> nsresult,

}


impl nsIPermission {
    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t capability; */
    #[inline]
    pub unsafe fn get_capability(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_capability)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t expireType; */
    #[inline]
    pub unsafe fn get_expireType(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expireType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t expireTime; */
    #[inline]
    pub unsafe fn get_expireTime(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expireTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
    #[inline]
    pub unsafe fn matches(&self, principal: Option<&nsIPrincipal>, exactHost: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).matches)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), exactHost, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
    #[inline]
    pub unsafe fn matchesURI(&self, uri: Option<&nsIURI>, exactHost: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).matchesURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), exactHost, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


