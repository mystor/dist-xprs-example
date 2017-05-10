//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheEntry.idl
//


pub mod nsICacheEntry_consts {
    pub const NO_EXPIRATION_TIME: i64 = 4294967295;
}


#[repr(C)]
pub struct nsICacheEntry {
    vtable: *const nsICacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x607c2a2c, 0x0a48, 0x40b9,
            [0xa9, 0x56, 0x8c, 0xf2, 0xbb, 0x98, 0x57, 0xcf])
    }
}

unsafe impl RefCounted for nsICacheEntry {
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
pub trait nsICacheEntryCoerce {
    fn coerce_from(v: &nsICacheEntry) -> &Self;
}

impl nsICacheEntryCoerce for nsICacheEntry {
    #[inline]
    fn coerce_from(v: &nsICacheEntry) -> &Self {
        v
    }
}

impl nsICacheEntry {
    #[inline]
    pub fn coerce<T: nsICacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString key; */
    pub get_key: unsafe extern "C" fn (this: *const nsICacheEntry, aKey: *mut nsACString) -> nsresult,

    /* readonly attribute boolean persistent; */
    pub get_persistent: unsafe extern "C" fn (this: *const nsICacheEntry, aPersistent: *mut bool) -> nsresult,

    /* readonly attribute long fetchCount; */
    pub get_fetchCount: unsafe extern "C" fn (this: *const nsICacheEntry, aFetchCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute uint32_t lastFetched; */
    pub get_lastFetched: unsafe extern "C" fn (this: *const nsICacheEntry, aLastFetched: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsICacheEntry, aLastModified: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t expirationTime; */
    pub get_expirationTime: unsafe extern "C" fn (this: *const nsICacheEntry, aExpirationTime: *mut uint32_t) -> nsresult,

    /* void setExpirationTime (in uint32_t expirationTime); */
    pub setExpirationTime: unsafe extern "C" fn (this: *const nsICacheEntry, expirationTime: uint32_t) -> nsresult,

    /* readonly attribute uint64_t onStartTime; */
    pub get_onStartTime: unsafe extern "C" fn (this: *const nsICacheEntry, aOnStartTime: *mut uint64_t) -> nsresult,

    /* readonly attribute uint64_t onStopTime; */
    pub get_onStopTime: unsafe extern "C" fn (this: *const nsICacheEntry, aOnStopTime: *mut uint64_t) -> nsresult,

    /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
    pub setNetworkTimes: unsafe extern "C" fn (this: *const nsICacheEntry, onStartTime: uint64_t, onStopTime: uint64_t) -> nsresult,

    /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
    pub forceValidFor: unsafe extern "C" fn (this: *const nsICacheEntry, aSecondsToTheFuture: libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isForcedValid; */
    pub get_isForcedValid: unsafe extern "C" fn (this: *const nsICacheEntry, aIsForcedValid: *mut bool) -> nsresult,

    /* nsIInputStream openInputStream (in long long offset); */
    pub openInputStream: unsafe extern "C" fn (this: *const nsICacheEntry, offset: libc::int64_t, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIOutputStream openOutputStream (in long long offset); */
    pub openOutputStream: unsafe extern "C" fn (this: *const nsICacheEntry, offset: libc::int64_t, _retval: *mut *const nsIOutputStream) -> nsresult,

    /* attribute int64_t predictedDataSize; */
    pub get_predictedDataSize: unsafe extern "C" fn (this: *const nsICacheEntry, aPredictedDataSize: *mut int64_t) -> nsresult,
    pub set_predictedDataSize: unsafe extern "C" fn (this: *const nsICacheEntry, aPredictedDataSize: int64_t) -> nsresult,

    /* attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsICacheEntry, aSecurityInfo: *mut *const nsISupports) -> nsresult,
    pub set_securityInfo: unsafe extern "C" fn (this: *const nsICacheEntry, aSecurityInfo: *const nsISupports) -> nsresult,

    /* readonly attribute unsigned long storageDataSize; */
    pub get_storageDataSize: unsafe extern "C" fn (this: *const nsICacheEntry, aStorageDataSize: *mut libc::uint32_t) -> nsresult,

    /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
    pub asyncDoom: unsafe extern "C" fn (this: *const nsICacheEntry, listener: *const nsICacheEntryDoomCallback) -> nsresult,

    /* string getMetaDataElement (in string key); */
    pub getMetaDataElement: unsafe extern "C" fn (this: *const nsICacheEntry, key: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* void setMetaDataElement (in string key, in string value); */
    pub setMetaDataElement: unsafe extern "C" fn (this: *const nsICacheEntry, key: *const libc::c_char, value: *const libc::c_char) -> nsresult,

    /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
    pub visitMetaData: unsafe extern "C" fn (this: *const nsICacheEntry, visitor: *const nsICacheEntryMetaDataVisitor) -> nsresult,

    /* void metaDataReady (); */
    pub metaDataReady: unsafe extern "C" fn (this: *const nsICacheEntry) -> nsresult,

    /* void setValid (); */
    pub setValid: unsafe extern "C" fn (this: *const nsICacheEntry) -> nsresult,

    /* readonly attribute uint32_t diskStorageSizeInKB; */
    pub get_diskStorageSizeInKB: unsafe extern "C" fn (this: *const nsICacheEntry, aDiskStorageSizeInKB: *mut uint32_t) -> nsresult,

    /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
    pub recreate: unsafe extern "C" fn (this: *const nsICacheEntry, aMemoryOnly: bool, _retval: *mut *const nsICacheEntry) -> nsresult,

    /* readonly attribute long long dataSize; */
    pub get_dataSize: unsafe extern "C" fn (this: *const nsICacheEntry, aDataSize: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long altDataSize; */
    pub get_altDataSize: unsafe extern "C" fn (this: *const nsICacheEntry, aAltDataSize: *mut libc::int64_t) -> nsresult,

    /* nsIOutputStream openAlternativeOutputStream (in ACString type); */
    pub openAlternativeOutputStream: unsafe extern "C" fn (this: *const nsICacheEntry, type_: *const nsACString, _retval: *mut *const nsIOutputStream) -> nsresult,

    /* nsIInputStream openAlternativeInputStream (in ACString type); */
    pub openAlternativeInputStream: unsafe extern "C" fn (this: *const nsICacheEntry, type_: *const nsACString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* readonly attribute nsILoadContextInfo loadContextInfo; */
    pub get_loadContextInfo: unsafe extern "C" fn (this: *const nsICacheEntry, aLoadContextInfo: *mut *const nsILoadContextInfo) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsICacheEntry) -> nsresult,

    /* void markValid (); */
    pub markValid: unsafe extern "C" fn (this: *const nsICacheEntry) -> nsresult,

    /* void maybeMarkValid (); */
    pub maybeMarkValid: unsafe extern "C" fn (this: *const nsICacheEntry) -> nsresult,

    /* boolean hasWriteAccess (in boolean aWriteAllowed); */
    pub hasWriteAccess: unsafe extern "C" fn (this: *const nsICacheEntry, aWriteAllowed: bool, _retval: *mut bool) -> nsresult,

}


impl nsICacheEntry {
    /* readonly attribute ACString key; */
    #[inline]
    pub unsafe fn get_key(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_key)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean persistent; */
    #[inline]
    pub unsafe fn get_persistent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_persistent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long fetchCount; */
    #[inline]
    pub unsafe fn get_fetchCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fetchCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lastFetched; */
    #[inline]
    pub unsafe fn get_lastFetched(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastFetched)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t expirationTime; */
    #[inline]
    pub unsafe fn get_expirationTime(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expirationTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setExpirationTime (in uint32_t expirationTime); */
    #[inline]
    pub unsafe fn setExpirationTime(&self, expirationTime: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setExpirationTime)(self as *const _, expirationTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint64_t onStartTime; */
    #[inline]
    pub unsafe fn get_onStartTime(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_onStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint64_t onStopTime; */
    #[inline]
    pub unsafe fn get_onStopTime(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_onStopTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setNetworkTimes (in uint64_t onStartTime, in uint64_t onStopTime); */
    #[inline]
    pub unsafe fn setNetworkTimes(&self, onStartTime: uint64_t, onStopTime: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).setNetworkTimes)(self as *const _, onStartTime, onStopTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceValidFor (in unsigned long aSecondsToTheFuture); */
    #[inline]
    pub unsafe fn forceValidFor(&self, aSecondsToTheFuture: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).forceValidFor)(self as *const _, aSecondsToTheFuture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isForcedValid; */
    #[inline]
    pub unsafe fn get_isForcedValid(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isForcedValid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIInputStream openInputStream (in long long offset); */
    #[inline]
    pub unsafe fn openInputStream(&self, offset: libc::int64_t) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openInputStream)(self as *const _, offset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIOutputStream openOutputStream (in long long offset); */
    #[inline]
    pub unsafe fn openOutputStream(&self, offset: libc::int64_t) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openOutputStream)(self as *const _, offset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute int64_t predictedDataSize; */
    #[inline]
    pub unsafe fn get_predictedDataSize(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_predictedDataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_predictedDataSize(&self, aPredictedDataSize: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_predictedDataSize)(self as *const _, aPredictedDataSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_securityInfo(&self, aSecurityInfo: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_securityInfo)(self as *const _, aSecurityInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long storageDataSize; */
    #[inline]
    pub unsafe fn get_storageDataSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_storageDataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void asyncDoom (in nsICacheEntryDoomCallback listener); */
    #[inline]
    pub unsafe fn asyncDoom(&self, listener: Option<&nsICacheEntryDoomCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncDoom)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string getMetaDataElement (in string key); */
    #[inline]
    pub unsafe fn getMetaDataElement(&self, key: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getMetaDataElement)(self as *const _, key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setMetaDataElement (in string key, in string value); */
    #[inline]
    pub unsafe fn setMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setMetaDataElement)(self as *const _, key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void visitMetaData (in nsICacheEntryMetaDataVisitor visitor); */
    #[inline]
    pub unsafe fn visitMetaData(&self, visitor: Option<&nsICacheEntryMetaDataVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitMetaData)(self as *const _, visitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void metaDataReady (); */
    #[inline]
    pub unsafe fn metaDataReady(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).metaDataReady)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setValid (); */
    #[inline]
    pub unsafe fn setValid(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setValid)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute uint32_t diskStorageSizeInKB; */
    #[inline]
    pub unsafe fn get_diskStorageSizeInKB(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_diskStorageSizeInKB)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsICacheEntry recreate ([optional] in boolean aMemoryOnly); */
    #[inline]
    pub unsafe fn recreate(&self, aMemoryOnly: bool) -> Result<Option<RefPtr<nsICacheEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).recreate)(self as *const _, aMemoryOnly, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long long dataSize; */
    #[inline]
    pub unsafe fn get_dataSize(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dataSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long altDataSize; */
    #[inline]
    pub unsafe fn get_altDataSize(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_altDataSize)(self as *const _, &mut _retval as *mut _) {
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

    /* nsIInputStream openAlternativeInputStream (in ACString type); */
    #[inline]
    pub unsafe fn openAlternativeInputStream(&self, type_: &[u8]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let type_ = nsCString::from(type_);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openAlternativeInputStream)(self as *const _, &*type_, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadContextInfo loadContextInfo; */
    #[inline]
    pub unsafe fn get_loadContextInfo(&self, ) -> Result<Option<RefPtr<nsILoadContextInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadContextInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void markValid (); */
    #[inline]
    pub unsafe fn markValid(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).markValid)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void maybeMarkValid (); */
    #[inline]
    pub unsafe fn maybeMarkValid(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).maybeMarkValid)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasWriteAccess (in boolean aWriteAllowed); */
    #[inline]
    pub unsafe fn hasWriteAccess(&self, aWriteAllowed: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasWriteAccess)(self as *const _, aWriteAllowed, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsICacheEntryMetaDataVisitor {
    vtable: *const nsICacheEntryMetaDataVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheEntryMetaDataVisitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfea3e276, 0x6ba5, 0x4ceb,
            [0xa5, 0x81, 0x80, 0x7d, 0x1f, 0x43, 0xf6, 0xd0])
    }
}

unsafe impl RefCounted for nsICacheEntryMetaDataVisitor {
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
pub trait nsICacheEntryMetaDataVisitorCoerce {
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self;
}

impl nsICacheEntryMetaDataVisitorCoerce for nsICacheEntryMetaDataVisitor {
    #[inline]
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self {
        v
    }
}

impl nsICacheEntryMetaDataVisitor {
    #[inline]
    pub fn coerce<T: nsICacheEntryMetaDataVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheEntryMetaDataVisitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheEntryMetaDataVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheEntryMetaDataVisitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheEntryMetaDataVisitorVTable {
    pub __base: nsISupportsVTable,

    /* void onMetaDataElement (in string key, in string value); */
    pub onMetaDataElement: unsafe extern "C" fn (this: *const nsICacheEntryMetaDataVisitor, key: *const libc::c_char, value: *const libc::c_char) -> nsresult,

}


impl nsICacheEntryMetaDataVisitor {
    /* void onMetaDataElement (in string key, in string value); */
    #[inline]
    pub unsafe fn onMetaDataElement(&self, key: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).onMetaDataElement)(self as *const _, key, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


