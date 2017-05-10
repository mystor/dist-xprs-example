//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/IPeerConnection.idl
//


#[repr(C)]
pub struct IPeerConnectionManager {
    vtable: *const IPeerConnectionManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for IPeerConnectionManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2218bd2, 0x2648, 0x4701,
            [0x8f, 0xa6, 0x30, 0x5d, 0x33, 0x79, 0xe9, 0xf8])
    }
}

unsafe impl RefCounted for IPeerConnectionManager {
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
pub trait IPeerConnectionManagerCoerce {
    fn coerce_from(v: &IPeerConnectionManager) -> &Self;
}

impl IPeerConnectionManagerCoerce for IPeerConnectionManager {
    #[inline]
    fn coerce_from(v: &IPeerConnectionManager) -> &Self {
        v
    }
}

impl IPeerConnectionManager {
    #[inline]
    pub fn coerce<T: IPeerConnectionManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for IPeerConnectionManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> IPeerConnectionManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &IPeerConnectionManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct IPeerConnectionManagerVTable {
    pub __base: nsISupportsVTable,

    /* boolean hasActivePeerConnection (in unsigned long innerWindowID); */
    pub hasActivePeerConnection: unsafe extern "C" fn (this: *const IPeerConnectionManager, innerWindowID: libc::uint32_t, _retval: *mut bool) -> nsresult,

}


impl IPeerConnectionManager {
    /* boolean hasActivePeerConnection (in unsigned long innerWindowID); */
    #[inline]
    pub unsafe fn hasActivePeerConnection(&self, innerWindowID: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasActivePeerConnection)(self as *const _, innerWindowID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct IPeerConnectionObserver {
    vtable: *const IPeerConnectionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for IPeerConnectionObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd7dfe148, 0x0416, 0x446b,
            [0xa1, 0x28, 0x66, 0xa7, 0xc7, 0x1a, 0xe8, 0xd3])
    }
}

unsafe impl RefCounted for IPeerConnectionObserver {
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
pub trait IPeerConnectionObserverCoerce {
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self;
}

impl IPeerConnectionObserverCoerce for IPeerConnectionObserver {
    #[inline]
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self {
        v
    }
}

impl IPeerConnectionObserver {
    #[inline]
    pub fn coerce<T: IPeerConnectionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for IPeerConnectionObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> IPeerConnectionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &IPeerConnectionObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct IPeerConnectionObserverVTable {
    pub __base: nsISupportsVTable,

}


impl IPeerConnectionObserver {
}


pub mod IPeerConnection_consts {
    pub const kHintAudio: i64 = 1;
    pub const kHintVideo: i64 = 2;
    pub const kActionNone: i64 = -1;
    pub const kActionOffer: i64 = 0;
    pub const kActionAnswer: i64 = 1;
    pub const kActionPRAnswer: i64 = 2;
    pub const kActionRollback: i64 = 3;
    pub const kIceGathering: i64 = 0;
    pub const kIceWaiting: i64 = 1;
    pub const kIceChecking: i64 = 2;
    pub const kIceConnected: i64 = 3;
    pub const kIceFailed: i64 = 4;
    pub const kNew: i64 = 0;
    pub const kNegotiating: i64 = 1;
    pub const kActive: i64 = 2;
    pub const kClosing: i64 = 3;
    pub const kClosed: i64 = 4;
    pub const kDataChannelReliable: i64 = 0;
    pub const kDataChannelPartialReliableRexmit: i64 = 1;
    pub const kDataChannelPartialReliableTimed: i64 = 2;
    pub const kNoError: i64 = 0;
    pub const kInvalidCandidate: i64 = 2;
    pub const kInvalidMediastreamTrack: i64 = 3;
    pub const kInvalidState: i64 = 4;
    pub const kInvalidSessionDescription: i64 = 5;
    pub const kIncompatibleSessionDescription: i64 = 6;
    pub const kIncompatibleMediaStreamTrack: i64 = 8;
    pub const kInternalError: i64 = 9;
    pub const kMaxErrorType: i64 = 9;
}


#[repr(C)]
pub struct IPeerConnection {
    vtable: *const IPeerConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for IPeerConnection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x14afc8e7, 0xe421, 0x4d0c,
            [0x99, 0xa5, 0x69, 0x30, 0x8d, 0x87, 0x14, 0x81])
    }
}

unsafe impl RefCounted for IPeerConnection {
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
pub trait IPeerConnectionCoerce {
    fn coerce_from(v: &IPeerConnection) -> &Self;
}

impl IPeerConnectionCoerce for IPeerConnection {
    #[inline]
    fn coerce_from(v: &IPeerConnection) -> &Self {
        v
    }
}

impl IPeerConnection {
    #[inline]
    pub fn coerce<T: IPeerConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for IPeerConnection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> IPeerConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &IPeerConnection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct IPeerConnectionVTable {
    pub __base: nsISupportsVTable,

}


impl IPeerConnection {
}


