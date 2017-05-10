//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGSettingsService.idl
//


#[repr(C)]
pub struct nsIGSettingsCollection {
    vtable: *const nsIGSettingsCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGSettingsCollection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x16d5b0ed, 0xe756, 0x4f1b,
            [0xa8, 0xce, 0x91, 0x32, 0xe8, 0x69, 0xac, 0xd8])
    }
}

unsafe impl RefCounted for nsIGSettingsCollection {
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
pub trait nsIGSettingsCollectionCoerce {
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self;
}

impl nsIGSettingsCollectionCoerce for nsIGSettingsCollection {
    #[inline]
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self {
        v
    }
}

impl nsIGSettingsCollection {
    #[inline]
    pub fn coerce<T: nsIGSettingsCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGSettingsCollection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGSettingsCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGSettingsCollectionVTable {
    pub __base: nsISupportsVTable,

    /* void setString (in AUTF8String key, in AUTF8String value); */
    pub setString: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, value: *const nsACString) -> nsresult,

    /* void setBoolean (in AUTF8String key, in boolean value); */
    pub setBoolean: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, value: bool) -> nsresult,

    /* void setInt (in AUTF8String key, in long value); */
    pub setInt: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, value: libc::int32_t) -> nsresult,

    /* AUTF8String getString (in AUTF8String key); */
    pub getString: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* boolean getBoolean (in AUTF8String key); */
    pub getBoolean: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, _retval: *mut bool) -> nsresult,

    /* long getInt (in AUTF8String key); */
    pub getInt: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, _retval: *mut libc::int32_t) -> nsresult,

    /* nsIArray getStringList (in AUTF8String key); */
    pub getStringList: unsafe extern "C" fn (this: *const nsIGSettingsCollection, key: *const nsACString, _retval: *mut *const nsIArray) -> nsresult,

}


impl nsIGSettingsCollection {
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

    /* void setBoolean (in AUTF8String key, in boolean value); */
    #[inline]
    pub unsafe fn setBoolean(&self, key: &[u8], value: bool) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).setBoolean)(self as *const _, &*key, value) {
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

    /* boolean getBoolean (in AUTF8String key); */
    #[inline]
    pub unsafe fn getBoolean(&self, key: &[u8]) -> Result<bool, nsresult> {
        let key = nsCString::from(key);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getBoolean)(self as *const _, &*key, &mut _retval as *mut _) {
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

}


#[repr(C)]
pub struct nsIGSettingsService {
    vtable: *const nsIGSettingsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGSettingsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x849c088b, 0x57d1, 0x4f24,
            [0xb7, 0xb2, 0x3d, 0xc4, 0xac, 0xb0, 0x4c, 0x0a])
    }
}

unsafe impl RefCounted for nsIGSettingsService {
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
pub trait nsIGSettingsServiceCoerce {
    fn coerce_from(v: &nsIGSettingsService) -> &Self;
}

impl nsIGSettingsServiceCoerce for nsIGSettingsService {
    #[inline]
    fn coerce_from(v: &nsIGSettingsService) -> &Self {
        v
    }
}

impl nsIGSettingsService {
    #[inline]
    pub fn coerce<T: nsIGSettingsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGSettingsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGSettingsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGSettingsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGSettingsServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
    pub getCollectionForSchema: unsafe extern "C" fn (this: *const nsIGSettingsService, schema: *const nsACString, _retval: *mut *const nsIGSettingsCollection) -> nsresult,

}


impl nsIGSettingsService {
    /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
    #[inline]
    pub unsafe fn getCollectionForSchema(&self, schema: &[u8]) -> Result<Option<RefPtr<nsIGSettingsCollection>>, nsresult> {
        let schema = nsCString::from(schema);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCollectionForSchema)(self as *const _, &*schema, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


