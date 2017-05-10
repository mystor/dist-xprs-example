//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringEnumerator.idl
//


#[repr(C)]
pub struct nsIStringEnumerator {
    vtable: *const nsIStringEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStringEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x50d3ef6c, 0x9380, 0x4f06,
            [0x9f, 0xb2, 0x95, 0x48, 0x8f, 0x7d, 0x14, 0x1c])
    }
}

unsafe impl RefCounted for nsIStringEnumerator {
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
pub trait nsIStringEnumeratorCoerce {
    fn coerce_from(v: &nsIStringEnumerator) -> &Self;
}

impl nsIStringEnumeratorCoerce for nsIStringEnumerator {
    #[inline]
    fn coerce_from(v: &nsIStringEnumerator) -> &Self {
        v
    }
}

impl nsIStringEnumerator {
    #[inline]
    pub fn coerce<T: nsIStringEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStringEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStringEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStringEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* boolean hasMore (); */
    pub hasMore: unsafe extern "C" fn (this: *const nsIStringEnumerator, _retval: *mut bool) -> nsresult,

    /* AString getNext (); */
    pub getNext: unsafe extern "C" fn (this: *const nsIStringEnumerator, _retval: *mut nsAString) -> nsresult,

}


impl nsIStringEnumerator {
    /* boolean hasMore (); */
    #[inline]
    pub unsafe fn hasMore(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMore)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getNext (); */
    #[inline]
    pub unsafe fn getNext(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getNext)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIUTF8StringEnumerator {
    vtable: *const nsIUTF8StringEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUTF8StringEnumerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9bdf1010, 0x3695, 0x4907,
            [0x95, 0xed, 0x83, 0xd0, 0x41, 0x0e, 0xc3, 0x07])
    }
}

unsafe impl RefCounted for nsIUTF8StringEnumerator {
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
pub trait nsIUTF8StringEnumeratorCoerce {
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self;
}

impl nsIUTF8StringEnumeratorCoerce for nsIUTF8StringEnumerator {
    #[inline]
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self {
        v
    }
}

impl nsIUTF8StringEnumerator {
    #[inline]
    pub fn coerce<T: nsIUTF8StringEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUTF8StringEnumerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUTF8StringEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUTF8StringEnumeratorVTable {
    pub __base: nsISupportsVTable,

    /* boolean hasMore (); */
    pub hasMore: unsafe extern "C" fn (this: *const nsIUTF8StringEnumerator, _retval: *mut bool) -> nsresult,

    /* AUTF8String getNext (); */
    pub getNext: unsafe extern "C" fn (this: *const nsIUTF8StringEnumerator, _retval: *mut nsACString) -> nsresult,

}


impl nsIUTF8StringEnumerator {
    /* boolean hasMore (); */
    #[inline]
    pub unsafe fn hasMore(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMore)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getNext (); */
    #[inline]
    pub unsafe fn getNext(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getNext)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


