//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncShutdown.idl
//


#[repr(C)]
pub struct nsIAsyncShutdownBlocker {
    vtable: *const nsIAsyncShutdownBlockerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncShutdownBlocker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4ef43f29, 0x6715, 0x4b57,
            [0xa7, 0x50, 0x2f, 0xf8, 0x36, 0x95, 0xdd, 0xce])
    }
}

unsafe impl RefCounted for nsIAsyncShutdownBlocker {
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
pub trait nsIAsyncShutdownBlockerCoerce {
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self;
}

impl nsIAsyncShutdownBlockerCoerce for nsIAsyncShutdownBlocker {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self {
        v
    }
}

impl nsIAsyncShutdownBlocker {
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownBlockerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncShutdownBlocker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncShutdownBlockerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBlocker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncShutdownBlockerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIAsyncShutdownBlocker, aName: *mut nsAString) -> nsresult,

    /* void blockShutdown (in nsIAsyncShutdownClient aBarrierClient); */
    pub blockShutdown: unsafe extern "C" fn (this: *const nsIAsyncShutdownBlocker, aBarrierClient: *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIPropertyBag state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIAsyncShutdownBlocker, aState: *mut *const nsIPropertyBag) -> nsresult,

}


impl nsIAsyncShutdownBlocker {
    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void blockShutdown (in nsIAsyncShutdownClient aBarrierClient); */
    #[inline]
    pub unsafe fn blockShutdown(&self, aBarrierClient: Option<&nsIAsyncShutdownClient>) -> Result<(), nsresult> {

        match ((*self.vtable).blockShutdown)(self as *const _, aBarrierClient.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIPropertyBag state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<Option<RefPtr<nsIPropertyBag>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_state)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIAsyncShutdownClient {
    vtable: *const nsIAsyncShutdownClientVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncShutdownClient {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd2031049, 0xb990, 0x43a2,
            [0x95, 0xbe, 0x59, 0xf8, 0xa3, 0xca, 0x59, 0x54])
    }
}

unsafe impl RefCounted for nsIAsyncShutdownClient {
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
pub trait nsIAsyncShutdownClientCoerce {
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self;
}

impl nsIAsyncShutdownClientCoerce for nsIAsyncShutdownClient {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self {
        v
    }
}

impl nsIAsyncShutdownClient {
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownClientCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncShutdownClient {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncShutdownClientCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownClient) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncShutdownClientVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIAsyncShutdownClient, aName: *mut nsAString) -> nsresult,

    /* void addBlocker (in nsIAsyncShutdownBlocker aBlocker, in AString aFileName, in long aLineNumber, in AString aStack); */
    pub addBlocker: unsafe extern "C" fn (this: *const nsIAsyncShutdownClient, aBlocker: *const nsIAsyncShutdownBlocker, aFileName: *const nsAString, aLineNumber: libc::int32_t, aStack: *const nsAString) -> nsresult,

    /* void removeBlocker (in nsIAsyncShutdownBlocker aBlocker); */
    pub removeBlocker: unsafe extern "C" fn (this: *const nsIAsyncShutdownClient, aBlocker: *const nsIAsyncShutdownBlocker) -> nsresult,

    /* readonly attribute jsval jsclient; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_jsclient: *const ::libc::c_void,

}


impl nsIAsyncShutdownClient {
    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addBlocker (in nsIAsyncShutdownBlocker aBlocker, in AString aFileName, in long aLineNumber, in AString aStack); */
    #[inline]
    pub unsafe fn addBlocker(&self, aBlocker: Option<&nsIAsyncShutdownBlocker>, aFileName: &[u16], aLineNumber: libc::int32_t, aStack: &[u16]) -> Result<(), nsresult> {
        let aFileName = nsString::from(aFileName);
        let aStack = nsString::from(aStack);
        match ((*self.vtable).addBlocker)(self as *const _, aBlocker.map_or(::std::ptr::null(), |x| x as *const _), &*aFileName, aLineNumber, &*aStack) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeBlocker (in nsIAsyncShutdownBlocker aBlocker); */
    #[inline]
    pub unsafe fn removeBlocker(&self, aBlocker: Option<&nsIAsyncShutdownBlocker>) -> Result<(), nsresult> {

        match ((*self.vtable).removeBlocker)(self as *const _, aBlocker.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute jsval jsclient; */


}


#[repr(C)]
pub struct nsIAsyncShutdownCompletionCallback {
    vtable: *const nsIAsyncShutdownCompletionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncShutdownCompletionCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x910c9309, 0x1da0, 0x4dd0,
            [0x8b, 0xdb, 0xa3, 0x25, 0xa3, 0x8c, 0x60, 0x4e])
    }
}

unsafe impl RefCounted for nsIAsyncShutdownCompletionCallback {
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
pub trait nsIAsyncShutdownCompletionCallbackCoerce {
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self;
}

impl nsIAsyncShutdownCompletionCallbackCoerce for nsIAsyncShutdownCompletionCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self {
        v
    }
}

impl nsIAsyncShutdownCompletionCallback {
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownCompletionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncShutdownCompletionCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncShutdownCompletionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownCompletionCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncShutdownCompletionCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void done (); */
    pub done: unsafe extern "C" fn (this: *const nsIAsyncShutdownCompletionCallback) -> nsresult,

}


impl nsIAsyncShutdownCompletionCallback {
    /* void done (); */
    #[inline]
    pub unsafe fn done(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).done)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAsyncShutdownBarrier {
    vtable: *const nsIAsyncShutdownBarrierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncShutdownBarrier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x50fa8a86, 0x9c91, 0x4256,
            [0x83, 0x89, 0x17, 0xd3, 0x10, 0xad, 0xec, 0x90])
    }
}

unsafe impl RefCounted for nsIAsyncShutdownBarrier {
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
pub trait nsIAsyncShutdownBarrierCoerce {
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self;
}

impl nsIAsyncShutdownBarrierCoerce for nsIAsyncShutdownBarrier {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self {
        v
    }
}

impl nsIAsyncShutdownBarrier {
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownBarrierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncShutdownBarrier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncShutdownBarrierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownBarrier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncShutdownBarrierVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAsyncShutdownClient client; */
    pub get_client: unsafe extern "C" fn (this: *const nsIAsyncShutdownBarrier, aClient: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIPropertyBag state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIAsyncShutdownBarrier, aState: *mut *const nsIPropertyBag) -> nsresult,

    /* void wait (in nsIAsyncShutdownCompletionCallback aOnReady); */
    pub wait: unsafe extern "C" fn (this: *const nsIAsyncShutdownBarrier, aOnReady: *const nsIAsyncShutdownCompletionCallback) -> nsresult,

}


impl nsIAsyncShutdownBarrier {
    /* readonly attribute nsIAsyncShutdownClient client; */
    #[inline]
    pub unsafe fn get_client(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_client)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIPropertyBag state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<Option<RefPtr<nsIPropertyBag>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_state)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void wait (in nsIAsyncShutdownCompletionCallback aOnReady); */
    #[inline]
    pub unsafe fn wait(&self, aOnReady: Option<&nsIAsyncShutdownCompletionCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).wait)(self as *const _, aOnReady.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAsyncShutdownService {
    vtable: *const nsIAsyncShutdownServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncShutdownService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdb365c78, 0xc860, 0x4e64,
            [0x9a, 0x63, 0x25, 0xb7, 0x3f, 0x89, 0xa0, 0x16])
    }
}

unsafe impl RefCounted for nsIAsyncShutdownService {
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
pub trait nsIAsyncShutdownServiceCoerce {
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self;
}

impl nsIAsyncShutdownServiceCoerce for nsIAsyncShutdownService {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self {
        v
    }
}

impl nsIAsyncShutdownService {
    #[inline]
    pub fn coerce<T: nsIAsyncShutdownServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncShutdownService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncShutdownServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncShutdownService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncShutdownServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIAsyncShutdownBarrier makeBarrier (in AString aName); */
    pub makeBarrier: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aName: *const nsAString, _retval: *mut *const nsIAsyncShutdownBarrier) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient profileBeforeChange; */
    pub get_profileBeforeChange: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aProfileBeforeChange: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient profileChangeTeardown; */
    pub get_profileChangeTeardown: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aProfileChangeTeardown: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient quitApplicationGranted; */
    pub get_quitApplicationGranted: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aQuitApplicationGranted: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient sendTelemetry; */
    pub get_sendTelemetry: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aSendTelemetry: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient webWorkersShutdown; */
    pub get_webWorkersShutdown: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aWebWorkersShutdown: *mut *const nsIAsyncShutdownClient) -> nsresult,

    /* readonly attribute nsIAsyncShutdownClient xpcomWillShutdown; */
    pub get_xpcomWillShutdown: unsafe extern "C" fn (this: *const nsIAsyncShutdownService, aXpcomWillShutdown: *mut *const nsIAsyncShutdownClient) -> nsresult,

}


impl nsIAsyncShutdownService {
    /* nsIAsyncShutdownBarrier makeBarrier (in AString aName); */
    #[inline]
    pub unsafe fn makeBarrier(&self, aName: &[u16]) -> Result<Option<RefPtr<nsIAsyncShutdownBarrier>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).makeBarrier)(self as *const _, &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient profileBeforeChange; */
    #[inline]
    pub unsafe fn get_profileBeforeChange(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_profileBeforeChange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient profileChangeTeardown; */
    #[inline]
    pub unsafe fn get_profileChangeTeardown(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_profileChangeTeardown)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient quitApplicationGranted; */
    #[inline]
    pub unsafe fn get_quitApplicationGranted(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_quitApplicationGranted)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient sendTelemetry; */
    #[inline]
    pub unsafe fn get_sendTelemetry(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sendTelemetry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient webWorkersShutdown; */
    #[inline]
    pub unsafe fn get_webWorkersShutdown(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_webWorkersShutdown)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAsyncShutdownClient xpcomWillShutdown; */
    #[inline]
    pub unsafe fn get_xpcomWillShutdown(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_xpcomWillShutdown)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


