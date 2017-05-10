//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProxyInfo.idl
//


pub mod nsIProxyInfo_consts {
    pub const TRANSPARENT_PROXY_RESOLVES_HOST: i64 = 1;
}


#[repr(C)]
pub struct nsIProxyInfo {
    vtable: *const nsIProxyInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProxyInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x63fff172, 0x2564, 0x4138,
            [0x96, 0xc6, 0x3a, 0xe7, 0xd2, 0x45, 0xfb, 0xed])
    }
}

unsafe impl RefCounted for nsIProxyInfo {
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
pub trait nsIProxyInfoCoerce {
    fn coerce_from(v: &nsIProxyInfo) -> &Self;
}

impl nsIProxyInfoCoerce for nsIProxyInfo {
    #[inline]
    fn coerce_from(v: &nsIProxyInfo) -> &Self {
        v
    }
}

impl nsIProxyInfo {
    #[inline]
    pub fn coerce<T: nsIProxyInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProxyInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProxyInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxyInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProxyInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIProxyInfo, aHost: *mut nsACString) -> nsresult,

    /* readonly attribute long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIProxyInfo, aPort: *mut libc::int32_t) -> nsresult,

    /* readonly attribute ACString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIProxyInfo, aType: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long flags; */
    pub get_flags: unsafe extern "C" fn (this: *const nsIProxyInfo, aFlags: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long resolveFlags; */
    pub get_resolveFlags: unsafe extern "C" fn (this: *const nsIProxyInfo, aResolveFlags: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute ACString username; */
    pub get_username: unsafe extern "C" fn (this: *const nsIProxyInfo, aUsername: *mut nsACString) -> nsresult,

    /* readonly attribute ACString password; */
    pub get_password: unsafe extern "C" fn (this: *const nsIProxyInfo, aPassword: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long failoverTimeout; */
    pub get_failoverTimeout: unsafe extern "C" fn (this: *const nsIProxyInfo, aFailoverTimeout: *mut libc::uint32_t) -> nsresult,

    /* attribute nsIProxyInfo failoverProxy; */
    pub get_failoverProxy: unsafe extern "C" fn (this: *const nsIProxyInfo, aFailoverProxy: *mut *const nsIProxyInfo) -> nsresult,
    pub set_failoverProxy: unsafe extern "C" fn (this: *const nsIProxyInfo, aFailoverProxy: *const nsIProxyInfo) -> nsresult,

}


impl nsIProxyInfo {
    /* readonly attribute AUTF8String host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* readonly attribute unsigned long flags; */
    #[inline]
    pub unsafe fn get_flags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_flags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long resolveFlags; */
    #[inline]
    pub unsafe fn get_resolveFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_resolveFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString username; */
    #[inline]
    pub unsafe fn get_username(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_username)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString password; */
    #[inline]
    pub unsafe fn get_password(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_password)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long failoverTimeout; */
    #[inline]
    pub unsafe fn get_failoverTimeout(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_failoverTimeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIProxyInfo failoverProxy; */
    #[inline]
    pub unsafe fn get_failoverProxy(&self, ) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_failoverProxy)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_failoverProxy(&self, aFailoverProxy: Option<&nsIProxyInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).set_failoverProxy)(self as *const _, aFailoverProxy.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


