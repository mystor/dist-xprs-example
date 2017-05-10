//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserDOMWindow.idl
//


#[repr(C)]
pub struct nsIOpenURIInFrameParams {
    vtable: *const nsIOpenURIInFrameParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOpenURIInFrameParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe774db14, 0x79ac, 0x4156,
            [0xa7, 0xa3, 0xaa, 0x3f, 0xd0, 0xa2, 0x2c, 0x10])
    }
}

unsafe impl RefCounted for nsIOpenURIInFrameParams {
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
pub trait nsIOpenURIInFrameParamsCoerce {
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self;
}

impl nsIOpenURIInFrameParamsCoerce for nsIOpenURIInFrameParams {
    #[inline]
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self {
        v
    }
}

impl nsIOpenURIInFrameParams {
    #[inline]
    pub fn coerce<T: nsIOpenURIInFrameParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOpenURIInFrameParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOpenURIInFrameParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOpenURIInFrameParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOpenURIInFrameParamsVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIOpenURIInFrameParams, aReferrer: *mut nsAString) -> nsresult,
    pub set_referrer: unsafe extern "C" fn (this: *const nsIOpenURIInFrameParams, aReferrer: *const nsAString) -> nsresult,

    /* attribute boolean isPrivate; */
    pub get_isPrivate: unsafe extern "C" fn (this: *const nsIOpenURIInFrameParams, aIsPrivate: *mut bool) -> nsresult,
    pub set_isPrivate: unsafe extern "C" fn (this: *const nsIOpenURIInFrameParams, aIsPrivate: bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval openerOriginAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_openerOriginAttributes: *const ::libc::c_void,

}


impl nsIOpenURIInFrameParams {
    /* attribute DOMString referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_referrer)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_referrer(&self, aReferrer: &[u16]) -> Result<(), nsresult> {
        let aReferrer = nsString::from(aReferrer);
        match ((*self.vtable).set_referrer)(self as *const _, &*aReferrer) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isPrivate; */
    #[inline]
    pub unsafe fn get_isPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isPrivate(&self, aIsPrivate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isPrivate)(self as *const _, aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] readonly attribute jsval openerOriginAttributes; */


}


pub mod nsIBrowserDOMWindow_consts {
    pub const OPEN_DEFAULTWINDOW: i64 = 0;
    pub const OPEN_CURRENTWINDOW: i64 = 1;
    pub const OPEN_NEWWINDOW: i64 = 2;
    pub const OPEN_NEWTAB: i64 = 3;
    pub const OPEN_SWITCHTAB: i64 = 4;
    pub const OPEN_NEW: i64 = 0;
    pub const OPEN_EXTERNAL: i64 = 1;
    pub const OPEN_NO_OPENER: i64 = 4;
}


#[repr(C)]
pub struct nsIBrowserDOMWindow {
    vtable: *const nsIBrowserDOMWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserDOMWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9d17f3dd, 0x672b, 0x451e,
            [0xaf, 0xd2, 0xb1, 0x11, 0x5d, 0xf7, 0x80, 0xd5])
    }
}

unsafe impl RefCounted for nsIBrowserDOMWindow {
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
pub trait nsIBrowserDOMWindowCoerce {
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self;
}

impl nsIBrowserDOMWindowCoerce for nsIBrowserDOMWindow {
    #[inline]
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self {
        v
    }
}

impl nsIBrowserDOMWindow {
    #[inline]
    pub fn coerce<T: nsIBrowserDOMWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserDOMWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserDOMWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserDOMWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserDOMWindowVTable {
    pub __base: nsISupportsVTable,

    /* mozIDOMWindowProxy openURI (in nsIURI aURI, in mozIDOMWindowProxy aOpener, in short aWhere, in long aFlags); */
    pub openURI: unsafe extern "C" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, aOpener: *const mozIDOMWindowProxy, aWhere: libc::int16_t, aFlags: libc::int32_t, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* nsIFrameLoaderOwner openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in unsigned long long aNextTabParentId); */
    pub openURIInFrame: unsafe extern "C" fn (this: *const nsIBrowserDOMWindow, aURI: *const nsIURI, params: *const nsIOpenURIInFrameParams, aWhere: libc::int16_t, aFlags: libc::int32_t, aNextTabParentId: libc::uint64_t, _retval: *mut *const nsIFrameLoaderOwner) -> nsresult,

    /* boolean isTabContentWindow (in nsIDOMWindow aWindow); */
    pub isTabContentWindow: unsafe extern "C" fn (this: *const nsIBrowserDOMWindow, aWindow: *const nsIDOMWindow, _retval: *mut bool) -> nsresult,

    /* boolean canClose (); */
    pub canClose: unsafe extern "C" fn (this: *const nsIBrowserDOMWindow, _retval: *mut bool) -> nsresult,

}


impl nsIBrowserDOMWindow {
    /* mozIDOMWindowProxy openURI (in nsIURI aURI, in mozIDOMWindowProxy aOpener, in short aWhere, in long aFlags); */
    #[inline]
    pub unsafe fn openURI(&self, aURI: Option<&nsIURI>, aOpener: Option<&mozIDOMWindowProxy>, aWhere: libc::int16_t, aFlags: libc::int32_t) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aOpener.map_or(::std::ptr::null(), |x| x as *const _), aWhere, aFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIFrameLoaderOwner openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in unsigned long long aNextTabParentId); */
    #[inline]
    pub unsafe fn openURIInFrame(&self, aURI: Option<&nsIURI>, params: Option<&nsIOpenURIInFrameParams>, aWhere: libc::int16_t, aFlags: libc::int32_t, aNextTabParentId: libc::uint64_t) -> Result<Option<RefPtr<nsIFrameLoaderOwner>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openURIInFrame)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), params.map_or(::std::ptr::null(), |x| x as *const _), aWhere, aFlags, aNextTabParentId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isTabContentWindow (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn isTabContentWindow(&self, aWindow: Option<&nsIDOMWindow>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isTabContentWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canClose (); */
    #[inline]
    pub unsafe fn canClose(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canClose)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


