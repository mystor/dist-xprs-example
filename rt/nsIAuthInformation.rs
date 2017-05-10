//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthInformation.idl
//


pub mod nsIAuthInformation_consts {
    pub const AUTH_HOST: i64 = 1;
    pub const AUTH_PROXY: i64 = 2;
    pub const NEED_DOMAIN: i64 = 4;
    pub const ONLY_PASSWORD: i64 = 8;
    pub const PREVIOUS_FAILED: i64 = 16;
    pub const CROSS_ORIGIN_SUB_RESOURCE: i64 = 32;
}


#[repr(C)]
pub struct nsIAuthInformation {
    vtable: *const nsIAuthInformationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthInformation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0d73639c, 0x2a92, 0x4518,
            [0x9f, 0x92, 0x28, 0xf7, 0x1f, 0xea, 0x5f, 0x20])
    }
}

unsafe impl RefCounted for nsIAuthInformation {
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
pub trait nsIAuthInformationCoerce {
    fn coerce_from(v: &nsIAuthInformation) -> &Self;
}

impl nsIAuthInformationCoerce for nsIAuthInformation {
    #[inline]
    fn coerce_from(v: &nsIAuthInformation) -> &Self {
        v
    }
}

impl nsIAuthInformation {
    #[inline]
    pub fn coerce<T: nsIAuthInformationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthInformation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthInformationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthInformation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthInformationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long flags; */
    pub get_flags: unsafe extern "C" fn (this: *const nsIAuthInformation, aFlags: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute AString realm; */
    pub get_realm: unsafe extern "C" fn (this: *const nsIAuthInformation, aRealm: *mut nsAString) -> nsresult,

    /* readonly attribute AUTF8String authenticationScheme; */
    pub get_authenticationScheme: unsafe extern "C" fn (this: *const nsIAuthInformation, aAuthenticationScheme: *mut nsACString) -> nsresult,

    /* attribute AString username; */
    pub get_username: unsafe extern "C" fn (this: *const nsIAuthInformation, aUsername: *mut nsAString) -> nsresult,
    pub set_username: unsafe extern "C" fn (this: *const nsIAuthInformation, aUsername: *const nsAString) -> nsresult,

    /* attribute AString password; */
    pub get_password: unsafe extern "C" fn (this: *const nsIAuthInformation, aPassword: *mut nsAString) -> nsresult,
    pub set_password: unsafe extern "C" fn (this: *const nsIAuthInformation, aPassword: *const nsAString) -> nsresult,

    /* attribute AString domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsIAuthInformation, aDomain: *mut nsAString) -> nsresult,
    pub set_domain: unsafe extern "C" fn (this: *const nsIAuthInformation, aDomain: *const nsAString) -> nsresult,

}


impl nsIAuthInformation {
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

    /* readonly attribute AString realm; */
    #[inline]
    pub unsafe fn get_realm(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_realm)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String authenticationScheme; */
    #[inline]
    pub unsafe fn get_authenticationScheme(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_authenticationScheme)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString username; */
    #[inline]
    pub unsafe fn get_username(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_username)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_username(&self, aUsername: &[u16]) -> Result<(), nsresult> {
        let aUsername = nsString::from(aUsername);
        match ((*self.vtable).set_username)(self as *const _, &*aUsername) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString password; */
    #[inline]
    pub unsafe fn get_password(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_password)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_password(&self, aPassword: &[u16]) -> Result<(), nsresult> {
        let aPassword = nsString::from(aPassword);
        match ((*self.vtable).set_password)(self as *const _, &*aPassword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_domain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_domain(&self, aDomain: &[u16]) -> Result<(), nsresult> {
        let aDomain = nsString::from(aDomain);
        match ((*self.vtable).set_domain)(self as *const _, &*aDomain) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


