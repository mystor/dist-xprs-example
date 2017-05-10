//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMRect.idl
//


#[repr(C)]
pub struct nsIDOMRect {
    vtable: *const nsIDOMRectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMRect {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x71735f62, 0xac5c, 0x4236,
            [0x9a, 0x1f, 0x5f, 0xfb, 0x28, 0x0d, 0x53, 0x1c])
    }
}

unsafe impl RefCounted for nsIDOMRect {
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
pub trait nsIDOMRectCoerce {
    fn coerce_from(v: &nsIDOMRect) -> &Self;
}

impl nsIDOMRectCoerce for nsIDOMRect {
    #[inline]
    fn coerce_from(v: &nsIDOMRect) -> &Self {
        v
    }
}

impl nsIDOMRect {
    #[inline]
    pub fn coerce<T: nsIDOMRectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMRect {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMRectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMRect) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMRectVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMCSSPrimitiveValue top; */
    pub get_top: unsafe extern "C" fn (this: *const nsIDOMRect, aTop: *mut *const nsIDOMCSSPrimitiveValue) -> nsresult,

    /* readonly attribute nsIDOMCSSPrimitiveValue right; */
    pub get_right: unsafe extern "C" fn (this: *const nsIDOMRect, aRight: *mut *const nsIDOMCSSPrimitiveValue) -> nsresult,

    /* readonly attribute nsIDOMCSSPrimitiveValue bottom; */
    pub get_bottom: unsafe extern "C" fn (this: *const nsIDOMRect, aBottom: *mut *const nsIDOMCSSPrimitiveValue) -> nsresult,

    /* readonly attribute nsIDOMCSSPrimitiveValue left; */
    pub get_left: unsafe extern "C" fn (this: *const nsIDOMRect, aLeft: *mut *const nsIDOMCSSPrimitiveValue) -> nsresult,

}


impl nsIDOMRect {
    /* readonly attribute nsIDOMCSSPrimitiveValue top; */
    #[inline]
    pub unsafe fn get_top(&self, ) -> Result<Option<RefPtr<nsIDOMCSSPrimitiveValue>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_top)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSPrimitiveValue right; */
    #[inline]
    pub unsafe fn get_right(&self, ) -> Result<Option<RefPtr<nsIDOMCSSPrimitiveValue>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_right)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSPrimitiveValue bottom; */
    #[inline]
    pub unsafe fn get_bottom(&self, ) -> Result<Option<RefPtr<nsIDOMCSSPrimitiveValue>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_bottom)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSPrimitiveValue left; */
    #[inline]
    pub unsafe fn get_left(&self, ) -> Result<Option<RefPtr<nsIDOMCSSPrimitiveValue>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_left)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


