//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemoryReporter.idl
//


#[repr(C)]
pub struct nsIMemoryReporterCallback {
    vtable: *const nsIMemoryReporterCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemoryReporterCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62ef0e1c, 0xdbd6, 0x11e3,
            [0xaa, 0x75, 0x3c, 0x97, 0x0e, 0x9f, 0x42, 0x38])
    }
}

unsafe impl RefCounted for nsIMemoryReporterCallback {
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
pub trait nsIMemoryReporterCallbackCoerce {
    fn coerce_from(v: &nsIMemoryReporterCallback) -> &Self;
}

impl nsIMemoryReporterCallbackCoerce for nsIMemoryReporterCallback {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterCallback) -> &Self {
        v
    }
}

impl nsIMemoryReporterCallback {
    #[inline]
    pub fn coerce<T: nsIMemoryReporterCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemoryReporterCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryReporterCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryReporterCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data); */
    pub callback: unsafe extern "C" fn (this: *const nsIMemoryReporterCallback, process: *const nsACString, path: *const nsACString, kind: int32_t, units: int32_t, amount: int64_t, description: *const nsACString, data: *const nsISupports) -> nsresult,

}


impl nsIMemoryReporterCallback {
    /* void callback (in ACString process, in AUTF8String path, in int32_t kind, in int32_t units, in int64_t amount, in AUTF8String description, in nsISupports data); */
    #[inline]
    pub unsafe fn callback(&self, process: &[u8], path: &[u8], kind: int32_t, units: int32_t, amount: int64_t, description: &[u8], data: Option<&nsISupports>) -> Result<(), nsresult> {
        let process = nsCString::from(process);
        let path = nsCString::from(path);
        let description = nsCString::from(description);
        match ((*self.vtable).callback)(self as *const _, &*process, &*path, kind, units, amount, &*description, data.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIMemoryReporter_consts {
    pub const KIND_NONHEAP: i64 = 0;
    pub const KIND_HEAP: i64 = 1;
    pub const KIND_OTHER: i64 = 2;
    pub const UNITS_BYTES: i64 = 0;
    pub const UNITS_COUNT: i64 = 1;
    pub const UNITS_COUNT_CUMULATIVE: i64 = 2;
    pub const UNITS_PERCENTAGE: i64 = 3;
}


#[repr(C)]
pub struct nsIMemoryReporter {
    vtable: *const nsIMemoryReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemoryReporter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x92a36db1, 0x46bd, 0x4fe6,
            [0x98, 0x8e, 0x47, 0xdb, 0x47, 0x23, 0x6d, 0x8b])
    }
}

unsafe impl RefCounted for nsIMemoryReporter {
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
pub trait nsIMemoryReporterCoerce {
    fn coerce_from(v: &nsIMemoryReporter) -> &Self;
}

impl nsIMemoryReporterCoerce for nsIMemoryReporter {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporter) -> &Self {
        v
    }
}

impl nsIMemoryReporter {
    #[inline]
    pub fn coerce<T: nsIMemoryReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemoryReporter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryReporterVTable {
    pub __base: nsISupportsVTable,

    /* void collectReports (in nsIMemoryReporterCallback callback, in nsISupports data, in boolean anonymize); */
    pub collectReports: unsafe extern "C" fn (this: *const nsIMemoryReporter, callback: *const nsIMemoryReporterCallback, data: *const nsISupports, anonymize: bool) -> nsresult,

}


impl nsIMemoryReporter {
    /* void collectReports (in nsIMemoryReporterCallback callback, in nsISupports data, in boolean anonymize); */
    #[inline]
    pub unsafe fn collectReports(&self, callback: Option<&nsIMemoryReporterCallback>, data: Option<&nsISupports>, anonymize: bool) -> Result<(), nsresult> {

        match ((*self.vtable).collectReports)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _), data.map_or(::std::ptr::null(), |x| x as *const _), anonymize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIFinishReportingCallback {
    vtable: *const nsIFinishReportingCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFinishReportingCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x548b3909, 0xc04d, 0x4ca6,
            [0x84, 0x66, 0xb8, 0xbe, 0xe3, 0x83, 0x74, 0x57])
    }
}

unsafe impl RefCounted for nsIFinishReportingCallback {
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
pub trait nsIFinishReportingCallbackCoerce {
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self;
}

impl nsIFinishReportingCallbackCoerce for nsIFinishReportingCallback {
    #[inline]
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self {
        v
    }
}

impl nsIFinishReportingCallback {
    #[inline]
    pub fn coerce<T: nsIFinishReportingCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFinishReportingCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFinishReportingCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFinishReportingCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFinishReportingCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in nsISupports data); */
    pub callback: unsafe extern "C" fn (this: *const nsIFinishReportingCallback, data: *const nsISupports) -> nsresult,

}


impl nsIFinishReportingCallback {
    /* void callback (in nsISupports data); */
    #[inline]
    pub unsafe fn callback(&self, data: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).callback)(self as *const _, data.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIMemoryReporterManager {
    vtable: *const nsIMemoryReporterManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemoryReporterManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2998574d, 0x8993, 0x407a,
            [0xb1, 0xa5, 0x8a, 0xd7, 0x41, 0x76, 0x53, 0xe1])
    }
}

unsafe impl RefCounted for nsIMemoryReporterManager {
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
pub trait nsIMemoryReporterManagerCoerce {
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self;
}

impl nsIMemoryReporterManagerCoerce for nsIMemoryReporterManager {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self {
        v
    }
}

impl nsIMemoryReporterManager {
    #[inline]
    pub fn coerce<T: nsIMemoryReporterManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemoryReporterManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryReporterManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryReporterManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryReporterManagerVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void init (); */
    pub init: unsafe extern "C" fn (this: *const nsIMemoryReporterManager) -> nsresult,

    /* void registerStrongReporter (in nsIMemoryReporter reporter); */
    pub registerStrongReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void registerStrongAsyncReporter (in nsIMemoryReporter reporter); */
    pub registerStrongAsyncReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void registerWeakReporter (in nsIMemoryReporter reporter); */
    pub registerWeakReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void registerWeakAsyncReporter (in nsIMemoryReporter reporter); */
    pub registerWeakAsyncReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void unregisterStrongReporter (in nsIMemoryReporter reporter); */
    pub unregisterStrongReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void unregisterWeakReporter (in nsIMemoryReporter reporter); */
    pub unregisterWeakReporter: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, reporter: *const nsIMemoryReporter) -> nsresult,

    /* void blockRegistrationAndHideExistingReporters (); */
    pub blockRegistrationAndHideExistingReporters: unsafe extern "C" fn (this: *const nsIMemoryReporterManager) -> nsresult,

    /* void unblockRegistrationAndRestoreOriginalReporters (); */
    pub unblockRegistrationAndRestoreOriginalReporters: unsafe extern "C" fn (this: *const nsIMemoryReporterManager) -> nsresult,

    /* void registerStrongReporterEvenIfBlocked (in nsIMemoryReporter aReporter); */
    pub registerStrongReporterEvenIfBlocked: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aReporter: *const nsIMemoryReporter) -> nsresult,

    /* void getReports (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize); */
    pub getReports: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, handleReport: *const nsIMemoryReporterCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool) -> nsresult,

    /* [noscript] void getReportsExtended (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize, in boolean minimizeMemoryUsage, in AString DMDDumpIdent); */
    pub getReportsExtended: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, handleReport: *const nsIMemoryReporterCallback, handleReportData: *const nsISupports, finishReporting: *const nsIFinishReportingCallback, finishReportingData: *const nsISupports, anonymize: bool, minimizeMemoryUsage: bool, DMDDumpIdent: *const nsAString) -> nsresult,

    /* [noscript] void getReportsForThisProcessExtended (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in boolean anonymize, in FILE DMDFile, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData); */
    /// Unable to call function as its signature contains a non-rust type
    pub getReportsForThisProcessExtended: *const ::libc::c_void,

    /* [noscript] void endReport (); */
    pub endReport: unsafe extern "C" fn (this: *const nsIMemoryReporterManager) -> nsresult,

    /* [must_use] readonly attribute int64_t vsize; */
    pub get_vsize: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aVsize: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t vsizeMaxContiguous; */
    pub get_vsizeMaxContiguous: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aVsizeMaxContiguous: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t resident; */
    pub get_resident: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aResident: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t residentFast; */
    pub get_residentFast: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aResidentFast: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t residentPeak; */
    pub get_residentPeak: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aResidentPeak: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t residentUnique; */
    pub get_residentUnique: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aResidentUnique: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t heapAllocated; */
    pub get_heapAllocated: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aHeapAllocated: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t heapOverheadFraction; */
    pub get_heapOverheadFraction: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aHeapOverheadFraction: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeGCHeap; */
    pub get_JSMainRuntimeGCHeap: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeGCHeap: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeTemporaryPeak; */
    pub get_JSMainRuntimeTemporaryPeak: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeTemporaryPeak: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsSystem; */
    pub get_JSMainRuntimeCompartmentsSystem: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeCompartmentsSystem: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsUser; */
    pub get_JSMainRuntimeCompartmentsUser: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aJSMainRuntimeCompartmentsUser: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t imagesContentUsedUncompressed; */
    pub get_imagesContentUsedUncompressed: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aImagesContentUsedUncompressed: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t storageSQLite; */
    pub get_storageSQLite: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aStorageSQLite: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t lowMemoryEventsVirtual; */
    pub get_lowMemoryEventsVirtual: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aLowMemoryEventsVirtual: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t lowMemoryEventsPhysical; */
    pub get_lowMemoryEventsPhysical: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aLowMemoryEventsPhysical: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t ghostWindows; */
    pub get_ghostWindows: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aGhostWindows: *mut int64_t) -> nsresult,

    /* [must_use] readonly attribute int64_t pageFaultsHard; */
    pub get_pageFaultsHard: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aPageFaultsHard: *mut int64_t) -> nsresult,

    /* [infallible] readonly attribute boolean hasMozMallocUsableSize; */
    pub get_hasMozMallocUsableSize: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aHasMozMallocUsableSize: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isDMDEnabled; */
    pub get_isDMDEnabled: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aIsDMDEnabled: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isDMDRunning; */
    pub get_isDMDRunning: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, aIsDMDRunning: *mut bool) -> nsresult,

    /* [must_use] void minimizeMemoryUsage (in nsIRunnable callback); */
    pub minimizeMemoryUsage: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, callback: *const nsIRunnable) -> nsresult,

    /* [must_use] void sizeOfTab (in mozIDOMWindowProxy window, out int64_t jsObjectsSize, out int64_t jsStringsSize, out int64_t jsOtherSize, out int64_t domSize, out int64_t styleSize, out int64_t otherSize, out int64_t totalSize, out double jsMilliseconds, out double nonJSMilliseconds); */
    pub sizeOfTab: unsafe extern "C" fn (this: *const nsIMemoryReporterManager, window: *const mozIDOMWindowProxy, jsObjectsSize: *mut int64_t, jsStringsSize: *mut int64_t, jsOtherSize: *mut int64_t, domSize: *mut int64_t, styleSize: *mut int64_t, otherSize: *mut int64_t, totalSize: *mut int64_t, jsMilliseconds: *mut libc::c_double, nonJSMilliseconds: *mut libc::c_double) -> nsresult,

}


impl nsIMemoryReporterManager {
    /* [must_use] void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerStrongReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn registerStrongReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerStrongReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerStrongAsyncReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn registerStrongAsyncReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerStrongAsyncReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerWeakReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn registerWeakReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerWeakReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerWeakAsyncReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn registerWeakAsyncReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerWeakAsyncReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterStrongReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn unregisterStrongReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterStrongReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterWeakReporter (in nsIMemoryReporter reporter); */
    #[inline]
    pub unsafe fn unregisterWeakReporter(&self, reporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterWeakReporter)(self as *const _, reporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void blockRegistrationAndHideExistingReporters (); */
    #[inline]
    pub unsafe fn blockRegistrationAndHideExistingReporters(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).blockRegistrationAndHideExistingReporters)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unblockRegistrationAndRestoreOriginalReporters (); */
    #[inline]
    pub unsafe fn unblockRegistrationAndRestoreOriginalReporters(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unblockRegistrationAndRestoreOriginalReporters)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerStrongReporterEvenIfBlocked (in nsIMemoryReporter aReporter); */
    #[inline]
    pub unsafe fn registerStrongReporterEvenIfBlocked(&self, aReporter: Option<&nsIMemoryReporter>) -> Result<(), nsresult> {

        match ((*self.vtable).registerStrongReporterEvenIfBlocked)(self as *const _, aReporter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getReports (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize); */
    #[inline]
    pub unsafe fn getReports(&self, handleReport: Option<&nsIMemoryReporterCallback>, handleReportData: Option<&nsISupports>, finishReporting: Option<&nsIFinishReportingCallback>, finishReportingData: Option<&nsISupports>, anonymize: bool) -> Result<(), nsresult> {

        match ((*self.vtable).getReports)(self as *const _, handleReport.map_or(::std::ptr::null(), |x| x as *const _), handleReportData.map_or(::std::ptr::null(), |x| x as *const _), finishReporting.map_or(::std::ptr::null(), |x| x as *const _), finishReportingData.map_or(::std::ptr::null(), |x| x as *const _), anonymize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getReportsExtended (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData, in boolean anonymize, in boolean minimizeMemoryUsage, in AString DMDDumpIdent); */
    #[inline]
    pub unsafe fn getReportsExtended(&self, handleReport: Option<&nsIMemoryReporterCallback>, handleReportData: Option<&nsISupports>, finishReporting: Option<&nsIFinishReportingCallback>, finishReportingData: Option<&nsISupports>, anonymize: bool, minimizeMemoryUsage: bool, DMDDumpIdent: &[u16]) -> Result<(), nsresult> {
        let DMDDumpIdent = nsString::from(DMDDumpIdent);
        match ((*self.vtable).getReportsExtended)(self as *const _, handleReport.map_or(::std::ptr::null(), |x| x as *const _), handleReportData.map_or(::std::ptr::null(), |x| x as *const _), finishReporting.map_or(::std::ptr::null(), |x| x as *const _), finishReportingData.map_or(::std::ptr::null(), |x| x as *const _), anonymize, minimizeMemoryUsage, &*DMDDumpIdent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getReportsForThisProcessExtended (in nsIMemoryReporterCallback handleReport, in nsISupports handleReportData, in boolean anonymize, in FILE DMDFile, in nsIFinishReportingCallback finishReporting, in nsISupports finishReportingData); */


    /* [noscript] void endReport (); */
    #[inline]
    pub unsafe fn endReport(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endReport)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute int64_t vsize; */
    #[inline]
    pub unsafe fn get_vsize(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_vsize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t vsizeMaxContiguous; */
    #[inline]
    pub unsafe fn get_vsizeMaxContiguous(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_vsizeMaxContiguous)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t resident; */
    #[inline]
    pub unsafe fn get_resident(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_resident)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t residentFast; */
    #[inline]
    pub unsafe fn get_residentFast(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_residentFast)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t residentPeak; */
    #[inline]
    pub unsafe fn get_residentPeak(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_residentPeak)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t residentUnique; */
    #[inline]
    pub unsafe fn get_residentUnique(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_residentUnique)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t heapAllocated; */
    #[inline]
    pub unsafe fn get_heapAllocated(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_heapAllocated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t heapOverheadFraction; */
    #[inline]
    pub unsafe fn get_heapOverheadFraction(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_heapOverheadFraction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t JSMainRuntimeGCHeap; */
    #[inline]
    pub unsafe fn get_JSMainRuntimeGCHeap(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_JSMainRuntimeGCHeap)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t JSMainRuntimeTemporaryPeak; */
    #[inline]
    pub unsafe fn get_JSMainRuntimeTemporaryPeak(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_JSMainRuntimeTemporaryPeak)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsSystem; */
    #[inline]
    pub unsafe fn get_JSMainRuntimeCompartmentsSystem(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_JSMainRuntimeCompartmentsSystem)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t JSMainRuntimeCompartmentsUser; */
    #[inline]
    pub unsafe fn get_JSMainRuntimeCompartmentsUser(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_JSMainRuntimeCompartmentsUser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t imagesContentUsedUncompressed; */
    #[inline]
    pub unsafe fn get_imagesContentUsedUncompressed(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_imagesContentUsedUncompressed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t storageSQLite; */
    #[inline]
    pub unsafe fn get_storageSQLite(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_storageSQLite)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t lowMemoryEventsVirtual; */
    #[inline]
    pub unsafe fn get_lowMemoryEventsVirtual(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lowMemoryEventsVirtual)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t lowMemoryEventsPhysical; */
    #[inline]
    pub unsafe fn get_lowMemoryEventsPhysical(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lowMemoryEventsPhysical)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t ghostWindows; */
    #[inline]
    pub unsafe fn get_ghostWindows(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_ghostWindows)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute int64_t pageFaultsHard; */
    #[inline]
    pub unsafe fn get_pageFaultsHard(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_pageFaultsHard)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasMozMallocUsableSize; */
    #[inline]
    pub unsafe fn get_hasMozMallocUsableSize(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasMozMallocUsableSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isDMDEnabled; */
    #[inline]
    pub unsafe fn get_isDMDEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDMDEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isDMDRunning; */
    #[inline]
    pub unsafe fn get_isDMDRunning(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDMDRunning)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void minimizeMemoryUsage (in nsIRunnable callback); */
    #[inline]
    pub unsafe fn minimizeMemoryUsage(&self, callback: Option<&nsIRunnable>) -> Result<(), nsresult> {

        match ((*self.vtable).minimizeMemoryUsage)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void sizeOfTab (in mozIDOMWindowProxy window, out int64_t jsObjectsSize, out int64_t jsStringsSize, out int64_t jsOtherSize, out int64_t domSize, out int64_t styleSize, out int64_t otherSize, out int64_t totalSize, out double jsMilliseconds, out double nonJSMilliseconds); */
    #[inline]
    pub unsafe fn sizeOfTab(&self, window: Option<&mozIDOMWindowProxy>) -> Result<(int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, int64_t, libc::c_double, libc::c_double), nsresult> {
        let mut jsObjectsSize: int64_t = ::std::mem::zeroed();
        let mut jsStringsSize: int64_t = ::std::mem::zeroed();
        let mut jsOtherSize: int64_t = ::std::mem::zeroed();
        let mut domSize: int64_t = ::std::mem::zeroed();
        let mut styleSize: int64_t = ::std::mem::zeroed();
        let mut otherSize: int64_t = ::std::mem::zeroed();
        let mut totalSize: int64_t = ::std::mem::zeroed();
        let mut jsMilliseconds: libc::c_double = ::std::mem::zeroed();
        let mut nonJSMilliseconds: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).sizeOfTab)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), &mut jsObjectsSize as *mut _, &mut jsStringsSize as *mut _, &mut jsOtherSize as *mut _, &mut domSize as *mut _, &mut styleSize as *mut _, &mut otherSize as *mut _, &mut totalSize as *mut _, &mut jsMilliseconds as *mut _, &mut nonJSMilliseconds as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((jsObjectsSize, jsStringsSize, jsOtherSize, domSize, styleSize, otherSize, totalSize, jsMilliseconds, nonJSMilliseconds))
    }

}


