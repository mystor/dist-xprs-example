//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMemoryInfoDumper.idl
//


#[repr(C)]
pub struct nsIFinishDumpingCallback {
    vtable: *const nsIFinishDumpingCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFinishDumpingCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2dea18fc, 0xfbfa, 0x4bf7,
            [0xad, 0x45, 0x0e, 0xfa, 0xf5, 0x49, 0x5f, 0x5e])
    }
}

unsafe impl RefCounted for nsIFinishDumpingCallback {
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
pub trait nsIFinishDumpingCallbackCoerce {
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self;
}

impl nsIFinishDumpingCallbackCoerce for nsIFinishDumpingCallback {
    #[inline]
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self {
        v
    }
}

impl nsIFinishDumpingCallback {
    #[inline]
    pub fn coerce<T: nsIFinishDumpingCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFinishDumpingCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFinishDumpingCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFinishDumpingCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFinishDumpingCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in nsISupports data); */
    pub callback: unsafe extern "C" fn (this: *const nsIFinishDumpingCallback, data: *const nsISupports) -> nsresult,

}


impl nsIFinishDumpingCallback {
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
pub struct nsIDumpGCAndCCLogsCallback {
    vtable: *const nsIDumpGCAndCCLogsCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDumpGCAndCCLogsCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdc1b2b24, 0x65bd, 0x441b,
            [0xb6, 0xbd, 0xcb, 0x58, 0x25, 0xa7, 0xed, 0x14])
    }
}

unsafe impl RefCounted for nsIDumpGCAndCCLogsCallback {
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
pub trait nsIDumpGCAndCCLogsCallbackCoerce {
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self;
}

impl nsIDumpGCAndCCLogsCallbackCoerce for nsIDumpGCAndCCLogsCallback {
    #[inline]
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self {
        v
    }
}

impl nsIDumpGCAndCCLogsCallback {
    #[inline]
    pub fn coerce<T: nsIDumpGCAndCCLogsCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDumpGCAndCCLogsCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDumpGCAndCCLogsCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDumpGCAndCCLogsCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDumpGCAndCCLogsCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
    pub onDump: unsafe extern "C" fn (this: *const nsIDumpGCAndCCLogsCallback, aGCLog: *const nsIFile, aCCLog: *const nsIFile, aIsParent: bool) -> nsresult,

    /* void onFinish (); */
    pub onFinish: unsafe extern "C" fn (this: *const nsIDumpGCAndCCLogsCallback) -> nsresult,

}


impl nsIDumpGCAndCCLogsCallback {
    /* void onDump (in nsIFile aGCLog, in nsIFile aCCLog, in bool aIsParent); */
    #[inline]
    pub unsafe fn onDump(&self, aGCLog: Option<&nsIFile>, aCCLog: Option<&nsIFile>, aIsParent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onDump)(self as *const _, aGCLog.map_or(::std::ptr::null(), |x| x as *const _), aCCLog.map_or(::std::ptr::null(), |x| x as *const _), aIsParent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onFinish (); */
    #[inline]
    pub unsafe fn onFinish(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onFinish)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIMemoryInfoDumper {
    vtable: *const nsIMemoryInfoDumperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMemoryInfoDumper {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x48541b74, 0x47ee, 0x4a62,
            [0x95, 0x57, 0x7f, 0x4b, 0x80, 0x9b, 0xda, 0x5c])
    }
}

unsafe impl RefCounted for nsIMemoryInfoDumper {
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
pub trait nsIMemoryInfoDumperCoerce {
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self;
}

impl nsIMemoryInfoDumperCoerce for nsIMemoryInfoDumper {
    #[inline]
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self {
        v
    }
}

impl nsIMemoryInfoDumper {
    #[inline]
    pub fn coerce<T: nsIMemoryInfoDumperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMemoryInfoDumper {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMemoryInfoDumperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMemoryInfoDumper) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMemoryInfoDumperVTable {
    pub __base: nsISupportsVTable,

    /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize); */
    pub dumpMemoryReportsToNamedFile: unsafe extern "C" fn (this: *const nsIMemoryInfoDumper, aFilename: *const nsAString, aFinishDumping: *const nsIFinishDumpingCallback, aFinishDumpingData: *const nsISupports, aAnonymize: bool) -> nsresult,

    /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
    pub dumpMemoryInfoToTempDir: unsafe extern "C" fn (this: *const nsIMemoryInfoDumper, aIdentifier: *const nsAString, aAnonymize: bool, aMinimizeMemoryUsage: bool) -> nsresult,

    /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
    pub dumpGCAndCCLogsToFile: unsafe extern "C" fn (this: *const nsIMemoryInfoDumper, aIdentifier: *const nsAString, aDumpAllTraces: bool, aDumpChildProcesses: bool, aCallback: *const nsIDumpGCAndCCLogsCallback) -> nsresult,

    /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
    pub dumpGCAndCCLogsToSink: unsafe extern "C" fn (this: *const nsIMemoryInfoDumper, aDumpAllTraces: bool, aSink: *const nsICycleCollectorLogSink) -> nsresult,

}


impl nsIMemoryInfoDumper {
    /* void dumpMemoryReportsToNamedFile (in AString aFilename, in nsIFinishDumpingCallback aFinishDumping, in nsISupports aFinishDumpingData, in boolean aAnonymize); */
    #[inline]
    pub unsafe fn dumpMemoryReportsToNamedFile(&self, aFilename: &[u16], aFinishDumping: Option<&nsIFinishDumpingCallback>, aFinishDumpingData: Option<&nsISupports>, aAnonymize: bool) -> Result<(), nsresult> {
        let aFilename = nsString::from(aFilename);
        match ((*self.vtable).dumpMemoryReportsToNamedFile)(self as *const _, &*aFilename, aFinishDumping.map_or(::std::ptr::null(), |x| x as *const _), aFinishDumpingData.map_or(::std::ptr::null(), |x| x as *const _), aAnonymize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpMemoryInfoToTempDir (in AString aIdentifier, in boolean aAnonymize, in boolean aMinimizeMemoryUsage); */
    #[inline]
    pub unsafe fn dumpMemoryInfoToTempDir(&self, aIdentifier: &[u16], aAnonymize: bool, aMinimizeMemoryUsage: bool) -> Result<(), nsresult> {
        let aIdentifier = nsString::from(aIdentifier);
        match ((*self.vtable).dumpMemoryInfoToTempDir)(self as *const _, &*aIdentifier, aAnonymize, aMinimizeMemoryUsage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpGCAndCCLogsToFile (in AString aIdentifier, in bool aDumpAllTraces, in bool aDumpChildProcesses, in nsIDumpGCAndCCLogsCallback aCallback); */
    #[inline]
    pub unsafe fn dumpGCAndCCLogsToFile(&self, aIdentifier: &[u16], aDumpAllTraces: bool, aDumpChildProcesses: bool, aCallback: Option<&nsIDumpGCAndCCLogsCallback>) -> Result<(), nsresult> {
        let aIdentifier = nsString::from(aIdentifier);
        match ((*self.vtable).dumpGCAndCCLogsToFile)(self as *const _, &*aIdentifier, aDumpAllTraces, aDumpChildProcesses, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpGCAndCCLogsToSink (in bool aDumpAllTraces, in nsICycleCollectorLogSink aSink); */
    #[inline]
    pub unsafe fn dumpGCAndCCLogsToSink(&self, aDumpAllTraces: bool, aSink: Option<&nsICycleCollectorLogSink>) -> Result<(), nsresult> {

        match ((*self.vtable).dumpGCAndCCLogsToSink)(self as *const _, aDumpAllTraces, aSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


