//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServiceWorkerManager.idl
//


#[repr(C)]
pub struct nsIServiceWorkerUnregisterCallback {
    vtable: *const nsIServiceWorkerUnregisterCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerUnregisterCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x52ee2c9d, 0xee87, 0x4caf,
            [0x95, 0x88, 0x23, 0xae, 0x77, 0xff, 0x87, 0x98])
    }
}

unsafe impl RefCounted for nsIServiceWorkerUnregisterCallback {
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
pub trait nsIServiceWorkerUnregisterCallbackCoerce {
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self;
}

impl nsIServiceWorkerUnregisterCallbackCoerce for nsIServiceWorkerUnregisterCallback {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self {
        v
    }
}

impl nsIServiceWorkerUnregisterCallback {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerUnregisterCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerUnregisterCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerUnregisterCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerUnregisterCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void unregisterSucceeded (in bool aState); */
    pub unregisterSucceeded: unsafe extern "C" fn (this: *const nsIServiceWorkerUnregisterCallback, aState: bool) -> nsresult,

    /* void unregisterFailed (); */
    pub unregisterFailed: unsafe extern "C" fn (this: *const nsIServiceWorkerUnregisterCallback) -> nsresult,

}


impl nsIServiceWorkerUnregisterCallback {
    /* void unregisterSucceeded (in bool aState); */
    #[inline]
    pub unsafe fn unregisterSucceeded(&self, aState: bool) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterSucceeded)(self as *const _, aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterFailed (); */
    #[inline]
    pub unsafe fn unregisterFailed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterFailed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIServiceWorkerInfo_consts {
    pub const STATE_INSTALLING: i64 = 0;
    pub const STATE_INSTALLED: i64 = 1;
    pub const STATE_ACTIVATING: i64 = 2;
    pub const STATE_ACTIVATED: i64 = 3;
    pub const STATE_REDUNDANT: i64 = 4;
    pub const STATE_UNKNOWN: i64 = 5;
}


#[repr(C)]
pub struct nsIServiceWorkerInfo {
    vtable: *const nsIServiceWorkerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x76e357ed, 0x208d, 0x4e4c,
            [0x91, 0x65, 0x1c, 0x40, 0x59, 0x70, 0x78, 0x79])
    }
}

unsafe impl RefCounted for nsIServiceWorkerInfo {
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
pub trait nsIServiceWorkerInfoCoerce {
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self;
}

impl nsIServiceWorkerInfoCoerce for nsIServiceWorkerInfo {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self {
        v
    }
}

impl nsIServiceWorkerInfo {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString scriptSpec; */
    pub get_scriptSpec: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aScriptSpec: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString cacheName; */
    pub get_cacheName: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aCacheName: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned short state; */
    pub get_state: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aState: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute nsIWorkerDebugger debugger; */
    pub get_debugger: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aDebugger: *mut *const nsIWorkerDebugger) -> nsresult,

    /* readonly attribute bool handlesFetchEvents; */
    pub get_handlesFetchEvents: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aHandlesFetchEvents: *mut bool) -> nsresult,

    /* readonly attribute PRTime installedTime; */
    pub get_installedTime: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aInstalledTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime activatedTime; */
    pub get_activatedTime: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aActivatedTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime redundantTime; */
    pub get_redundantTime: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo, aRedundantTime: *mut PRTime) -> nsresult,

    /* void attachDebugger (); */
    pub attachDebugger: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo) -> nsresult,

    /* void detachDebugger (); */
    pub detachDebugger: unsafe extern "C" fn (this: *const nsIServiceWorkerInfo) -> nsresult,

}


impl nsIServiceWorkerInfo {
    /* readonly attribute DOMString scriptSpec; */
    #[inline]
    pub unsafe fn get_scriptSpec(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scriptSpec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString cacheName; */
    #[inline]
    pub unsafe fn get_cacheName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cacheName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIWorkerDebugger debugger; */
    #[inline]
    pub unsafe fn get_debugger(&self, ) -> Result<Option<RefPtr<nsIWorkerDebugger>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_debugger)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute bool handlesFetchEvents; */
    #[inline]
    pub unsafe fn get_handlesFetchEvents(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_handlesFetchEvents)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime installedTime; */
    #[inline]
    pub unsafe fn get_installedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_installedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime activatedTime; */
    #[inline]
    pub unsafe fn get_activatedTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_activatedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime redundantTime; */
    #[inline]
    pub unsafe fn get_redundantTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_redundantTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void attachDebugger (); */
    #[inline]
    pub unsafe fn attachDebugger(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).attachDebugger)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void detachDebugger (); */
    #[inline]
    pub unsafe fn detachDebugger(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).detachDebugger)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoListener {
    vtable: *const nsIServiceWorkerRegistrationInfoListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerRegistrationInfoListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87e63548, 0xd440, 0x4b8a,
            [0xb1, 0x58, 0x65, 0xad, 0x1d, 0xe0, 0x21, 0x1e])
    }
}

unsafe impl RefCounted for nsIServiceWorkerRegistrationInfoListener {
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
pub trait nsIServiceWorkerRegistrationInfoListenerCoerce {
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self;
}

impl nsIServiceWorkerRegistrationInfoListenerCoerce for nsIServiceWorkerRegistrationInfoListener {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self {
        v
    }
}

impl nsIServiceWorkerRegistrationInfoListener {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerRegistrationInfoListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerRegistrationInfoListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerRegistrationInfoListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onChange (); */
    pub onChange: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfoListener) -> nsresult,

}


impl nsIServiceWorkerRegistrationInfoListener {
    /* void onChange (); */
    #[inline]
    pub unsafe fn onChange(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onChange)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfo {
    vtable: *const nsIServiceWorkerRegistrationInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerRegistrationInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xddbc1fd4, 0x2f2e, 0x4fca,
            [0xa3, 0x95, 0x6e, 0x01, 0x0b, 0xbe, 0xdf, 0xe3])
    }
}

unsafe impl RefCounted for nsIServiceWorkerRegistrationInfo {
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
pub trait nsIServiceWorkerRegistrationInfoCoerce {
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self;
}

impl nsIServiceWorkerRegistrationInfoCoerce for nsIServiceWorkerRegistrationInfo {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self {
        v
    }
}

impl nsIServiceWorkerRegistrationInfo {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerRegistrationInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerRegistrationInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerRegistrationInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub get_principal: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* readonly attribute DOMString scope; */
    pub get_scope: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aScope: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString scriptSpec; */
    pub get_scriptSpec: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aScriptSpec: *mut nsAString) -> nsresult,

    /* readonly attribute PRTime lastUpdateTime; */
    pub get_lastUpdateTime: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aLastUpdateTime: *mut PRTime) -> nsresult,

    /* readonly attribute nsIServiceWorkerInfo installingWorker; */
    pub get_installingWorker: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aInstallingWorker: *mut *const nsIServiceWorkerInfo) -> nsresult,

    /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
    pub get_waitingWorker: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aWaitingWorker: *mut *const nsIServiceWorkerInfo) -> nsresult,

    /* readonly attribute nsIServiceWorkerInfo activeWorker; */
    pub get_activeWorker: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aActiveWorker: *mut *const nsIServiceWorkerInfo) -> nsresult,

    /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
    pub getWorkerByID: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, aID: libc::uint64_t, _retval: *mut *const nsIServiceWorkerInfo) -> nsresult,

    /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, listener: *const nsIServiceWorkerRegistrationInfoListener) -> nsresult,

    /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIServiceWorkerRegistrationInfo, listener: *const nsIServiceWorkerRegistrationInfoListener) -> nsresult,

}


impl nsIServiceWorkerRegistrationInfo {
    /* readonly attribute nsIPrincipal principal; */
    #[inline]
    pub unsafe fn get_principal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString scope; */
    #[inline]
    pub unsafe fn get_scope(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scope)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString scriptSpec; */
    #[inline]
    pub unsafe fn get_scriptSpec(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scriptSpec)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime lastUpdateTime; */
    #[inline]
    pub unsafe fn get_lastUpdateTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_lastUpdateTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIServiceWorkerInfo installingWorker; */
    #[inline]
    pub unsafe fn get_installingWorker(&self, ) -> Result<Option<RefPtr<nsIServiceWorkerInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_installingWorker)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
    #[inline]
    pub unsafe fn get_waitingWorker(&self, ) -> Result<Option<RefPtr<nsIServiceWorkerInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_waitingWorker)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIServiceWorkerInfo activeWorker; */
    #[inline]
    pub unsafe fn get_activeWorker(&self, ) -> Result<Option<RefPtr<nsIServiceWorkerInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeWorker)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
    #[inline]
    pub unsafe fn getWorkerByID(&self, aID: libc::uint64_t) -> Result<Option<RefPtr<nsIServiceWorkerInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWorkerByID)(self as *const _, aID, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    #[inline]
    pub unsafe fn addListener(&self, listener: Option<&nsIServiceWorkerRegistrationInfoListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    #[inline]
    pub unsafe fn removeListener(&self, listener: Option<&nsIServiceWorkerRegistrationInfoListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIServiceWorkerManagerListener {
    vtable: *const nsIServiceWorkerManagerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerManagerListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9e523e7c, 0xad6f, 0x4df0,
            [0x80, 0x77, 0xc7, 0x4a, 0xeb, 0xbc, 0x67, 0x9d])
    }
}

unsafe impl RefCounted for nsIServiceWorkerManagerListener {
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
pub trait nsIServiceWorkerManagerListenerCoerce {
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self;
}

impl nsIServiceWorkerManagerListenerCoerce for nsIServiceWorkerManagerListener {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self {
        v
    }
}

impl nsIServiceWorkerManagerListener {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerManagerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerManagerListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerManagerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerManagerListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
    pub onRegister: unsafe extern "C" fn (this: *const nsIServiceWorkerManagerListener, aInfo: *const nsIServiceWorkerRegistrationInfo) -> nsresult,

    /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
    pub onUnregister: unsafe extern "C" fn (this: *const nsIServiceWorkerManagerListener, aInfo: *const nsIServiceWorkerRegistrationInfo) -> nsresult,

}


impl nsIServiceWorkerManagerListener {
    /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
    #[inline]
    pub unsafe fn onRegister(&self, aInfo: Option<&nsIServiceWorkerRegistrationInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onRegister)(self as *const _, aInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
    #[inline]
    pub unsafe fn onUnregister(&self, aInfo: Option<&nsIServiceWorkerRegistrationInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onUnregister)(self as *const _, aInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIServiceWorkerManager {
    vtable: *const nsIServiceWorkerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServiceWorkerManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7404c8e8, 0x4d47, 0x4449,
            [0x8e, 0xd1, 0x47, 0xd1, 0x26, 0x1d, 0x4e, 0x33])
    }
}

unsafe impl RefCounted for nsIServiceWorkerManager {
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
pub trait nsIServiceWorkerManagerCoerce {
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self;
}

impl nsIServiceWorkerManagerCoerce for nsIServiceWorkerManager {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self {
        v
    }
}

impl nsIServiceWorkerManager {
    #[inline]
    pub fn coerce<T: nsIServiceWorkerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServiceWorkerManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServiceWorkerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServiceWorkerManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsISupports register (in mozIDOMWindow aWindow, in nsIURI aScope, in nsIURI aScriptURI, in nsLoadFlags aLoadFlags); */
    pub register: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindow, aScope: *const nsIURI, aScriptURI: *const nsIURI, aLoadFlags: nsLoadFlags, _retval: *mut *const nsISupports) -> nsresult,

    /* void unregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in DOMString aScope); */
    pub unregister: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const nsAString) -> nsresult,

    /* nsISupports getRegistrations (in mozIDOMWindow aWindow); */
    pub getRegistrations: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindow, _retval: *mut *const nsISupports) -> nsresult,

    /* nsISupports getRegistration (in mozIDOMWindow aWindow, in DOMString aScope); */
    pub getRegistration: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindow, aScope: *const nsAString, _retval: *mut *const nsISupports) -> nsresult,

    /* nsISupports getReadyPromise (in mozIDOMWindow aWindow); */
    pub getReadyPromise: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindow, _retval: *mut *const nsISupports) -> nsresult,

    /* void removeReadyPromise (in mozIDOMWindow aWindow); */
    pub removeReadyPromise: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindow) -> nsresult,

    /* nsIServiceWorkerRegistrationInfo getRegistrationByPrincipal (in nsIPrincipal aPrincipal, in DOMString aScope); */
    pub getRegistrationByPrincipal: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aScope: *const nsAString, _retval: *mut *const nsIServiceWorkerRegistrationInfo) -> nsresult,

    /* [nostdcall,notxpcom] void MaybeStartControlling (in nsIDocument aDoc, in DOMString aDocumentId); */
    pub MaybeStartControlling: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aDoc: *const nsIDocument, aDocumentId: *const nsAString) -> libc::c_void,

    /* [nostdcall,notxpcom] void MaybeStopControlling (in nsIDocument aDoc); */
    pub MaybeStopControlling: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aDoc: *const nsIDocument) -> libc::c_void,

    /* [noscript] nsISupports GetInstalling (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    pub GetInstalling: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const nsPIDOMWindowInner, aScope: *const nsAString, _retval: *mut *const nsISupports) -> nsresult,

    /* [noscript] nsISupports GetWaiting (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    pub GetWaiting: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const nsPIDOMWindowInner, aScope: *const nsAString, _retval: *mut *const nsISupports) -> nsresult,

    /* [noscript] nsISupports GetActive (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    pub GetActive: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const nsPIDOMWindowInner, aScope: *const nsAString, _retval: *mut *const nsISupports) -> nsresult,

    /* [noscript] nsISupports GetDocumentController (in nsPIDOMWindowInner aWindow); */
    pub GetDocumentController: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const nsPIDOMWindowInner, _retval: *mut *const nsISupports) -> nsresult,

    /* void removeAndPropagate (in AUTF8String aHost); */
    pub removeAndPropagate: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aHost: *const nsACString) -> nsresult,

    /* DOMString getScopeForUrl (in nsIPrincipal aPrincipal, in DOMString aPath); */
    pub getScopeForUrl: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aPath: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* nsIArray getAllRegistrations (); */
    pub getAllRegistrations: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, _retval: *mut *const nsIArray) -> nsresult,

    /* [implicit_jscontext] void propagateSoftUpdate (in jsval aOriginAttributes, in DOMString aScope); */
    /// Unable to call function as its signature contains a non-rust type
    pub propagateSoftUpdate: *const ::libc::c_void,

    /* void propagateUnregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in DOMString aScope); */
    pub propagateUnregister: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const nsAString) -> nsresult,

    /* void sendNotificationClickEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    pub sendNotificationClickEvent: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aOriginSuffix: *const nsACString, scope: *const nsACString, aID: *const nsAString, aTitle: *const nsAString, aDir: *const nsAString, aLang: *const nsAString, aBody: *const nsAString, aTag: *const nsAString, aIcon: *const nsAString, aData: *const nsAString, aBehavior: *const nsAString) -> nsresult,

    /* void sendNotificationCloseEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    pub sendNotificationCloseEvent: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aOriginSuffix: *const nsACString, scope: *const nsACString, aID: *const nsAString, aTitle: *const nsAString, aDir: *const nsAString, aLang: *const nsAString, aBody: *const nsAString, aTag: *const nsAString, aIcon: *const nsAString, aData: *const nsAString, aBehavior: *const nsAString) -> nsresult,

    /* [optional_argc] void sendPushEvent (in ACString aOriginAttributes, in ACString aScope, [optional] in uint32_t aDataLength, [array, size_is (aDataLength), optional] in uint8_t aDataBytes); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendPushEvent: *const ::libc::c_void,

    /* void sendPushSubscriptionChangeEvent (in ACString aOriginAttributes, in ACString scope); */
    pub sendPushSubscriptionChangeEvent: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aOriginAttributes: *const nsACString, scope: *const nsACString) -> nsresult,

    /* void addListener (in nsIServiceWorkerManagerListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aListener: *const nsIServiceWorkerManagerListener) -> nsresult,

    /* void removeListener (in nsIServiceWorkerManagerListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aListener: *const nsIServiceWorkerManagerListener) -> nsresult,

    /* bool shouldReportToWindow (in mozIDOMWindowProxy aWindow, in ACString aScope); */
    pub shouldReportToWindow: unsafe extern "C" fn (this: *const nsIServiceWorkerManager, aWindow: *const mozIDOMWindowProxy, aScope: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl nsIServiceWorkerManager {
    /* nsISupports register (in mozIDOMWindow aWindow, in nsIURI aScope, in nsIURI aScriptURI, in nsLoadFlags aLoadFlags); */
    #[inline]
    pub unsafe fn register(&self, aWindow: Option<&mozIDOMWindow>, aScope: Option<&nsIURI>, aScriptURI: Option<&nsIURI>, aLoadFlags: nsLoadFlags) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).register)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aScope.map_or(::std::ptr::null(), |x| x as *const _), aScriptURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void unregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in DOMString aScope); */
    #[inline]
    pub unsafe fn unregister(&self, aPrincipal: Option<&nsIPrincipal>, aCallback: Option<&nsIServiceWorkerUnregisterCallback>, aScope: &[u16]) -> Result<(), nsresult> {
        let aScope = nsString::from(aScope);
        match ((*self.vtable).unregister)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), &*aScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISupports getRegistrations (in mozIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn getRegistrations(&self, aWindow: Option<&mozIDOMWindow>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRegistrations)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISupports getRegistration (in mozIDOMWindow aWindow, in DOMString aScope); */
    #[inline]
    pub unsafe fn getRegistration(&self, aWindow: Option<&mozIDOMWindow>, aScope: &[u16]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let aScope = nsString::from(aScope);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRegistration)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISupports getReadyPromise (in mozIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn getReadyPromise(&self, aWindow: Option<&mozIDOMWindow>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getReadyPromise)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeReadyPromise (in mozIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn removeReadyPromise(&self, aWindow: Option<&mozIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).removeReadyPromise)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIServiceWorkerRegistrationInfo getRegistrationByPrincipal (in nsIPrincipal aPrincipal, in DOMString aScope); */
    #[inline]
    pub unsafe fn getRegistrationByPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, aScope: &[u16]) -> Result<Option<RefPtr<nsIServiceWorkerRegistrationInfo>>, nsresult> {
        let aScope = nsString::from(aScope);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRegistrationByPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [nostdcall,notxpcom] void MaybeStartControlling (in nsIDocument aDoc, in DOMString aDocumentId); */
    #[inline]
    pub unsafe fn MaybeStartControlling(&self, aDoc: Option<&nsIDocument>, aDocumentId: &[u16]) -> () {
        let aDocumentId = nsString::from(aDocumentId);
        let _retval = ((*self.vtable).MaybeStartControlling)(self as *const _, aDoc.map_or(::std::ptr::null(), |x| x as *const _), &*aDocumentId);
        ()
    }

    /* [nostdcall,notxpcom] void MaybeStopControlling (in nsIDocument aDoc); */
    #[inline]
    pub unsafe fn MaybeStopControlling(&self, aDoc: Option<&nsIDocument>) -> () {

        let _retval = ((*self.vtable).MaybeStopControlling)(self as *const _, aDoc.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

    /* [noscript] nsISupports GetInstalling (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    #[inline]
    pub unsafe fn GetInstalling(&self, aWindow: Option<&nsPIDOMWindowInner>, aScope: &[u16]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let aScope = nsString::from(aScope);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetInstalling)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsISupports GetWaiting (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    #[inline]
    pub unsafe fn GetWaiting(&self, aWindow: Option<&nsPIDOMWindowInner>, aScope: &[u16]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let aScope = nsString::from(aScope);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetWaiting)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsISupports GetActive (in nsPIDOMWindowInner aWindow, in DOMString aScope); */
    #[inline]
    pub unsafe fn GetActive(&self, aWindow: Option<&nsPIDOMWindowInner>, aScope: &[u16]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let aScope = nsString::from(aScope);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetActive)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsISupports GetDocumentController (in nsPIDOMWindowInner aWindow); */
    #[inline]
    pub unsafe fn GetDocumentController(&self, aWindow: Option<&nsPIDOMWindowInner>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetDocumentController)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeAndPropagate (in AUTF8String aHost); */
    #[inline]
    pub unsafe fn removeAndPropagate(&self, aHost: &[u8]) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).removeAndPropagate)(self as *const _, &*aHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString getScopeForUrl (in nsIPrincipal aPrincipal, in DOMString aPath); */
    #[inline]
    pub unsafe fn getScopeForUrl(&self, aPrincipal: Option<&nsIPrincipal>, aPath: &[u16]) -> Result<nsString, nsresult> {
        let aPath = nsString::from(aPath);
        let mut _retval = nsString::new();
        match ((*self.vtable).getScopeForUrl)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aPath, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIArray getAllRegistrations (); */
    #[inline]
    pub unsafe fn getAllRegistrations(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAllRegistrations)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] void propagateSoftUpdate (in jsval aOriginAttributes, in DOMString aScope); */


    /* void propagateUnregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in DOMString aScope); */
    #[inline]
    pub unsafe fn propagateUnregister(&self, aPrincipal: Option<&nsIPrincipal>, aCallback: Option<&nsIServiceWorkerUnregisterCallback>, aScope: &[u16]) -> Result<(), nsresult> {
        let aScope = nsString::from(aScope);
        match ((*self.vtable).propagateUnregister)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), &*aScope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNotificationClickEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    #[inline]
    pub unsafe fn sendNotificationClickEvent(&self, aOriginSuffix: &[u8], scope: &[u8], aID: &[u16], aTitle: &[u16], aDir: &[u16], aLang: &[u16], aBody: &[u16], aTag: &[u16], aIcon: &[u16], aData: &[u16], aBehavior: &[u16]) -> Result<(), nsresult> {
        let aOriginSuffix = nsCString::from(aOriginSuffix);
        let scope = nsCString::from(scope);
        let aID = nsString::from(aID);
        let aTitle = nsString::from(aTitle);
        let aDir = nsString::from(aDir);
        let aLang = nsString::from(aLang);
        let aBody = nsString::from(aBody);
        let aTag = nsString::from(aTag);
        let aIcon = nsString::from(aIcon);
        let aData = nsString::from(aData);
        let aBehavior = nsString::from(aBehavior);
        match ((*self.vtable).sendNotificationClickEvent)(self as *const _, &*aOriginSuffix, &*scope, &*aID, &*aTitle, &*aDir, &*aLang, &*aBody, &*aTag, &*aIcon, &*aData, &*aBehavior) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendNotificationCloseEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    #[inline]
    pub unsafe fn sendNotificationCloseEvent(&self, aOriginSuffix: &[u8], scope: &[u8], aID: &[u16], aTitle: &[u16], aDir: &[u16], aLang: &[u16], aBody: &[u16], aTag: &[u16], aIcon: &[u16], aData: &[u16], aBehavior: &[u16]) -> Result<(), nsresult> {
        let aOriginSuffix = nsCString::from(aOriginSuffix);
        let scope = nsCString::from(scope);
        let aID = nsString::from(aID);
        let aTitle = nsString::from(aTitle);
        let aDir = nsString::from(aDir);
        let aLang = nsString::from(aLang);
        let aBody = nsString::from(aBody);
        let aTag = nsString::from(aTag);
        let aIcon = nsString::from(aIcon);
        let aData = nsString::from(aData);
        let aBehavior = nsString::from(aBehavior);
        match ((*self.vtable).sendNotificationCloseEvent)(self as *const _, &*aOriginSuffix, &*scope, &*aID, &*aTitle, &*aDir, &*aLang, &*aBody, &*aTag, &*aIcon, &*aData, &*aBehavior) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [optional_argc] void sendPushEvent (in ACString aOriginAttributes, in ACString aScope, [optional] in uint32_t aDataLength, [array, size_is (aDataLength), optional] in uint8_t aDataBytes); */


    /* void sendPushSubscriptionChangeEvent (in ACString aOriginAttributes, in ACString scope); */
    #[inline]
    pub unsafe fn sendPushSubscriptionChangeEvent(&self, aOriginAttributes: &[u8], scope: &[u8]) -> Result<(), nsresult> {
        let aOriginAttributes = nsCString::from(aOriginAttributes);
        let scope = nsCString::from(scope);
        match ((*self.vtable).sendPushSubscriptionChangeEvent)(self as *const _, &*aOriginAttributes, &*scope) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addListener (in nsIServiceWorkerManagerListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aListener: Option<&nsIServiceWorkerManagerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIServiceWorkerManagerListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aListener: Option<&nsIServiceWorkerManagerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool shouldReportToWindow (in mozIDOMWindowProxy aWindow, in ACString aScope); */
    #[inline]
    pub unsafe fn shouldReportToWindow(&self, aWindow: Option<&mozIDOMWindowProxy>, aScope: &[u8]) -> Result<bool, nsresult> {
        let aScope = nsCString::from(aScope);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldReportToWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), &*aScope, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


