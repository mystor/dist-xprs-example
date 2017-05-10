//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIOfflineCacheUpdate.idl
//


pub mod nsIOfflineCacheUpdateObserver_consts {
    pub const STATE_ERROR: i64 = 1;
    pub const STATE_CHECKING: i64 = 2;
    pub const STATE_NOUPDATE: i64 = 3;
    pub const STATE_OBSOLETE: i64 = 4;
    pub const STATE_DOWNLOADING: i64 = 5;
    pub const STATE_ITEMSTARTED: i64 = 6;
    pub const STATE_ITEMCOMPLETED: i64 = 7;
    pub const STATE_ITEMPROGRESS: i64 = 8;
    pub const STATE_FINISHED: i64 = 10;
}


#[repr(C)]
pub struct nsIOfflineCacheUpdateObserver {
    vtable: *const nsIOfflineCacheUpdateObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOfflineCacheUpdateObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x47360d57, 0x8ef4, 0x4a5d,
            [0x88, 0x65, 0x1a, 0x27, 0xa7, 0x39, 0xad, 0x1a])
    }
}

unsafe impl RefCounted for nsIOfflineCacheUpdateObserver {
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
pub trait nsIOfflineCacheUpdateObserverCoerce {
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self;
}

impl nsIOfflineCacheUpdateObserverCoerce for nsIOfflineCacheUpdateObserver {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdateObserver {
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOfflineCacheUpdateObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOfflineCacheUpdateObserverVTable {
    pub __base: nsISupportsVTable,

    /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
    pub updateStateChanged: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateObserver, aUpdate: *const nsIOfflineCacheUpdate, state: uint32_t) -> nsresult,

    /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
    pub applicationCacheAvailable: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateObserver, applicationCache: *const nsIApplicationCache) -> nsresult,

}


impl nsIOfflineCacheUpdateObserver {
    /* void updateStateChanged (in nsIOfflineCacheUpdate aUpdate, in uint32_t state); */
    #[inline]
    pub unsafe fn updateStateChanged(&self, aUpdate: Option<&nsIOfflineCacheUpdate>, state: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).updateStateChanged)(self as *const _, aUpdate.map_or(::std::ptr::null(), |x| x as *const _), state) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void applicationCacheAvailable (in nsIApplicationCache applicationCache); */
    #[inline]
    pub unsafe fn applicationCacheAvailable(&self, applicationCache: Option<&nsIApplicationCache>) -> Result<(), nsresult> {

        match ((*self.vtable).applicationCacheAvailable)(self as *const _, applicationCache.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIOfflineCacheUpdate {
    vtable: *const nsIOfflineCacheUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOfflineCacheUpdate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6e3e26ea, 0x45b2, 0x4db7,
            [0x9e, 0x4a, 0x93, 0xb9, 0x65, 0x67, 0x92, 0x98])
    }
}

unsafe impl RefCounted for nsIOfflineCacheUpdate {
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
pub trait nsIOfflineCacheUpdateCoerce {
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self;
}

impl nsIOfflineCacheUpdateCoerce for nsIOfflineCacheUpdate {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdate {
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOfflineCacheUpdate {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOfflineCacheUpdateVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short status; */
    pub get_status: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aStatus: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean partial; */
    pub get_partial: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aPartial: *mut bool) -> nsresult,

    /* readonly attribute boolean isUpgrade; */
    pub get_isUpgrade: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aIsUpgrade: *mut bool) -> nsresult,

    /* readonly attribute ACString updateDomain; */
    pub get_updateDomain: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aUpdateDomain: *mut nsACString) -> nsresult,

    /* readonly attribute nsIURI manifestURI; */
    pub get_manifestURI: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute boolean succeeded; */
    pub get_succeeded: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aSucceeded: *mut bool) -> nsresult,

    /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument, [optional] in nsIFile aCustomProfileDir); */
    pub init: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const nsIDOMDocument, aCustomProfileDir: *const nsIFile) -> nsresult,

    /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal); */
    pub initPartial: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aClientID: *const nsACString, aDocumentURI: *const nsIURI, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    pub initForUpdateCheck: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> nsresult,

    /* void addDynamicURI (in nsIURI aURI); */
    pub addDynamicURI: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aURI: *const nsIURI) -> nsresult,

    /* void schedule (); */
    pub schedule: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate) -> nsresult,

    /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aObserver: *const nsIOfflineCacheUpdateObserver, aHoldWeak: bool) -> nsresult,

    /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aObserver: *const nsIOfflineCacheUpdateObserver) -> nsresult,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate) -> nsresult,

    /* readonly attribute uint64_t byteProgress; */
    pub get_byteProgress: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdate, aByteProgress: *mut uint64_t) -> nsresult,

}


impl nsIOfflineCacheUpdate {
    /* readonly attribute unsigned short status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean partial; */
    #[inline]
    pub unsafe fn get_partial(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_partial)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isUpgrade; */
    #[inline]
    pub unsafe fn get_isUpgrade(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isUpgrade)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString updateDomain; */
    #[inline]
    pub unsafe fn get_updateDomain(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_updateDomain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI manifestURI; */
    #[inline]
    pub unsafe fn get_manifestURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_manifestURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean succeeded; */
    #[inline]
    pub unsafe fn get_succeeded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_succeeded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void init (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument, [optional] in nsIFile aCustomProfileDir); */
    #[inline]
    pub unsafe fn init(&self, aManifestURI: Option<&nsIURI>, aDocumentURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aDocument: Option<&nsIDOMDocument>, aCustomProfileDir: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aDocument.map_or(::std::ptr::null(), |x| x as *const _), aCustomProfileDir.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initPartial (in nsIURI aManifestURI, in ACString aClientID, in nsIURI aDocumentURI, in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn initPartial(&self, aManifestURI: Option<&nsIURI>, aClientID: &[u8], aDocumentURI: Option<&nsIURI>, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {
        let aClientID = nsCString::from(aClientID);
        match ((*self.vtable).initPartial)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), &*aClientID, aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initForUpdateCheck (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn initForUpdateCheck(&self, aManifestURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).initForUpdateCheck)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addDynamicURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn addDynamicURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).addDynamicURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void schedule (); */
    #[inline]
    pub unsafe fn schedule(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).schedule)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserver (in nsIOfflineCacheUpdateObserver aObserver, [optional] in boolean aHoldWeak); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsIOfflineCacheUpdateObserver>, aHoldWeak: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aHoldWeak) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIOfflineCacheUpdateObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsIOfflineCacheUpdateObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint64_t byteProgress; */
    #[inline]
    pub unsafe fn get_byteProgress(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_byteProgress)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIOfflineCacheUpdateService_consts {
    pub const ALLOW_NO_WARN: i64 = 3;
}


#[repr(C)]
pub struct nsIOfflineCacheUpdateService {
    vtable: *const nsIOfflineCacheUpdateServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOfflineCacheUpdateService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44971e74, 0x37e4, 0x4140,
            [0x86, 0x77, 0xa4, 0xcf, 0x21, 0x3a, 0x3f, 0x4b])
    }
}

unsafe impl RefCounted for nsIOfflineCacheUpdateService {
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
pub trait nsIOfflineCacheUpdateServiceCoerce {
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self;
}

impl nsIOfflineCacheUpdateServiceCoerce for nsIOfflineCacheUpdateService {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self {
        v
    }
}

impl nsIOfflineCacheUpdateService {
    #[inline]
    pub fn coerce<T: nsIOfflineCacheUpdateServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOfflineCacheUpdateService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOfflineCacheUpdateServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOfflineCacheUpdateService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOfflineCacheUpdateServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numUpdates; */
    pub get_numUpdates: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aNumUpdates: *mut libc::uint32_t) -> nsresult,

    /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
    pub getUpdate: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, index: libc::uint32_t, _retval: *mut *const nsIOfflineCacheUpdate) -> nsresult,

    /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
    pub scheduleUpdate: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aWindow: *const mozIDOMWindow, _retval: *mut *const nsIOfflineCacheUpdate) -> nsresult,

    /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
    pub scheduleAppUpdate: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aProfileDir: *const nsIFile, _retval: *mut *const nsIOfflineCacheUpdate) -> nsresult,

    /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument); */
    pub scheduleOnDocumentStop: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aDocumentURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aDocument: *const nsIDOMDocument) -> nsresult,

    /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    pub checkForUpdate: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aManifestURI: *const nsIURI, aLoadingPrincipal: *const nsIPrincipal, aObserver: *const nsIObserver) -> nsresult,

    /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal, in nsIPrefBranch aPrefBranch); */
    pub offlineAppAllowed: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aPrincipal: *const nsIPrincipal, aPrefBranch: *const nsIPrefBranch, _retval: *mut bool) -> nsresult,

    /* boolean offlineAppAllowedForURI (in nsIURI aURI, in nsIPrefBranch aPrefBranch); */
    pub offlineAppAllowedForURI: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aURI: *const nsIURI, aPrefBranch: *const nsIPrefBranch, _retval: *mut bool) -> nsresult,

    /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
    pub allowOfflineApp: unsafe extern "C" fn (this: *const nsIOfflineCacheUpdateService, aPrincipal: *const nsIPrincipal) -> nsresult,

}


impl nsIOfflineCacheUpdateService {
    /* readonly attribute unsigned long numUpdates; */
    #[inline]
    pub unsafe fn get_numUpdates(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numUpdates)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIOfflineCacheUpdate getUpdate (in unsigned long index); */
    #[inline]
    pub unsafe fn getUpdate(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIOfflineCacheUpdate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUpdate)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIOfflineCacheUpdate scheduleUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in mozIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn scheduleUpdate(&self, aManifestURI: Option<&nsIURI>, aDocumentURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aWindow: Option<&mozIDOMWindow>) -> Result<Option<RefPtr<nsIOfflineCacheUpdate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).scheduleUpdate)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIOfflineCacheUpdate scheduleAppUpdate (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIFile aProfileDir); */
    #[inline]
    pub unsafe fn scheduleAppUpdate(&self, aManifestURI: Option<&nsIURI>, aDocumentURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aProfileDir: Option<&nsIFile>) -> Result<Option<RefPtr<nsIOfflineCacheUpdate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).scheduleAppUpdate)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aProfileDir.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void scheduleOnDocumentStop (in nsIURI aManifestURI, in nsIURI aDocumentURI, in nsIPrincipal aLoadingPrincipal, in nsIDOMDocument aDocument); */
    #[inline]
    pub unsafe fn scheduleOnDocumentStop(&self, aManifestURI: Option<&nsIURI>, aDocumentURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aDocument: Option<&nsIDOMDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).scheduleOnDocumentStop)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aDocumentURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void checkForUpdate (in nsIURI aManifestURI, in nsIPrincipal aLoadingPrincipal, in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn checkForUpdate(&self, aManifestURI: Option<&nsIURI>, aLoadingPrincipal: Option<&nsIPrincipal>, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).checkForUpdate)(self as *const _, aManifestURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean offlineAppAllowed (in nsIPrincipal aPrincipal, in nsIPrefBranch aPrefBranch); */
    #[inline]
    pub unsafe fn offlineAppAllowed(&self, aPrincipal: Option<&nsIPrincipal>, aPrefBranch: Option<&nsIPrefBranch>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).offlineAppAllowed)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aPrefBranch.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean offlineAppAllowedForURI (in nsIURI aURI, in nsIPrefBranch aPrefBranch); */
    #[inline]
    pub unsafe fn offlineAppAllowedForURI(&self, aURI: Option<&nsIURI>, aPrefBranch: Option<&nsIPrefBranch>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).offlineAppAllowedForURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aPrefBranch.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void allowOfflineApp (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn allowOfflineApp(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).allowOfflineApp)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


