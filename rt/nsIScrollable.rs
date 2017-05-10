//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScrollable.idl
//


pub mod nsIScrollable_consts {
    pub const ScrollOrientation_X: i64 = 1;
    pub const ScrollOrientation_Y: i64 = 2;
    pub const Scrollbar_Auto: i64 = 1;
    pub const Scrollbar_Never: i64 = 2;
    pub const Scrollbar_Always: i64 = 3;
}


#[repr(C)]
pub struct nsIScrollable {
    vtable: *const nsIScrollableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScrollable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3507fc93, 0x313e, 0x4a4c,
            [0x8c, 0xa8, 0x4d, 0x0e, 0xa0, 0xf9, 0x73, 0x15])
    }
}

unsafe impl RefCounted for nsIScrollable {
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
pub trait nsIScrollableCoerce {
    fn coerce_from(v: &nsIScrollable) -> &Self;
}

impl nsIScrollableCoerce for nsIScrollable {
    #[inline]
    fn coerce_from(v: &nsIScrollable) -> &Self {
        v
    }
}

impl nsIScrollable {
    #[inline]
    pub fn coerce<T: nsIScrollableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScrollable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScrollableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScrollable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScrollableVTable {
    pub __base: nsISupportsVTable,

    /* long getDefaultScrollbarPreferences (in long scrollOrientation); */
    pub getDefaultScrollbarPreferences: unsafe extern "C" fn (this: *const nsIScrollable, scrollOrientation: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void setDefaultScrollbarPreferences (in long scrollOrientation, in long scrollbarPref); */
    pub setDefaultScrollbarPreferences: unsafe extern "C" fn (this: *const nsIScrollable, scrollOrientation: libc::int32_t, scrollbarPref: libc::int32_t) -> nsresult,

    /* void getScrollbarVisibility (out boolean verticalVisible, out boolean horizontalVisible); */
    pub getScrollbarVisibility: unsafe extern "C" fn (this: *const nsIScrollable, verticalVisible: *mut bool, horizontalVisible: *mut bool) -> nsresult,

}


impl nsIScrollable {
    /* long getDefaultScrollbarPreferences (in long scrollOrientation); */
    #[inline]
    pub unsafe fn getDefaultScrollbarPreferences(&self, scrollOrientation: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getDefaultScrollbarPreferences)(self as *const _, scrollOrientation, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setDefaultScrollbarPreferences (in long scrollOrientation, in long scrollbarPref); */
    #[inline]
    pub unsafe fn setDefaultScrollbarPreferences(&self, scrollOrientation: libc::int32_t, scrollbarPref: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDefaultScrollbarPreferences)(self as *const _, scrollOrientation, scrollbarPref) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getScrollbarVisibility (out boolean verticalVisible, out boolean horizontalVisible); */
    #[inline]
    pub unsafe fn getScrollbarVisibility(&self, ) -> Result<(bool, bool), nsresult> {
        let mut verticalVisible: bool = ::std::mem::zeroed();
        let mut horizontalVisible: bool = ::std::mem::zeroed();
        match ((*self.vtable).getScrollbarVisibility)(self as *const _, &mut verticalVisible as *mut _, &mut horizontalVisible as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((verticalVisible, horizontalVisible))
    }

}


