//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalProtocolHandler.idl
//


#[repr(C)]
pub struct nsIExternalProtocolHandler {
    vtable: *const nsIExternalProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExternalProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0e61f3b2, 0x34d7, 0x4c79,
            [0xbf, 0xdc, 0x48, 0x60, 0xbc, 0x73, 0x41, 0xb7])
    }
}

unsafe impl RefCounted for nsIExternalProtocolHandler {
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
pub trait nsIExternalProtocolHandlerCoerce {
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self;
}

impl nsIExternalProtocolHandlerCoerce for nsIExternalProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self {
        v
    }
}

impl nsIExternalProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIExternalProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExternalProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolHandlerCoerce> nsIExternalProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExternalProtocolHandlerVTable {
    pub __base: nsIProtocolHandlerVTable,

    /* boolean externalAppExistsForScheme (in ACString scheme); */
    pub externalAppExistsForScheme: unsafe extern "C" fn (this: *const nsIExternalProtocolHandler, scheme: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl nsIExternalProtocolHandler {
    /* boolean externalAppExistsForScheme (in ACString scheme); */
    #[inline]
    pub unsafe fn externalAppExistsForScheme(&self, scheme: &[u8]) -> Result<bool, nsresult> {
        let scheme = nsCString::from(scheme);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).externalAppExistsForScheme)(self as *const _, &*scheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


