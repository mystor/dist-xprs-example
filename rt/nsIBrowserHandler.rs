//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserHandler.idl
//


#[repr(C)]
pub struct nsIBrowserHandler {
    vtable: *const nsIBrowserHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8d3f5a9d, 0x118d, 0x4548,
            [0xa1, 0x37, 0xcf, 0x77, 0x18, 0x67, 0x90, 0x69])
    }
}

unsafe impl RefCounted for nsIBrowserHandler {
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
pub trait nsIBrowserHandlerCoerce {
    fn coerce_from(v: &nsIBrowserHandler) -> &Self;
}

impl nsIBrowserHandlerCoerce for nsIBrowserHandler {
    #[inline]
    fn coerce_from(v: &nsIBrowserHandler) -> &Self {
        v
    }
}

impl nsIBrowserHandler {
    #[inline]
    pub fn coerce<T: nsIBrowserHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserHandlerVTable {
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String startPage; */
    pub get_startPage: unsafe extern "C" fn (this: *const nsIBrowserHandler, aStartPage: *mut nsACString) -> nsresult,
    pub set_startPage: unsafe extern "C" fn (this: *const nsIBrowserHandler, aStartPage: *const nsACString) -> nsresult,

    /* attribute AUTF8String defaultArgs; */
    pub get_defaultArgs: unsafe extern "C" fn (this: *const nsIBrowserHandler, aDefaultArgs: *mut nsACString) -> nsresult,
    pub set_defaultArgs: unsafe extern "C" fn (this: *const nsIBrowserHandler, aDefaultArgs: *const nsACString) -> nsresult,

    /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
    pub getFeatures: unsafe extern "C" fn (this: *const nsIBrowserHandler, aCmdLine: *const nsICommandLine, _retval: *mut nsACString) -> nsresult,

}


impl nsIBrowserHandler {
    /* attribute AUTF8String startPage; */
    #[inline]
    pub unsafe fn get_startPage(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_startPage)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_startPage(&self, aStartPage: &[u8]) -> Result<(), nsresult> {
        let aStartPage = nsCString::from(aStartPage);
        match ((*self.vtable).set_startPage)(self as *const _, &*aStartPage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String defaultArgs; */
    #[inline]
    pub unsafe fn get_defaultArgs(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_defaultArgs)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultArgs(&self, aDefaultArgs: &[u8]) -> Result<(), nsresult> {
        let aDefaultArgs = nsCString::from(aDefaultArgs);
        match ((*self.vtable).set_defaultArgs)(self as *const _, &*aDefaultArgs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
    #[inline]
    pub unsafe fn getFeatures(&self, aCmdLine: Option<&nsICommandLine>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getFeatures)(self as *const _, aCmdLine.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


