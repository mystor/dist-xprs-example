//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISessionStartup.idl
//


pub mod nsISessionStartup_consts {
    pub const NO_SESSION: i64 = 0;
    pub const RECOVER_SESSION: i64 = 1;
    pub const RESUME_SESSION: i64 = 2;
    pub const DEFER_SESSION: i64 = 3;
}


#[repr(C)]
pub struct nsISessionStartup {
    vtable: *const nsISessionStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISessionStartup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x934697e4, 0x3807, 0x47f8,
            [0xb6, 0xc9, 0x6c, 0xaa, 0x8d, 0x83, 0xcc, 0xd1])
    }
}

unsafe impl RefCounted for nsISessionStartup {
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
pub trait nsISessionStartupCoerce {
    fn coerce_from(v: &nsISessionStartup) -> &Self;
}

impl nsISessionStartupCoerce for nsISessionStartup {
    #[inline]
    fn coerce_from(v: &nsISessionStartup) -> &Self {
        v
    }
}

impl nsISessionStartup {
    #[inline]
    pub fn coerce<T: nsISessionStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISessionStartup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISessionStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISessionStartup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISessionStartupVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute jsval onceInitialized; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onceInitialized: *const ::libc::c_void,

    /* readonly attribute jsval state; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_state: *const ::libc::c_void,

    /* boolean doRestore (); */
    pub doRestore: unsafe extern "C" fn (this: *const nsISessionStartup, _retval: *mut bool) -> nsresult,

    /* boolean isAutomaticRestoreEnabled (); */
    pub isAutomaticRestoreEnabled: unsafe extern "C" fn (this: *const nsISessionStartup, _retval: *mut bool) -> nsresult,

    /* readonly attribute bool willOverrideHomepage; */
    pub get_willOverrideHomepage: unsafe extern "C" fn (this: *const nsISessionStartup, aWillOverrideHomepage: *mut bool) -> nsresult,

    /* readonly attribute unsigned long sessionType; */
    pub get_sessionType: unsafe extern "C" fn (this: *const nsISessionStartup, aSessionType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute bool previousSessionCrashed; */
    pub get_previousSessionCrashed: unsafe extern "C" fn (this: *const nsISessionStartup, aPreviousSessionCrashed: *mut bool) -> nsresult,

}


impl nsISessionStartup {
    /* readonly attribute jsval onceInitialized; */


    /* readonly attribute jsval state; */


    /* boolean doRestore (); */
    #[inline]
    pub unsafe fn doRestore(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).doRestore)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isAutomaticRestoreEnabled (); */
    #[inline]
    pub unsafe fn isAutomaticRestoreEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAutomaticRestoreEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool willOverrideHomepage; */
    #[inline]
    pub unsafe fn get_willOverrideHomepage(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_willOverrideHomepage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long sessionType; */
    #[inline]
    pub unsafe fn get_sessionType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sessionType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool previousSessionCrashed; */
    #[inline]
    pub unsafe fn get_previousSessionCrashed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_previousSessionCrashed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


