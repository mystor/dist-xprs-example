//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISound.idl
//


pub mod nsISound_consts {
    pub const EVENT_NEW_MAIL_RECEIVED: i64 = 0;
    pub const EVENT_ALERT_DIALOG_OPEN: i64 = 1;
    pub const EVENT_CONFIRM_DIALOG_OPEN: i64 = 2;
    pub const EVENT_PROMPT_DIALOG_OPEN: i64 = 3;
    pub const EVENT_SELECT_DIALOG_OPEN: i64 = 4;
    pub const EVENT_MENU_EXECUTE: i64 = 5;
    pub const EVENT_MENU_POPUP: i64 = 6;
    pub const EVENT_EDITOR_MAX_LEN: i64 = 7;
}


#[repr(C)]
pub struct nsISound {
    vtable: *const nsISoundVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISound {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc3c28d92, 0xa17f, 0x43df,
            [0x97, 0x6d, 0x4e, 0xea, 0xe6, 0xf9, 0x95, 0xfc])
    }
}

unsafe impl RefCounted for nsISound {
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
pub trait nsISoundCoerce {
    fn coerce_from(v: &nsISound) -> &Self;
}

impl nsISoundCoerce for nsISound {
    #[inline]
    fn coerce_from(v: &nsISound) -> &Self {
        v
    }
}

impl nsISound {
    #[inline]
    pub fn coerce<T: nsISoundCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISound {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISoundCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISound) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISoundVTable {
    pub __base: nsISupportsVTable,

    /* void play (in nsIURL aURL); */
    pub play: unsafe extern "C" fn (this: *const nsISound, aURL: *const nsIURL) -> nsresult,

    /* void playSystemSound (in AString soundAlias); */
    pub playSystemSound: unsafe extern "C" fn (this: *const nsISound, soundAlias: *const nsAString) -> nsresult,

    /* void beep (); */
    pub beep: unsafe extern "C" fn (this: *const nsISound) -> nsresult,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsISound) -> nsresult,

    /* void playEventSound (in unsigned long aEventId); */
    pub playEventSound: unsafe extern "C" fn (this: *const nsISound, aEventId: libc::uint32_t) -> nsresult,

}


impl nsISound {
    /* void play (in nsIURL aURL); */
    #[inline]
    pub unsafe fn play(&self, aURL: Option<&nsIURL>) -> Result<(), nsresult> {

        match ((*self.vtable).play)(self as *const _, aURL.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void playSystemSound (in AString soundAlias); */
    #[inline]
    pub unsafe fn playSystemSound(&self, soundAlias: &[u16]) -> Result<(), nsresult> {
        let soundAlias = nsString::from(soundAlias);
        match ((*self.vtable).playSystemSound)(self as *const _, &*soundAlias) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beep (); */
    #[inline]
    pub unsafe fn beep(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beep)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void playEventSound (in unsigned long aEventId); */
    #[inline]
    pub unsafe fn playEventSound(&self, aEventId: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).playEventSound)(self as *const _, aEventId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


