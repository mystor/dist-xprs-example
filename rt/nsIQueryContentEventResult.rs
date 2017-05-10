//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQueryContentEventResult.idl
//


#[repr(C)]
pub struct nsIQueryContentEventResult {
    vtable: *const nsIQueryContentEventResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQueryContentEventResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe2c39e0e, 0x345f, 0x451a,
            [0xa7, 0xb2, 0xe0, 0x23, 0x0d, 0x55, 0x58, 0x47])
    }
}

unsafe impl RefCounted for nsIQueryContentEventResult {
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
pub trait nsIQueryContentEventResultCoerce {
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self;
}

impl nsIQueryContentEventResultCoerce for nsIQueryContentEventResult {
    #[inline]
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self {
        v
    }
}

impl nsIQueryContentEventResult {
    #[inline]
    pub fn coerce<T: nsIQueryContentEventResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQueryContentEventResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQueryContentEventResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQueryContentEventResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQueryContentEventResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long offset; */
    pub get_offset: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aOffset: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long tentativeCaretOffset; */
    pub get_tentativeCaretOffset: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aTentativeCaretOffset: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean reversed; */
    pub get_reversed: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aReversed: *mut bool) -> nsresult,

    /* readonly attribute long left; */
    pub get_left: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aLeft: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long top; */
    pub get_top: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aTop: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aHeight: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aText: *mut nsAString) -> nsresult,

    /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
    pub getCharacterRect: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, offset: libc::int32_t, left: *mut libc::int32_t, top: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean succeeded; */
    pub get_succeeded: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aSucceeded: *mut bool) -> nsresult,

    /* readonly attribute boolean notFound; */
    pub get_notFound: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aNotFound: *mut bool) -> nsresult,

    /* readonly attribute boolean tentativeCaretOffsetNotFound; */
    pub get_tentativeCaretOffsetNotFound: unsafe extern "C" fn (this: *const nsIQueryContentEventResult, aTentativeCaretOffsetNotFound: *mut bool) -> nsresult,

}


impl nsIQueryContentEventResult {
    /* readonly attribute unsigned long offset; */
    #[inline]
    pub unsafe fn get_offset(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long tentativeCaretOffset; */
    #[inline]
    pub unsafe fn get_tentativeCaretOffset(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tentativeCaretOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean reversed; */
    #[inline]
    pub unsafe fn get_reversed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reversed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long left; */
    #[inline]
    pub unsafe fn get_left(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_left)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long top; */
    #[inline]
    pub unsafe fn get_top(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_top)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCharacterRect (in long offset, out long left, out long top, out long width, out long height); */
    #[inline]
    pub unsafe fn getCharacterRect(&self, offset: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut left: libc::int32_t = ::std::mem::zeroed();
        let mut top: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCharacterRect)(self as *const _, offset, &mut left as *mut _, &mut top as *mut _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((left, top, width, height))
    }

    /* readonly attribute boolean succeeded; */
    #[inline]
    pub unsafe fn get_succeeded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_succeeded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean notFound; */
    #[inline]
    pub unsafe fn get_notFound(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_notFound)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean tentativeCaretOffsetNotFound; */
    #[inline]
    pub unsafe fn get_tentativeCaretOffsetNotFound(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_tentativeCaretOffsetNotFound)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


