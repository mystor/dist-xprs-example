//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionDisplay.idl
//


pub mod nsISelectionDisplay_consts {
    pub const DISPLAY_TEXT: i64 = 1;
    pub const DISPLAY_IMAGES: i64 = 2;
    pub const DISPLAY_FRAMES: i64 = 4;
    pub const DISPLAY_ALL: i64 = 7;
}


#[repr(C)]
pub struct nsISelectionDisplay {
    vtable: *const nsISelectionDisplayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISelectionDisplay {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0ddf9e1c, 0x1dd2, 0x11b2,
            [0xa1, 0x83, 0x90, 0x8a, 0x08, 0xaa, 0x75, 0xae])
    }
}

unsafe impl RefCounted for nsISelectionDisplay {
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
pub trait nsISelectionDisplayCoerce {
    fn coerce_from(v: &nsISelectionDisplay) -> &Self;
}

impl nsISelectionDisplayCoerce for nsISelectionDisplay {
    #[inline]
    fn coerce_from(v: &nsISelectionDisplay) -> &Self {
        v
    }
}

impl nsISelectionDisplay {
    #[inline]
    pub fn coerce<T: nsISelectionDisplayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISelectionDisplay {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISelectionDisplayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionDisplay) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISelectionDisplayVTable {
    pub __base: nsISupportsVTable,

    /* void setSelectionFlags (in short toggle); */
    pub setSelectionFlags: unsafe extern "C" fn (this: *const nsISelectionDisplay, toggle: libc::int16_t) -> nsresult,

    /* short getSelectionFlags (); */
    pub getSelectionFlags: unsafe extern "C" fn (this: *const nsISelectionDisplay, _retval: *mut libc::int16_t) -> nsresult,

}


impl nsISelectionDisplay {
    /* void setSelectionFlags (in short toggle); */
    #[inline]
    pub unsafe fn setSelectionFlags(&self, toggle: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setSelectionFlags)(self as *const _, toggle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* short getSelectionFlags (); */
    #[inline]
    pub unsafe fn getSelectionFlags(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getSelectionFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


