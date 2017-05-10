//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBaseWindow.idl
//


pub type nativeWindow = *const libc::c_void;


pub mod nsIBaseWindow_consts {
    pub const eRepaint: i64 = 1;
    pub const eDelayResize: i64 = 2;
}


#[repr(C)]
pub struct nsIBaseWindow {
    vtable: *const nsIBaseWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBaseWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xca635529, 0xa977, 0x4552,
            [0x9b, 0x8a, 0x66, 0x18, 0x7e, 0x54, 0xd8, 0x82])
    }
}

unsafe impl RefCounted for nsIBaseWindow {
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
pub trait nsIBaseWindowCoerce {
    fn coerce_from(v: &nsIBaseWindow) -> &Self;
}

impl nsIBaseWindowCoerce for nsIBaseWindow {
    #[inline]
    fn coerce_from(v: &nsIBaseWindow) -> &Self {
        v
    }
}

impl nsIBaseWindow {
    #[inline]
    pub fn coerce<T: nsIBaseWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBaseWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBaseWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBaseWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBaseWindowVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void initWindow (in nativeWindow parentNativeWindow, in nsIWidget parentWidget, in long x, in long y, in long cx, in long cy); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWindow: *const ::libc::c_void,

    /* void create (); */
    pub create: unsafe extern "C" fn (this: *const nsIBaseWindow) -> nsresult,

    /* void destroy (); */
    pub destroy: unsafe extern "C" fn (this: *const nsIBaseWindow) -> nsresult,

    /* void setPosition (in long x, in long y); */
    pub setPosition: unsafe extern "C" fn (this: *const nsIBaseWindow, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* void setPositionDesktopPix (in long x, in long y); */
    pub setPositionDesktopPix: unsafe extern "C" fn (this: *const nsIBaseWindow, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* void getPosition (out long x, out long y); */
    pub getPosition: unsafe extern "C" fn (this: *const nsIBaseWindow, x: *mut libc::int32_t, y: *mut libc::int32_t) -> nsresult,

    /* void setSize (in long cx, in long cy, in boolean fRepaint); */
    pub setSize: unsafe extern "C" fn (this: *const nsIBaseWindow, cx: libc::int32_t, cy: libc::int32_t, fRepaint: bool) -> nsresult,

    /* void getSize (out long cx, out long cy); */
    pub getSize: unsafe extern "C" fn (this: *const nsIBaseWindow, cx: *mut libc::int32_t, cy: *mut libc::int32_t) -> nsresult,

    /* void setPositionAndSize (in long x, in long y, in long cx, in long cy, in unsigned long flags); */
    pub setPositionAndSize: unsafe extern "C" fn (this: *const nsIBaseWindow, x: libc::int32_t, y: libc::int32_t, cx: libc::int32_t, cy: libc::int32_t, flags: libc::uint32_t) -> nsresult,

    /* void getPositionAndSize (out long x, out long y, out long cx, out long cy); */
    pub getPositionAndSize: unsafe extern "C" fn (this: *const nsIBaseWindow, x: *mut libc::int32_t, y: *mut libc::int32_t, cx: *mut libc::int32_t, cy: *mut libc::int32_t) -> nsresult,

    /* void repaint (in boolean force); */
    pub repaint: unsafe extern "C" fn (this: *const nsIBaseWindow, force: bool) -> nsresult,

    /* [noscript] attribute nsIWidget parentWidget; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_parentWidget: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_parentWidget: *const ::libc::c_void,

    /* attribute nativeWindow parentNativeWindow; */
    pub get_parentNativeWindow: unsafe extern "C" fn (this: *const nsIBaseWindow, aParentNativeWindow: *mut nativeWindow) -> nsresult,
    pub set_parentNativeWindow: unsafe extern "C" fn (this: *const nsIBaseWindow, aParentNativeWindow: nativeWindow) -> nsresult,

    /* readonly attribute DOMString nativeHandle; */
    pub get_nativeHandle: unsafe extern "C" fn (this: *const nsIBaseWindow, aNativeHandle: *mut nsAString) -> nsresult,

    /* attribute boolean visibility; */
    pub get_visibility: unsafe extern "C" fn (this: *const nsIBaseWindow, aVisibility: *mut bool) -> nsresult,
    pub set_visibility: unsafe extern "C" fn (this: *const nsIBaseWindow, aVisibility: bool) -> nsresult,

    /* attribute boolean enabled; */
    pub get_enabled: unsafe extern "C" fn (this: *const nsIBaseWindow, aEnabled: *mut bool) -> nsresult,
    pub set_enabled: unsafe extern "C" fn (this: *const nsIBaseWindow, aEnabled: bool) -> nsresult,

    /* [noscript] readonly attribute nsIWidget mainWidget; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_mainWidget: *const ::libc::c_void,

    /* readonly attribute double unscaledDevicePixelsPerCSSPixel; */
    pub get_unscaledDevicePixelsPerCSSPixel: unsafe extern "C" fn (this: *const nsIBaseWindow, aUnscaledDevicePixelsPerCSSPixel: *mut libc::c_double) -> nsresult,

    /* readonly attribute double devicePixelsPerDesktopPixel; */
    pub get_devicePixelsPerDesktopPixel: unsafe extern "C" fn (this: *const nsIBaseWindow, aDevicePixelsPerDesktopPixel: *mut libc::c_double) -> nsresult,

    /* void setFocus (); */
    pub setFocus: unsafe extern "C" fn (this: *const nsIBaseWindow) -> nsresult,

    /* attribute wstring title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIBaseWindow, aTitle: *mut *const libc::int16_t) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIBaseWindow, aTitle: *const libc::int16_t) -> nsresult,

}


impl nsIBaseWindow {
    /* [noscript] void initWindow (in nativeWindow parentNativeWindow, in nsIWidget parentWidget, in long x, in long y, in long cx, in long cy); */


    /* void create (); */
    #[inline]
    pub unsafe fn create(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).create)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void destroy (); */
    #[inline]
    pub unsafe fn destroy(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroy)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPosition (in long x, in long y); */
    #[inline]
    pub unsafe fn setPosition(&self, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setPosition)(self as *const _, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPositionDesktopPix (in long x, in long y); */
    #[inline]
    pub unsafe fn setPositionDesktopPix(&self, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setPositionDesktopPix)(self as *const _, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPosition (out long x, out long y); */
    #[inline]
    pub unsafe fn getPosition(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPosition)(self as *const _, &mut x as *mut _, &mut y as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y))
    }

    /* void setSize (in long cx, in long cy, in boolean fRepaint); */
    #[inline]
    pub unsafe fn setSize(&self, cx: libc::int32_t, cy: libc::int32_t, fRepaint: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSize)(self as *const _, cx, cy, fRepaint) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getSize (out long cx, out long cy); */
    #[inline]
    pub unsafe fn getSize(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut cx: libc::int32_t = ::std::mem::zeroed();
        let mut cy: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getSize)(self as *const _, &mut cx as *mut _, &mut cy as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((cx, cy))
    }

    /* void setPositionAndSize (in long x, in long y, in long cx, in long cy, in unsigned long flags); */
    #[inline]
    pub unsafe fn setPositionAndSize(&self, x: libc::int32_t, y: libc::int32_t, cx: libc::int32_t, cy: libc::int32_t, flags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setPositionAndSize)(self as *const _, x, y, cx, cy, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPositionAndSize (out long x, out long y, out long cx, out long cy); */
    #[inline]
    pub unsafe fn getPositionAndSize(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut cx: libc::int32_t = ::std::mem::zeroed();
        let mut cy: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPositionAndSize)(self as *const _, &mut x as *mut _, &mut y as *mut _, &mut cx as *mut _, &mut cy as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, cx, cy))
    }

    /* void repaint (in boolean force); */
    #[inline]
    pub unsafe fn repaint(&self, force: bool) -> Result<(), nsresult> {

        match ((*self.vtable).repaint)(self as *const _, force) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsIWidget parentWidget; */



    /* attribute nativeWindow parentNativeWindow; */
    #[inline]
    pub unsafe fn get_parentNativeWindow(&self, ) -> Result<nativeWindow, nsresult> {
        let mut _retval: nativeWindow = ::std::mem::zeroed();
        match ((*self.vtable).get_parentNativeWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_parentNativeWindow(&self, aParentNativeWindow: nativeWindow) -> Result<(), nsresult> {

        match ((*self.vtable).set_parentNativeWindow)(self as *const _, aParentNativeWindow) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString nativeHandle; */
    #[inline]
    pub unsafe fn get_nativeHandle(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_nativeHandle)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* attribute boolean enabled; */
    #[inline]
    pub unsafe fn get_enabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enabled(&self, aEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_enabled)(self as *const _, aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute nsIWidget mainWidget; */


    /* readonly attribute double unscaledDevicePixelsPerCSSPixel; */
    #[inline]
    pub unsafe fn get_unscaledDevicePixelsPerCSSPixel(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_unscaledDevicePixelsPerCSSPixel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double devicePixelsPerDesktopPixel; */
    #[inline]
    pub unsafe fn get_devicePixelsPerDesktopPixel(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_devicePixelsPerDesktopPixel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

}


