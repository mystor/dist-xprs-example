//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthModule.idl
//


pub mod nsIAuthModule_consts {
    pub const REQ_DEFAULT: i64 = 0;
    pub const REQ_MUTUAL_AUTH: i64 = 1;
    pub const REQ_DELEGATE: i64 = 2;
    pub const REQ_PROXY_AUTH: i64 = 4;
    pub const NTLM_MODULE_SAMBA_AUTH_PROXY: i64 = 0;
    pub const NTLM_MODULE_SAMBA_AUTH_DIRECT: i64 = 1;
    pub const NTLM_MODULE_WIN_API_PROXY: i64 = 2;
    pub const NTLM_MODULE_WIN_API_DIRECT: i64 = 3;
    pub const NTLM_MODULE_GENERIC_PROXY: i64 = 4;
    pub const NTLM_MODULE_GENERIC_DIRECT: i64 = 5;
    pub const NTLM_MODULE_KERBEROS_PROXY: i64 = 6;
    pub const NTLM_MODULE_KERBEROS_DIRECT: i64 = 7;
}


#[repr(C)]
pub struct nsIAuthModule {
    vtable: *const nsIAuthModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthModule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6e35dbc0, 0x49ef, 0x4e2c,
            [0xb1, 0xea, 0xb7, 0x2e, 0xc6, 0x44, 0x50, 0xa2])
    }
}

unsafe impl RefCounted for nsIAuthModule {
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
pub trait nsIAuthModuleCoerce {
    fn coerce_from(v: &nsIAuthModule) -> &Self;
}

impl nsIAuthModuleCoerce for nsIAuthModule {
    #[inline]
    fn coerce_from(v: &nsIAuthModule) -> &Self {
        v
    }
}

impl nsIAuthModule {
    #[inline]
    pub fn coerce<T: nsIAuthModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthModule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthModule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthModuleVTable {
    pub __base: nsISupportsVTable,

    /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
    pub init: unsafe extern "C" fn (this: *const nsIAuthModule, aServiceName: *const libc::c_char, aServiceFlags: libc::uint32_t, aDomain: *const libc::int16_t, aUsername: *const libc::int16_t, aPassword: *const libc::int16_t) -> nsresult,

    /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub getNextToken: unsafe extern "C" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t, aOutToken: *mut *const libc::c_void, aOutTokenLength: *mut libc::uint32_t) -> nsresult,

    /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub wrap: unsafe extern "C" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t, confidential: bool, aOutToken: *mut *const libc::c_void, aOutTokenLength: *mut libc::uint32_t) -> nsresult,

    /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub unwrap: unsafe extern "C" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t, aOutToken: *mut *const libc::c_void, aOutTokenLength: *mut libc::uint32_t) -> nsresult,

}


impl nsIAuthModule {
    /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
    #[inline]
    pub unsafe fn init(&self, aServiceName: *const libc::c_char, aServiceFlags: libc::uint32_t, aDomain: *const libc::int16_t, aUsername: *const libc::int16_t, aPassword: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aServiceName, aServiceFlags, aDomain, aUsername, aPassword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    #[inline]
    pub unsafe fn getNextToken(&self, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t) -> Result<(*const libc::c_void, libc::uint32_t), nsresult> {
        let mut aOutToken: *const libc::c_void = ::std::ptr::null();
        let mut aOutTokenLength: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getNextToken)(self as *const _, aInToken, aInTokenLength, &mut aOutToken as *mut _, &mut aOutTokenLength as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aOutToken, aOutTokenLength))
    }

    /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    #[inline]
    pub unsafe fn wrap(&self, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t, confidential: bool) -> Result<(*const libc::c_void, libc::uint32_t), nsresult> {
        let mut aOutToken: *const libc::c_void = ::std::ptr::null();
        let mut aOutTokenLength: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).wrap)(self as *const _, aInToken, aInTokenLength, confidential, &mut aOutToken as *mut _, &mut aOutTokenLength as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aOutToken, aOutTokenLength))
    }

    /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    #[inline]
    pub unsafe fn unwrap(&self, aInToken: *const libc::c_void, aInTokenLength: libc::uint32_t) -> Result<(*const libc::c_void, libc::uint32_t), nsresult> {
        let mut aOutToken: *const libc::c_void = ::std::ptr::null();
        let mut aOutTokenLength: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).unwrap)(self as *const _, aInToken, aInTokenLength, &mut aOutToken as *mut _, &mut aOutTokenLength as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aOutToken, aOutTokenLength))
    }

}


