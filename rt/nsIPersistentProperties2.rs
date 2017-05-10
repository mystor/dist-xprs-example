//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPersistentProperties2.idl
//


#[repr(C)]
pub struct nsIPropertyElement {
    vtable: *const nsIPropertyElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPropertyElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x283ee646, 0x1aef, 0x11d4,
            [0x98, 0xb3, 0x00, 0xc0, 0x4f, 0xa0, 0xce, 0x9a])
    }
}

unsafe impl RefCounted for nsIPropertyElement {
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
pub trait nsIPropertyElementCoerce {
    fn coerce_from(v: &nsIPropertyElement) -> &Self;
}

impl nsIPropertyElementCoerce for nsIPropertyElement {
    #[inline]
    fn coerce_from(v: &nsIPropertyElement) -> &Self {
        v
    }
}

impl nsIPropertyElement {
    #[inline]
    pub fn coerce<T: nsIPropertyElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPropertyElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPropertyElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPropertyElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPropertyElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String key; */
    pub get_key: unsafe extern "C" fn (this: *const nsIPropertyElement, aKey: *mut nsACString) -> nsresult,
    pub set_key: unsafe extern "C" fn (this: *const nsIPropertyElement, aKey: *const nsACString) -> nsresult,

    /* attribute AString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIPropertyElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIPropertyElement, aValue: *const nsAString) -> nsresult,

}


impl nsIPropertyElement {
    /* attribute AUTF8String key; */
    #[inline]
    pub unsafe fn get_key(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_key)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_key(&self, aKey: &[u8]) -> Result<(), nsresult> {
        let aKey = nsCString::from(aKey);
        match ((*self.vtable).set_key)(self as *const _, &*aKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).set_value)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPersistentProperties {
    vtable: *const nsIPersistentPropertiesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPersistentProperties {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x706867af, 0x0400, 0x4faa,
            [0xbe, 0xb1, 0x0d, 0xae, 0x87, 0x30, 0x87, 0x84])
    }
}

unsafe impl RefCounted for nsIPersistentProperties {
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
pub trait nsIPersistentPropertiesCoerce {
    fn coerce_from(v: &nsIPersistentProperties) -> &Self;
}

impl nsIPersistentPropertiesCoerce for nsIPersistentProperties {
    #[inline]
    fn coerce_from(v: &nsIPersistentProperties) -> &Self {
        v
    }
}

impl nsIPersistentProperties {
    #[inline]
    pub fn coerce<T: nsIPersistentPropertiesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPersistentProperties {
    type Target = nsIProperties;
    #[inline]
    fn deref(&self) -> &nsIProperties {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPropertiesCoerce> nsIPersistentPropertiesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPersistentProperties) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPersistentPropertiesVTable {
    pub __base: nsIPropertiesVTable,

    /* void load (in nsIInputStream input); */
    pub load: unsafe extern "C" fn (this: *const nsIPersistentProperties, input: *const nsIInputStream) -> nsresult,

    /* void save (in nsIOutputStream output, in AUTF8String header); */
    pub save: unsafe extern "C" fn (this: *const nsIPersistentProperties, output: *const nsIOutputStream, header: *const nsACString) -> nsresult,

    /* nsISimpleEnumerator enumerate (); */
    pub enumerate: unsafe extern "C" fn (this: *const nsIPersistentProperties, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* AString getStringProperty (in AUTF8String key); */
    pub getStringProperty: unsafe extern "C" fn (this: *const nsIPersistentProperties, key: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* AString setStringProperty (in AUTF8String key, in AString value); */
    pub setStringProperty: unsafe extern "C" fn (this: *const nsIPersistentProperties, key: *const nsACString, value: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIPersistentProperties {
    /* void load (in nsIInputStream input); */
    #[inline]
    pub unsafe fn load(&self, input: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).load)(self as *const _, input.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void save (in nsIOutputStream output, in AUTF8String header); */
    #[inline]
    pub unsafe fn save(&self, output: Option<&nsIOutputStream>, header: &[u8]) -> Result<(), nsresult> {
        let header = nsCString::from(header);
        match ((*self.vtable).save)(self as *const _, output.map_or(::std::ptr::null(), |x| x as *const _), &*header) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator enumerate (); */
    #[inline]
    pub unsafe fn enumerate(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerate)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getStringProperty (in AUTF8String key); */
    #[inline]
    pub unsafe fn getStringProperty(&self, key: &[u8]) -> Result<nsString, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringProperty)(self as *const _, &*key, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString setStringProperty (in AUTF8String key, in AString value); */
    #[inline]
    pub unsafe fn setStringProperty(&self, key: &[u8], value: &[u16]) -> Result<nsString, nsresult> {
        let key = nsCString::from(key);
        let value = nsString::from(value);
        let mut _retval = nsString::new();
        match ((*self.vtable).setStringProperty)(self as *const _, &*key, &*value, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


