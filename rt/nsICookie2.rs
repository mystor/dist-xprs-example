//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookie2.idl
//


#[repr(C)]
pub struct nsICookie2 {
    vtable: *const nsICookie2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookie2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x05c420e5, 0x03d0, 0x4c7b,
            [0xa6, 0x05, 0xdf, 0x7e, 0xbe, 0x5c, 0xa3, 0x26])
    }
}

unsafe impl RefCounted for nsICookie2 {
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
pub trait nsICookie2Coerce {
    fn coerce_from(v: &nsICookie2) -> &Self;
}

impl nsICookie2Coerce for nsICookie2 {
    #[inline]
    fn coerce_from(v: &nsICookie2) -> &Self {
        v
    }
}

impl nsICookie2 {
    #[inline]
    pub fn coerce<T: nsICookie2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookie2 {
    type Target = nsICookie;
    #[inline]
    fn deref(&self) -> &nsICookie {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICookieCoerce> nsICookie2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsICookie2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookie2VTable {
    pub __base: nsICookieVTable,

    /* readonly attribute AUTF8String rawHost; */
    pub get_rawHost: unsafe extern "C" fn (this: *const nsICookie2, aRawHost: *mut nsACString) -> nsresult,

    /* readonly attribute boolean isSession; */
    pub get_isSession: unsafe extern "C" fn (this: *const nsICookie2, aIsSession: *mut bool) -> nsresult,

    /* readonly attribute int64_t expiry; */
    pub get_expiry: unsafe extern "C" fn (this: *const nsICookie2, aExpiry: *mut int64_t) -> nsresult,

    /* readonly attribute boolean isHttpOnly; */
    pub get_isHttpOnly: unsafe extern "C" fn (this: *const nsICookie2, aIsHttpOnly: *mut bool) -> nsresult,

    /* readonly attribute int64_t creationTime; */
    pub get_creationTime: unsafe extern "C" fn (this: *const nsICookie2, aCreationTime: *mut int64_t) -> nsresult,

    /* readonly attribute int64_t lastAccessed; */
    pub get_lastAccessed: unsafe extern "C" fn (this: *const nsICookie2, aLastAccessed: *mut int64_t) -> nsresult,

}


impl nsICookie2 {
    /* readonly attribute AUTF8String rawHost; */
    #[inline]
    pub unsafe fn get_rawHost(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_rawHost)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSession; */
    #[inline]
    pub unsafe fn get_isSession(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSession)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t expiry; */
    #[inline]
    pub unsafe fn get_expiry(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expiry)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isHttpOnly; */
    #[inline]
    pub unsafe fn get_isHttpOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isHttpOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t creationTime; */
    #[inline]
    pub unsafe fn get_creationTime(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_creationTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t lastAccessed; */
    #[inline]
    pub unsafe fn get_lastAccessed(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastAccessed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


