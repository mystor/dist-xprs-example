//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPrompt.idl
//


pub mod nsIAuthPrompt_consts {
    pub const SAVE_PASSWORD_NEVER: i64 = 0;
    pub const SAVE_PASSWORD_FOR_SESSION: i64 = 1;
    pub const SAVE_PASSWORD_PERMANENTLY: i64 = 2;
}


#[repr(C)]
pub struct nsIAuthPrompt {
    vtable: *const nsIAuthPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthPrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x358089f9, 0xee4b, 0x4711,
            [0x82, 0xfd, 0xbc, 0xd0, 0x7f, 0xc6, 0x20, 0x61])
    }
}

unsafe impl RefCounted for nsIAuthPrompt {
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
pub trait nsIAuthPromptCoerce {
    fn coerce_from(v: &nsIAuthPrompt) -> &Self;
}

impl nsIAuthPromptCoerce for nsIAuthPrompt {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt) -> &Self {
        v
    }
}

impl nsIAuthPrompt {
    #[inline]
    pub fn coerce<T: nsIAuthPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthPrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthPromptVTable {
    pub __base: nsISupportsVTable,

    /* boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result); */
    pub prompt: unsafe extern "C" fn (this: *const nsIAuthPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t, defaultText: *const libc::int16_t, result: *mut *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd); */
    pub promptUsernameAndPassword: unsafe extern "C" fn (this: *const nsIAuthPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t, user: *mut *const libc::int16_t, pwd: *mut *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd); */
    pub promptPassword: unsafe extern "C" fn (this: *const nsIAuthPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t, pwd: *mut *const libc::int16_t, _retval: *mut bool) -> nsresult,

}


impl nsIAuthPrompt {
    /* boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result); */
    #[inline]
    pub unsafe fn prompt(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t, defaultText: *const libc::int16_t) -> Result<(*const libc::int16_t, bool), nsresult> {
        let mut result: *const libc::int16_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).prompt)(self as *const _, dialogTitle, text, passwordRealm, savePassword, defaultText, &mut result as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((result, _retval))
    }

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd); */
    #[inline]
    pub unsafe fn promptUsernameAndPassword(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t) -> Result<(*const libc::int16_t, *const libc::int16_t, bool), nsresult> {
        let mut user: *const libc::int16_t = ::std::mem::zeroed();
        let mut pwd: *const libc::int16_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptUsernameAndPassword)(self as *const _, dialogTitle, text, passwordRealm, savePassword, &mut user as *mut _, &mut pwd as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((user, pwd, _retval))
    }

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd); */
    #[inline]
    pub unsafe fn promptPassword(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, passwordRealm: *const libc::int16_t, savePassword: uint32_t) -> Result<(*const libc::int16_t, bool), nsresult> {
        let mut pwd: *const libc::int16_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptPassword)(self as *const _, dialogTitle, text, passwordRealm, savePassword, &mut pwd as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((pwd, _retval))
    }

}


