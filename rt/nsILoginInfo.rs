//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoginInfo.idl
//


#[repr(C)]
pub struct nsILoginInfo {
    vtable: *const nsILoginInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoginInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc41b7dff, 0x6b9b, 0x42fe,
            [0xb7, 0x8d, 0x11, 0x30, 0x51, 0xfa, 0xcb, 0x05])
    }
}

unsafe impl RefCounted for nsILoginInfo {
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
pub trait nsILoginInfoCoerce {
    fn coerce_from(v: &nsILoginInfo) -> &Self;
}

impl nsILoginInfoCoerce for nsILoginInfo {
    #[inline]
    fn coerce_from(v: &nsILoginInfo) -> &Self {
        v
    }
}

impl nsILoginInfo {
    #[inline]
    pub fn coerce<T: nsILoginInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoginInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoginInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoginInfoVTable {
    pub __base: nsISupportsVTable,

    /* attribute AString hostname; */
    pub get_hostname: unsafe extern "C" fn (this: *const nsILoginInfo, aHostname: *mut nsAString) -> nsresult,
    pub set_hostname: unsafe extern "C" fn (this: *const nsILoginInfo, aHostname: *const nsAString) -> nsresult,

    /* attribute AString formSubmitURL; */
    pub get_formSubmitURL: unsafe extern "C" fn (this: *const nsILoginInfo, aFormSubmitURL: *mut nsAString) -> nsresult,
    pub set_formSubmitURL: unsafe extern "C" fn (this: *const nsILoginInfo, aFormSubmitURL: *const nsAString) -> nsresult,

    /* attribute AString httpRealm; */
    pub get_httpRealm: unsafe extern "C" fn (this: *const nsILoginInfo, aHttpRealm: *mut nsAString) -> nsresult,
    pub set_httpRealm: unsafe extern "C" fn (this: *const nsILoginInfo, aHttpRealm: *const nsAString) -> nsresult,

    /* attribute AString username; */
    pub get_username: unsafe extern "C" fn (this: *const nsILoginInfo, aUsername: *mut nsAString) -> nsresult,
    pub set_username: unsafe extern "C" fn (this: *const nsILoginInfo, aUsername: *const nsAString) -> nsresult,

    /* attribute AString usernameField; */
    pub get_usernameField: unsafe extern "C" fn (this: *const nsILoginInfo, aUsernameField: *mut nsAString) -> nsresult,
    pub set_usernameField: unsafe extern "C" fn (this: *const nsILoginInfo, aUsernameField: *const nsAString) -> nsresult,

    /* attribute AString password; */
    pub get_password: unsafe extern "C" fn (this: *const nsILoginInfo, aPassword: *mut nsAString) -> nsresult,
    pub set_password: unsafe extern "C" fn (this: *const nsILoginInfo, aPassword: *const nsAString) -> nsresult,

    /* attribute AString passwordField; */
    pub get_passwordField: unsafe extern "C" fn (this: *const nsILoginInfo, aPasswordField: *mut nsAString) -> nsresult,
    pub set_passwordField: unsafe extern "C" fn (this: *const nsILoginInfo, aPasswordField: *const nsAString) -> nsresult,

    /* void init (in AString aHostname, in AString aFormSubmitURL, in AString aHttpRealm, in AString aUsername, in AString aPassword, in AString aUsernameField, in AString aPasswordField); */
    pub init: unsafe extern "C" fn (this: *const nsILoginInfo, aHostname: *const nsAString, aFormSubmitURL: *const nsAString, aHttpRealm: *const nsAString, aUsername: *const nsAString, aPassword: *const nsAString, aUsernameField: *const nsAString, aPasswordField: *const nsAString) -> nsresult,

    /* boolean equals (in nsILoginInfo aLoginInfo); */
    pub equals: unsafe extern "C" fn (this: *const nsILoginInfo, aLoginInfo: *const nsILoginInfo, _retval: *mut bool) -> nsresult,

    /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
    pub matches: unsafe extern "C" fn (this: *const nsILoginInfo, aLoginInfo: *const nsILoginInfo, ignorePassword: bool, _retval: *mut bool) -> nsresult,

    /* nsILoginInfo clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsILoginInfo, _retval: *mut *const nsILoginInfo) -> nsresult,

}


impl nsILoginInfo {
    /* attribute AString hostname; */
    #[inline]
    pub unsafe fn get_hostname(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hostname)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hostname(&self, aHostname: &[u16]) -> Result<(), nsresult> {
        let aHostname = nsString::from(aHostname);
        match ((*self.vtable).set_hostname)(self as *const _, &*aHostname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString formSubmitURL; */
    #[inline]
    pub unsafe fn get_formSubmitURL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_formSubmitURL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formSubmitURL(&self, aFormSubmitURL: &[u16]) -> Result<(), nsresult> {
        let aFormSubmitURL = nsString::from(aFormSubmitURL);
        match ((*self.vtable).set_formSubmitURL)(self as *const _, &*aFormSubmitURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString httpRealm; */
    #[inline]
    pub unsafe fn get_httpRealm(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_httpRealm)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_httpRealm(&self, aHttpRealm: &[u16]) -> Result<(), nsresult> {
        let aHttpRealm = nsString::from(aHttpRealm);
        match ((*self.vtable).set_httpRealm)(self as *const _, &*aHttpRealm) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* attribute AString usernameField; */
    #[inline]
    pub unsafe fn get_usernameField(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_usernameField)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_usernameField(&self, aUsernameField: &[u16]) -> Result<(), nsresult> {
        let aUsernameField = nsString::from(aUsernameField);
        match ((*self.vtable).set_usernameField)(self as *const _, &*aUsernameField) {
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

    /* attribute AString passwordField; */
    #[inline]
    pub unsafe fn get_passwordField(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_passwordField)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_passwordField(&self, aPasswordField: &[u16]) -> Result<(), nsresult> {
        let aPasswordField = nsString::from(aPasswordField);
        match ((*self.vtable).set_passwordField)(self as *const _, &*aPasswordField) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void init (in AString aHostname, in AString aFormSubmitURL, in AString aHttpRealm, in AString aUsername, in AString aPassword, in AString aUsernameField, in AString aPasswordField); */
    #[inline]
    pub unsafe fn init(&self, aHostname: &[u16], aFormSubmitURL: &[u16], aHttpRealm: &[u16], aUsername: &[u16], aPassword: &[u16], aUsernameField: &[u16], aPasswordField: &[u16]) -> Result<(), nsresult> {
        let aHostname = nsString::from(aHostname);
        let aFormSubmitURL = nsString::from(aFormSubmitURL);
        let aHttpRealm = nsString::from(aHttpRealm);
        let aUsername = nsString::from(aUsername);
        let aPassword = nsString::from(aPassword);
        let aUsernameField = nsString::from(aUsernameField);
        let aPasswordField = nsString::from(aPasswordField);
        match ((*self.vtable).init)(self as *const _, &*aHostname, &*aFormSubmitURL, &*aHttpRealm, &*aUsername, &*aPassword, &*aUsernameField, &*aPasswordField) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean equals (in nsILoginInfo aLoginInfo); */
    #[inline]
    pub unsafe fn equals(&self, aLoginInfo: Option<&nsILoginInfo>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, aLoginInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
    #[inline]
    pub unsafe fn matches(&self, aLoginInfo: Option<&nsILoginInfo>, ignorePassword: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).matches)(self as *const _, aLoginInfo.map_or(::std::ptr::null(), |x| x as *const _), ignorePassword, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsILoginInfo clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsILoginInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


