//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPromptService.idl
//


pub mod nsIPromptService_consts {
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
pub struct nsIPromptService {
    vtable: *const nsIPromptServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPromptService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x404ebfa2, 0xd8f4, 0x4c94,
            [0x84, 0x16, 0xe6, 0x5a, 0x55, 0xf9, 0xdf, 0x5a])
    }
}

unsafe impl RefCounted for nsIPromptService {
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
pub trait nsIPromptServiceCoerce {
    fn coerce_from(v: &nsIPromptService) -> &Self;
}

impl nsIPromptServiceCoerce for nsIPromptService {
    #[inline]
    fn coerce_from(v: &nsIPromptService) -> &Self {
        v
    }
}

impl nsIPromptService {
    #[inline]
    pub fn coerce<T: nsIPromptServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPromptService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPromptServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPromptServiceVTable {
    pub __base: nsISupportsVTable,

    /* void alert (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    pub alert: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t) -> nsresult,

    /* void alertCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub alertCheck: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool) -> nsresult,

    /* boolean confirm (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    pub confirm: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* boolean confirmCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub confirmCheck: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool, _retval: *mut bool) -> nsresult,

    /* int32_t confirmEx (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
    pub confirmEx: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aButtonFlags: libc::uint32_t, aButton0Title: *const libc::int16_t, aButton1Title: *const libc::int16_t, aButton2Title: *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool, _retval: *mut int32_t) -> nsresult,

    /* boolean prompt (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
    pub prompt: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aValue: *mut *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean promptUsernameAndPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub promptUsernameAndPassword: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aUsername: *mut *const libc::int16_t, aPassword: *mut *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean promptPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub promptPassword: unsafe extern "C" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aPassword: *mut *const libc::int16_t, aCheckMsg: *const libc::int16_t, aCheckState: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean select (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in uint32_t aCount, [array, size_is (aCount)] in wstring aSelectList, out long aOutSelection); */
    /// Unable to call function as its signature contains a non-rust type
    pub select: *const ::libc::c_void,

}


impl nsIPromptService {
    /* void alert (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    #[inline]
    pub unsafe fn alert(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).alert)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void alertCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn alertCheck(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut aCheckState: bool = ::std::mem::zeroed();
        match ((*self.vtable).alertCheck)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, aCheckMsg, &mut aCheckState as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(aCheckState)
    }

    /* boolean confirm (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    #[inline]
    pub unsafe fn confirm(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).confirm)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean confirmCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn confirmCheck(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<(bool, bool), nsresult> {
        let mut aCheckState: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).confirmCheck)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, aCheckMsg, &mut aCheckState as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCheckState, _retval))
    }

    /* int32_t confirmEx (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn confirmEx(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aButtonFlags: libc::uint32_t, aButton0Title: *const libc::int16_t, aButton1Title: *const libc::int16_t, aButton2Title: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<(bool, int32_t), nsresult> {
        let mut aCheckState: bool = ::std::mem::zeroed();
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).confirmEx)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, &mut aCheckState as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCheckState, _retval))
    }

    /* boolean prompt (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn prompt(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, bool, bool), nsresult> {
        let mut aValue: *const libc::int16_t = ::std::mem::zeroed();
        let mut aCheckState: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).prompt)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, &mut aValue as *mut _, aCheckMsg, &mut aCheckState as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aValue, aCheckState, _retval))
    }

    /* boolean promptUsernameAndPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn promptUsernameAndPassword(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, *const libc::int16_t, bool, bool), nsresult> {
        let mut aUsername: *const libc::int16_t = ::std::mem::zeroed();
        let mut aPassword: *const libc::int16_t = ::std::mem::zeroed();
        let mut aCheckState: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptUsernameAndPassword)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, &mut aUsername as *mut _, &mut aPassword as *mut _, aCheckMsg, &mut aCheckState as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aUsername, aPassword, aCheckState, _retval))
    }

    /* boolean promptPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    #[inline]
    pub unsafe fn promptPassword(&self, aParent: Option<&mozIDOMWindowProxy>, aDialogTitle: *const libc::int16_t, aText: *const libc::int16_t, aCheckMsg: *const libc::int16_t) -> Result<(*const libc::int16_t, bool, bool), nsresult> {
        let mut aPassword: *const libc::int16_t = ::std::mem::zeroed();
        let mut aCheckState: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptPassword)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aDialogTitle, aText, &mut aPassword as *mut _, aCheckMsg, &mut aCheckState as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aPassword, aCheckState, _retval))
    }

    /* boolean select (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in uint32_t aCount, [array, size_is (aCount)] in wstring aSelectList, out long aOutSelection); */


}


