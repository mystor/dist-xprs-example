//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadManager.idl
//


#[repr(C)]
pub struct nsIDownloadManagerResult {
    vtable: *const nsIDownloadManagerResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadManagerResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c07ffeb, 0x791b, 0x49f3,
            [0xae, 0x38, 0x2c, 0x33, 0x1f, 0xd5, 0x5a, 0x52])
    }
}

unsafe impl RefCounted for nsIDownloadManagerResult {
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
pub trait nsIDownloadManagerResultCoerce {
    fn coerce_from(v: &nsIDownloadManagerResult) -> &Self;
}

impl nsIDownloadManagerResultCoerce for nsIDownloadManagerResult {
    #[inline]
    fn coerce_from(v: &nsIDownloadManagerResult) -> &Self {
        v
    }
}

impl nsIDownloadManagerResult {
    #[inline]
    pub fn coerce<T: nsIDownloadManagerResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadManagerResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadManagerResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadManagerResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadManagerResultVTable {
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsresult aStatus, in nsIDownload aDownload); */
    pub handleResult: unsafe extern "C" fn (this: *const nsIDownloadManagerResult, aStatus: nsresult, aDownload: *const nsIDownload) -> nsresult,

}


impl nsIDownloadManagerResult {
    /* void handleResult (in nsresult aStatus, in nsIDownload aDownload); */
    #[inline]
    pub unsafe fn handleResult(&self, aStatus: nsresult, aDownload: Option<&nsIDownload>) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, aStatus, aDownload.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIDownloadManager_consts {
    pub const DOWNLOAD_TYPE_DOWNLOAD: i64 = 0;
    pub const DOWNLOAD_NOTSTARTED: i64 = -1;
    pub const DOWNLOAD_DOWNLOADING: i64 = 0;
    pub const DOWNLOAD_FINISHED: i64 = 1;
    pub const DOWNLOAD_FAILED: i64 = 2;
    pub const DOWNLOAD_CANCELED: i64 = 3;
    pub const DOWNLOAD_PAUSED: i64 = 4;
    pub const DOWNLOAD_QUEUED: i64 = 5;
    pub const DOWNLOAD_BLOCKED_PARENTAL: i64 = 6;
    pub const DOWNLOAD_SCANNING: i64 = 7;
    pub const DOWNLOAD_DIRTY: i64 = 8;
    pub const DOWNLOAD_BLOCKED_POLICY: i64 = 9;
}


#[repr(C)]
pub struct nsIDownloadManager {
    vtable: *const nsIDownloadManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb29aac15, 0x7ec4, 0x4ab3,
            [0xa5, 0x3b, 0x08, 0xf7, 0x8a, 0xed, 0x3b, 0x34])
    }
}

unsafe impl RefCounted for nsIDownloadManager {
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
pub trait nsIDownloadManagerCoerce {
    fn coerce_from(v: &nsIDownloadManager) -> &Self;
}

impl nsIDownloadManagerCoerce for nsIDownloadManager {
    #[inline]
    fn coerce_from(v: &nsIDownloadManager) -> &Self {
        v
    }
}

impl nsIDownloadManager {
    #[inline]
    pub fn coerce<T: nsIDownloadManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsIDownload addDownload (in short aDownloadType, in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime aStartTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
    pub addDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aDownloadType: libc::int16_t, aSource: *const nsIURI, aTarget: *const nsIURI, aDisplayName: *const nsAString, aMIMEInfo: *const nsIMIMEInfo, aStartTime: PRTime, aTempFile: *const nsIFile, aCancelable: *const nsICancelable, aIsPrivate: bool, _retval: *mut *const nsIDownload) -> nsresult,

    /* nsIDownload getDownload (in unsigned long aID); */
    pub getDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t, _retval: *mut *const nsIDownload) -> nsresult,

    /* void getDownloadByGUID (in ACString aGUID, in nsIDownloadManagerResult aCallback); */
    pub getDownloadByGUID: unsafe extern "C" fn (this: *const nsIDownloadManager, aGUID: *const nsACString, aCallback: *const nsIDownloadManagerResult) -> nsresult,

    /* void cancelDownload (in unsigned long aID); */
    pub cancelDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t) -> nsresult,

    /* void removeDownload (in unsigned long aID); */
    pub removeDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t) -> nsresult,

    /* void removeDownloadsByTimeframe (in long long aBeginTime, in long long aEndTime); */
    pub removeDownloadsByTimeframe: unsafe extern "C" fn (this: *const nsIDownloadManager, aBeginTime: libc::int64_t, aEndTime: libc::int64_t) -> nsresult,

    /* void pauseDownload (in unsigned long aID); */
    pub pauseDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t) -> nsresult,

    /* void resumeDownload (in unsigned long aID); */
    pub resumeDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t) -> nsresult,

    /* void retryDownload (in unsigned long aID); */
    pub retryDownload: unsafe extern "C" fn (this: *const nsIDownloadManager, aID: libc::uint32_t) -> nsresult,

    /* readonly attribute mozIStorageConnection DBConnection; */
    pub get_DBConnection: unsafe extern "C" fn (this: *const nsIDownloadManager, aDBConnection: *mut *const mozIStorageConnection) -> nsresult,

    /* readonly attribute mozIStorageConnection privateDBConnection; */
    pub get_privateDBConnection: unsafe extern "C" fn (this: *const nsIDownloadManager, aPrivateDBConnection: *mut *const mozIStorageConnection) -> nsresult,

    /* readonly attribute boolean canCleanUp; */
    pub get_canCleanUp: unsafe extern "C" fn (this: *const nsIDownloadManager, aCanCleanUp: *mut bool) -> nsresult,

    /* readonly attribute boolean canCleanUpPrivate; */
    pub get_canCleanUpPrivate: unsafe extern "C" fn (this: *const nsIDownloadManager, aCanCleanUpPrivate: *mut bool) -> nsresult,

    /* void cleanUp (); */
    pub cleanUp: unsafe extern "C" fn (this: *const nsIDownloadManager) -> nsresult,

    /* void cleanUpPrivate (); */
    pub cleanUpPrivate: unsafe extern "C" fn (this: *const nsIDownloadManager) -> nsresult,

    /* readonly attribute long activeDownloadCount; */
    pub get_activeDownloadCount: unsafe extern "C" fn (this: *const nsIDownloadManager, aActiveDownloadCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long activePrivateDownloadCount; */
    pub get_activePrivateDownloadCount: unsafe extern "C" fn (this: *const nsIDownloadManager, aActivePrivateDownloadCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsISimpleEnumerator activeDownloads; */
    pub get_activeDownloads: unsafe extern "C" fn (this: *const nsIDownloadManager, aActiveDownloads: *mut *const nsISimpleEnumerator) -> nsresult,

    /* readonly attribute nsISimpleEnumerator activePrivateDownloads; */
    pub get_activePrivateDownloads: unsafe extern "C" fn (this: *const nsIDownloadManager, aActivePrivateDownloads: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void addListener (in nsIDownloadProgressListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIDownloadManager, aListener: *const nsIDownloadProgressListener) -> nsresult,

    /* void addPrivacyAwareListener (in nsIDownloadProgressListener aListener); */
    pub addPrivacyAwareListener: unsafe extern "C" fn (this: *const nsIDownloadManager, aListener: *const nsIDownloadProgressListener) -> nsresult,

    /* void removeListener (in nsIDownloadProgressListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIDownloadManager, aListener: *const nsIDownloadProgressListener) -> nsresult,

    /* readonly attribute nsIFile defaultDownloadsDirectory; */
    pub get_defaultDownloadsDirectory: unsafe extern "C" fn (this: *const nsIDownloadManager, aDefaultDownloadsDirectory: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIFile userDownloadsDirectory; */
    pub get_userDownloadsDirectory: unsafe extern "C" fn (this: *const nsIDownloadManager, aUserDownloadsDirectory: *mut *const nsIFile) -> nsresult,

}


impl nsIDownloadManager {
    /* nsIDownload addDownload (in short aDownloadType, in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime aStartTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn addDownload(&self, aDownloadType: libc::int16_t, aSource: Option<&nsIURI>, aTarget: Option<&nsIURI>, aDisplayName: &[u16], aMIMEInfo: Option<&nsIMIMEInfo>, aStartTime: PRTime, aTempFile: Option<&nsIFile>, aCancelable: Option<&nsICancelable>, aIsPrivate: bool) -> Result<Option<RefPtr<nsIDownload>>, nsresult> {
        let aDisplayName = nsString::from(aDisplayName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).addDownload)(self as *const _, aDownloadType, aSource.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), &*aDisplayName, aMIMEInfo.map_or(::std::ptr::null(), |x| x as *const _), aStartTime, aTempFile.map_or(::std::ptr::null(), |x| x as *const _), aCancelable.map_or(::std::ptr::null(), |x| x as *const _), aIsPrivate, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDownload getDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn getDownload(&self, aID: libc::uint32_t) -> Result<Option<RefPtr<nsIDownload>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDownload)(self as *const _, aID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getDownloadByGUID (in ACString aGUID, in nsIDownloadManagerResult aCallback); */
    #[inline]
    pub unsafe fn getDownloadByGUID(&self, aGUID: &[u8], aCallback: Option<&nsIDownloadManagerResult>) -> Result<(), nsresult> {
        let aGUID = nsCString::from(aGUID);
        match ((*self.vtable).getDownloadByGUID)(self as *const _, &*aGUID, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancelDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn cancelDownload(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).cancelDownload)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn removeDownload(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeDownload)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDownloadsByTimeframe (in long long aBeginTime, in long long aEndTime); */
    #[inline]
    pub unsafe fn removeDownloadsByTimeframe(&self, aBeginTime: libc::int64_t, aEndTime: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeDownloadsByTimeframe)(self as *const _, aBeginTime, aEndTime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pauseDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn pauseDownload(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).pauseDownload)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumeDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn resumeDownload(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).resumeDownload)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void retryDownload (in unsigned long aID); */
    #[inline]
    pub unsafe fn retryDownload(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).retryDownload)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute mozIStorageConnection DBConnection; */
    #[inline]
    pub unsafe fn get_DBConnection(&self, ) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DBConnection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIStorageConnection privateDBConnection; */
    #[inline]
    pub unsafe fn get_privateDBConnection(&self, ) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_privateDBConnection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean canCleanUp; */
    #[inline]
    pub unsafe fn get_canCleanUp(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canCleanUp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canCleanUpPrivate; */
    #[inline]
    pub unsafe fn get_canCleanUpPrivate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canCleanUpPrivate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void cleanUp (); */
    #[inline]
    pub unsafe fn cleanUp(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cleanUp)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cleanUpPrivate (); */
    #[inline]
    pub unsafe fn cleanUpPrivate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cleanUpPrivate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long activeDownloadCount; */
    #[inline]
    pub unsafe fn get_activeDownloadCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_activeDownloadCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long activePrivateDownloadCount; */
    #[inline]
    pub unsafe fn get_activePrivateDownloadCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_activePrivateDownloadCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsISimpleEnumerator activeDownloads; */
    #[inline]
    pub unsafe fn get_activeDownloads(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeDownloads)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISimpleEnumerator activePrivateDownloads; */
    #[inline]
    pub unsafe fn get_activePrivateDownloads(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activePrivateDownloads)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addListener (in nsIDownloadProgressListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aListener: Option<&nsIDownloadProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addPrivacyAwareListener (in nsIDownloadProgressListener aListener); */
    #[inline]
    pub unsafe fn addPrivacyAwareListener(&self, aListener: Option<&nsIDownloadProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addPrivacyAwareListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIDownloadProgressListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aListener: Option<&nsIDownloadProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile defaultDownloadsDirectory; */
    #[inline]
    pub unsafe fn get_defaultDownloadsDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultDownloadsDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile userDownloadsDirectory; */
    #[inline]
    pub unsafe fn get_userDownloadsDirectory(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_userDownloadsDirectory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


