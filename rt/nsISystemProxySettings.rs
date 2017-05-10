//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISystemProxySettings.idl
//


#[repr(C)]
pub struct nsISystemProxySettings {
    vtable: *const nsISystemProxySettingsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISystemProxySettings {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x971591cd, 0x277e, 0x409a,
            [0xbb, 0xf6, 0x0a, 0x79, 0x87, 0x9c, 0xd3, 0x07])
    }
}

unsafe impl RefCounted for nsISystemProxySettings {
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
pub trait nsISystemProxySettingsCoerce {
    fn coerce_from(v: &nsISystemProxySettings) -> &Self;
}

impl nsISystemProxySettingsCoerce for nsISystemProxySettings {
    #[inline]
    fn coerce_from(v: &nsISystemProxySettings) -> &Self {
        v
    }
}

impl nsISystemProxySettings {
    #[inline]
    pub fn coerce<T: nsISystemProxySettingsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISystemProxySettings {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISystemProxySettingsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISystemProxySettings) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISystemProxySettingsVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute bool mainThreadOnly; */
    pub get_mainThreadOnly: unsafe extern "C" fn (this: *const nsISystemProxySettings, aMainThreadOnly: *mut bool) -> nsresult,

    /* readonly attribute AUTF8String PACURI; */
    pub get_PACURI: unsafe extern "C" fn (this: *const nsISystemProxySettings, aPACURI: *mut nsACString) -> nsresult,

    /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
    pub getProxyForURI: unsafe extern "C" fn (this: *const nsISystemProxySettings, testSpec: *const nsACString, testScheme: *const nsACString, testHost: *const nsACString, testPort: int32_t, _retval: *mut nsACString) -> nsresult,

}


impl nsISystemProxySettings {
    /* readonly attribute bool mainThreadOnly; */
    #[inline]
    pub unsafe fn get_mainThreadOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mainThreadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String PACURI; */
    #[inline]
    pub unsafe fn get_PACURI(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_PACURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
    #[inline]
    pub unsafe fn getProxyForURI(&self, testSpec: &[u8], testScheme: &[u8], testHost: &[u8], testPort: int32_t) -> Result<nsCString, nsresult> {
        let testSpec = nsCString::from(testSpec);
        let testScheme = nsCString::from(testScheme);
        let testHost = nsCString::from(testHost);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getProxyForURI)(self as *const _, &*testSpec, &*testScheme, &*testHost, testPort, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


