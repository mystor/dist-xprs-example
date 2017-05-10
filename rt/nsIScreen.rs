//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScreen.idl
//


#[repr(C)]
pub struct nsIScreen {
    vtable: *const nsIScreenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScreen {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x826e80c8, 0xd70f, 0x42e2,
            [0x8a, 0xa9, 0x82, 0xc0, 0x5f, 0x2a, 0x37, 0x0a])
    }
}

unsafe impl RefCounted for nsIScreen {
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
pub trait nsIScreenCoerce {
    fn coerce_from(v: &nsIScreen) -> &Self;
}

impl nsIScreenCoerce for nsIScreen {
    #[inline]
    fn coerce_from(v: &nsIScreen) -> &Self {
        v
    }
}

impl nsIScreen {
    #[inline]
    pub fn coerce<T: nsIScreenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScreen {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScreenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScreen) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScreenVTable {
    pub __base: nsISupportsVTable,

    /* void GetRect (out long left, out long top, out long width, out long height); */
    pub GetRect: unsafe extern "C" fn (this: *const nsIScreen, left: *mut libc::int32_t, top: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void GetAvailRect (out long left, out long top, out long width, out long height); */
    pub GetAvailRect: unsafe extern "C" fn (this: *const nsIScreen, left: *mut libc::int32_t, top: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
    pub GetRectDisplayPix: unsafe extern "C" fn (this: *const nsIScreen, left: *mut libc::int32_t, top: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
    pub GetAvailRectDisplayPix: unsafe extern "C" fn (this: *const nsIScreen, left: *mut libc::int32_t, top: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long pixelDepth; */
    pub get_pixelDepth: unsafe extern "C" fn (this: *const nsIScreen, aPixelDepth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long colorDepth; */
    pub get_colorDepth: unsafe extern "C" fn (this: *const nsIScreen, aColorDepth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute double contentsScaleFactor; */
    pub get_contentsScaleFactor: unsafe extern "C" fn (this: *const nsIScreen, aContentsScaleFactor: *mut libc::c_double) -> nsresult,

    /* readonly attribute double defaultCSSScaleFactor; */
    pub get_defaultCSSScaleFactor: unsafe extern "C" fn (this: *const nsIScreen, aDefaultCSSScaleFactor: *mut libc::c_double) -> nsresult,

}


impl nsIScreen {
    /* void GetRect (out long left, out long top, out long width, out long height); */
    #[inline]
    pub unsafe fn GetRect(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut left: libc::int32_t = ::std::mem::zeroed();
        let mut top: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetRect)(self as *const _, &mut left as *mut _, &mut top as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((left, top, width, height))
    }

    /* void GetAvailRect (out long left, out long top, out long width, out long height); */
    #[inline]
    pub unsafe fn GetAvailRect(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut left: libc::int32_t = ::std::mem::zeroed();
        let mut top: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetAvailRect)(self as *const _, &mut left as *mut _, &mut top as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((left, top, width, height))
    }

    /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
    #[inline]
    pub unsafe fn GetRectDisplayPix(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut left: libc::int32_t = ::std::mem::zeroed();
        let mut top: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetRectDisplayPix)(self as *const _, &mut left as *mut _, &mut top as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((left, top, width, height))
    }

    /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
    #[inline]
    pub unsafe fn GetAvailRectDisplayPix(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut left: libc::int32_t = ::std::mem::zeroed();
        let mut top: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).GetAvailRectDisplayPix)(self as *const _, &mut left as *mut _, &mut top as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((left, top, width, height))
    }

    /* readonly attribute long pixelDepth; */
    #[inline]
    pub unsafe fn get_pixelDepth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pixelDepth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long colorDepth; */
    #[inline]
    pub unsafe fn get_colorDepth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_colorDepth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double contentsScaleFactor; */
    #[inline]
    pub unsafe fn get_contentsScaleFactor(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_contentsScaleFactor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double defaultCSSScaleFactor; */
    #[inline]
    pub unsafe fn get_defaultCSSScaleFactor(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultCSSScaleFactor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


