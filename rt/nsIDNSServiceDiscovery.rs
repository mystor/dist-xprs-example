//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSServiceDiscovery.idl
//


#[repr(C)]
pub struct nsIDNSServiceInfo {
    vtable: *const nsIDNSServiceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSServiceInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x670ed0f9, 0x2fa5, 0x4544,
            [0xbf, 0x1e, 0xea, 0x58, 0xac, 0x17, 0x93, 0x74])
    }
}

unsafe impl RefCounted for nsIDNSServiceInfo {
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
pub trait nsIDNSServiceInfoCoerce {
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self;
}

impl nsIDNSServiceInfoCoerce for nsIDNSServiceInfo {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self {
        v
    }
}

impl nsIDNSServiceInfo {
    #[inline]
    pub fn coerce<T: nsIDNSServiceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSServiceInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSServiceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSServiceInfoVTable {
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aHost: *mut nsACString) -> nsresult,
    pub set_host: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aHost: *const nsACString) -> nsresult,

    /* attribute AUTF8String address; */
    pub get_address: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aAddress: *mut nsACString) -> nsresult,
    pub set_address: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aAddress: *const nsACString) -> nsresult,

    /* attribute unsigned short port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aPort: *mut libc::uint16_t) -> nsresult,
    pub set_port: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aPort: libc::uint16_t) -> nsresult,

    /* attribute AUTF8String serviceName; */
    pub get_serviceName: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aServiceName: *mut nsACString) -> nsresult,
    pub set_serviceName: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aServiceName: *const nsACString) -> nsresult,

    /* attribute AUTF8String serviceType; */
    pub get_serviceType: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aServiceType: *mut nsACString) -> nsresult,
    pub set_serviceType: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aServiceType: *const nsACString) -> nsresult,

    /* attribute AUTF8String domainName; */
    pub get_domainName: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aDomainName: *mut nsACString) -> nsresult,
    pub set_domainName: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aDomainName: *const nsACString) -> nsresult,

    /* attribute nsIPropertyBag2 attributes; */
    pub get_attributes: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aAttributes: *mut *const nsIPropertyBag2) -> nsresult,
    pub set_attributes: unsafe extern "C" fn (this: *const nsIDNSServiceInfo, aAttributes: *const nsIPropertyBag2) -> nsresult,

}


impl nsIDNSServiceInfo {
    /* attribute AUTF8String host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_host(&self, aHost: &[u8]) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).set_host)(self as *const _, &*aHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String address; */
    #[inline]
    pub unsafe fn get_address(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_address)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_address(&self, aAddress: &[u8]) -> Result<(), nsresult> {
        let aAddress = nsCString::from(aAddress);
        match ((*self.vtable).set_address)(self as *const _, &*aAddress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned short port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_port(&self, aPort: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_port)(self as *const _, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String serviceName; */
    #[inline]
    pub unsafe fn get_serviceName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_serviceName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_serviceName(&self, aServiceName: &[u8]) -> Result<(), nsresult> {
        let aServiceName = nsCString::from(aServiceName);
        match ((*self.vtable).set_serviceName)(self as *const _, &*aServiceName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String serviceType; */
    #[inline]
    pub unsafe fn get_serviceType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_serviceType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_serviceType(&self, aServiceType: &[u8]) -> Result<(), nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        match ((*self.vtable).set_serviceType)(self as *const _, &*aServiceType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String domainName; */
    #[inline]
    pub unsafe fn get_domainName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_domainName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_domainName(&self, aDomainName: &[u8]) -> Result<(), nsresult> {
        let aDomainName = nsCString::from(aDomainName);
        match ((*self.vtable).set_domainName)(self as *const _, &*aDomainName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIPropertyBag2 attributes; */
    #[inline]
    pub unsafe fn get_attributes(&self, ) -> Result<Option<RefPtr<nsIPropertyBag2>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_attributes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_attributes(&self, aAttributes: Option<&nsIPropertyBag2>) -> Result<(), nsresult> {

        match ((*self.vtable).set_attributes)(self as *const _, aAttributes.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDNSServiceDiscoveryListener {
    vtable: *const nsIDNSServiceDiscoveryListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSServiceDiscoveryListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3025b7f2, 0x97bb, 0x435b,
            [0xb4, 0x3d, 0x26, 0x73, 0x1b, 0x3f, 0x5f, 0xc4])
    }
}

unsafe impl RefCounted for nsIDNSServiceDiscoveryListener {
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
pub trait nsIDNSServiceDiscoveryListenerCoerce {
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self;
}

impl nsIDNSServiceDiscoveryListenerCoerce for nsIDNSServiceDiscoveryListener {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self {
        v
    }
}

impl nsIDNSServiceDiscoveryListener {
    #[inline]
    pub fn coerce<T: nsIDNSServiceDiscoveryListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSServiceDiscoveryListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSServiceDiscoveryListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscoveryListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSServiceDiscoveryListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onDiscoveryStarted (in AUTF8String aServiceType); */
    pub onDiscoveryStarted: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const nsACString) -> nsresult,

    /* void onDiscoveryStopped (in AUTF8String aServiceType); */
    pub onDiscoveryStopped: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const nsACString) -> nsresult,

    /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
    pub onServiceFound: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceInfo: *const nsIDNSServiceInfo) -> nsresult,

    /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
    pub onServiceLost: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceInfo: *const nsIDNSServiceInfo) -> nsresult,

    /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    pub onStartDiscoveryFailed: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const nsACString, aErrorCode: libc::int32_t) -> nsresult,

    /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    pub onStopDiscoveryFailed: unsafe extern "C" fn (this: *const nsIDNSServiceDiscoveryListener, aServiceType: *const nsACString, aErrorCode: libc::int32_t) -> nsresult,

}


impl nsIDNSServiceDiscoveryListener {
    /* void onDiscoveryStarted (in AUTF8String aServiceType); */
    #[inline]
    pub unsafe fn onDiscoveryStarted(&self, aServiceType: &[u8]) -> Result<(), nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        match ((*self.vtable).onDiscoveryStarted)(self as *const _, &*aServiceType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDiscoveryStopped (in AUTF8String aServiceType); */
    #[inline]
    pub unsafe fn onDiscoveryStopped(&self, aServiceType: &[u8]) -> Result<(), nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        match ((*self.vtable).onDiscoveryStopped)(self as *const _, &*aServiceType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onServiceFound (in nsIDNSServiceInfo aServiceInfo); */
    #[inline]
    pub unsafe fn onServiceFound(&self, aServiceInfo: Option<&nsIDNSServiceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onServiceFound)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onServiceLost (in nsIDNSServiceInfo aServiceInfo); */
    #[inline]
    pub unsafe fn onServiceLost(&self, aServiceInfo: Option<&nsIDNSServiceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onServiceLost)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStartDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    #[inline]
    pub unsafe fn onStartDiscoveryFailed(&self, aServiceType: &[u8], aErrorCode: libc::int32_t) -> Result<(), nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        match ((*self.vtable).onStartDiscoveryFailed)(self as *const _, &*aServiceType, aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStopDiscoveryFailed (in AUTF8String aServiceType, in long aErrorCode); */
    #[inline]
    pub unsafe fn onStopDiscoveryFailed(&self, aServiceType: &[u8], aErrorCode: libc::int32_t) -> Result<(), nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        match ((*self.vtable).onStopDiscoveryFailed)(self as *const _, &*aServiceType, aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIDNSRegistrationListener_consts {
    pub const ERROR_SERVICE_NOT_RUNNING: i64 = -65563;
}


#[repr(C)]
pub struct nsIDNSRegistrationListener {
    vtable: *const nsIDNSRegistrationListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSRegistrationListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe165e4be, 0xabf4, 0x4963,
            [0xa6, 0x6d, 0xed, 0x3c, 0xa1, 0x16, 0xe5, 0xe4])
    }
}

unsafe impl RefCounted for nsIDNSRegistrationListener {
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
pub trait nsIDNSRegistrationListenerCoerce {
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self;
}

impl nsIDNSRegistrationListenerCoerce for nsIDNSRegistrationListener {
    #[inline]
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self {
        v
    }
}

impl nsIDNSRegistrationListener {
    #[inline]
    pub fn coerce<T: nsIDNSRegistrationListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSRegistrationListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSRegistrationListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSRegistrationListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSRegistrationListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
    pub onServiceRegistered: unsafe extern "C" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo) -> nsresult,

    /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
    pub onServiceUnregistered: unsafe extern "C" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo) -> nsresult,

    /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub onRegistrationFailed: unsafe extern "C" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: libc::int32_t) -> nsresult,

    /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub onUnregistrationFailed: unsafe extern "C" fn (this: *const nsIDNSRegistrationListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: libc::int32_t) -> nsresult,

}


impl nsIDNSRegistrationListener {
    /* void onServiceRegistered (in nsIDNSServiceInfo aServiceInfo); */
    #[inline]
    pub unsafe fn onServiceRegistered(&self, aServiceInfo: Option<&nsIDNSServiceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onServiceRegistered)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onServiceUnregistered (in nsIDNSServiceInfo aServiceInfo); */
    #[inline]
    pub unsafe fn onServiceUnregistered(&self, aServiceInfo: Option<&nsIDNSServiceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onServiceUnregistered)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onRegistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    #[inline]
    pub unsafe fn onRegistrationFailed(&self, aServiceInfo: Option<&nsIDNSServiceInfo>, aErrorCode: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onRegistrationFailed)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _), aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onUnregistrationFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    #[inline]
    pub unsafe fn onUnregistrationFailed(&self, aServiceInfo: Option<&nsIDNSServiceInfo>, aErrorCode: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onUnregistrationFailed)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _), aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDNSServiceResolveListener {
    vtable: *const nsIDNSServiceResolveListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSServiceResolveListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24ee6408, 0x648e, 0x421d,
            [0xac, 0xcf, 0xc6, 0xe5, 0xad, 0xec, 0xcf, 0x97])
    }
}

unsafe impl RefCounted for nsIDNSServiceResolveListener {
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
pub trait nsIDNSServiceResolveListenerCoerce {
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self;
}

impl nsIDNSServiceResolveListenerCoerce for nsIDNSServiceResolveListener {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self {
        v
    }
}

impl nsIDNSServiceResolveListener {
    #[inline]
    pub fn coerce<T: nsIDNSServiceResolveListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSServiceResolveListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSServiceResolveListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceResolveListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSServiceResolveListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
    pub onServiceResolved: unsafe extern "C" fn (this: *const nsIDNSServiceResolveListener, aServiceInfo: *const nsIDNSServiceInfo) -> nsresult,

    /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    pub onResolveFailed: unsafe extern "C" fn (this: *const nsIDNSServiceResolveListener, aServiceInfo: *const nsIDNSServiceInfo, aErrorCode: libc::int32_t) -> nsresult,

}


impl nsIDNSServiceResolveListener {
    /* void onServiceResolved (in nsIDNSServiceInfo aServiceInfo); */
    #[inline]
    pub unsafe fn onServiceResolved(&self, aServiceInfo: Option<&nsIDNSServiceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).onServiceResolved)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onResolveFailed (in nsIDNSServiceInfo aServiceInfo, in long aErrorCode); */
    #[inline]
    pub unsafe fn onResolveFailed(&self, aServiceInfo: Option<&nsIDNSServiceInfo>, aErrorCode: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onResolveFailed)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _), aErrorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDNSServiceDiscovery {
    vtable: *const nsIDNSServiceDiscoveryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSServiceDiscovery {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6487899b, 0xbeb1, 0x455a,
            [0xba, 0x65, 0xe4, 0xfd, 0x46, 0x50, 0x66, 0xd7])
    }
}

unsafe impl RefCounted for nsIDNSServiceDiscovery {
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
pub trait nsIDNSServiceDiscoveryCoerce {
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self;
}

impl nsIDNSServiceDiscoveryCoerce for nsIDNSServiceDiscovery {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self {
        v
    }
}

impl nsIDNSServiceDiscovery {
    #[inline]
    pub fn coerce<T: nsIDNSServiceDiscoveryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSServiceDiscovery {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSServiceDiscoveryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSServiceDiscovery) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSServiceDiscoveryVTable {
    pub __base: nsISupportsVTable,

    /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
    pub startDiscovery: unsafe extern "C" fn (this: *const nsIDNSServiceDiscovery, aServiceType: *const nsACString, aListener: *const nsIDNSServiceDiscoveryListener, _retval: *mut *const nsICancelable) -> nsresult,

    /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
    pub registerService: unsafe extern "C" fn (this: *const nsIDNSServiceDiscovery, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSRegistrationListener, _retval: *mut *const nsICancelable) -> nsresult,

    /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
    pub resolveService: unsafe extern "C" fn (this: *const nsIDNSServiceDiscovery, aServiceInfo: *const nsIDNSServiceInfo, aListener: *const nsIDNSServiceResolveListener) -> nsresult,

}


impl nsIDNSServiceDiscovery {
    /* nsICancelable startDiscovery (in AUTF8String aServiceType, in nsIDNSServiceDiscoveryListener aListener); */
    #[inline]
    pub unsafe fn startDiscovery(&self, aServiceType: &[u8], aListener: Option<&nsIDNSServiceDiscoveryListener>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let aServiceType = nsCString::from(aServiceType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).startDiscovery)(self as *const _, &*aServiceType, aListener.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsICancelable registerService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSRegistrationListener aListener); */
    #[inline]
    pub unsafe fn registerService(&self, aServiceInfo: Option<&nsIDNSServiceInfo>, aListener: Option<&nsIDNSRegistrationListener>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).registerService)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void resolveService (in nsIDNSServiceInfo aServiceInfo, in nsIDNSServiceResolveListener aListener); */
    #[inline]
    pub unsafe fn resolveService(&self, aServiceInfo: Option<&nsIDNSServiceInfo>, aListener: Option<&nsIDNSServiceResolveListener>) -> Result<(), nsresult> {

        match ((*self.vtable).resolveService)(self as *const _, aServiceInfo.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


