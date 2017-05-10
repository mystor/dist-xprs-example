//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableRegion.idl
//


#[repr(C)]
pub struct nsIScriptableRegion {
    vtable: *const nsIScriptableRegionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableRegion {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5f44cc7, 0x2820, 0x489b,
            [0xb8, 0x17, 0xae, 0x8a, 0x08, 0x50, 0x6f, 0xf6])
    }
}

unsafe impl RefCounted for nsIScriptableRegion {
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
pub trait nsIScriptableRegionCoerce {
    fn coerce_from(v: &nsIScriptableRegion) -> &Self;
}

impl nsIScriptableRegionCoerce for nsIScriptableRegion {
    #[inline]
    fn coerce_from(v: &nsIScriptableRegion) -> &Self {
        v
    }
}

impl nsIScriptableRegion {
    #[inline]
    pub fn coerce<T: nsIScriptableRegionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableRegion {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableRegionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableRegion) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableRegionVTable {
    pub __base: nsISupportsVTable,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsIScriptableRegion) -> nsresult,

    /* void setToRegion (in nsIScriptableRegion aRegion); */
    pub setToRegion: unsafe extern "C" fn (this: *const nsIScriptableRegion, aRegion: *const nsIScriptableRegion) -> nsresult,

    /* void setToRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    pub setToRect: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> nsresult,

    /* void intersectRegion (in nsIScriptableRegion aRegion); */
    pub intersectRegion: unsafe extern "C" fn (this: *const nsIScriptableRegion, aRegion: *const nsIScriptableRegion) -> nsresult,

    /* void intersectRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    pub intersectRect: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> nsresult,

    /* void unionRegion (in nsIScriptableRegion aRegion); */
    pub unionRegion: unsafe extern "C" fn (this: *const nsIScriptableRegion, aRegion: *const nsIScriptableRegion) -> nsresult,

    /* void unionRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    pub unionRect: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> nsresult,

    /* void subtractRegion (in nsIScriptableRegion aRegion); */
    pub subtractRegion: unsafe extern "C" fn (this: *const nsIScriptableRegion, aRegion: *const nsIScriptableRegion) -> nsresult,

    /* void subtractRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    pub subtractRect: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> nsresult,

    /* boolean isEmpty (); */
    pub isEmpty: unsafe extern "C" fn (this: *const nsIScriptableRegion, _retval: *mut bool) -> nsresult,

    /* boolean isEqualRegion (in nsIScriptableRegion aRegion); */
    pub isEqualRegion: unsafe extern "C" fn (this: *const nsIScriptableRegion, aRegion: *const nsIScriptableRegion, _retval: *mut bool) -> nsresult,

    /* void getBoundingBox (out long aX, out long aY, out long aWidth, out long aHeight); */
    pub getBoundingBox: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: *mut libc::int32_t, aY: *mut libc::int32_t, aWidth: *mut libc::int32_t, aHeight: *mut libc::int32_t) -> nsresult,

    /* void offset (in long aXOffset, in long aYOffset); */
    pub offset: unsafe extern "C" fn (this: *const nsIScriptableRegion, aXOffset: libc::int32_t, aYOffset: libc::int32_t) -> nsresult,

    /* [implicit_jscontext] jsval getRects (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getRects: *const ::libc::c_void,

    /* boolean containsRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    pub containsRect: unsafe extern "C" fn (this: *const nsIScriptableRegion, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* [noscript] readonly attribute nsIntRegion region; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_region: *const ::libc::c_void,

}


impl nsIScriptableRegion {
    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setToRegion (in nsIScriptableRegion aRegion); */
    #[inline]
    pub unsafe fn setToRegion(&self, aRegion: Option<&nsIScriptableRegion>) -> Result<(), nsresult> {

        match ((*self.vtable).setToRegion)(self as *const _, aRegion.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setToRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    #[inline]
    pub unsafe fn setToRect(&self, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setToRect)(self as *const _, aX, aY, aWidth, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void intersectRegion (in nsIScriptableRegion aRegion); */
    #[inline]
    pub unsafe fn intersectRegion(&self, aRegion: Option<&nsIScriptableRegion>) -> Result<(), nsresult> {

        match ((*self.vtable).intersectRegion)(self as *const _, aRegion.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void intersectRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    #[inline]
    pub unsafe fn intersectRect(&self, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).intersectRect)(self as *const _, aX, aY, aWidth, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unionRegion (in nsIScriptableRegion aRegion); */
    #[inline]
    pub unsafe fn unionRegion(&self, aRegion: Option<&nsIScriptableRegion>) -> Result<(), nsresult> {

        match ((*self.vtable).unionRegion)(self as *const _, aRegion.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unionRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    #[inline]
    pub unsafe fn unionRect(&self, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).unionRect)(self as *const _, aX, aY, aWidth, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void subtractRegion (in nsIScriptableRegion aRegion); */
    #[inline]
    pub unsafe fn subtractRegion(&self, aRegion: Option<&nsIScriptableRegion>) -> Result<(), nsresult> {

        match ((*self.vtable).subtractRegion)(self as *const _, aRegion.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void subtractRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    #[inline]
    pub unsafe fn subtractRect(&self, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).subtractRect)(self as *const _, aX, aY, aWidth, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isEmpty (); */
    #[inline]
    pub unsafe fn isEmpty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEmpty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isEqualRegion (in nsIScriptableRegion aRegion); */
    #[inline]
    pub unsafe fn isEqualRegion(&self, aRegion: Option<&nsIScriptableRegion>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isEqualRegion)(self as *const _, aRegion.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getBoundingBox (out long aX, out long aY, out long aWidth, out long aHeight); */
    #[inline]
    pub unsafe fn getBoundingBox(&self, ) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut aX: libc::int32_t = ::std::mem::zeroed();
        let mut aY: libc::int32_t = ::std::mem::zeroed();
        let mut aWidth: libc::int32_t = ::std::mem::zeroed();
        let mut aHeight: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getBoundingBox)(self as *const _, &mut aX as *mut _, &mut aY as *mut _, &mut aWidth as *mut _, &mut aHeight as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aX, aY, aWidth, aHeight))
    }

    /* void offset (in long aXOffset, in long aYOffset); */
    #[inline]
    pub unsafe fn offset(&self, aXOffset: libc::int32_t, aYOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).offset)(self as *const _, aXOffset, aYOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getRects (); */


    /* boolean containsRect (in long aX, in long aY, in long aWidth, in long aHeight); */
    #[inline]
    pub unsafe fn containsRect(&self, aX: libc::int32_t, aY: libc::int32_t, aWidth: libc::int32_t, aHeight: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).containsRect)(self as *const _, aX, aY, aWidth, aHeight, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsIntRegion region; */


}


