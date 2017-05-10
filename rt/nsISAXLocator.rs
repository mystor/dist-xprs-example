//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXLocator.idl
//


#[repr(C)]
pub struct nsISAXLocator {
    vtable: *const nsISAXLocatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXLocator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a307c6c, 0x6cc9, 0x11da,
            [0xbe, 0x43, 0x00, 0x14, 0x22, 0x10, 0x69, 0x90])
    }
}

unsafe impl RefCounted for nsISAXLocator {
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
pub trait nsISAXLocatorCoerce {
    fn coerce_from(v: &nsISAXLocator) -> &Self;
}

impl nsISAXLocatorCoerce for nsISAXLocator {
    #[inline]
    fn coerce_from(v: &nsISAXLocator) -> &Self {
        v
    }
}

impl nsISAXLocator {
    #[inline]
    pub fn coerce<T: nsISAXLocatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXLocator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXLocatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXLocator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXLocatorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long columnNumber; */
    pub get_columnNumber: unsafe extern "C" fn (this: *const nsISAXLocator, aColumnNumber: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long lineNumber; */
    pub get_lineNumber: unsafe extern "C" fn (this: *const nsISAXLocator, aLineNumber: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AString publicId; */
    pub get_publicId: unsafe extern "C" fn (this: *const nsISAXLocator, aPublicId: *mut nsAString) -> nsresult,

    /* readonly attribute AString systemId; */
    pub get_systemId: unsafe extern "C" fn (this: *const nsISAXLocator, aSystemId: *mut nsAString) -> nsresult,

}


impl nsISAXLocator {
    /* readonly attribute long columnNumber; */
    #[inline]
    pub unsafe fn get_columnNumber(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long lineNumber; */
    #[inline]
    pub unsafe fn get_lineNumber(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lineNumber)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString publicId; */
    #[inline]
    pub unsafe fn get_publicId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_publicId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString systemId; */
    #[inline]
    pub unsafe fn get_systemId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_systemId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


