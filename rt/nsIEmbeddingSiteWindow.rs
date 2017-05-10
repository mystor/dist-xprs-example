//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEmbeddingSiteWindow.idl
//


pub mod nsIEmbeddingSiteWindow_consts {
    pub const DIM_FLAGS_POSITION: i64 = 1;
    pub const DIM_FLAGS_SIZE_INNER: i64 = 2;
    pub const DIM_FLAGS_SIZE_OUTER: i64 = 4;
    pub const DIM_FLAGS_IGNORE_X: i64 = 8;
    pub const DIM_FLAGS_IGNORE_Y: i64 = 16;
    pub const DIM_FLAGS_IGNORE_CX: i64 = 32;
    pub const DIM_FLAGS_IGNORE_CY: i64 = 64;
}


#[repr(C)]
pub struct nsIEmbeddingSiteWindow {
    vtable: *const nsIEmbeddingSiteWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEmbeddingSiteWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0b976267, 0x4aaa, 0x4f36,
            [0xa2, 0xd4, 0x27, 0xb5, 0xca, 0x8d, 0x73, 0xbb])
    }
}

unsafe impl RefCounted for nsIEmbeddingSiteWindow {
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
pub trait nsIEmbeddingSiteWindowCoerce {
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self;
}

impl nsIEmbeddingSiteWindowCoerce for nsIEmbeddingSiteWindow {
    #[inline]
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self {
        v
    }
}

impl nsIEmbeddingSiteWindow {
    #[inline]
    pub fn coerce<T: nsIEmbeddingSiteWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEmbeddingSiteWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEmbeddingSiteWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEmbeddingSiteWindowVTable {
    pub __base: nsISupportsVTable,

    /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
    pub setDimensions: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, flags: libc::uint32_t, x: libc::int32_t, y: libc::int32_t, cx: libc::int32_t, cy: libc::int32_t) -> nsresult,

    /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
    pub getDimensions: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, flags: libc::uint32_t, x: *mut libc::int32_t, y: *mut libc::int32_t, cx: *mut libc::int32_t, cy: *mut libc::int32_t) -> nsresult,

    /* void setFocus (); */
    pub setFocus: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow) -> nsresult,

    /* attribute boolean visibility; */
    pub get_visibility: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, aVisibility: *mut bool) -> nsresult,
    pub set_visibility: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, aVisibility: bool) -> nsresult,

    /* attribute wstring title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, aTitle: *mut *const libc::int16_t) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, aTitle: *const libc::int16_t) -> nsresult,

    /* [noscript] readonly attribute voidPtr siteWindow; */
    pub get_siteWindow: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow, aSiteWindow: *mut *const libc::c_void) -> nsresult,

    /* void blur (); */
    pub blur: unsafe extern "C" fn (this: *const nsIEmbeddingSiteWindow) -> nsresult,

}


impl nsIEmbeddingSiteWindow {
    /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
    #[inline]
    pub unsafe fn setDimensions(&self, flags: libc::uint32_t, x: libc::int32_t, y: libc::int32_t, cx: libc::int32_t, cy: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDimensions)(self as *const _, flags, x, y, cx, cy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
    #[inline]
    pub unsafe fn getDimensions(&self, flags: libc::uint32_t) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut cx: libc::int32_t = ::std::mem::zeroed();
        let mut cy: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getDimensions)(self as *const _, flags, &mut x as *mut _, &mut y as *mut _, &mut cx as *mut _, &mut cy as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, cx, cy))
    }

    /* void setFocus (); */
    #[inline]
    pub unsafe fn setFocus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setFocus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean visibility; */
    #[inline]
    pub unsafe fn get_visibility(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_visibility)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_visibility(&self, aVisibility: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_visibility)(self as *const _, aVisibility) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_title)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_title(&self, aTitle: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_title)(self as *const _, aTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute voidPtr siteWindow; */
    #[inline]
    pub unsafe fn get_siteWindow(&self, ) -> Result<*const libc::c_void, nsresult> {
        let mut _retval: *const libc::c_void = ::std::ptr::null();
        match ((*self.vtable).get_siteWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void blur (); */
    #[inline]
    pub unsafe fn blur(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).blur)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


