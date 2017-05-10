//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMClientRect.idl
//


#[repr(C)]
pub struct nsIDOMClientRect {
    vtable: *const nsIDOMClientRectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMClientRect {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb2f824c4, 0xd9d3, 0x499b,
            [0x8d, 0x3b, 0x45, 0xc8, 0x24, 0x54, 0x97, 0xc6])
    }
}

unsafe impl RefCounted for nsIDOMClientRect {
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
pub trait nsIDOMClientRectCoerce {
    fn coerce_from(v: &nsIDOMClientRect) -> &Self;
}

impl nsIDOMClientRectCoerce for nsIDOMClientRect {
    #[inline]
    fn coerce_from(v: &nsIDOMClientRect) -> &Self {
        v
    }
}

impl nsIDOMClientRect {
    #[inline]
    pub fn coerce<T: nsIDOMClientRectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMClientRect {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMClientRectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMClientRect) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMClientRectVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute float left; */
    pub get_left: unsafe extern "C" fn (this: *const nsIDOMClientRect, aLeft: *mut libc::c_float) -> nsresult,

    /* readonly attribute float top; */
    pub get_top: unsafe extern "C" fn (this: *const nsIDOMClientRect, aTop: *mut libc::c_float) -> nsresult,

    /* readonly attribute float right; */
    pub get_right: unsafe extern "C" fn (this: *const nsIDOMClientRect, aRight: *mut libc::c_float) -> nsresult,

    /* readonly attribute float bottom; */
    pub get_bottom: unsafe extern "C" fn (this: *const nsIDOMClientRect, aBottom: *mut libc::c_float) -> nsresult,

    /* readonly attribute float width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMClientRect, aWidth: *mut libc::c_float) -> nsresult,

    /* readonly attribute float height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMClientRect, aHeight: *mut libc::c_float) -> nsresult,

}


impl nsIDOMClientRect {
    /* readonly attribute float left; */
    #[inline]
    pub unsafe fn get_left(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_left)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float top; */
    #[inline]
    pub unsafe fn get_top(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_top)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float right; */
    #[inline]
    pub unsafe fn get_right(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_right)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float bottom; */
    #[inline]
    pub unsafe fn get_bottom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_bottom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


