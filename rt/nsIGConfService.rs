//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGConfService.idl
//


#[repr(C)]
pub struct nsIGConfService {
    vtable: *const nsIGConfServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGConfService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5009acae, 0x6973, 0x48c3,
            [0xb6, 0xd6, 0x52, 0xc6, 0x92, 0xcc, 0x5d, 0x9d])
    }
}

unsafe impl RefCounted for nsIGConfService {
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
pub trait nsIGConfServiceCoerce {
    fn coerce_from(v: &nsIGConfService) -> &Self;
}

impl nsIGConfServiceCoerce for nsIGConfService {
    #[inline]
    fn coerce_from(v: &nsIGConfService) -> &Self {
        v
    }
}

impl nsIGConfService {
    #[inline]
    pub fn coerce<T: nsIGConfServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGConfService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGConfServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGConfService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGConfServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean getBool (in AUTF8String key); */
    pub getBool: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, _retval: *mut bool) -> nsresult,

    /* AUTF8String getString (in AUTF8String key); */
    pub getString: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* long getInt (in AUTF8String key); */
    pub getInt: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, _retval: *mut libc::int32_t) -> nsresult,

    /* float getFloat (in AUTF8String key); */
    pub getFloat: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, _retval: *mut libc::c_float) -> nsresult,

    /* nsIArray getStringList (in AUTF8String key); */
    pub getStringList: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, _retval: *mut *const nsIArray) -> nsresult,

    /* void setBool (in AUTF8String key, in boolean value); */
    pub setBool: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, value: bool) -> nsresult,

    /* void setString (in AUTF8String key, in AUTF8String value); */
    pub setString: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, value: *const nsACString) -> nsresult,

    /* void setInt (in AUTF8String key, in long value); */
    pub setInt: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, value: libc::int32_t) -> nsresult,

    /* void setFloat (in AUTF8String key, in float value); */
    pub setFloat: unsafe extern "C" fn (this: *const nsIGConfService, key: *const nsACString, value: libc::c_float) -> nsresult,

    /* AUTF8String getAppForProtocol (in AUTF8String scheme, out boolean enabled); */
    pub getAppForProtocol: unsafe extern "C" fn (this: *const nsIGConfService, scheme: *const nsACString, enabled: *mut bool, _retval: *mut nsACString) -> nsresult,

    /* boolean handlerRequiresTerminal (in AUTF8String scheme); */
    pub handlerRequiresTerminal: unsafe extern "C" fn (this: *const nsIGConfService, scheme: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void setAppForProtocol (in AUTF8String scheme, in AUTF8String command); */
    pub setAppForProtocol: unsafe extern "C" fn (this: *const nsIGConfService, scheme: *const nsACString, command: *const nsACString) -> nsresult,

}


impl nsIGConfService {
    /* boolean getBool (in AUTF8String key); */
    #[inline]
    pub unsafe fn getBool(&self, key: &[u8]) -> Result<bool, nsresult> {
        let key = nsCString::from(key);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getBool)(self as *const _, &*key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getString (in AUTF8String key); */
    #[inline]
    pub unsafe fn getString(&self, key: &[u8]) -> Result<nsCString, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getString)(self as *const _, &*key, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getInt (in AUTF8String key); */
    #[inline]
    pub unsafe fn getInt(&self, key: &[u8]) -> Result<libc::int32_t, nsresult> {
        let key = nsCString::from(key);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getInt)(self as *const _, &*key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* float getFloat (in AUTF8String key); */
    #[inline]
    pub unsafe fn getFloat(&self, key: &[u8]) -> Result<libc::c_float, nsresult> {
        let key = nsCString::from(key);
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getFloat)(self as *const _, &*key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIArray getStringList (in AUTF8String key); */
    #[inline]
    pub unsafe fn getStringList(&self, key: &[u8]) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStringList)(self as *const _, &*key, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setBool (in AUTF8String key, in boolean value); */
    #[inline]
    pub unsafe fn setBool(&self, key: &[u8], value: bool) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).setBool)(self as *const _, &*key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setString (in AUTF8String key, in AUTF8String value); */
    #[inline]
    pub unsafe fn setString(&self, key: &[u8], value: &[u8]) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        let value = nsCString::from(value);
        match ((*self.vtable).setString)(self as *const _, &*key, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setInt (in AUTF8String key, in long value); */
    #[inline]
    pub unsafe fn setInt(&self, key: &[u8], value: libc::int32_t) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).setInt)(self as *const _, &*key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFloat (in AUTF8String key, in float value); */
    #[inline]
    pub unsafe fn setFloat(&self, key: &[u8], value: libc::c_float) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).setFloat)(self as *const _, &*key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AUTF8String getAppForProtocol (in AUTF8String scheme, out boolean enabled); */
    #[inline]
    pub unsafe fn getAppForProtocol(&self, scheme: &[u8]) -> Result<(bool, nsCString), nsresult> {
        let scheme = nsCString::from(scheme);
        let mut enabled: bool = ::std::mem::zeroed();
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAppForProtocol)(self as *const _, &*scheme, &mut enabled as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((enabled, _retval))
    }

    /* boolean handlerRequiresTerminal (in AUTF8String scheme); */
    #[inline]
    pub unsafe fn handlerRequiresTerminal(&self, scheme: &[u8]) -> Result<bool, nsresult> {
        let scheme = nsCString::from(scheme);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handlerRequiresTerminal)(self as *const _, &*scheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAppForProtocol (in AUTF8String scheme, in AUTF8String command); */
    #[inline]
    pub unsafe fn setAppForProtocol(&self, scheme: &[u8], command: &[u8]) -> Result<(), nsresult> {
        let scheme = nsCString::from(scheme);
        let command = nsCString::from(command);
        match ((*self.vtable).setAppForProtocol)(self as *const _, &*scheme, &*command) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


