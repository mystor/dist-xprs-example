//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICycleCollectorListener.idl
//


#[repr(C)]
pub struct nsICycleCollectorHandler {
    vtable: *const nsICycleCollectorHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICycleCollectorHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7f093367, 0x1492, 0x4b89,
            [0x87, 0xaf, 0xc0, 0x1d, 0xbc, 0x83, 0x12, 0x46])
    }
}

unsafe impl RefCounted for nsICycleCollectorHandler {
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
pub trait nsICycleCollectorHandlerCoerce {
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self;
}

impl nsICycleCollectorHandlerCoerce for nsICycleCollectorHandler {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self {
        v
    }
}

impl nsICycleCollectorHandler {
    #[inline]
    pub fn coerce<T: nsICycleCollectorHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICycleCollectorHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICycleCollectorHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICycleCollectorHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
    pub noteRefCountedObject: unsafe extern "C" fn (this: *const nsICycleCollectorHandler, aAddress: *const nsACString, aRefCount: libc::uint32_t, aObjectDescription: *const nsACString) -> nsresult,

    /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
    pub noteGCedObject: unsafe extern "C" fn (this: *const nsICycleCollectorHandler, aAddress: *const nsACString, aMarked: bool, aObjectDescription: *const nsACString, aCompartmentAddress: *const nsACString) -> nsresult,

    /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
    pub noteEdge: unsafe extern "C" fn (this: *const nsICycleCollectorHandler, aFromAddress: *const nsACString, aToAddress: *const nsACString, aEdgeName: *const nsACString) -> nsresult,

    /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
    pub describeRoot: unsafe extern "C" fn (this: *const nsICycleCollectorHandler, aAddress: *const nsACString, aKnownEdges: libc::uint32_t) -> nsresult,

    /* void describeGarbage (in ACString aAddress); */
    pub describeGarbage: unsafe extern "C" fn (this: *const nsICycleCollectorHandler, aAddress: *const nsACString) -> nsresult,

}


impl nsICycleCollectorHandler {
    /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
    #[inline]
    pub unsafe fn noteRefCountedObject(&self, aAddress: &[u8], aRefCount: libc::uint32_t, aObjectDescription: &[u8]) -> Result<(), nsresult> {
        let aAddress = nsCString::from(aAddress);
        let aObjectDescription = nsCString::from(aObjectDescription);
        match ((*self.vtable).noteRefCountedObject)(self as *const _, &*aAddress, aRefCount, &*aObjectDescription) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
    #[inline]
    pub unsafe fn noteGCedObject(&self, aAddress: &[u8], aMarked: bool, aObjectDescription: &[u8], aCompartmentAddress: &[u8]) -> Result<(), nsresult> {
        let aAddress = nsCString::from(aAddress);
        let aObjectDescription = nsCString::from(aObjectDescription);
        let aCompartmentAddress = nsCString::from(aCompartmentAddress);
        match ((*self.vtable).noteGCedObject)(self as *const _, &*aAddress, aMarked, &*aObjectDescription, &*aCompartmentAddress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
    #[inline]
    pub unsafe fn noteEdge(&self, aFromAddress: &[u8], aToAddress: &[u8], aEdgeName: &[u8]) -> Result<(), nsresult> {
        let aFromAddress = nsCString::from(aFromAddress);
        let aToAddress = nsCString::from(aToAddress);
        let aEdgeName = nsCString::from(aEdgeName);
        match ((*self.vtable).noteEdge)(self as *const _, &*aFromAddress, &*aToAddress, &*aEdgeName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
    #[inline]
    pub unsafe fn describeRoot(&self, aAddress: &[u8], aKnownEdges: libc::uint32_t) -> Result<(), nsresult> {
        let aAddress = nsCString::from(aAddress);
        match ((*self.vtable).describeRoot)(self as *const _, &*aAddress, aKnownEdges) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void describeGarbage (in ACString aAddress); */
    #[inline]
    pub unsafe fn describeGarbage(&self, aAddress: &[u8]) -> Result<(), nsresult> {
        let aAddress = nsCString::from(aAddress);
        match ((*self.vtable).describeGarbage)(self as *const _, &*aAddress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICycleCollectorLogSink {
    vtable: *const nsICycleCollectorLogSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICycleCollectorLogSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ad9875f, 0xd0e4, 0x4ac2,
            [0x87, 0xe3, 0xf1, 0x27, 0xf6, 0xc0, 0x2c, 0xe1])
    }
}

unsafe impl RefCounted for nsICycleCollectorLogSink {
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
pub trait nsICycleCollectorLogSinkCoerce {
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self;
}

impl nsICycleCollectorLogSinkCoerce for nsICycleCollectorLogSink {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self {
        v
    }
}

impl nsICycleCollectorLogSink {
    #[inline]
    pub fn coerce<T: nsICycleCollectorLogSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICycleCollectorLogSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICycleCollectorLogSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorLogSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICycleCollectorLogSinkVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void open (out FILE aGCLog, out FILE aCCLog); */
    /// Unable to call function as its signature contains a non-rust type
    pub open: *const ::libc::c_void,

    /* void closeGCLog (); */
    pub closeGCLog: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink) -> nsresult,

    /* void closeCCLog (); */
    pub closeCCLog: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink) -> nsresult,

    /* attribute AString filenameIdentifier; */
    pub get_filenameIdentifier: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aFilenameIdentifier: *mut nsAString) -> nsresult,
    pub set_filenameIdentifier: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aFilenameIdentifier: *const nsAString) -> nsresult,

    /* attribute int32_t processIdentifier; */
    pub get_processIdentifier: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aProcessIdentifier: *mut int32_t) -> nsresult,
    pub set_processIdentifier: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aProcessIdentifier: int32_t) -> nsresult,

    /* readonly attribute nsIFile gcLog; */
    pub get_gcLog: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aGcLog: *mut *const nsIFile) -> nsresult,

    /* readonly attribute nsIFile ccLog; */
    pub get_ccLog: unsafe extern "C" fn (this: *const nsICycleCollectorLogSink, aCcLog: *mut *const nsIFile) -> nsresult,

}


impl nsICycleCollectorLogSink {
    /* [noscript] void open (out FILE aGCLog, out FILE aCCLog); */


    /* void closeGCLog (); */
    #[inline]
    pub unsafe fn closeGCLog(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeGCLog)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeCCLog (); */
    #[inline]
    pub unsafe fn closeCCLog(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closeCCLog)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString filenameIdentifier; */
    #[inline]
    pub unsafe fn get_filenameIdentifier(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_filenameIdentifier)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_filenameIdentifier(&self, aFilenameIdentifier: &[u16]) -> Result<(), nsresult> {
        let aFilenameIdentifier = nsString::from(aFilenameIdentifier);
        match ((*self.vtable).set_filenameIdentifier)(self as *const _, &*aFilenameIdentifier) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute int32_t processIdentifier; */
    #[inline]
    pub unsafe fn get_processIdentifier(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_processIdentifier)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_processIdentifier(&self, aProcessIdentifier: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_processIdentifier)(self as *const _, aProcessIdentifier) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile gcLog; */
    #[inline]
    pub unsafe fn get_gcLog(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_gcLog)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile ccLog; */
    #[inline]
    pub unsafe fn get_ccLog(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ccLog)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsICycleCollectorListener {
    vtable: *const nsICycleCollectorListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICycleCollectorListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x703b53b6, 0x24f6, 0x40c6,
            [0x9e, 0xa9, 0xae, 0xb2, 0xdc, 0x53, 0xd1, 0x70])
    }
}

unsafe impl RefCounted for nsICycleCollectorListener {
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
pub trait nsICycleCollectorListenerCoerce {
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self;
}

impl nsICycleCollectorListenerCoerce for nsICycleCollectorListener {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self {
        v
    }
}

impl nsICycleCollectorListener {
    #[inline]
    pub fn coerce<T: nsICycleCollectorListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICycleCollectorListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICycleCollectorListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICycleCollectorListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICycleCollectorListenerVTable {
    pub __base: nsISupportsVTable,

    /* nsICycleCollectorListener allTraces (); */
    pub allTraces: unsafe extern "C" fn (this: *const nsICycleCollectorListener, _retval: *mut *const nsICycleCollectorListener) -> nsresult,

    /* readonly attribute boolean wantAllTraces; */
    pub get_wantAllTraces: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aWantAllTraces: *mut bool) -> nsresult,

    /* attribute boolean disableLog; */
    pub get_disableLog: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aDisableLog: *mut bool) -> nsresult,
    pub set_disableLog: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aDisableLog: bool) -> nsresult,

    /* attribute nsICycleCollectorLogSink logSink; */
    pub get_logSink: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aLogSink: *mut *const nsICycleCollectorLogSink) -> nsresult,
    pub set_logSink: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aLogSink: *const nsICycleCollectorLogSink) -> nsresult,

    /* attribute boolean wantAfterProcessing; */
    pub get_wantAfterProcessing: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aWantAfterProcessing: *mut bool) -> nsresult,
    pub set_wantAfterProcessing: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aWantAfterProcessing: bool) -> nsresult,

    /* boolean processNext (in nsICycleCollectorHandler aHandler); */
    pub processNext: unsafe extern "C" fn (this: *const nsICycleCollectorListener, aHandler: *const nsICycleCollectorHandler, _retval: *mut bool) -> nsresult,

    /* [noscript] nsCycleCollectorLoggerPtr asLogger (); */
    /// Unable to call function as its signature contains a non-rust type
    pub asLogger: *const ::libc::c_void,

}


impl nsICycleCollectorListener {
    /* nsICycleCollectorListener allTraces (); */
    #[inline]
    pub unsafe fn allTraces(&self, ) -> Result<Option<RefPtr<nsICycleCollectorListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).allTraces)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean wantAllTraces; */
    #[inline]
    pub unsafe fn get_wantAllTraces(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_wantAllTraces)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean disableLog; */
    #[inline]
    pub unsafe fn get_disableLog(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disableLog)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_disableLog(&self, aDisableLog: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_disableLog)(self as *const _, aDisableLog) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsICycleCollectorLogSink logSink; */
    #[inline]
    pub unsafe fn get_logSink(&self, ) -> Result<Option<RefPtr<nsICycleCollectorLogSink>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_logSink)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_logSink(&self, aLogSink: Option<&nsICycleCollectorLogSink>) -> Result<(), nsresult> {

        match ((*self.vtable).set_logSink)(self as *const _, aLogSink.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean wantAfterProcessing; */
    #[inline]
    pub unsafe fn get_wantAfterProcessing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_wantAfterProcessing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_wantAfterProcessing(&self, aWantAfterProcessing: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_wantAfterProcessing)(self as *const _, aWantAfterProcessing) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean processNext (in nsICycleCollectorHandler aHandler); */
    #[inline]
    pub unsafe fn processNext(&self, aHandler: Option<&nsICycleCollectorHandler>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).processNext)(self as *const _, aHandler.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] nsCycleCollectorLoggerPtr asLogger (); */


}


