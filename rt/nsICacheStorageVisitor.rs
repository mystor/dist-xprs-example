//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheStorageVisitor.idl
//


#[repr(C)]
pub struct nsICacheStorageVisitor {
    vtable: *const nsICacheStorageVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheStorageVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6cc7c253, 0x93b6, 0x482b,
            [0x8e, 0x9d, 0x1e, 0x04, 0xd8, 0xe9, 0xd6, 0x55])
    }
}

unsafe impl RefCounted for nsICacheStorageVisitor {
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
pub trait nsICacheStorageVisitorCoerce {
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self;
}

impl nsICacheStorageVisitorCoerce for nsICacheStorageVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self {
        v
    }
}

impl nsICacheStorageVisitor {
    #[inline]
    pub fn coerce<T: nsICacheStorageVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheStorageVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheStorageVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheStorageVisitorVTable {
    pub __base: nsISupportsVTable,

    /* void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory); */
    pub onCacheStorageInfo: unsafe extern "C" fn (this: *const nsICacheStorageVisitor, aEntryCount: uint32_t, aConsumption: uint64_t, aCapacity: uint64_t, aDiskDirectory: *const nsIFile) -> nsresult,

    /* void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo); */
    pub onCacheEntryInfo: unsafe extern "C" fn (this: *const nsICacheStorageVisitor, aURI: *const nsIURI, aIdEnhance: *const nsACString, aDataSize: int64_t, aFetchCount: libc::int32_t, aLastModifiedTime: uint32_t, aExpirationTime: uint32_t, aPinned: bool, aInfo: *const nsILoadContextInfo) -> nsresult,

    /* void onCacheEntryVisitCompleted (); */
    pub onCacheEntryVisitCompleted: unsafe extern "C" fn (this: *const nsICacheStorageVisitor) -> nsresult,

}


impl nsICacheStorageVisitor {
    /* void onCacheStorageInfo (in uint32_t aEntryCount, in uint64_t aConsumption, in uint64_t aCapacity, in nsIFile aDiskDirectory); */
    #[inline]
    pub unsafe fn onCacheStorageInfo(&self, aEntryCount: uint32_t, aConsumption: uint64_t, aCapacity: uint64_t, aDiskDirectory: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheStorageInfo)(self as *const _, aEntryCount, aConsumption, aCapacity, aDiskDirectory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCacheEntryInfo (in nsIURI aURI, in ACString aIdEnhance, in int64_t aDataSize, in long aFetchCount, in uint32_t aLastModifiedTime, in uint32_t aExpirationTime, in boolean aPinned, in nsILoadContextInfo aInfo); */
    #[inline]
    pub unsafe fn onCacheEntryInfo(&self, aURI: Option<&nsIURI>, aIdEnhance: &[u8], aDataSize: int64_t, aFetchCount: libc::int32_t, aLastModifiedTime: uint32_t, aExpirationTime: uint32_t, aPinned: bool, aInfo: Option<&nsILoadContextInfo>) -> Result<(), nsresult> {
        let aIdEnhance = nsCString::from(aIdEnhance);
        match ((*self.vtable).onCacheEntryInfo)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdEnhance, aDataSize, aFetchCount, aLastModifiedTime, aExpirationTime, aPinned, aInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onCacheEntryVisitCompleted (); */
    #[inline]
    pub unsafe fn onCacheEntryVisitCompleted(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onCacheEntryVisitCompleted)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


