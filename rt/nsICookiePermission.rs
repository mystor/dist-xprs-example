//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookiePermission.idl
//


pub type nsCookieAccess = libc::int32_t;


pub mod nsICookiePermission_consts {
    pub const ACCESS_DEFAULT: i64 = 0;
    pub const ACCESS_ALLOW: i64 = 1;
    pub const ACCESS_DENY: i64 = 2;
    pub const ACCESS_SESSION: i64 = 8;
    pub const ACCESS_ALLOW_FIRST_PARTY_ONLY: i64 = 9;
    pub const ACCESS_LIMIT_THIRD_PARTY: i64 = 10;
}


#[repr(C)]
pub struct nsICookiePermission {
    vtable: *const nsICookiePermissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookiePermission {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x11ddd4ed, 0x8f5b, 0x40b3,
            [0xb2, 0xa0, 0x27, 0xc2, 0x0e, 0xa1, 0xc8, 0x8d])
    }
}

unsafe impl RefCounted for nsICookiePermission {
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
pub trait nsICookiePermissionCoerce {
    fn coerce_from(v: &nsICookiePermission) -> &Self;
}

impl nsICookiePermissionCoerce for nsICookiePermission {
    #[inline]
    fn coerce_from(v: &nsICookiePermission) -> &Self {
        v
    }
}

impl nsICookiePermission {
    #[inline]
    pub fn coerce<T: nsICookiePermissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookiePermission {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICookiePermissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookiePermission) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookiePermissionVTable {
    pub __base: nsISupportsVTable,

    /* void setAccess (in nsIURI aURI, in nsCookieAccess aAccess); */
    pub setAccess: unsafe extern "C" fn (this: *const nsICookiePermission, aURI: *const nsIURI, aAccess: nsCookieAccess) -> nsresult,

    /* nsCookieAccess canAccess (in nsIURI aURI, in nsIChannel aChannel); */
    pub canAccess: unsafe extern "C" fn (this: *const nsICookiePermission, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut nsCookieAccess) -> nsresult,

    /* boolean canSetCookie (in nsIURI aURI, in nsIChannel aChannel, in nsICookie2 aCookie, inout boolean aIsSession, inout int64_t aExpiry); */
    pub canSetCookie: unsafe extern "C" fn (this: *const nsICookiePermission, aURI: *const nsIURI, aChannel: *const nsIChannel, aCookie: *const nsICookie2, aIsSession: *mut bool, aExpiry: *mut int64_t, _retval: *mut bool) -> nsresult,

}


impl nsICookiePermission {
    /* void setAccess (in nsIURI aURI, in nsCookieAccess aAccess); */
    #[inline]
    pub unsafe fn setAccess(&self, aURI: Option<&nsIURI>, aAccess: nsCookieAccess) -> Result<(), nsresult> {

        match ((*self.vtable).setAccess)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aAccess) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsCookieAccess canAccess (in nsIURI aURI, in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn canAccess(&self, aURI: Option<&nsIURI>, aChannel: Option<&nsIChannel>) -> Result<nsCookieAccess, nsresult> {
        let mut _retval: nsCookieAccess = ::std::mem::zeroed();
        match ((*self.vtable).canAccess)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canSetCookie (in nsIURI aURI, in nsIChannel aChannel, in nsICookie2 aCookie, inout boolean aIsSession, inout int64_t aExpiry); */
    #[inline]
    pub unsafe fn canSetCookie(&self, aURI: Option<&nsIURI>, aChannel: Option<&nsIChannel>, aCookie: Option<&nsICookie2>) -> Result<(bool, int64_t, bool), nsresult> {
        let mut aIsSession: bool = ::std::mem::zeroed();
        let mut aExpiry: int64_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canSetCookie)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), aCookie.map_or(::std::ptr::null(), |x| x as *const _), &mut aIsSession as *mut _, &mut aExpiry as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aIsSession, aExpiry, _retval))
    }

}


