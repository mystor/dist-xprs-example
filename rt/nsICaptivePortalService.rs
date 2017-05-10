//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICaptivePortalService.idl
//


#[repr(C)]
pub struct nsICaptivePortalServiceCallback {
    vtable: *const nsICaptivePortalServiceCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICaptivePortalServiceCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb5fd5629, 0xd04c, 0x4138,
            [0x95, 0x29, 0x93, 0x11, 0xf2, 0x91, 0xec, 0xd4])
    }
}

unsafe impl RefCounted for nsICaptivePortalServiceCallback {
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
pub trait nsICaptivePortalServiceCallbackCoerce {
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self;
}

impl nsICaptivePortalServiceCallbackCoerce for nsICaptivePortalServiceCallback {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self {
        v
    }
}

impl nsICaptivePortalServiceCallback {
    #[inline]
    pub fn coerce<T: nsICaptivePortalServiceCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICaptivePortalServiceCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICaptivePortalServiceCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalServiceCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICaptivePortalServiceCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in bool success, in nsresult error); */
    pub complete: unsafe extern "C" fn (this: *const nsICaptivePortalServiceCallback, success: bool, error: nsresult) -> nsresult,

}


impl nsICaptivePortalServiceCallback {
    /* void complete (in bool success, in nsresult error); */
    #[inline]
    pub unsafe fn complete(&self, success: bool, error: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, success, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsICaptivePortalService_consts {
    pub const UNKNOWN: i64 = 0;
    pub const NOT_CAPTIVE: i64 = 1;
    pub const UNLOCKED_PORTAL: i64 = 2;
    pub const LOCKED_PORTAL: i64 = 3;
}


#[repr(C)]
pub struct nsICaptivePortalService {
    vtable: *const nsICaptivePortalServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICaptivePortalService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbdbe0555, 0xfc3d, 0x4f7b,
            [0x92, 0x05, 0xc3, 0x09, 0xce, 0xb2, 0xd6, 0x41])
    }
}

unsafe impl RefCounted for nsICaptivePortalService {
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
pub trait nsICaptivePortalServiceCoerce {
    fn coerce_from(v: &nsICaptivePortalService) -> &Self;
}

impl nsICaptivePortalServiceCoerce for nsICaptivePortalService {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalService) -> &Self {
        v
    }
}

impl nsICaptivePortalService {
    #[inline]
    pub fn coerce<T: nsICaptivePortalServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICaptivePortalService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICaptivePortalServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICaptivePortalServiceVTable {
    pub __base: nsISupportsVTable,

    /* void recheckCaptivePortal (); */
    pub recheckCaptivePortal: unsafe extern "C" fn (this: *const nsICaptivePortalService) -> nsresult,

    /* readonly attribute long state; */
    pub get_state: unsafe extern "C" fn (this: *const nsICaptivePortalService, aState: *mut libc::int32_t) -> nsresult,

    /* readonly attribute unsigned long long lastChecked; */
    pub get_lastChecked: unsafe extern "C" fn (this: *const nsICaptivePortalService, aLastChecked: *mut libc::uint64_t) -> nsresult,

}


impl nsICaptivePortalService {
    /* void recheckCaptivePortal (); */
    #[inline]
    pub unsafe fn recheckCaptivePortal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).recheckCaptivePortal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long lastChecked; */
    #[inline]
    pub unsafe fn get_lastChecked(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastChecked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


