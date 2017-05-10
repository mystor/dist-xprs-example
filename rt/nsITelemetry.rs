//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITelemetry.idl
//


#[repr(C)]
pub struct nsIFetchTelemetryDataCallback {
    vtable: *const nsIFetchTelemetryDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFetchTelemetryDataCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3d3b9075, 0x5549, 0x4244,
            [0x9c, 0x08, 0xb6, 0x4f, 0xef, 0xa1, 0xdd, 0x60])
    }
}

unsafe impl RefCounted for nsIFetchTelemetryDataCallback {
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
pub trait nsIFetchTelemetryDataCallbackCoerce {
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self;
}

impl nsIFetchTelemetryDataCallbackCoerce for nsIFetchTelemetryDataCallback {
    #[inline]
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self {
        v
    }
}

impl nsIFetchTelemetryDataCallback {
    #[inline]
    pub fn coerce<T: nsIFetchTelemetryDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFetchTelemetryDataCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFetchTelemetryDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFetchTelemetryDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFetchTelemetryDataCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (); */
    pub complete: unsafe extern "C" fn (this: *const nsIFetchTelemetryDataCallback) -> nsresult,

}


impl nsIFetchTelemetryDataCallback {
    /* void complete (); */
    #[inline]
    pub unsafe fn complete(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsITelemetry_consts {
    pub const HISTOGRAM_EXPONENTIAL: i64 = 0;
    pub const HISTOGRAM_LINEAR: i64 = 1;
    pub const HISTOGRAM_BOOLEAN: i64 = 2;
    pub const HISTOGRAM_FLAG: i64 = 3;
    pub const HISTOGRAM_COUNT: i64 = 4;
    pub const HISTOGRAM_CATEGORICAL: i64 = 5;
    pub const SCALAR_COUNT: i64 = 0;
    pub const SCALAR_STRING: i64 = 1;
    pub const SCALAR_BOOLEAN: i64 = 2;
    pub const DATASET_RELEASE_CHANNEL_OPTOUT: i64 = 0;
    pub const DATASET_RELEASE_CHANNEL_OPTIN: i64 = 1;
}


#[repr(C)]
pub struct nsITelemetry {
    vtable: *const nsITelemetryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITelemetry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x273d2dd0, 0x6c63, 0x475a,
            [0xb8, 0x64, 0xcb, 0x65, 0x16, 0x0a, 0x19, 0x09])
    }
}

unsafe impl RefCounted for nsITelemetry {
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
pub trait nsITelemetryCoerce {
    fn coerce_from(v: &nsITelemetry) -> &Self;
}

impl nsITelemetryCoerce for nsITelemetry {
    #[inline]
    fn coerce_from(v: &nsITelemetry) -> &Self {
        v
    }
}

impl nsITelemetry {
    #[inline]
    pub fn coerce<T: nsITelemetryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITelemetry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITelemetryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITelemetry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITelemetryVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] readonly attribute jsval histogramSnapshots; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_histogramSnapshots: *const ::libc::c_void,

    /* [implicit_jscontext] jsval snapshotSubsessionHistograms ([optional] in boolean clear); */
    /// Unable to call function as its signature contains a non-rust type
    pub snapshotSubsessionHistograms: *const ::libc::c_void,

    /* readonly attribute uint32_t lastShutdownDuration; */
    pub get_lastShutdownDuration: unsafe extern "C" fn (this: *const nsITelemetry, aLastShutdownDuration: *mut uint32_t) -> nsresult,

    /* readonly attribute uint32_t failedProfileLockCount; */
    pub get_failedProfileLockCount: unsafe extern "C" fn (this: *const nsITelemetry, aFailedProfileLockCount: *mut uint32_t) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval slowSQL; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_slowSQL: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval debugSlowSQL; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_debugSlowSQL: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval webrtcStats; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_webrtcStats: *const ::libc::c_void,

    /* readonly attribute uint32_t maximalNumberOfConcurrentThreads; */
    pub get_maximalNumberOfConcurrentThreads: unsafe extern "C" fn (this: *const nsITelemetry, aMaximalNumberOfConcurrentThreads: *mut uint32_t) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval chromeHangs; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_chromeHangs: *const ::libc::c_void,

    /* void captureStack (in ACString name); */
    pub captureStack: unsafe extern "C" fn (this: *const nsITelemetry, name: *const nsACString) -> nsresult,

    /* [implicit_jscontext] jsval snapshotCapturedStacks ([optional] in boolean clear); */
    /// Unable to call function as its signature contains a non-rust type
    pub snapshotCapturedStacks: *const ::libc::c_void,

    /* [implicit_jscontext] nsISupports getLoadedModules (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getLoadedModules: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval threadHangStats; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_threadHangStats: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval lateWrites; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_lateWrites: *const ::libc::c_void,

    /* void registeredHistograms (in uint32_t dataset, out uint32_t count, [array, size_is (count), retval] out string histograms); */
    /// Unable to call function as its signature contains a non-rust type
    pub registeredHistograms: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getHistogramById (in ACString id); */
    /// Unable to call function as its signature contains a non-rust type
    pub getHistogramById: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval keyedHistogramSnapshots; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_keyedHistogramSnapshots: *const ::libc::c_void,

    /* void registeredKeyedHistograms (in uint32_t dataset, out uint32_t count, [array, size_is (count), retval] out string histograms); */
    /// Unable to call function as its signature contains a non-rust type
    pub registeredKeyedHistograms: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getKeyedHistogramById (in ACString id); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKeyedHistogramById: *const ::libc::c_void,

    /* attribute boolean canRecordBase; */
    pub get_canRecordBase: unsafe extern "C" fn (this: *const nsITelemetry, aCanRecordBase: *mut bool) -> nsresult,
    pub set_canRecordBase: unsafe extern "C" fn (this: *const nsITelemetry, aCanRecordBase: bool) -> nsresult,

    /* attribute boolean canRecordExtended; */
    pub get_canRecordExtended: unsafe extern "C" fn (this: *const nsITelemetry, aCanRecordExtended: *mut bool) -> nsresult,
    pub set_canRecordExtended: unsafe extern "C" fn (this: *const nsITelemetry, aCanRecordExtended: bool) -> nsresult,

    /* readonly attribute boolean isOfficialTelemetry; */
    pub get_isOfficialTelemetry: unsafe extern "C" fn (this: *const nsITelemetry, aIsOfficialTelemetry: *mut bool) -> nsresult,

    /* void setHistogramRecordingEnabled (in ACString id, in boolean enabled); */
    pub setHistogramRecordingEnabled: unsafe extern "C" fn (this: *const nsITelemetry, id: *const nsACString, enabled: bool) -> nsresult,

    /* void asyncFetchTelemetryData (in nsIFetchTelemetryDataCallback aCallback); */
    pub asyncFetchTelemetryData: unsafe extern "C" fn (this: *const nsITelemetry, aCallback: *const nsIFetchTelemetryDataCallback) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval fileIOReports; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_fileIOReports: *const ::libc::c_void,

    /* double msSinceProcessStart (); */
    pub msSinceProcessStart: unsafe extern "C" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> nsresult,

    /* double msSystemNow (); */
    pub msSystemNow: unsafe extern "C" fn (this: *const nsITelemetry, _retval: *mut libc::c_double) -> nsresult,

    /* [implicit_jscontext] void scalarAdd (in ACString aName, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub scalarAdd: *const ::libc::c_void,

    /* [implicit_jscontext] void scalarSet (in ACString aName, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub scalarSet: *const ::libc::c_void,

    /* [implicit_jscontext] void scalarSetMaximum (in ACString aName, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub scalarSetMaximum: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval snapshotScalars (in uint32_t aDataset, [optional] in boolean aClear); */
    /// Unable to call function as its signature contains a non-rust type
    pub snapshotScalars: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarAdd (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub keyedScalarAdd: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarSet (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub keyedScalarSet: *const ::libc::c_void,

    /* [implicit_jscontext] void keyedScalarSetMaximum (in ACString aName, in AString aKey, in jsval aValue); */
    /// Unable to call function as its signature contains a non-rust type
    pub keyedScalarSetMaximum: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] jsval snapshotKeyedScalars (in uint32_t aDataset, [optional] in boolean aClear); */
    /// Unable to call function as its signature contains a non-rust type
    pub snapshotKeyedScalars: *const ::libc::c_void,

    /* void clearScalars (); */
    pub clearScalars: unsafe extern "C" fn (this: *const nsITelemetry) -> nsresult,

    /* void flushBatchedChildTelemetry (); */
    pub flushBatchedChildTelemetry: unsafe extern "C" fn (this: *const nsITelemetry) -> nsresult,

    /* [implicit_jscontext,optional_argc] void recordEvent (in ACString aCategory, in ACString aMethod, in ACString aObject, [optional] in jsval aValue, [optional] in jsval extra); */
    /// Unable to call function as its signature contains a non-rust type
    pub recordEvent: *const ::libc::c_void,

    /* void setEventRecordingEnabled (in ACString aCategory, in boolean aEnabled); */
    pub setEventRecordingEnabled: unsafe extern "C" fn (this: *const nsITelemetry, aCategory: *const nsACString, aEnabled: bool) -> nsresult,

    /* [implicit_jscontext,optional_argc] jsval snapshotBuiltinEvents (in uint32_t aDataset, [optional] in boolean aClear); */
    /// Unable to call function as its signature contains a non-rust type
    pub snapshotBuiltinEvents: *const ::libc::c_void,

    /* void clearEvents (); */
    pub clearEvents: unsafe extern "C" fn (this: *const nsITelemetry) -> nsresult,

}


impl nsITelemetry {
    /* [implicit_jscontext] readonly attribute jsval histogramSnapshots; */


    /* [implicit_jscontext] jsval snapshotSubsessionHistograms ([optional] in boolean clear); */


    /* readonly attribute uint32_t lastShutdownDuration; */
    #[inline]
    pub unsafe fn get_lastShutdownDuration(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastShutdownDuration)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t failedProfileLockCount; */
    #[inline]
    pub unsafe fn get_failedProfileLockCount(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_failedProfileLockCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval slowSQL; */


    /* [implicit_jscontext] readonly attribute jsval debugSlowSQL; */


    /* [implicit_jscontext] readonly attribute jsval webrtcStats; */


    /* readonly attribute uint32_t maximalNumberOfConcurrentThreads; */
    #[inline]
    pub unsafe fn get_maximalNumberOfConcurrentThreads(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maximalNumberOfConcurrentThreads)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval chromeHangs; */


    /* void captureStack (in ACString name); */
    #[inline]
    pub unsafe fn captureStack(&self, name: &[u8]) -> Result<(), nsresult> {
        let name = nsCString::from(name);
        match ((*self.vtable).captureStack)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval snapshotCapturedStacks ([optional] in boolean clear); */


    /* [implicit_jscontext] nsISupports getLoadedModules (); */


    /* [implicit_jscontext] readonly attribute jsval threadHangStats; */


    /* [implicit_jscontext] readonly attribute jsval lateWrites; */


    /* void registeredHistograms (in uint32_t dataset, out uint32_t count, [array, size_is (count), retval] out string histograms); */


    /* [implicit_jscontext] jsval getHistogramById (in ACString id); */


    /* [implicit_jscontext] readonly attribute jsval keyedHistogramSnapshots; */


    /* void registeredKeyedHistograms (in uint32_t dataset, out uint32_t count, [array, size_is (count), retval] out string histograms); */


    /* [implicit_jscontext] jsval getKeyedHistogramById (in ACString id); */


    /* attribute boolean canRecordBase; */
    #[inline]
    pub unsafe fn get_canRecordBase(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canRecordBase)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_canRecordBase(&self, aCanRecordBase: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_canRecordBase)(self as *const _, aCanRecordBase) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean canRecordExtended; */
    #[inline]
    pub unsafe fn get_canRecordExtended(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canRecordExtended)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_canRecordExtended(&self, aCanRecordExtended: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_canRecordExtended)(self as *const _, aCanRecordExtended) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isOfficialTelemetry; */
    #[inline]
    pub unsafe fn get_isOfficialTelemetry(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOfficialTelemetry)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setHistogramRecordingEnabled (in ACString id, in boolean enabled); */
    #[inline]
    pub unsafe fn setHistogramRecordingEnabled(&self, id: &[u8], enabled: bool) -> Result<(), nsresult> {
        let id = nsCString::from(id);
        match ((*self.vtable).setHistogramRecordingEnabled)(self as *const _, &*id, enabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncFetchTelemetryData (in nsIFetchTelemetryDataCallback aCallback); */
    #[inline]
    pub unsafe fn asyncFetchTelemetryData(&self, aCallback: Option<&nsIFetchTelemetryDataCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncFetchTelemetryData)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] readonly attribute jsval fileIOReports; */


    /* double msSinceProcessStart (); */
    #[inline]
    pub unsafe fn msSinceProcessStart(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).msSinceProcessStart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double msSystemNow (); */
    #[inline]
    pub unsafe fn msSystemNow(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).msSystemNow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] void scalarAdd (in ACString aName, in jsval aValue); */


    /* [implicit_jscontext] void scalarSet (in ACString aName, in jsval aValue); */


    /* [implicit_jscontext] void scalarSetMaximum (in ACString aName, in jsval aValue); */


    /* [implicit_jscontext,optional_argc] jsval snapshotScalars (in uint32_t aDataset, [optional] in boolean aClear); */


    /* [implicit_jscontext] void keyedScalarAdd (in ACString aName, in AString aKey, in jsval aValue); */


    /* [implicit_jscontext] void keyedScalarSet (in ACString aName, in AString aKey, in jsval aValue); */


    /* [implicit_jscontext] void keyedScalarSetMaximum (in ACString aName, in AString aKey, in jsval aValue); */


    /* [implicit_jscontext,optional_argc] jsval snapshotKeyedScalars (in uint32_t aDataset, [optional] in boolean aClear); */


    /* void clearScalars (); */
    #[inline]
    pub unsafe fn clearScalars(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearScalars)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void flushBatchedChildTelemetry (); */
    #[inline]
    pub unsafe fn flushBatchedChildTelemetry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flushBatchedChildTelemetry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext,optional_argc] void recordEvent (in ACString aCategory, in ACString aMethod, in ACString aObject, [optional] in jsval aValue, [optional] in jsval extra); */


    /* void setEventRecordingEnabled (in ACString aCategory, in boolean aEnabled); */
    #[inline]
    pub unsafe fn setEventRecordingEnabled(&self, aCategory: &[u8], aEnabled: bool) -> Result<(), nsresult> {
        let aCategory = nsCString::from(aCategory);
        match ((*self.vtable).setEventRecordingEnabled)(self as *const _, &*aCategory, aEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext,optional_argc] jsval snapshotBuiltinEvents (in uint32_t aDataset, [optional] in boolean aClear); */


    /* void clearEvents (); */
    #[inline]
    pub unsafe fn clearEvents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearEvents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


