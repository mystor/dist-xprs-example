//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheStorageService.idl
//


pub mod nsICacheStorageService_consts {
    pub const PURGE_DISK_DATA_ONLY: i64 = 1;
    pub const PURGE_DISK_ALL: i64 = 2;
    pub const PURGE_EVERYTHING: i64 = 3;
}


#[repr(C)]
pub struct nsICacheStorageService {
    vtable: *const nsICacheStorageServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheStorageService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae29c44b, 0xfbc3, 0x4552,
            [0xaf, 0xaf, 0x0a, 0x15, 0x7c, 0xe7, 0x71, 0xe7])
    }
}

unsafe impl RefCounted for nsICacheStorageService {
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
pub trait nsICacheStorageServiceCoerce {
    fn coerce_from(v: &nsICacheStorageService) -> &Self;
}

impl nsICacheStorageServiceCoerce for nsICacheStorageService {
    #[inline]
    fn coerce_from(v: &nsICacheStorageService) -> &Self {
        v
    }
}

impl nsICacheStorageService {
    #[inline]
    pub fn coerce<T: nsICacheStorageServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheStorageService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheStorageServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheStorageServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub memoryCacheStorage: unsafe extern "C" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut *const nsICacheStorage) -> nsresult,

    /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
    pub diskCacheStorage: unsafe extern "C" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, aLookupAppCache: bool, _retval: *mut *const nsICacheStorage) -> nsresult,

    /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub pinningCacheStorage: unsafe extern "C" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut *const nsICacheStorage) -> nsresult,

    /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
    pub appCacheStorage: unsafe extern "C" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, aApplicationCache: *const nsIApplicationCache, _retval: *mut *const nsICacheStorage) -> nsresult,

    /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    pub synthesizedCacheStorage: unsafe extern "C" fn (this: *const nsICacheStorageService, aLoadContextInfo: *const nsILoadContextInfo, _retval: *mut *const nsICacheStorage) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsICacheStorageService) -> nsresult,

    /* void purgeFromMemory (in uint32_t aWhat); */
    pub purgeFromMemory: unsafe extern "C" fn (this: *const nsICacheStorageService, aWhat: uint32_t) -> nsresult,

    /* readonly attribute nsIEventTarget ioTarget; */
    pub get_ioTarget: unsafe extern "C" fn (this: *const nsICacheStorageService, aIoTarget: *mut *const nsIEventTarget) -> nsresult,

    /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
    pub asyncGetDiskConsumption: unsafe extern "C" fn (this: *const nsICacheStorageService, aObserver: *const nsICacheStorageConsumptionObserver) -> nsresult,

    /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    pub asyncVisitAllStorages: unsafe extern "C" fn (this: *const nsICacheStorageService, aVisitor: *const nsICacheStorageVisitor, aVisitEntries: bool) -> nsresult,

}


impl nsICacheStorageService {
    /* nsICacheStorage memoryCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn memoryCacheStorage(&self, aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<Option<RefPtr<nsICacheStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).memoryCacheStorage)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICacheStorage diskCacheStorage (in nsILoadContextInfo aLoadContextInfo, in bool aLookupAppCache); */
    #[inline]
    pub unsafe fn diskCacheStorage(&self, aLoadContextInfo: Option<&nsILoadContextInfo>, aLookupAppCache: bool) -> Result<Option<RefPtr<nsICacheStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).diskCacheStorage)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), aLookupAppCache, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICacheStorage pinningCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn pinningCacheStorage(&self, aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<Option<RefPtr<nsICacheStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).pinningCacheStorage)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICacheStorage appCacheStorage (in nsILoadContextInfo aLoadContextInfo, in nsIApplicationCache aApplicationCache); */
    #[inline]
    pub unsafe fn appCacheStorage(&self, aLoadContextInfo: Option<&nsILoadContextInfo>, aApplicationCache: Option<&nsIApplicationCache>) -> Result<Option<RefPtr<nsICacheStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).appCacheStorage)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), aApplicationCache.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICacheStorage synthesizedCacheStorage (in nsILoadContextInfo aLoadContextInfo); */
    #[inline]
    pub unsafe fn synthesizedCacheStorage(&self, aLoadContextInfo: Option<&nsILoadContextInfo>) -> Result<Option<RefPtr<nsICacheStorage>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).synthesizedCacheStorage)(self as *const _, aLoadContextInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void purgeFromMemory (in uint32_t aWhat); */
    #[inline]
    pub unsafe fn purgeFromMemory(&self, aWhat: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).purgeFromMemory)(self as *const _, aWhat) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIEventTarget ioTarget; */
    #[inline]
    pub unsafe fn get_ioTarget(&self, ) -> Result<Option<RefPtr<nsIEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ioTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void asyncGetDiskConsumption (in nsICacheStorageConsumptionObserver aObserver); */
    #[inline]
    pub unsafe fn asyncGetDiskConsumption(&self, aObserver: Option<&nsICacheStorageConsumptionObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncGetDiskConsumption)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncVisitAllStorages (in nsICacheStorageVisitor aVisitor, in boolean aVisitEntries); */
    #[inline]
    pub unsafe fn asyncVisitAllStorages(&self, aVisitor: Option<&nsICacheStorageVisitor>, aVisitEntries: bool) -> Result<(), nsresult> {

        match ((*self.vtable).asyncVisitAllStorages)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _), aVisitEntries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICacheStorageConsumptionObserver {
    vtable: *const nsICacheStorageConsumptionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheStorageConsumptionObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7728ab5b, 0x4c01, 0x4483,
            [0xa6, 0x06, 0x32, 0xbf, 0x5b, 0x81, 0x36, 0xcb])
    }
}

unsafe impl RefCounted for nsICacheStorageConsumptionObserver {
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
pub trait nsICacheStorageConsumptionObserverCoerce {
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self;
}

impl nsICacheStorageConsumptionObserverCoerce for nsICacheStorageConsumptionObserver {
    #[inline]
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self {
        v
    }
}

impl nsICacheStorageConsumptionObserver {
    #[inline]
    pub fn coerce<T: nsICacheStorageConsumptionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheStorageConsumptionObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheStorageConsumptionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheStorageConsumptionObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheStorageConsumptionObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
    pub onNetworkCacheDiskConsumption: unsafe extern "C" fn (this: *const nsICacheStorageConsumptionObserver, aDiskSize: int64_t) -> nsresult,

}


impl nsICacheStorageConsumptionObserver {
    /* void onNetworkCacheDiskConsumption (in int64_t aDiskSize); */
    #[inline]
    pub unsafe fn onNetworkCacheDiskConsumption(&self, aDiskSize: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).onNetworkCacheDiskConsumption)(self as *const _, aDiskSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


