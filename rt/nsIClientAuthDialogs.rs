//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIClientAuthDialogs.idl
//


#[repr(C)]
pub struct nsIClientAuthDialogs {
    vtable: *const nsIClientAuthDialogsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClientAuthDialogs {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfa4c7520, 0x1433, 0x11d5,
            [0xba, 0x24, 0x00, 0x10, 0x83, 0x03, 0xb1, 0x17])
    }
}

unsafe impl RefCounted for nsIClientAuthDialogs {
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
pub trait nsIClientAuthDialogsCoerce {
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self;
}

impl nsIClientAuthDialogsCoerce for nsIClientAuthDialogs {
    #[inline]
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self {
        v
    }
}

impl nsIClientAuthDialogs {
    #[inline]
    pub fn coerce<T: nsIClientAuthDialogsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClientAuthDialogs {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClientAuthDialogsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClientAuthDialogs) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClientAuthDialogsVTable {
    pub __base: nsISupportsVTable,

    /* boolean chooseCertificate (in nsIInterfaceRequestor ctx, in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex); */
    pub chooseCertificate: unsafe extern "C" fn (this: *const nsIClientAuthDialogs, ctx: *const nsIInterfaceRequestor, hostname: *const nsACString, port: libc::int32_t, organization: *const nsACString, issuerOrg: *const nsACString, certList: *const nsIArray, selectedIndex: *mut libc::uint32_t, _retval: *mut bool) -> nsresult,

}


impl nsIClientAuthDialogs {
    /* boolean chooseCertificate (in nsIInterfaceRequestor ctx, in AUTF8String hostname, in long port, in AUTF8String organization, in AUTF8String issuerOrg, in nsIArray certList, out unsigned long selectedIndex); */
    #[inline]
    pub unsafe fn chooseCertificate(&self, ctx: Option<&nsIInterfaceRequestor>, hostname: &[u8], port: libc::int32_t, organization: &[u8], issuerOrg: &[u8], certList: Option<&nsIArray>) -> Result<(libc::uint32_t, bool), nsresult> {
        let hostname = nsCString::from(hostname);
        let organization = nsCString::from(organization);
        let issuerOrg = nsCString::from(issuerOrg);
        let mut selectedIndex: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).chooseCertificate)(self as *const _, ctx.map_or(::std::ptr::null(), |x| x as *const _), &*hostname, port, &*organization, &*issuerOrg, certList.map_or(::std::ptr::null(), |x| x as *const _), &mut selectedIndex as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((selectedIndex, _retval))
    }

}


#[repr(C)]
pub struct nsIClientAuthUserDecision {
    vtable: *const nsIClientAuthUserDecisionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClientAuthUserDecision {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x95c4373e, 0xbdd4, 0x4a63,
            [0xb4, 0x31, 0xf5, 0xb0, 0x00, 0x36, 0x77, 0x21])
    }
}

unsafe impl RefCounted for nsIClientAuthUserDecision {
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
pub trait nsIClientAuthUserDecisionCoerce {
    fn coerce_from(v: &nsIClientAuthUserDecision) -> &Self;
}

impl nsIClientAuthUserDecisionCoerce for nsIClientAuthUserDecision {
    #[inline]
    fn coerce_from(v: &nsIClientAuthUserDecision) -> &Self {
        v
    }
}

impl nsIClientAuthUserDecision {
    #[inline]
    pub fn coerce<T: nsIClientAuthUserDecisionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClientAuthUserDecision {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClientAuthUserDecisionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClientAuthUserDecision) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClientAuthUserDecisionVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean rememberClientAuthCertificate; */
    pub get_rememberClientAuthCertificate: unsafe extern "C" fn (this: *const nsIClientAuthUserDecision, aRememberClientAuthCertificate: *mut bool) -> nsresult,
    pub set_rememberClientAuthCertificate: unsafe extern "C" fn (this: *const nsIClientAuthUserDecision, aRememberClientAuthCertificate: bool) -> nsresult,

}


impl nsIClientAuthUserDecision {
    /* attribute boolean rememberClientAuthCertificate; */
    #[inline]
    pub unsafe fn get_rememberClientAuthCertificate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rememberClientAuthCertificate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rememberClientAuthCertificate(&self, aRememberClientAuthCertificate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_rememberClientAuthCertificate)(self as *const _, aRememberClientAuthCertificate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


