//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpProtocolHandler.idl
//


#[repr(C)]
pub struct nsIHttpProtocolHandler {
    vtable: *const nsIHttpProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc48126d9, 0x2ddd, 0x485b,
            [0xa5, 0x1a, 0x37, 0x8e, 0x91, 0x7e, 0x75, 0xf8])
    }
}

unsafe impl RefCounted for nsIHttpProtocolHandler {
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
pub trait nsIHttpProtocolHandlerCoerce {
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self;
}

impl nsIHttpProtocolHandlerCoerce for nsIHttpProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self {
        v
    }
}

impl nsIHttpProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIHttpProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpProtocolHandler {
    type Target = nsIProxiedProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProxiedProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProxiedProtocolHandlerCoerce> nsIHttpProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpProtocolHandlerVTable {
    pub __base: nsIProxiedProtocolHandlerVTable,

    /* [must_use] readonly attribute ACString userAgent; */
    pub get_userAgent: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aUserAgent: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString appName; */
    pub get_appName: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aAppName: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString appVersion; */
    pub get_appVersion: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aAppVersion: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString platform; */
    pub get_platform: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aPlatform: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString oscpu; */
    pub get_oscpu: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aOscpu: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute ACString misc; */
    pub get_misc: unsafe extern "C" fn (this: *const nsIHttpProtocolHandler, aMisc: *mut nsACString) -> nsresult,

}


impl nsIHttpProtocolHandler {
    /* [must_use] readonly attribute ACString userAgent; */
    #[inline]
    pub unsafe fn get_userAgent(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_userAgent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString appName; */
    #[inline]
    pub unsafe fn get_appName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_appName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString appVersion; */
    #[inline]
    pub unsafe fn get_appVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_appVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString platform; */
    #[inline]
    pub unsafe fn get_platform(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_platform)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString oscpu; */
    #[inline]
    pub unsafe fn get_oscpu(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_oscpu)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString misc; */
    #[inline]
    pub unsafe fn get_misc(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_misc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


