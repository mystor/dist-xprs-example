//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICaptivePortalDetector.idl
//


#[repr(C)]
pub struct nsICaptivePortalCallback {
    vtable: *const nsICaptivePortalCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICaptivePortalCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x593fdeec, 0x6284, 0x4de8,
            [0xb4, 0x16, 0x8e, 0x63, 0xcb, 0xdc, 0x69, 0x5e])
    }
}

unsafe impl RefCounted for nsICaptivePortalCallback {
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
pub trait nsICaptivePortalCallbackCoerce {
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self;
}

impl nsICaptivePortalCallbackCoerce for nsICaptivePortalCallback {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self {
        v
    }
}

impl nsICaptivePortalCallback {
    #[inline]
    pub fn coerce<T: nsICaptivePortalCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICaptivePortalCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICaptivePortalCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICaptivePortalCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void prepare (); */
    pub prepare: unsafe extern "C" fn (this: *const nsICaptivePortalCallback) -> nsresult,

    /* void complete (in bool success); */
    pub complete: unsafe extern "C" fn (this: *const nsICaptivePortalCallback, success: bool) -> nsresult,

}


impl nsICaptivePortalCallback {
    /* void prepare (); */
    #[inline]
    pub unsafe fn prepare(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).prepare)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void complete (in bool success); */
    #[inline]
    pub unsafe fn complete(&self, success: bool) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, success) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICaptivePortalDetector {
    vtable: *const nsICaptivePortalDetectorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICaptivePortalDetector {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2f827c5a, 0xf551, 0x477f,
            [0xaf, 0x09, 0x71, 0xad, 0xbf, 0xbd, 0x85, 0x4a])
    }
}

unsafe impl RefCounted for nsICaptivePortalDetector {
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
pub trait nsICaptivePortalDetectorCoerce {
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self;
}

impl nsICaptivePortalDetectorCoerce for nsICaptivePortalDetector {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self {
        v
    }
}

impl nsICaptivePortalDetector {
    #[inline]
    pub fn coerce<T: nsICaptivePortalDetectorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICaptivePortalDetector {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICaptivePortalDetectorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICaptivePortalDetector) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICaptivePortalDetectorVTable {
    pub __base: nsISupportsVTable,

    /* void checkCaptivePortal (in wstring ifname, in nsICaptivePortalCallback callback); */
    pub checkCaptivePortal: unsafe extern "C" fn (this: *const nsICaptivePortalDetector, ifname: *const libc::int16_t, callback: *const nsICaptivePortalCallback) -> nsresult,

    /* void abort (in wstring ifname); */
    pub abort: unsafe extern "C" fn (this: *const nsICaptivePortalDetector, ifname: *const libc::int16_t) -> nsresult,

    /* void cancelLogin (in wstring eventId); */
    pub cancelLogin: unsafe extern "C" fn (this: *const nsICaptivePortalDetector, eventId: *const libc::int16_t) -> nsresult,

    /* void finishPreparation (in wstring ifname); */
    pub finishPreparation: unsafe extern "C" fn (this: *const nsICaptivePortalDetector, ifname: *const libc::int16_t) -> nsresult,

}


impl nsICaptivePortalDetector {
    /* void checkCaptivePortal (in wstring ifname, in nsICaptivePortalCallback callback); */
    #[inline]
    pub unsafe fn checkCaptivePortal(&self, ifname: *const libc::int16_t, callback: Option<&nsICaptivePortalCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).checkCaptivePortal)(self as *const _, ifname, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void abort (in wstring ifname); */
    #[inline]
    pub unsafe fn abort(&self, ifname: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).abort)(self as *const _, ifname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancelLogin (in wstring eventId); */
    #[inline]
    pub unsafe fn cancelLogin(&self, eventId: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).cancelLogin)(self as *const _, eventId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishPreparation (in wstring ifname); */
    #[inline]
    pub unsafe fn finishPreparation(&self, ifname: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).finishPreparation)(self as *const _, ifname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


