//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICrashReporter.idl
//


#[repr(C)]
pub struct nsICrashReporter {
    vtable: *const nsICrashReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICrashReporter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4b74c39a, 0xcf69, 0x4a8a,
            [0x8e, 0x63, 0x16, 0x9d, 0x81, 0xad, 0x1e, 0xcf])
    }
}

unsafe impl RefCounted for nsICrashReporter {
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
pub trait nsICrashReporterCoerce {
    fn coerce_from(v: &nsICrashReporter) -> &Self;
}

impl nsICrashReporterCoerce for nsICrashReporter {
    #[inline]
    fn coerce_from(v: &nsICrashReporter) -> &Self {
        v
    }
}

impl nsICrashReporter {
    #[inline]
    pub fn coerce<T: nsICrashReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICrashReporter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICrashReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICrashReporter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICrashReporterVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean enabled; */
    pub get_enabled: unsafe extern "C" fn (this: *const nsICrashReporter, aEnabled: *mut bool) -> nsresult,

    /* [noscript] void setEnabled (in bool enabled); */
    pub setEnabled: unsafe extern "C" fn (this: *const nsICrashReporter, enabled: bool) -> nsresult,

    /* attribute nsIURL serverURL; */
    pub get_serverURL: unsafe extern "C" fn (this: *const nsICrashReporter, aServerURL: *mut *const nsIURL) -> nsresult,
    pub set_serverURL: unsafe extern "C" fn (this: *const nsICrashReporter, aServerURL: *const nsIURL) -> nsresult,

    /* attribute nsIFile minidumpPath; */
    pub get_minidumpPath: unsafe extern "C" fn (this: *const nsICrashReporter, aMinidumpPath: *mut *const nsIFile) -> nsresult,
    pub set_minidumpPath: unsafe extern "C" fn (this: *const nsICrashReporter, aMinidumpPath: *const nsIFile) -> nsresult,

    /* nsIFile getMinidumpForID (in AString id); */
    pub getMinidumpForID: unsafe extern "C" fn (this: *const nsICrashReporter, id: *const nsAString, _retval: *mut *const nsIFile) -> nsresult,

    /* nsIFile getExtraFileForID (in AString id); */
    pub getExtraFileForID: unsafe extern "C" fn (this: *const nsICrashReporter, id: *const nsAString, _retval: *mut *const nsIFile) -> nsresult,

    /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
    pub annotateCrashReport: unsafe extern "C" fn (this: *const nsICrashReporter, key: *const nsACString, data: *const nsACString) -> nsresult,

    /* void appendAppNotesToCrashReport (in ACString data); */
    pub appendAppNotesToCrashReport: unsafe extern "C" fn (this: *const nsICrashReporter, data: *const nsACString) -> nsresult,

    /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
    pub registerAppMemory: unsafe extern "C" fn (this: *const nsICrashReporter, ptr: libc::uint64_t, size: libc::uint64_t) -> nsresult,

    /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
    pub writeMinidumpForException: unsafe extern "C" fn (this: *const nsICrashReporter, aExceptionInfo: *const libc::c_void) -> nsresult,

    /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
    pub appendObjCExceptionInfoToAppNotes: unsafe extern "C" fn (this: *const nsICrashReporter, aException: *const libc::c_void) -> nsresult,

    /* attribute boolean submitReports; */
    pub get_submitReports: unsafe extern "C" fn (this: *const nsICrashReporter, aSubmitReports: *mut bool) -> nsresult,
    pub set_submitReports: unsafe extern "C" fn (this: *const nsICrashReporter, aSubmitReports: bool) -> nsresult,

    /* void UpdateCrashEventsDir (); */
    pub UpdateCrashEventsDir: unsafe extern "C" fn (this: *const nsICrashReporter) -> nsresult,

    /* void saveMemoryReport (); */
    pub saveMemoryReport: unsafe extern "C" fn (this: *const nsICrashReporter) -> nsresult,

    /* void setTelemetrySessionId (in AUTF8String id); */
    pub setTelemetrySessionId: unsafe extern "C" fn (this: *const nsICrashReporter, id: *const nsACString) -> nsresult,

}


impl nsICrashReporter {
    /* readonly attribute boolean enabled; */
    #[inline]
    pub unsafe fn get_enabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void setEnabled (in bool enabled); */
    #[inline]
    pub unsafe fn setEnabled(&self, enabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setEnabled)(self as *const _, enabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURL serverURL; */
    #[inline]
    pub unsafe fn get_serverURL(&self, ) -> Result<Option<RefPtr<nsIURL>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_serverURL)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_serverURL(&self, aServerURL: Option<&nsIURL>) -> Result<(), nsresult> {

        match ((*self.vtable).set_serverURL)(self as *const _, aServerURL.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFile minidumpPath; */
    #[inline]
    pub unsafe fn get_minidumpPath(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_minidumpPath)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_minidumpPath(&self, aMinidumpPath: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_minidumpPath)(self as *const _, aMinidumpPath.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIFile getMinidumpForID (in AString id); */
    #[inline]
    pub unsafe fn getMinidumpForID(&self, id: &[u16]) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let id = nsString::from(id);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMinidumpForID)(self as *const _, &*id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIFile getExtraFileForID (in AString id); */
    #[inline]
    pub unsafe fn getExtraFileForID(&self, id: &[u16]) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let id = nsString::from(id);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getExtraFileForID)(self as *const _, &*id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void annotateCrashReport (in AUTF8String key, in AUTF8String data); */
    #[inline]
    pub unsafe fn annotateCrashReport(&self, key: &[u8], data: &[u8]) -> Result<(), nsresult> {
        let key = nsCString::from(key);
        let data = nsCString::from(data);
        match ((*self.vtable).annotateCrashReport)(self as *const _, &*key, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendAppNotesToCrashReport (in ACString data); */
    #[inline]
    pub unsafe fn appendAppNotesToCrashReport(&self, data: &[u8]) -> Result<(), nsresult> {
        let data = nsCString::from(data);
        match ((*self.vtable).appendAppNotesToCrashReport)(self as *const _, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerAppMemory (in unsigned long long ptr, in unsigned long long size); */
    #[inline]
    pub unsafe fn registerAppMemory(&self, ptr: libc::uint64_t, size: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).registerAppMemory)(self as *const _, ptr, size) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void writeMinidumpForException (in voidPtr aExceptionInfo); */
    #[inline]
    pub unsafe fn writeMinidumpForException(&self, aExceptionInfo: *const libc::c_void) -> Result<(), nsresult> {

        match ((*self.vtable).writeMinidumpForException)(self as *const _, aExceptionInfo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void appendObjCExceptionInfoToAppNotes (in voidPtr aException); */
    #[inline]
    pub unsafe fn appendObjCExceptionInfoToAppNotes(&self, aException: *const libc::c_void) -> Result<(), nsresult> {

        match ((*self.vtable).appendObjCExceptionInfoToAppNotes)(self as *const _, aException) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean submitReports; */
    #[inline]
    pub unsafe fn get_submitReports(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_submitReports)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_submitReports(&self, aSubmitReports: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_submitReports)(self as *const _, aSubmitReports) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void UpdateCrashEventsDir (); */
    #[inline]
    pub unsafe fn UpdateCrashEventsDir(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).UpdateCrashEventsDir)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveMemoryReport (); */
    #[inline]
    pub unsafe fn saveMemoryReport(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).saveMemoryReport)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setTelemetrySessionId (in AUTF8String id); */
    #[inline]
    pub unsafe fn setTelemetrySessionId(&self, id: &[u8]) -> Result<(), nsresult> {
        let id = nsCString::from(id);
        match ((*self.vtable).setTelemetrySessionId)(self as *const _, &*id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


