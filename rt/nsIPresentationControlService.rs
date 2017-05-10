//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationControlService.idl
//


#[repr(C)]
pub struct nsITCPDeviceInfo {
    vtable: *const nsITCPDeviceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITCPDeviceInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x296fd171, 0xe4d0, 0x4de0,
            [0x99, 0xff, 0xad, 0x8e, 0xd5, 0x2d, 0xde, 0xf3])
    }
}

unsafe impl RefCounted for nsITCPDeviceInfo {
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
pub trait nsITCPDeviceInfoCoerce {
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self;
}

impl nsITCPDeviceInfoCoerce for nsITCPDeviceInfo {
    #[inline]
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self {
        v
    }
}

impl nsITCPDeviceInfo {
    #[inline]
    pub fn coerce<T: nsITCPDeviceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITCPDeviceInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITCPDeviceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITCPDeviceInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITCPDeviceInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String id; */
    pub get_id: unsafe extern "C" fn (this: *const nsITCPDeviceInfo, aId: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String address; */
    pub get_address: unsafe extern "C" fn (this: *const nsITCPDeviceInfo, aAddress: *mut nsACString) -> nsresult,

    /* readonly attribute uint16_t port; */
    pub get_port: unsafe extern "C" fn (this: *const nsITCPDeviceInfo, aPort: *mut uint16_t) -> nsresult,

    /* readonly attribute AUTF8String certFingerprint; */
    pub get_certFingerprint: unsafe extern "C" fn (this: *const nsITCPDeviceInfo, aCertFingerprint: *mut nsACString) -> nsresult,

}


impl nsITCPDeviceInfo {
    /* readonly attribute AUTF8String id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String address; */
    #[inline]
    pub unsafe fn get_address(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_address)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint16_t port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String certFingerprint; */
    #[inline]
    pub unsafe fn get_certFingerprint(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_certFingerprint)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIPresentationControlServerListener {
    vtable: *const nsIPresentationControlServerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationControlServerListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x09bddfaf, 0xfcc2, 0x4dc9,
            [0xb3, 0x3e, 0xa5, 0x09, 0xa1, 0xc2, 0xfb, 0x6d])
    }
}

unsafe impl RefCounted for nsIPresentationControlServerListener {
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
pub trait nsIPresentationControlServerListenerCoerce {
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self;
}

impl nsIPresentationControlServerListenerCoerce for nsIPresentationControlServerListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self {
        v
    }
}

impl nsIPresentationControlServerListener {
    #[inline]
    pub fn coerce<T: nsIPresentationControlServerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationControlServerListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationControlServerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlServerListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationControlServerListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
    pub onServerReady: unsafe extern "C" fn (this: *const nsIPresentationControlServerListener, aPort: uint16_t, aCertFingerprint: *const nsACString) -> nsresult,

    /* void onServerStopped (in nsresult aResult); */
    pub onServerStopped: unsafe extern "C" fn (this: *const nsIPresentationControlServerListener, aResult: nsresult) -> nsresult,

    /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aUrl, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    pub onSessionRequest: unsafe extern "C" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, aUrl: *const nsAString, aPresentationId: *const nsAString, aControlChannel: *const nsIPresentationControlChannel) -> nsresult,

    /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
    pub onTerminateRequest: unsafe extern "C" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, aPresentationId: *const nsAString, aControlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> nsresult,

    /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString url, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    pub onReconnectRequest: unsafe extern "C" fn (this: *const nsIPresentationControlServerListener, aDeviceInfo: *const nsITCPDeviceInfo, url: *const nsAString, aPresentationId: *const nsAString, aControlChannel: *const nsIPresentationControlChannel) -> nsresult,

}


impl nsIPresentationControlServerListener {
    /* void onServerReady (in uint16_t aPort, in AUTF8String aCertFingerprint); */
    #[inline]
    pub unsafe fn onServerReady(&self, aPort: uint16_t, aCertFingerprint: &[u8]) -> Result<(), nsresult> {
        let aCertFingerprint = nsCString::from(aCertFingerprint);
        match ((*self.vtable).onServerReady)(self as *const _, aPort, &*aCertFingerprint) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onServerStopped (in nsresult aResult); */
    #[inline]
    pub unsafe fn onServerStopped(&self, aResult: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onServerStopped)(self as *const _, aResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSessionRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aUrl, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    #[inline]
    pub unsafe fn onSessionRequest(&self, aDeviceInfo: Option<&nsITCPDeviceInfo>, aUrl: &[u16], aPresentationId: &[u16], aControlChannel: Option<&nsIPresentationControlChannel>) -> Result<(), nsresult> {
        let aUrl = nsString::from(aUrl);
        let aPresentationId = nsString::from(aPresentationId);
        match ((*self.vtable).onSessionRequest)(self as *const _, aDeviceInfo.map_or(::std::ptr::null(), |x| x as *const _), &*aUrl, &*aPresentationId, aControlChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onTerminateRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel, in boolean aIsFromReceiver); */
    #[inline]
    pub unsafe fn onTerminateRequest(&self, aDeviceInfo: Option<&nsITCPDeviceInfo>, aPresentationId: &[u16], aControlChannel: Option<&nsIPresentationControlChannel>, aIsFromReceiver: bool) -> Result<(), nsresult> {
        let aPresentationId = nsString::from(aPresentationId);
        match ((*self.vtable).onTerminateRequest)(self as *const _, aDeviceInfo.map_or(::std::ptr::null(), |x| x as *const _), &*aPresentationId, aControlChannel.map_or(::std::ptr::null(), |x| x as *const _), aIsFromReceiver) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onReconnectRequest (in nsITCPDeviceInfo aDeviceInfo, in DOMString url, in DOMString aPresentationId, in nsIPresentationControlChannel aControlChannel); */
    #[inline]
    pub unsafe fn onReconnectRequest(&self, aDeviceInfo: Option<&nsITCPDeviceInfo>, url: &[u16], aPresentationId: &[u16], aControlChannel: Option<&nsIPresentationControlChannel>) -> Result<(), nsresult> {
        let url = nsString::from(url);
        let aPresentationId = nsString::from(aPresentationId);
        match ((*self.vtable).onReconnectRequest)(self as *const _, aDeviceInfo.map_or(::std::ptr::null(), |x| x as *const _), &*url, &*aPresentationId, aControlChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationControlService {
    vtable: *const nsIPresentationControlServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationControlService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55d6b605, 0x2389, 0x4aae,
            [0xa8, 0xfe, 0x60, 0xd4, 0x44, 0x05, 0x40, 0xea])
    }
}

unsafe impl RefCounted for nsIPresentationControlService {
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
pub trait nsIPresentationControlServiceCoerce {
    fn coerce_from(v: &nsIPresentationControlService) -> &Self;
}

impl nsIPresentationControlServiceCoerce for nsIPresentationControlService {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlService) -> &Self {
        v
    }
}

impl nsIPresentationControlService {
    #[inline]
    pub fn coerce<T: nsIPresentationControlServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationControlService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationControlServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationControlServiceVTable {
    pub __base: nsISupportsVTable,

    /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
    pub startServer: unsafe extern "C" fn (this: *const nsIPresentationControlService, aEncrypted: bool, aPort: uint16_t) -> nsresult,

    /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
    pub connect: unsafe extern "C" fn (this: *const nsIPresentationControlService, aDeviceInfo: *const nsITCPDeviceInfo, _retval: *mut *const nsIPresentationControlChannel) -> nsresult,

    /* boolean isCompatibleServer (in uint32_t aVersion); */
    pub isCompatibleServer: unsafe extern "C" fn (this: *const nsIPresentationControlService, aVersion: uint32_t, _retval: *mut bool) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIPresentationControlService) -> nsresult,

    /* readonly attribute uint16_t port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIPresentationControlService, aPort: *mut uint16_t) -> nsresult,

    /* readonly attribute uint32_t version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIPresentationControlService, aVersion: *mut uint32_t) -> nsresult,

    /* attribute AUTF8String id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIPresentationControlService, aId: *mut nsACString) -> nsresult,
    pub set_id: unsafe extern "C" fn (this: *const nsIPresentationControlService, aId: *const nsACString) -> nsresult,

    /* attribute AUTF8String certFingerprint; */
    pub get_certFingerprint: unsafe extern "C" fn (this: *const nsIPresentationControlService, aCertFingerprint: *mut nsACString) -> nsresult,
    pub set_certFingerprint: unsafe extern "C" fn (this: *const nsIPresentationControlService, aCertFingerprint: *const nsACString) -> nsresult,

    /* attribute nsIPresentationControlServerListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIPresentationControlService, aListener: *mut *const nsIPresentationControlServerListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIPresentationControlService, aListener: *const nsIPresentationControlServerListener) -> nsresult,

}


impl nsIPresentationControlService {
    /* void startServer (in boolean aEncrypted, [optional] in uint16_t aPort); */
    #[inline]
    pub unsafe fn startServer(&self, aEncrypted: bool, aPort: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).startServer)(self as *const _, aEncrypted, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPresentationControlChannel connect (in nsITCPDeviceInfo aDeviceInfo); */
    #[inline]
    pub unsafe fn connect(&self, aDeviceInfo: Option<&nsITCPDeviceInfo>) -> Result<Option<RefPtr<nsIPresentationControlChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).connect)(self as *const _, aDeviceInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isCompatibleServer (in uint32_t aVersion); */
    #[inline]
    pub unsafe fn isCompatibleServer(&self, aVersion: uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCompatibleServer)(self as *const _, aVersion, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* readonly attribute uint16_t port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_version)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AUTF8String id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_id(&self, aId: &[u8]) -> Result<(), nsresult> {
        let aId = nsCString::from(aId);
        match ((*self.vtable).set_id)(self as *const _, &*aId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String certFingerprint; */
    #[inline]
    pub unsafe fn get_certFingerprint(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_certFingerprint)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_certFingerprint(&self, aCertFingerprint: &[u8]) -> Result<(), nsresult> {
        let aCertFingerprint = nsCString::from(aCertFingerprint);
        match ((*self.vtable).set_certFingerprint)(self as *const _, &*aCertFingerprint) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIPresentationControlServerListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIPresentationControlServerListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIPresentationControlServerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


