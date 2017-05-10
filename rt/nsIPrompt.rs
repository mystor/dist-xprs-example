//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrompt.idl
//


pub mod nsIPrompt_consts {
    pub const BUTTON_POS_0: i64 = 1;
    pub const BUTTON_POS_1: i64 = 256;
    pub const BUTTON_POS_2: i64 = 65536;
    pub const BUTTON_TITLE_OK: i64 = 1;
    pub const BUTTON_TITLE_CANCEL: i64 = 2;
    pub const BUTTON_TITLE_YES: i64 = 3;
    pub const BUTTON_TITLE_NO: i64 = 4;
    pub const BUTTON_TITLE_SAVE: i64 = 5;
    pub const BUTTON_TITLE_DONT_SAVE: i64 = 6;
    pub const BUTTON_TITLE_REVERT: i64 = 7;
    pub const BUTTON_TITLE_IS_STRING: i64 = 127;
    pub const BUTTON_POS_0_DEFAULT: i64 = 0;
    pub const BUTTON_POS_1_DEFAULT: i64 = 16777216;
    pub const BUTTON_POS_2_DEFAULT: i64 = 33554432;
    pub const BUTTON_DELAY_ENABLE: i64 = 67108864;
    pub const STD_OK_CANCEL_BUTTONS: i64 = 513;
    pub const STD_YES_NO_BUTTONS: i64 = 1027;
}


#[repr(C)]
pub struct nsIPrompt {
    vtable: *const nsIPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa63f70c0, 0x148b, 0x11d3,
            [0x93, 0x33, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40])
    }
}

unsafe impl RefCounted for nsIPrompt {
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
pub trait nsIPromptCoerce {
    fn coerce_from(v: &nsIPrompt) -> &Self;
}

impl nsIPromptCoerce for nsIPrompt {
    #[inline]
    fn coerce_from(v: &nsIPrompt) -> &Self {
        v
    }
}

impl nsIPrompt {
    #[inline]
    pub fn coerce<T: nsIPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPromptVTable {
    pub __base: nsISupportsVTable,

    /* void alert (in wstring dialogTitle, in wstring text); */
    pub alert: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t) -> nsresult,

    /* void alertCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    pub alertCheck: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool) -> nsresult,

    /* boolean confirm (in wstring dialogTitle, in wstring text); */
    pub confirm: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* boolean confirmCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    pub confirmCheck: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool, _retval: *mut bool) -> nsresult,

    /* int32_t confirmEx (in wstring dialogTitle, in wstring text, in unsigned long buttonFlags, in wstring button0Title, in wstring button1Title, in wstring button2Title, in wstring checkMsg, inout boolean checkValue); */
    pub confirmEx: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, buttonFlags: libc::uint32_t, button0Title: *const libc::int16_t, button1Title: *const libc::int16_t, button2Title: *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool, _retval: *mut int32_t) -> nsresult,

    /* boolean prompt (in wstring dialogTitle, in wstring text, inout wstring value, in wstring checkMsg, inout boolean checkValue); */
    pub prompt: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, value: *mut *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    pub promptPassword: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, password: *mut *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, inout wstring username, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    pub promptUsernameAndPassword: unsafe extern "C" fn (this: *const nsIPrompt, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, username: *mut *const libc::int16_t, password: *mut *const libc::int16_t, checkMsg: *const libc::int16_t, checkValue: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean select (in wstring dialogTitle, in wstring text, in uint32_t count, [array, size_is (count)] in wstring selectList, out long outSelection); */
    /// Unable to call function as its signature contains a non-rust type
    pub select: *const ::libc::c_void,

}


impl nsIPrompt {
    /* void alert (in wstring dialogTitle, in wstring text); */
    #[inline]
    pub unsafe fn alert(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).alert)(self as *const _, dialogTitle, text) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void alertCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn alertCheck(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut checkValue: bool = ::std::mem::zeroed();
        match ((*self.vtable).alertCheck)(self as *const _, dialogTitle, text, checkMsg, &mut checkValue as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(checkValue)
    }

    /* boolean confirm (in wstring dialogTitle, in wstring text); */
    #[inline]
    pub unsafe fn confirm(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).confirm)(self as *const _, dialogTitle, text, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean confirmCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn confirmCheck(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<(bool, bool), nsresult> {
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).confirmCheck)(self as *const _, dialogTitle, text, checkMsg, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((checkValue, _retval))
    }

    /* int32_t confirmEx (in wstring dialogTitle, in wstring text, in unsigned long buttonFlags, in wstring button0Title, in wstring button1Title, in wstring button2Title, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn confirmEx(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, buttonFlags: libc::uint32_t, button0Title: *const libc::int16_t, button1Title: *const libc::int16_t, button2Title: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<(bool, int32_t), nsresult> {
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).confirmEx)(self as *const _, dialogTitle, text, buttonFlags, button0Title, button1Title, button2Title, checkMsg, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((checkValue, _retval))
    }

    /* boolean prompt (in wstring dialogTitle, in wstring text, inout wstring value, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn prompt(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, bool, bool), nsresult> {
        let mut value: *const libc::int16_t = ::std::mem::zeroed();
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).prompt)(self as *const _, dialogTitle, text, &mut value as *mut _, checkMsg, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((value, checkValue, _retval))
    }

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn promptPassword(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, bool, bool), nsresult> {
        let mut password: *const libc::int16_t = ::std::mem::zeroed();
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptPassword)(self as *const _, dialogTitle, text, &mut password as *mut _, checkMsg, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((password, checkValue, _retval))
    }

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, inout wstring username, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    #[inline]
    pub unsafe fn promptUsernameAndPassword(&self, dialogTitle: *const libc::int16_t, text: *const libc::int16_t, checkMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, *const libc::int16_t, bool, bool), nsresult> {
        let mut username: *const libc::int16_t = ::std::mem::zeroed();
        let mut password: *const libc::int16_t = ::std::mem::zeroed();
        let mut checkValue: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptUsernameAndPassword)(self as *const _, dialogTitle, text, &mut username as *mut _, &mut password as *mut _, checkMsg, &mut checkValue as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((username, password, checkValue, _retval))
    }

    /* boolean select (in wstring dialogTitle, in wstring text, in uint32_t count, [array, size_is (count)] in wstring selectList, out long outSelection); */


}


