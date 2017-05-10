//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheSession.idl
//


#[repr(C)]
pub struct nsICacheSession {
    vtable: *const nsICacheSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheSession {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1dd7708c, 0xde48, 0x4ffe,
            [0xb5, 0xaa, 0xcd, 0x21, 0x8c, 0x76, 0x28, 0x87])
    }
}

unsafe impl RefCounted for nsICacheSession {
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
pub trait nsICacheSessionCoerce {
    fn coerce_from(v: &nsICacheSession) -> &Self;
}

impl nsICacheSessionCoerce for nsICacheSession {
    #[inline]
    fn coerce_from(v: &nsICacheSession) -> &Self {
        v
    }
}

impl nsICacheSession {
    #[inline]
    pub fn coerce<T: nsICacheSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheSession {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheSession) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheSessionVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean doomEntriesIfExpired; */
    pub get_doomEntriesIfExpired: unsafe extern "C" fn (this: *const nsICacheSession, aDoomEntriesIfExpired: *mut bool) -> nsresult,
    pub set_doomEntriesIfExpired: unsafe extern "C" fn (this: *const nsICacheSession, aDoomEntriesIfExpired: bool) -> nsresult,

    /* attribute nsIFile profileDirectory; */
    pub get_profileDirectory: unsafe extern "C" fn (this: *const nsICacheSession, aProfileDirectory: *mut *const nsIFile) -> nsresult,
    pub set_profileDirectory: unsafe extern "C" fn (this: *const nsICacheSession, aProfileDirectory: *const nsIFile) -> nsresult,

    /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
    pub openCacheEntry: unsafe extern "C" fn (this: *const nsICacheSession, key: *const nsACString, accessRequested: nsCacheAccessMode, blockingMode: bool, _retval: *mut *const nsICacheEntryDescriptor) -> nsresult,

    /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
    pub asyncOpenCacheEntry: unsafe extern "C" fn (this: *const nsICacheSession, key: *const nsACString, accessRequested: nsCacheAccessMode, listener: *const nsICacheListener, noWait: bool) -> nsresult,

    /* void evictEntries (); */
    pub evictEntries: unsafe extern "C" fn (this: *const nsICacheSession) -> nsresult,

    /* boolean isStorageEnabled (); */
    pub isStorageEnabled: unsafe extern "C" fn (this: *const nsICacheSession, _retval: *mut bool) -> nsresult,

    /* void doomEntry (in ACString key, in nsICacheListener listener); */
    pub doomEntry: unsafe extern "C" fn (this: *const nsICacheSession, key: *const nsACString, listener: *const nsICacheListener) -> nsresult,

    /* attribute boolean isPrivate; */
    pub get_isPrivate: unsafe extern "C" fn (this: *const nsICacheSession, aIsPrivate: *mut bool) -> nsresult,
    pub set_isPrivate: unsafe extern "C" fn (this: *const nsICacheSession, aIsPrivate: bool) -> nsresult,

}


impl nsICacheSession {
    /* attribute boolean doomEntriesIfExpired; */
    #[inline]
    pub unsafe fn get_doomEntriesIfExpired(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_doomEntriesIfExpired)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_doomEntriesIfExpired(&self, aDoomEntriesIfExpired: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_doomEntriesIfExpired)(self as *const _, aDoomEntriesIfExpired) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFile profileDirectory; */
    #[inline]
    pub unsafe fn get_profileDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_profileDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_profileDirectory(&self, aProfileDirectory: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_profileDirectory)(self as *const _, aProfileDirectory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsICacheEntryDescriptor openCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in boolean blockingMode); */
    #[inline]
    pub unsafe fn openCacheEntry(&self, key: &[u8], accessRequested: nsCacheAccessMode, blockingMode: bool) -> Result<Option<RefPtr<nsICacheEntryDescriptor>>, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openCacheEntry)(self as *const _, &*key, accessRequested, blockingMode, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void asyncOpenCacheEntry (in ACString key, in nsCacheAccessMode accessRequested, in nsICacheListener listener, [optional] in boolean noWait); */
    #[inline]
    pub unsafe fn asyncOpenCacheEntry(&self, key: &[u8], accessRequested: nsCacheAccessMode, listener: Option<&nsICacheListener>, noWait: bool) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).asyncOpenCacheEntry)(self as *const _, &*key, accessRequested, listener.map_or(::std::ptr::null(), |x| x as *const _), noWait) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evictEntries (); */
    #[inline]
    pub unsafe fn evictEntries(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).evictEntries)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isStorageEnabled (); */
    #[inline]
    pub unsafe fn isStorageEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isStorageEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doomEntry (in ACString key, in nsICacheListener listener); */
    #[inline]
    pub unsafe fn doomEntry(&self, key: &[u8], listener: Option<&nsICacheListener>) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).doomEntry)(self as *const _, &*key, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isPrivate; */
    #[inline]
    pub unsafe fn get_isPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isPrivate(&self, aIsPrivate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isPrivate)(self as *const _, aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


