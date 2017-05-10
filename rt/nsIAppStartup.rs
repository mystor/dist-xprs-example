//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAppStartup.idl
//


pub mod nsIAppStartup_consts {
    pub const eConsiderQuit: i64 = 1;
    pub const eAttemptQuit: i64 = 2;
    pub const eForceQuit: i64 = 3;
    pub const eRestart: i64 = 16;
    pub const eRestarti386: i64 = 32;
    pub const eRestartx86_64: i64 = 64;
    pub const eRestartNotSameProfile: i64 = 256;
}


#[repr(C)]
pub struct nsIAppStartup {
    vtable: *const nsIAppStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAppStartup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6621f6d5, 0x6c04, 0x4a0e,
            [0x9e, 0x74, 0x44, 0x7d, 0xb2, 0x21, 0x48, 0x4e])
    }
}

unsafe impl RefCounted for nsIAppStartup {
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
pub trait nsIAppStartupCoerce {
    fn coerce_from(v: &nsIAppStartup) -> &Self;
}

impl nsIAppStartupCoerce for nsIAppStartup {
    #[inline]
    fn coerce_from(v: &nsIAppStartup) -> &Self {
        v
    }
}

impl nsIAppStartup {
    #[inline]
    pub fn coerce<T: nsIAppStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAppStartup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAppStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppStartup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAppStartupVTable {
    pub __base: nsISupportsVTable,

    /* void createHiddenWindow (); */
    pub createHiddenWindow: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* void destroyHiddenWindow (); */
    pub destroyHiddenWindow: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* void run (); */
    pub run: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* void enterLastWindowClosingSurvivalArea (); */
    pub enterLastWindowClosingSurvivalArea: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* void exitLastWindowClosingSurvivalArea (); */
    pub exitLastWindowClosingSurvivalArea: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* readonly attribute boolean automaticSafeModeNecessary; */
    pub get_automaticSafeModeNecessary: unsafe extern "C" fn (this: *const nsIAppStartup, aAutomaticSafeModeNecessary: *mut bool) -> nsresult,

    /* void restartInSafeMode (in uint32_t aQuitMode); */
    pub restartInSafeMode: unsafe extern "C" fn (this: *const nsIAppStartup, aQuitMode: uint32_t) -> nsresult,

    /* void createInstanceWithProfile (in nsIToolkitProfile aProfile); */
    pub createInstanceWithProfile: unsafe extern "C" fn (this: *const nsIAppStartup, aProfile: *const nsIToolkitProfile) -> nsresult,

    /* bool trackStartupCrashBegin (); */
    pub trackStartupCrashBegin: unsafe extern "C" fn (this: *const nsIAppStartup, _retval: *mut bool) -> nsresult,

    /* void trackStartupCrashEnd (); */
    pub trackStartupCrashEnd: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* void quit (in uint32_t aMode); */
    pub quit: unsafe extern "C" fn (this: *const nsIAppStartup, aMode: uint32_t) -> nsresult,

    /* readonly attribute boolean shuttingDown; */
    pub get_shuttingDown: unsafe extern "C" fn (this: *const nsIAppStartup, aShuttingDown: *mut bool) -> nsresult,

    /* readonly attribute boolean startingUp; */
    pub get_startingUp: unsafe extern "C" fn (this: *const nsIAppStartup, aStartingUp: *mut bool) -> nsresult,

    /* [noscript] void doneStartingUp (); */
    pub doneStartingUp: unsafe extern "C" fn (this: *const nsIAppStartup) -> nsresult,

    /* readonly attribute boolean restarting; */
    pub get_restarting: unsafe extern "C" fn (this: *const nsIAppStartup, aRestarting: *mut bool) -> nsresult,

    /* readonly attribute boolean wasRestarted; */
    pub get_wasRestarted: unsafe extern "C" fn (this: *const nsIAppStartup, aWasRestarted: *mut bool) -> nsresult,

    /* [implicit_jscontext] jsval getStartupInfo (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getStartupInfo: *const ::libc::c_void,

    /* attribute boolean interrupted; */
    pub get_interrupted: unsafe extern "C" fn (this: *const nsIAppStartup, aInterrupted: *mut bool) -> nsresult,
    pub set_interrupted: unsafe extern "C" fn (this: *const nsIAppStartup, aInterrupted: bool) -> nsresult,

}


impl nsIAppStartup {
    /* void createHiddenWindow (); */
    #[inline]
    pub unsafe fn createHiddenWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).createHiddenWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void destroyHiddenWindow (); */
    #[inline]
    pub unsafe fn destroyHiddenWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroyHiddenWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void run (); */
    #[inline]
    pub unsafe fn run(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).run)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enterLastWindowClosingSurvivalArea (); */
    #[inline]
    pub unsafe fn enterLastWindowClosingSurvivalArea(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).enterLastWindowClosingSurvivalArea)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void exitLastWindowClosingSurvivalArea (); */
    #[inline]
    pub unsafe fn exitLastWindowClosingSurvivalArea(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).exitLastWindowClosingSurvivalArea)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean automaticSafeModeNecessary; */
    #[inline]
    pub unsafe fn get_automaticSafeModeNecessary(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_automaticSafeModeNecessary)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void restartInSafeMode (in uint32_t aQuitMode); */
    #[inline]
    pub unsafe fn restartInSafeMode(&self, aQuitMode: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).restartInSafeMode)(self as *const _, aQuitMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createInstanceWithProfile (in nsIToolkitProfile aProfile); */
    #[inline]
    pub unsafe fn createInstanceWithProfile(&self, aProfile: Option<&nsIToolkitProfile>) -> Result<(), nsresult> {

        match ((*self.vtable).createInstanceWithProfile)(self as *const _, aProfile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool trackStartupCrashBegin (); */
    #[inline]
    pub unsafe fn trackStartupCrashBegin(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).trackStartupCrashBegin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void trackStartupCrashEnd (); */
    #[inline]
    pub unsafe fn trackStartupCrashEnd(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).trackStartupCrashEnd)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void quit (in uint32_t aMode); */
    #[inline]
    pub unsafe fn quit(&self, aMode: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).quit)(self as *const _, aMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean shuttingDown; */
    #[inline]
    pub unsafe fn get_shuttingDown(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_shuttingDown)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean startingUp; */
    #[inline]
    pub unsafe fn get_startingUp(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_startingUp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void doneStartingUp (); */
    #[inline]
    pub unsafe fn doneStartingUp(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).doneStartingUp)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean restarting; */
    #[inline]
    pub unsafe fn get_restarting(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_restarting)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean wasRestarted; */
    #[inline]
    pub unsafe fn get_wasRestarted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_wasRestarted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval getStartupInfo (); */


    /* attribute boolean interrupted; */
    #[inline]
    pub unsafe fn get_interrupted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_interrupted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_interrupted(&self, aInterrupted: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_interrupted)(self as *const _, aInterrupted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


