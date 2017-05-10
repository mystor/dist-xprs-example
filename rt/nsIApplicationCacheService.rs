//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationCacheService.idl
//


#[repr(C)]
pub struct nsIApplicationCacheService {
    vtable: *const nsIApplicationCacheServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationCacheService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8b6546c, 0x6cec, 0x4bda,
            [0x82, 0xdf, 0x08, 0xe0, 0x06, 0xa9, 0x7b, 0x56])
    }
}

unsafe impl RefCounted for nsIApplicationCacheService {
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
pub trait nsIApplicationCacheServiceCoerce {
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self;
}

impl nsIApplicationCacheServiceCoerce for nsIApplicationCacheService {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self {
        v
    }
}

impl nsIApplicationCacheService {
    #[inline]
    pub fn coerce<T: nsIApplicationCacheServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationCacheService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationCacheServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationCacheService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationCacheServiceVTable {
    pub __base: nsISupportsVTable,

    /* ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo); */
    pub buildGroupIDForInfo: unsafe extern "C" fn (this: *const nsIApplicationCacheService, aManifestURL: *const nsIURI, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut nsACString) -> nsresult,

    /* ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix); */
    pub buildGroupIDForSuffix: unsafe extern "C" fn (this: *const nsIApplicationCacheService, aManifestURL: *const nsIURI, aOriginSuffix: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* nsIApplicationCache createApplicationCache (in ACString group); */
    pub createApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheService, group: *const nsACString, _retval: *mut *const nsIApplicationCache) -> nsresult,

    /* nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota); */
    pub createCustomApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheService, group: *const nsACString, profileDir: *const nsIFile, quota: int32_t, _retval: *mut *const nsIApplicationCache) -> nsresult,

    /* nsIApplicationCache getApplicationCache (in ACString clientID); */
    pub getApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheService, clientID: *const nsACString, _retval: *mut *const nsIApplicationCache) -> nsresult,

    /* nsIApplicationCache getActiveCache (in ACString group); */
    pub getActiveCache: unsafe extern "C" fn (this: *const nsIApplicationCacheService, group: *const nsACString, _retval: *mut *const nsIApplicationCache) -> nsresult,

    /* void deactivateGroup (in ACString group); */
    pub deactivateGroup: unsafe extern "C" fn (this: *const nsIApplicationCacheService, group: *const nsACString) -> nsresult,

    /* void evict (in nsILoadContextInfo aLoadContextInfo); */
    pub evict: unsafe extern "C" fn (this: *const nsIApplicationCacheService, aLoadContextInfo: *const nsILoadContextInfo) -> nsresult,

    /* void evictMatchingOriginAttributes (in AString aPattern); */
    pub evictMatchingOriginAttributes: unsafe extern "C" fn (this: *const nsIApplicationCacheService, aPattern: *const nsAString) -> nsresult,

    /* nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo); */
    pub chooseApplicationCache: unsafe extern "C" fn (this: *const nsIApplicationCacheService, key: *const nsACString, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut *const nsIApplicationCache) -> nsresult,

    /* void cacheOpportunistically (in nsIApplicationCache cache, in ACString key); */
    pub cacheOpportunistically: unsafe extern "C" fn (this: *const nsIApplicationCacheService, cache: *const nsIApplicationCache, key: *const nsACString) -> nsresult,

    /* void getGroups ([optional] out unsigned long count, [array, size_is (count), retval] out string groupIDs); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGroups: *const ::libc::c_void,

    /* void getGroupsTimeOrdered ([optional] out unsigned long count, [array, size_is (count), retval] out string groupIDs); */
    /// Unable to call function as its signature contains a non-rust type
    pub getGroupsTimeOrdered: *const ::libc::c_void,

}


impl nsIApplicationCacheService {
    /* ACString buildGroupIDForInfo (in nsIURI aManifestURL, in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn buildGroupIDForInfo(&self, aManifestURL: Option<&nsIURI>, aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).buildGroupIDForInfo)(self as *const _, aManifestURL.map_or(::std::ptr::null(), |x| x as *const _), aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString buildGroupIDForSuffix (in nsIURI aManifestURL, in ACString aOriginSuffix); */
    #[inline]
    pub unsafe fn buildGroupIDForSuffix(&self, aManifestURL: Option<&nsIURI>, aOriginSuffix: &[u8]) -> Result<nsCString, nsresult> {
        let aOriginSuffix = nsCString::from(aOriginSuffix);
        let mut _retval = nsCString::new();
        match ((*self.vtable).buildGroupIDForSuffix)(self as *const _, aManifestURL.map_or(::std::ptr::null(), |x| x as *const _), &*aOriginSuffix, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIApplicationCache createApplicationCache (in ACString group); */
    #[inline]
    pub unsafe fn createApplicationCache(&self, group: &[u8]) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let group = nsCString::from(group);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createApplicationCache)(self as *const _, &*group, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIApplicationCache createCustomApplicationCache (in ACString group, in nsIFile profileDir, in int32_t quota); */
    #[inline]
    pub unsafe fn createCustomApplicationCache(&self, group: &[u8], profileDir: Option<&nsIFile>, quota: int32_t) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let group = nsCString::from(group);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createCustomApplicationCache)(self as *const _, &*group, profileDir.map_or(::std::ptr::null(), |x| x as *const _), quota, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIApplicationCache getApplicationCache (in ACString clientID); */
    #[inline]
    pub unsafe fn getApplicationCache(&self, clientID: &[u8]) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let clientID = nsCString::from(clientID);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getApplicationCache)(self as *const _, &*clientID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIApplicationCache getActiveCache (in ACString group); */
    #[inline]
    pub unsafe fn getActiveCache(&self, group: &[u8]) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let group = nsCString::from(group);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getActiveCache)(self as *const _, &*group, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void deactivateGroup (in ACString group); */
    #[inline]
    pub unsafe fn deactivateGroup(&self, group: &[u8]) -> Result<(), nsresult> {
        let group = nsCString::from(group);
        match ((*self.vtable).deactivateGroup)(self as *const _, &*group) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evict (in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn evict(&self, aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).evict)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evictMatchingOriginAttributes (in AString aPattern); */
    #[inline]
    pub unsafe fn evictMatchingOriginAttributes(&self, aPattern: &[u16]) -> Result<(), nsresult> {
        let aPattern = nsString::from(aPattern);
        match ((*self.vtable).evictMatchingOriginAttributes)(self as *const _, &*aPattern) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIApplicationCache chooseApplicationCache (in ACString key, [optional] in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn chooseApplicationCache(&self, key: &[u8], aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<Option<RefPtr<nsIApplicationCache>>, nsresult> {
        let key = nsCString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).chooseApplicationCache)(self as *const _, &*key, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void cacheOpportunistically (in nsIApplicationCache cache, in ACString key); */
    #[inline]
    pub unsafe fn cacheOpportunistically(&self, cache: Option<&nsIApplicationCache>, key: &[u8]) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        match ((*self.vtable).cacheOpportunistically)(self as *const _, cache.map_or(::std::ptr::null(), |x| x as *const _), &*key) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getGroups ([optional] out unsigned long count, [array, size_is (count), retval] out string groupIDs); */


    /* void getGroupsTimeOrdered ([optional] out unsigned long count, [array, size_is (count), retval] out string groupIDs); */


}


