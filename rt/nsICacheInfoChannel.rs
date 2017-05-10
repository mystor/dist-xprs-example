//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheInfoChannel.idl
//


#[repr(C)]
pub struct nsICacheInfoChannel {
    vtable: *const nsICacheInfoChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheInfoChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x72c34415, 0xc6eb, 0x48af,
            [0x85, 0x1f, 0x77, 0x2f, 0xa9, 0xee, 0x59, 0x72])
    }
}

unsafe impl RefCounted for nsICacheInfoChannel {
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
pub trait nsICacheInfoChannelCoerce {
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self;
}

impl nsICacheInfoChannelCoerce for nsICacheInfoChannel {
    #[inline]
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self {
        v
    }
}

impl nsICacheInfoChannel {
    #[inline]
    pub fn coerce<T: nsICacheInfoChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheInfoChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheInfoChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheInfoChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheInfoChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t cacheTokenExpirationTime; */
    pub get_cacheTokenExpirationTime: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aCacheTokenExpirationTime: *mut uint32_t) -> nsresult,

    /* attribute ACString cacheTokenCachedCharset; */
    pub get_cacheTokenCachedCharset: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aCacheTokenCachedCharset: *mut nsACString) -> nsresult,
    pub set_cacheTokenCachedCharset: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aCacheTokenCachedCharset: *const nsACString) -> nsresult,

    /* boolean isFromCache (); */
    pub isFromCache: unsafe extern "C" fn (this: *const nsICacheInfoChannel, _retval: *mut bool) -> nsresult,

    /* attribute nsISupports cacheKey; */
    pub get_cacheKey: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aCacheKey: *mut *const nsISupports) -> nsresult,
    pub set_cacheKey: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aCacheKey: *const nsISupports) -> nsresult,

    /* attribute boolean allowStaleCacheContent; */
    pub get_allowStaleCacheContent: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aAllowStaleCacheContent: *mut bool) -> nsresult,
    pub set_allowStaleCacheContent: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aAllowStaleCacheContent: bool) -> nsresult,

    /* void preferAlternativeDataType (in ACString type); */
    pub preferAlternativeDataType: unsafe extern "C" fn (this: *const nsICacheInfoChannel, type_: *const nsACString) -> nsresult,

    /* readonly attribute ACString alternativeDataType; */
    pub get_alternativeDataType: unsafe extern "C" fn (this: *const nsICacheInfoChannel, aAlternativeDataType: *mut nsACString) -> nsresult,

    /* nsIOutputStream openAlternativeOutputStream (in ACString type); */
    pub openAlternativeOutputStream: unsafe extern "C" fn (this: *const nsICacheInfoChannel, type_: *const nsACString, _retval: *mut *const nsIOutputStream) -> nsresult,

}


impl nsICacheInfoChannel {
    /* readonly attribute uint32_t cacheTokenExpirationTime; */
    #[inline]
    pub unsafe fn get_cacheTokenExpirationTime(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheTokenExpirationTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute ACString cacheTokenCachedCharset; */
    #[inline]
    pub unsafe fn get_cacheTokenCachedCharset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_cacheTokenCachedCharset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cacheTokenCachedCharset(&self, aCacheTokenCachedCharset: &[u8]) -> Result<(), nsresult> {
        let aCacheTokenCachedCharset = nsCString::from(aCacheTokenCachedCharset);
        match ((*self.vtable).set_cacheTokenCachedCharset)(self as *const _, &*aCacheTokenCachedCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isFromCache (); */
    #[inline]
    pub unsafe fn isFromCache(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isFromCache)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsISupports cacheKey; */
    #[inline]
    pub unsafe fn get_cacheKey(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cacheKey)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_cacheKey(&self, aCacheKey: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_cacheKey)(self as *const _, aCacheKey.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowStaleCacheContent; */
    #[inline]
    pub unsafe fn get_allowStaleCacheContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowStaleCacheContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowStaleCacheContent(&self, aAllowStaleCacheContent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowStaleCacheContent)(self as *const _, aAllowStaleCacheContent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void preferAlternativeDataType (in ACString type); */
    #[inline]
    pub unsafe fn preferAlternativeDataType(&self, type_: &[u8]) -> Result<(), nsresult> {
        let type_ = nsCString::from(type_);
        match ((*self.vtable).preferAlternativeDataType)(self as *const _, &*type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString alternativeDataType; */
    #[inline]
    pub unsafe fn get_alternativeDataType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_alternativeDataType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIOutputStream openAlternativeOutputStream (in ACString type); */
    #[inline]
    pub unsafe fn openAlternativeOutputStream(&self, type_: &[u8]) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let type_ = nsCString::from(type_);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openAlternativeOutputStream)(self as *const _, &*type_, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


