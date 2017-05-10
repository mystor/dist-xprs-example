//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationService.idl
//


#[repr(C)]
pub struct nsIPresentationServiceCallback {
    vtable: *const nsIPresentationServiceCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationServiceCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x12073206, 0x0065, 0x4b10,
            [0x94, 0x88, 0xa6, 0xeb, 0x9b, 0x23, 0xe6, 0x5b])
    }
}

unsafe impl RefCounted for nsIPresentationServiceCallback {
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
pub trait nsIPresentationServiceCallbackCoerce {
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self;
}

impl nsIPresentationServiceCallbackCoerce for nsIPresentationServiceCallback {
    #[inline]
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self {
        v
    }
}

impl nsIPresentationServiceCallback {
    #[inline]
    pub fn coerce<T: nsIPresentationServiceCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationServiceCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationServiceCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationServiceCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationServiceCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void notifySuccess (in DOMString url); */
    pub notifySuccess: unsafe extern "C" fn (this: *const nsIPresentationServiceCallback, url: *const nsAString) -> nsresult,

    /* void notifyError (in nsresult error); */
    pub notifyError: unsafe extern "C" fn (this: *const nsIPresentationServiceCallback, error: nsresult) -> nsresult,

}


impl nsIPresentationServiceCallback {
    /* void notifySuccess (in DOMString url); */
    #[inline]
    pub unsafe fn notifySuccess(&self, url: &[u16]) -> Result<(), nsresult> {
        let url = nsString::from(url);
        match ((*self.vtable).notifySuccess)(self as *const _, &*url) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyError (in nsresult error); */
    #[inline]
    pub unsafe fn notifyError(&self, error: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).notifyError)(self as *const _, error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIPresentationService_consts {
    pub const ROLE_CONTROLLER: i64 = 1;
    pub const ROLE_RECEIVER: i64 = 2;
    pub const CLOSED_REASON_ERROR: i64 = 1;
    pub const CLOSED_REASON_CLOSED: i64 = 2;
    pub const CLOSED_REASON_WENTAWAY: i64 = 3;
}


#[repr(C)]
pub struct nsIPresentationService {
    vtable: *const nsIPresentationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde42b741, 0x5619, 0x4650,
            [0xb9, 0x61, 0xc2, 0xce, 0xbb, 0x57, 0x2c, 0x95])
    }
}

unsafe impl RefCounted for nsIPresentationService {
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
pub trait nsIPresentationServiceCoerce {
    fn coerce_from(v: &nsIPresentationService) -> &Self;
}

impl nsIPresentationServiceCoerce for nsIPresentationService {
    #[inline]
    fn coerce_from(v: &nsIPresentationService) -> &Self {
        v
    }
}

impl nsIPresentationService {
    #[inline]
    pub fn coerce<T: nsIPresentationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationServiceVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void startSession (in URLArrayRef urls, in DOMString sessionId, in DOMString origin, in DOMString deviceId, in unsigned long long windowId, in nsIDOMEventTarget eventTarget, in nsIPrincipal principal, in nsIPresentationServiceCallback callback, in nsIPresentationTransportBuilderConstructor constructor); */
    /// Unable to call function as its signature contains a non-rust type
    pub startSession: *const ::libc::c_void,

    /* void sendSessionMessage (in DOMString sessionId, in uint8_t role, in DOMString data); */
    pub sendSessionMessage: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, data: *const nsAString) -> nsresult,

    /* void sendSessionBinaryMsg (in DOMString sessionId, in uint8_t role, in ACString data); */
    pub sendSessionBinaryMsg: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, data: *const nsACString) -> nsresult,

    /* void sendSessionBlob (in DOMString sessionId, in uint8_t role, in nsIDOMBlob blob); */
    pub sendSessionBlob: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, blob: *const nsIDOMBlob) -> nsresult,

    /* void closeSession (in DOMString sessionId, in uint8_t role, in uint8_t closedReason); */
    pub closeSession: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, closedReason: uint8_t) -> nsresult,

    /* void terminateSession (in DOMString sessionId, in uint8_t role); */
    pub terminateSession: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t) -> nsresult,

    /* [noscript] void reconnectSession (in URLArrayRef urls, in DOMString sessionId, in uint8_t role, in nsIPresentationServiceCallback callback); */
    /// Unable to call function as its signature contains a non-rust type
    pub reconnectSession: *const ::libc::c_void,

    /* [noscript] void registerAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
    /// Unable to call function as its signature contains a non-rust type
    pub registerAvailabilityListener: *const ::libc::c_void,

    /* [noscript] void unregisterAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */
    /// Unable to call function as its signature contains a non-rust type
    pub unregisterAvailabilityListener: *const ::libc::c_void,

    /* void registerSessionListener (in DOMString sessionId, in uint8_t role, in nsIPresentationSessionListener listener); */
    pub registerSessionListener: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, listener: *const nsIPresentationSessionListener) -> nsresult,

    /* void unregisterSessionListener (in DOMString sessionId, in uint8_t role); */
    pub unregisterSessionListener: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t) -> nsresult,

    /* void registerRespondingListener (in unsigned long long windowId, in nsIPresentationRespondingListener listener); */
    pub registerRespondingListener: unsafe extern "C" fn (this: *const nsIPresentationService, windowId: libc::uint64_t, listener: *const nsIPresentationRespondingListener) -> nsresult,

    /* void unregisterRespondingListener (in unsigned long long windowId); */
    pub unregisterRespondingListener: unsafe extern "C" fn (this: *const nsIPresentationService, windowId: libc::uint64_t) -> nsresult,

    /* void notifyReceiverReady (in DOMString sessionId, in unsigned long long windowId, in boolean isLoading, in nsIPresentationTransportBuilderConstructor constructor); */
    pub notifyReceiverReady: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, windowId: libc::uint64_t, isLoading: bool, constructor: *const nsIPresentationTransportBuilderConstructor) -> nsresult,

    /* void NotifyTransportClosed (in DOMString sessionId, in uint8_t role, in nsresult reason); */
    pub NotifyTransportClosed: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, reason: nsresult) -> nsresult,

    /* void untrackSessionInfo (in DOMString sessionId, in uint8_t role); */
    pub untrackSessionInfo: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t) -> nsresult,

    /* unsigned long long getWindowIdBySessionId (in DOMString sessionId, in uint8_t role); */
    pub getWindowIdBySessionId: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, _retval: *mut libc::uint64_t) -> nsresult,

    /* void updateWindowIdBySessionId (in DOMString sessionId, in uint8_t role, in unsigned long long windowId); */
    pub updateWindowIdBySessionId: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t, windowId: libc::uint64_t) -> nsresult,

    /* void buildTransport (in DOMString sessionId, in uint8_t role); */
    pub buildTransport: unsafe extern "C" fn (this: *const nsIPresentationService, sessionId: *const nsAString, role: uint8_t) -> nsresult,

}


impl nsIPresentationService {
    /* [noscript] void startSession (in URLArrayRef urls, in DOMString sessionId, in DOMString origin, in DOMString deviceId, in unsigned long long windowId, in nsIDOMEventTarget eventTarget, in nsIPrincipal principal, in nsIPresentationServiceCallback callback, in nsIPresentationTransportBuilderConstructor constructor); */


    /* void sendSessionMessage (in DOMString sessionId, in uint8_t role, in DOMString data); */
    #[inline]
    pub unsafe fn sendSessionMessage(&self, sessionId: &[u16], role: uint8_t, data: &[u16]) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        let data = nsString::from(data);
        match ((*self.vtable).sendSessionMessage)(self as *const _, &*sessionId, role, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendSessionBinaryMsg (in DOMString sessionId, in uint8_t role, in ACString data); */
    #[inline]
    pub unsafe fn sendSessionBinaryMsg(&self, sessionId: &[u16], role: uint8_t, data: &[u8]) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        let data = nsCString::from(data);
        match ((*self.vtable).sendSessionBinaryMsg)(self as *const _, &*sessionId, role, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendSessionBlob (in DOMString sessionId, in uint8_t role, in nsIDOMBlob blob); */
    #[inline]
    pub unsafe fn sendSessionBlob(&self, sessionId: &[u16], role: uint8_t, blob: Option<&nsIDOMBlob>) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).sendSessionBlob)(self as *const _, &*sessionId, role, blob.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closeSession (in DOMString sessionId, in uint8_t role, in uint8_t closedReason); */
    #[inline]
    pub unsafe fn closeSession(&self, sessionId: &[u16], role: uint8_t, closedReason: uint8_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).closeSession)(self as *const _, &*sessionId, role, closedReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminateSession (in DOMString sessionId, in uint8_t role); */
    #[inline]
    pub unsafe fn terminateSession(&self, sessionId: &[u16], role: uint8_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).terminateSession)(self as *const _, &*sessionId, role) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void reconnectSession (in URLArrayRef urls, in DOMString sessionId, in uint8_t role, in nsIPresentationServiceCallback callback); */


    /* [noscript] void registerAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */


    /* [noscript] void unregisterAvailabilityListener (in URLArrayRef availabilityUrls, in nsIPresentationAvailabilityListener listener); */


    /* void registerSessionListener (in DOMString sessionId, in uint8_t role, in nsIPresentationSessionListener listener); */
    #[inline]
    pub unsafe fn registerSessionListener(&self, sessionId: &[u16], role: uint8_t, listener: Option<&nsIPresentationSessionListener>) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).registerSessionListener)(self as *const _, &*sessionId, role, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterSessionListener (in DOMString sessionId, in uint8_t role); */
    #[inline]
    pub unsafe fn unregisterSessionListener(&self, sessionId: &[u16], role: uint8_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).unregisterSessionListener)(self as *const _, &*sessionId, role) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerRespondingListener (in unsigned long long windowId, in nsIPresentationRespondingListener listener); */
    #[inline]
    pub unsafe fn registerRespondingListener(&self, windowId: libc::uint64_t, listener: Option<&nsIPresentationRespondingListener>) -> Result<(), nsresult> {

        match ((*self.vtable).registerRespondingListener)(self as *const _, windowId, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterRespondingListener (in unsigned long long windowId); */
    #[inline]
    pub unsafe fn unregisterRespondingListener(&self, windowId: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterRespondingListener)(self as *const _, windowId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyReceiverReady (in DOMString sessionId, in unsigned long long windowId, in boolean isLoading, in nsIPresentationTransportBuilderConstructor constructor); */
    #[inline]
    pub unsafe fn notifyReceiverReady(&self, sessionId: &[u16], windowId: libc::uint64_t, isLoading: bool, constructor: Option<&nsIPresentationTransportBuilderConstructor>) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).notifyReceiverReady)(self as *const _, &*sessionId, windowId, isLoading, constructor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void NotifyTransportClosed (in DOMString sessionId, in uint8_t role, in nsresult reason); */
    #[inline]
    pub unsafe fn NotifyTransportClosed(&self, sessionId: &[u16], role: uint8_t, reason: nsresult) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).NotifyTransportClosed)(self as *const _, &*sessionId, role, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void untrackSessionInfo (in DOMString sessionId, in uint8_t role); */
    #[inline]
    pub unsafe fn untrackSessionInfo(&self, sessionId: &[u16], role: uint8_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).untrackSessionInfo)(self as *const _, &*sessionId, role) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long long getWindowIdBySessionId (in DOMString sessionId, in uint8_t role); */
    #[inline]
    pub unsafe fn getWindowIdBySessionId(&self, sessionId: &[u16], role: uint8_t) -> Result<libc::uint64_t, nsresult> {
        let sessionId = nsString::from(sessionId);
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).getWindowIdBySessionId)(self as *const _, &*sessionId, role, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void updateWindowIdBySessionId (in DOMString sessionId, in uint8_t role, in unsigned long long windowId); */
    #[inline]
    pub unsafe fn updateWindowIdBySessionId(&self, sessionId: &[u16], role: uint8_t, windowId: libc::uint64_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).updateWindowIdBySessionId)(self as *const _, &*sessionId, role, windowId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void buildTransport (in DOMString sessionId, in uint8_t role); */
    #[inline]
    pub unsafe fn buildTransport(&self, sessionId: &[u16], role: uint8_t) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).buildTransport)(self as *const _, &*sessionId, role) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


