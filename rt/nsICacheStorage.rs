//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheStorage.idl
//


pub mod nsICacheStorage_consts {
    pub const OPEN_NORMALLY: i64 = 0;
    pub const OPEN_TRUNCATE: i64 = 1;
    pub const OPEN_READONLY: i64 = 2;
    pub const OPEN_PRIORITY: i64 = 4;
    pub const OPEN_BYPASS_IF_BUSY: i64 = 8;
    pub const CHECK_MULTITHREADED: i64 = 16;
    pub const OPEN_SECRETLY: i64 = 32;
    pub const OPEN_INTERCEPTED: i64 = 64;
}


#[repr(C)]
pub struct nsICacheStorage {
    vtable: *const nsICacheStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35d104a6, 0xd252, 0x4fd4,
            [0x8a, 0x56, 0x3c, 0x14, 0x65, 0x7c, 0xad, 0x3b])
    }
}

unsafe impl RefCounted for nsICacheStorage {
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
pub trait nsICacheStorageCoerce {
    fn coerce_from(v: &nsICacheStorage) -> &Self;
}

impl nsICacheStorageCoerce for nsICacheStorage {
    #[inline]
    fn coerce_from(v: &nsICacheStorage) -> &Self {
        v
    }
}

impl nsICacheStorage {
    #[inline]
    pub fn coerce<T: nsICacheStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheStorageVTable {
    pub __base: nsISupportsVTable,

    /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
    pub asyncOpenURI: unsafe extern "C" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const nsACString, aFlags: uint32_t, aCallback: *const nsICacheEntryOpenCallback) -> nsresult,

    /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
    pub openTruncate: unsafe extern "C" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const nsACString, _retval: *mut *const nsICacheEntry) -> nsresult,

    /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
    pub exists: unsafe extern "C" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
    pub getCacheIndexEntryAttrs: unsafe extern "C" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const nsACString, aHasAltData: *mut bool, aSizeInKB: *mut uint32_t) -> nsresult,

    /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
    pub asyncDoomURI: unsafe extern "C" fn (this: *const nsICacheStorage, aURI: *const nsIURI, aIdExtension: *const nsACString, aCallback: *const nsICacheEntryDoomCallback) -> nsresult,

    /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
    pub asyncEvictStorage: unsafe extern "C" fn (this: *const nsICacheStorage, aCallback: *const nsICacheEntryDoomCallback) -> nsresult,

    /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    pub asyncVisitStorage: unsafe extern "C" fn (this: *const nsICacheStorage, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> nsresult,

}


impl nsICacheStorage {
    /* void asyncOpenURI (in nsIURI aURI, in ACString aIdExtension, in uint32_t aFlags, in nsICacheEntryOpenCallback aCallback); */
    #[inline]
    pub unsafe fn asyncOpenURI(&self, aURI: Option<&nsIURI>, aIdExtension: &[u8], aFlags: uint32_t, aCallback: Option<&nsICacheEntryOpenCallback>) -> Result<(), nsresult> {
        let aIdExtension = nsCString::from(aIdExtension);
        match ((*self.vtable).asyncOpenURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdExtension, aFlags, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsICacheEntry openTruncate (in nsIURI aURI, in ACString aIdExtension); */
    #[inline]
    pub unsafe fn openTruncate(&self, aURI: Option<&nsIURI>, aIdExtension: &[u8]) -> Result<Option<RefPtr<nsICacheEntry>>, nsresult> {
        let aIdExtension = nsCString::from(aIdExtension);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openTruncate)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdExtension, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean exists (in nsIURI aURI, in ACString aIdExtension); */
    #[inline]
    pub unsafe fn exists(&self, aURI: Option<&nsIURI>, aIdExtension: &[u8]) -> Result<bool, nsresult> {
        let aIdExtension = nsCString::from(aIdExtension);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).exists)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdExtension, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCacheIndexEntryAttrs (in nsIURI aURI, in ACString aIdExtension, out bool aHasAltData, out uint32_t aSizeInKB); */
    #[inline]
    pub unsafe fn getCacheIndexEntryAttrs(&self, aURI: Option<&nsIURI>, aIdExtension: &[u8]) -> Result<(bool, uint32_t), nsresult> {
        let aIdExtension = nsCString::from(aIdExtension);
        let mut aHasAltData: bool = ::std::mem::zeroed();
        let mut aSizeInKB: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCacheIndexEntryAttrs)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdExtension, &mut aHasAltData as *mut _, &mut aSizeInKB as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aHasAltData, aSizeInKB))
    }

    /* void asyncDoomURI (in nsIURI aURI, in ACString aIdExtension, in nsICacheEntryDoomCallback aCallback); */
    #[inline]
    pub unsafe fn asyncDoomURI(&self, aURI: Option<&nsIURI>, aIdExtension: &[u8], aCallback: Option<&nsICacheEntryDoomCallback>) -> Result<(), nsresult> {
        let aIdExtension = nsCString::from(aIdExtension);
        match ((*self.vtable).asyncDoomURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aIdExtension, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncEvictStorage (in nsICacheEntryDoomCallback aCallback); */
    #[inline]
    pub unsafe fn asyncEvictStorage(&self, aCallback: Option<&nsICacheEntryDoomCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncEvictStorage)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncVisitStorage (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    #[inline]
    pub unsafe fn asyncVisitStorage(&self, aVisitor: Option<&nsICacheStorageVisitor>, aVisitEntries: bool) -> Result<(), nsresult> {

        match ((*self.vtable).asyncVisitStorage)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _), aVisitEntries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


