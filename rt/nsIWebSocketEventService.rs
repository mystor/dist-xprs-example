//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketEventService.idl
//


pub mod nsIWebSocketFrame_consts {
    pub const OPCODE_CONTINUATION: i64 = 0;
    pub const OPCODE_TEXT: i64 = 1;
    pub const OPCODE_BINARY: i64 = 2;
    pub const OPCODE_CLOSE: i64 = 8;
    pub const OPCODE_PING: i64 = 9;
    pub const OPCODE_PONG: i64 = 10;
}


#[repr(C)]
pub struct nsIWebSocketFrame {
    vtable: *const nsIWebSocketFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebSocketFrame {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6714a6be, 0x2265, 0x4f73,
            [0xa9, 0x88, 0xd7, 0x8a, 0x12, 0x41, 0x60, 0x37])
    }
}

unsafe impl RefCounted for nsIWebSocketFrame {
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
pub trait nsIWebSocketFrameCoerce {
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self;
}

impl nsIWebSocketFrameCoerce for nsIWebSocketFrame {
    #[inline]
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self {
        v
    }
}

impl nsIWebSocketFrame {
    #[inline]
    pub fn coerce<T: nsIWebSocketFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebSocketFrame {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebSocketFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketFrame) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebSocketFrameVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
    pub get_timeStamp: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aTimeStamp: *mut DOMHighResTimeStamp) -> nsresult,

    /* [must_use] readonly attribute boolean finBit; */
    pub get_finBit: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aFinBit: *mut bool) -> nsresult,

    /* [must_use] readonly attribute boolean rsvBit1; */
    pub get_rsvBit1: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aRsvBit1: *mut bool) -> nsresult,

    /* [must_use] readonly attribute boolean rsvBit2; */
    pub get_rsvBit2: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aRsvBit2: *mut bool) -> nsresult,

    /* [must_use] readonly attribute boolean rsvBit3; */
    pub get_rsvBit3: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aRsvBit3: *mut bool) -> nsresult,

    /* [must_use] readonly attribute unsigned short opCode; */
    pub get_opCode: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aOpCode: *mut libc::uint16_t) -> nsresult,

    /* [must_use] readonly attribute boolean maskBit; */
    pub get_maskBit: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aMaskBit: *mut bool) -> nsresult,

    /* [must_use] readonly attribute unsigned long mask; */
    pub get_mask: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aMask: *mut libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute ACString payload; */
    pub get_payload: unsafe extern "C" fn (this: *const nsIWebSocketFrame, aPayload: *mut nsACString) -> nsresult,

}


impl nsIWebSocketFrame {
    /* [must_use] readonly attribute DOMHighResTimeStamp timeStamp; */
    #[inline]
    pub unsafe fn get_timeStamp(&self, ) -> Result<DOMHighResTimeStamp, nsresult> {
        let mut _retval: DOMHighResTimeStamp = ::std::mem::zeroed();
        match ((*self.vtable).get_timeStamp)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean finBit; */
    #[inline]
    pub unsafe fn get_finBit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_finBit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean rsvBit1; */
    #[inline]
    pub unsafe fn get_rsvBit1(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rsvBit1)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean rsvBit2; */
    #[inline]
    pub unsafe fn get_rsvBit2(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rsvBit2)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean rsvBit3; */
    #[inline]
    pub unsafe fn get_rsvBit3(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rsvBit3)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute unsigned short opCode; */
    #[inline]
    pub unsafe fn get_opCode(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_opCode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean maskBit; */
    #[inline]
    pub unsafe fn get_maskBit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_maskBit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute unsigned long mask; */
    #[inline]
    pub unsafe fn get_mask(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_mask)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString payload; */
    #[inline]
    pub unsafe fn get_payload(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_payload)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIWebSocketEventListener_consts {
    pub const TYPE_STRING: i64 = 0;
    pub const TYPE_BLOB: i64 = 1;
    pub const TYPE_ARRAYBUFFER: i64 = 2;
}


#[repr(C)]
pub struct nsIWebSocketEventListener {
    vtable: *const nsIWebSocketEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebSocketEventListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe7c005ab, 0xe694, 0x489b,
            [0xb7, 0x41, 0x96, 0xdb, 0x43, 0xff, 0xb1, 0x6f])
    }
}

unsafe impl RefCounted for nsIWebSocketEventListener {
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
pub trait nsIWebSocketEventListenerCoerce {
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self;
}

impl nsIWebSocketEventListenerCoerce for nsIWebSocketEventListener {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self {
        v
    }
}

impl nsIWebSocketEventListener {
    #[inline]
    pub fn coerce<T: nsIWebSocketEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebSocketEventListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebSocketEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebSocketEventListenerVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
    pub webSocketCreated: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aURI: *const nsAString, aProtocols: *const nsACString) -> nsresult,

    /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions); */
    pub webSocketOpened: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aEffectiveURI: *const nsAString, aProtocols: *const nsACString, aExtensions: *const nsACString) -> nsresult,

    /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
    pub webSocketMessageAvailable: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aMessage: *const nsACString, aType: libc::uint16_t) -> nsresult,

    /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
    pub webSocketClosed: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aWasClean: bool, aCode: libc::uint16_t, aReason: *const nsAString) -> nsresult,

    /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    pub frameReceived: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aFrame: *const nsIWebSocketFrame) -> nsresult,

    /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    pub frameSent: unsafe extern "C" fn (this: *const nsIWebSocketEventListener, aWebSocketSerialID: libc::uint32_t, aFrame: *const nsIWebSocketFrame) -> nsresult,

}


impl nsIWebSocketEventListener {
    /* [must_use] void webSocketCreated (in unsigned long aWebSocketSerialID, in AString aURI, in ACString aProtocols); */
    #[inline]
    pub unsafe fn webSocketCreated(&self, aWebSocketSerialID: libc::uint32_t, aURI: &[u16], aProtocols: &[u8]) -> Result<(), nsresult> {
        let aURI = nsString::from(aURI);
        let aProtocols = nsCString::from(aProtocols);
        match ((*self.vtable).webSocketCreated)(self as *const _, aWebSocketSerialID, &*aURI, &*aProtocols) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void webSocketOpened (in unsigned long aWebSocketSerialID, in AString aEffectiveURI, in ACString aProtocols, in ACString aExtensions); */
    #[inline]
    pub unsafe fn webSocketOpened(&self, aWebSocketSerialID: libc::uint32_t, aEffectiveURI: &[u16], aProtocols: &[u8], aExtensions: &[u8]) -> Result<(), nsresult> {
        let aEffectiveURI = nsString::from(aEffectiveURI);
        let aProtocols = nsCString::from(aProtocols);
        let aExtensions = nsCString::from(aExtensions);
        match ((*self.vtable).webSocketOpened)(self as *const _, aWebSocketSerialID, &*aEffectiveURI, &*aProtocols, &*aExtensions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void webSocketMessageAvailable (in unsigned long aWebSocketSerialID, in ACString aMessage, in unsigned short aType); */
    #[inline]
    pub unsafe fn webSocketMessageAvailable(&self, aWebSocketSerialID: libc::uint32_t, aMessage: &[u8], aType: libc::uint16_t) -> Result<(), nsresult> {
        let aMessage = nsCString::from(aMessage);
        match ((*self.vtable).webSocketMessageAvailable)(self as *const _, aWebSocketSerialID, &*aMessage, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void webSocketClosed (in unsigned long aWebSocketSerialID, in boolean aWasClean, in unsigned short aCode, in AString aReason); */
    #[inline]
    pub unsafe fn webSocketClosed(&self, aWebSocketSerialID: libc::uint32_t, aWasClean: bool, aCode: libc::uint16_t, aReason: &[u16]) -> Result<(), nsresult> {
        let aReason = nsString::from(aReason);
        match ((*self.vtable).webSocketClosed)(self as *const _, aWebSocketSerialID, aWasClean, aCode, &*aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void frameReceived (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    #[inline]
    pub unsafe fn frameReceived(&self, aWebSocketSerialID: libc::uint32_t, aFrame: Option<&nsIWebSocketFrame>) -> Result<(), nsresult> {

        match ((*self.vtable).frameReceived)(self as *const _, aWebSocketSerialID, aFrame.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void frameSent (in unsigned long aWebSocketSerialID, in nsIWebSocketFrame aFrame); */
    #[inline]
    pub unsafe fn frameSent(&self, aWebSocketSerialID: libc::uint32_t, aFrame: Option<&nsIWebSocketFrame>) -> Result<(), nsresult> {

        match ((*self.vtable).frameSent)(self as *const _, aWebSocketSerialID, aFrame.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWebSocketEventService {
    vtable: *const nsIWebSocketEventServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebSocketEventService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb89d1b90, 0x2cf3, 0x4d8f,
            [0xac, 0x21, 0x5a, 0xed, 0xfb, 0x25, 0xc7, 0x60])
    }
}

unsafe impl RefCounted for nsIWebSocketEventService {
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
pub trait nsIWebSocketEventServiceCoerce {
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self;
}

impl nsIWebSocketEventServiceCoerce for nsIWebSocketEventService {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self {
        v
    }
}

impl nsIWebSocketEventService {
    #[inline]
    pub fn coerce<T: nsIWebSocketEventServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebSocketEventService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebSocketEventServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketEventService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebSocketEventServiceVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIWebSocketEventService, aInnerWindowID: libc::uint64_t, aListener: *const nsIWebSocketEventListener) -> nsresult,

    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIWebSocketEventService, aInnerWindowID: libc::uint64_t, aListener: *const nsIWebSocketEventListener) -> nsresult,

}


impl nsIWebSocketEventService {
    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    #[inline]
    pub unsafe fn addListener(&self, aInnerWindowID: libc::uint64_t, aListener: Option<&nsIWebSocketEventListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, aInnerWindowID, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIWebSocketEventListener aListener); */
    #[inline]
    pub unsafe fn removeListener(&self, aInnerWindowID: libc::uint64_t, aListener: Option<&nsIWebSocketEventListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, aInnerWindowID, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


