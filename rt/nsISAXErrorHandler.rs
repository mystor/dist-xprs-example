//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXErrorHandler.idl
//


#[repr(C)]
pub struct nsISAXErrorHandler {
    vtable: *const nsISAXErrorHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXErrorHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe02b6693, 0x6cca, 0x11da,
            [0xbe, 0x43, 0x00, 0x14, 0x22, 0x10, 0x69, 0x90])
    }
}

unsafe impl RefCounted for nsISAXErrorHandler {
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
pub trait nsISAXErrorHandlerCoerce {
    fn coerce_from(v: &nsISAXErrorHandler) -> &Self;
}

impl nsISAXErrorHandlerCoerce for nsISAXErrorHandler {
    #[inline]
    fn coerce_from(v: &nsISAXErrorHandler) -> &Self {
        v
    }
}

impl nsISAXErrorHandler {
    #[inline]
    pub fn coerce<T: nsISAXErrorHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXErrorHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXErrorHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXErrorHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXErrorHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void error (in nsISAXLocator locator, in AString error); */
    pub error: unsafe extern "C" fn (this: *const nsISAXErrorHandler, locator: *const nsISAXLocator, error: *const nsAString) -> nsresult,

    /* void fatalError (in nsISAXLocator locator, in AString error); */
    pub fatalError: unsafe extern "C" fn (this: *const nsISAXErrorHandler, locator: *const nsISAXLocator, error: *const nsAString) -> nsresult,

    /* void ignorableWarning (in nsISAXLocator locator, in AString error); */
    pub ignorableWarning: unsafe extern "C" fn (this: *const nsISAXErrorHandler, locator: *const nsISAXLocator, error: *const nsAString) -> nsresult,

}


impl nsISAXErrorHandler {
    /* void error (in nsISAXLocator locator, in AString error); */
    #[inline]
    pub unsafe fn error(&self, locator: Option<&nsISAXLocator>, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).error)(self as *const _, locator.map_or(::std::ptr::null(), |x| x as *const _), &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void fatalError (in nsISAXLocator locator, in AString error); */
    #[inline]
    pub unsafe fn fatalError(&self, locator: Option<&nsISAXLocator>, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).fatalError)(self as *const _, locator.map_or(::std::ptr::null(), |x| x as *const _), &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ignorableWarning (in nsISAXLocator locator, in AString error); */
    #[inline]
    pub unsafe fn ignorableWarning(&self, locator: Option<&nsISAXLocator>, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).ignorableWarning)(self as *const _, locator.map_or(::std::ptr::null(), |x| x as *const _), &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


