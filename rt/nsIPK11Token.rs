//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPK11Token.idl
//


pub mod nsIPK11Token_consts {
    pub const ASK_EVERY_TIME: i64 = -1;
    pub const ASK_FIRST_TIME: i64 = 0;
    pub const ASK_EXPIRE_TIME: i64 = 1;
}


#[repr(C)]
pub struct nsIPK11Token {
    vtable: *const nsIPK11TokenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPK11Token {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x51191434, 0x1dd2, 0x11b2,
            [0xa1, 0x7c, 0xe4, 0x9c, 0x4e, 0x99, 0xa4, 0xe3])
    }
}

unsafe impl RefCounted for nsIPK11Token {
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
pub trait nsIPK11TokenCoerce {
    fn coerce_from(v: &nsIPK11Token) -> &Self;
}

impl nsIPK11TokenCoerce for nsIPK11Token {
    #[inline]
    fn coerce_from(v: &nsIPK11Token) -> &Self {
        v
    }
}

impl nsIPK11Token {
    #[inline]
    pub fn coerce<T: nsIPK11TokenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPK11Token {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPK11TokenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPK11Token) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPK11TokenVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String tokenName; */
    pub get_tokenName: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String tokenLabel; */
    pub get_tokenLabel: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenLabel: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String tokenManID; */
    pub get_tokenManID: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenManID: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String tokenHWVersion; */
    pub get_tokenHWVersion: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenHWVersion: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String tokenFWVersion; */
    pub get_tokenFWVersion: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenFWVersion: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String tokenSerialNumber; */
    pub get_tokenSerialNumber: unsafe extern "C" fn (this: *const nsIPK11Token, aTokenSerialNumber: *mut nsACString) -> nsresult,

    /* boolean isLoggedIn (); */
    pub isLoggedIn: unsafe extern "C" fn (this: *const nsIPK11Token, _retval: *mut bool) -> nsresult,

    /* void login (in boolean force); */
    pub login: unsafe extern "C" fn (this: *const nsIPK11Token, force: bool) -> nsresult,

    /* void logoutSimple (); */
    pub logoutSimple: unsafe extern "C" fn (this: *const nsIPK11Token) -> nsresult,

    /* void logoutAndDropAuthenticatedResources (); */
    pub logoutAndDropAuthenticatedResources: unsafe extern "C" fn (this: *const nsIPK11Token) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIPK11Token) -> nsresult,

    /* readonly attribute long minimumPasswordLength; */
    pub get_minimumPasswordLength: unsafe extern "C" fn (this: *const nsIPK11Token, aMinimumPasswordLength: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean needsUserInit; */
    pub get_needsUserInit: unsafe extern "C" fn (this: *const nsIPK11Token, aNeedsUserInit: *mut bool) -> nsresult,

    /* boolean checkPassword (in AUTF8String password); */
    pub checkPassword: unsafe extern "C" fn (this: *const nsIPK11Token, password: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void initPassword (in AUTF8String initialPassword); */
    pub initPassword: unsafe extern "C" fn (this: *const nsIPK11Token, initialPassword: *const nsACString) -> nsresult,

    /* void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
    pub changePassword: unsafe extern "C" fn (this: *const nsIPK11Token, oldPassword: *const nsACString, newPassword: *const nsACString) -> nsresult,

    /* long getAskPasswordTimes (); */
    pub getAskPasswordTimes: unsafe extern "C" fn (this: *const nsIPK11Token, _retval: *mut libc::int32_t) -> nsresult,

    /* long getAskPasswordTimeout (); */
    pub getAskPasswordTimeout: unsafe extern "C" fn (this: *const nsIPK11Token, _retval: *mut libc::int32_t) -> nsresult,

    /* void setAskPasswordDefaults ([const] in long askTimes, [const] in long timeout); */
    pub setAskPasswordDefaults: unsafe extern "C" fn (this: *const nsIPK11Token, askTimes: libc::int32_t, timeout: libc::int32_t) -> nsresult,

    /* readonly attribute boolean hasPassword; */
    pub get_hasPassword: unsafe extern "C" fn (this: *const nsIPK11Token, aHasPassword: *mut bool) -> nsresult,

    /* boolean isHardwareToken (); */
    pub isHardwareToken: unsafe extern "C" fn (this: *const nsIPK11Token, _retval: *mut bool) -> nsresult,

    /* boolean needsLogin (); */
    pub needsLogin: unsafe extern "C" fn (this: *const nsIPK11Token, _retval: *mut bool) -> nsresult,

}


impl nsIPK11Token {
    /* readonly attribute AUTF8String tokenName; */
    #[inline]
    pub unsafe fn get_tokenName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String tokenLabel; */
    #[inline]
    pub unsafe fn get_tokenLabel(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenLabel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String tokenManID; */
    #[inline]
    pub unsafe fn get_tokenManID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenManID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String tokenHWVersion; */
    #[inline]
    pub unsafe fn get_tokenHWVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenHWVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String tokenFWVersion; */
    #[inline]
    pub unsafe fn get_tokenFWVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenFWVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String tokenSerialNumber; */
    #[inline]
    pub unsafe fn get_tokenSerialNumber(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_tokenSerialNumber)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isLoggedIn (); */
    #[inline]
    pub unsafe fn isLoggedIn(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLoggedIn)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void login (in boolean force); */
    #[inline]
    pub unsafe fn login(&self, force: bool) -> Result<(), nsresult> {

        match ((*self.vtable).login)(self as *const _, force) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void logoutSimple (); */
    #[inline]
    pub unsafe fn logoutSimple(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).logoutSimple)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void logoutAndDropAuthenticatedResources (); */
    #[inline]
    pub unsafe fn logoutAndDropAuthenticatedResources(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).logoutAndDropAuthenticatedResources)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long minimumPasswordLength; */
    #[inline]
    pub unsafe fn get_minimumPasswordLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_minimumPasswordLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean needsUserInit; */
    #[inline]
    pub unsafe fn get_needsUserInit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_needsUserInit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean checkPassword (in AUTF8String password); */
    #[inline]
    pub unsafe fn checkPassword(&self, password: &[u8]) -> Result<bool, nsresult> {
        let password = nsCString::from(password);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkPassword)(self as *const _, &*password, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void initPassword (in AUTF8String initialPassword); */
    #[inline]
    pub unsafe fn initPassword(&self, initialPassword: &[u8]) -> Result<(), nsresult> {
        let initialPassword = nsCString::from(initialPassword);
        match ((*self.vtable).initPassword)(self as *const _, &*initialPassword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
    #[inline]
    pub unsafe fn changePassword(&self, oldPassword: &[u8], newPassword: &[u8]) -> Result<(), nsresult> {
        let oldPassword = nsCString::from(oldPassword);
        let newPassword = nsCString::from(newPassword);
        match ((*self.vtable).changePassword)(self as *const _, &*oldPassword, &*newPassword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getAskPasswordTimes (); */
    #[inline]
    pub unsafe fn getAskPasswordTimes(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getAskPasswordTimes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getAskPasswordTimeout (); */
    #[inline]
    pub unsafe fn getAskPasswordTimeout(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getAskPasswordTimeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAskPasswordDefaults ([const] in long askTimes, [const] in long timeout); */
    #[inline]
    pub unsafe fn setAskPasswordDefaults(&self, askTimes: libc::int32_t, timeout: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAskPasswordDefaults)(self as *const _, askTimes, timeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean hasPassword; */
    #[inline]
    pub unsafe fn get_hasPassword(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasPassword)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isHardwareToken (); */
    #[inline]
    pub unsafe fn isHardwareToken(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isHardwareToken)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean needsLogin (); */
    #[inline]
    pub unsafe fn needsLogin(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).needsLogin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


