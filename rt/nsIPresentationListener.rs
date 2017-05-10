//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationListener.idl
//


#[repr(C)]
pub struct nsIPresentationAvailabilityListener {
    vtable: *const nsIPresentationAvailabilityListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationAvailabilityListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0105f837, 0x4279, 0x4715,
            [0x9d, 0x5b, 0x2d, 0xc3, 0xf8, 0xb6, 0x53, 0x53])
    }
}

unsafe impl RefCounted for nsIPresentationAvailabilityListener {
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
pub trait nsIPresentationAvailabilityListenerCoerce {
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self;
}

impl nsIPresentationAvailabilityListenerCoerce for nsIPresentationAvailabilityListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self {
        v
    }
}

impl nsIPresentationAvailabilityListener {
    #[inline]
    pub fn coerce<T: nsIPresentationAvailabilityListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationAvailabilityListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationAvailabilityListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationAvailabilityListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationAvailabilityListenerVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void notifyAvailableChange (in URLArrayRef urls, in bool available); */
    /// Unable to call function as its signature contains a non-rust type
    pub notifyAvailableChange: *const ::libc::c_void,

}


impl nsIPresentationAvailabilityListener {
    /* [noscript] void notifyAvailableChange (in URLArrayRef urls, in bool available); */


}


pub mod nsIPresentationSessionListener_consts {
    pub const STATE_CONNECTING: i64 = 0;
    pub const STATE_CONNECTED: i64 = 1;
    pub const STATE_CLOSED: i64 = 2;
    pub const STATE_TERMINATED: i64 = 3;
}


#[repr(C)]
pub struct nsIPresentationSessionListener {
    vtable: *const nsIPresentationSessionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7dd48df8, 0x8f8c, 0x48c7,
            [0xac, 0x37, 0x7b, 0x9f, 0xd1, 0xac, 0xf2, 0xf8])
    }
}

unsafe impl RefCounted for nsIPresentationSessionListener {
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
pub trait nsIPresentationSessionListenerCoerce {
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self;
}

impl nsIPresentationSessionListenerCoerce for nsIPresentationSessionListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self {
        v
    }
}

impl nsIPresentationSessionListener {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionListenerVTable {
    pub __base: nsISupportsVTable,

    /* void notifyStateChange (in DOMString sessionId, in unsigned short state, in nsresult reason); */
    pub notifyStateChange: unsafe extern "C" fn (this: *const nsIPresentationSessionListener, sessionId: *const nsAString, state: libc::uint16_t, reason: nsresult) -> nsresult,

    /* void notifyMessage (in DOMString sessionId, in ACString data, in boolean isBinary); */
    pub notifyMessage: unsafe extern "C" fn (this: *const nsIPresentationSessionListener, sessionId: *const nsAString, data: *const nsACString, isBinary: bool) -> nsresult,

}


impl nsIPresentationSessionListener {
    /* void notifyStateChange (in DOMString sessionId, in unsigned short state, in nsresult reason); */
    #[inline]
    pub unsafe fn notifyStateChange(&self, sessionId: &[u16], state: libc::uint16_t, reason: nsresult) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).notifyStateChange)(self as *const _, &*sessionId, state, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyMessage (in DOMString sessionId, in ACString data, in boolean isBinary); */
    #[inline]
    pub unsafe fn notifyMessage(&self, sessionId: &[u16], data: &[u8], isBinary: bool) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        let data = nsCString::from(data);
        match ((*self.vtable).notifyMessage)(self as *const _, &*sessionId, &*data, isBinary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationRespondingListener {
    vtable: *const nsIPresentationRespondingListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationRespondingListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x27f101d7, 0x9ed1, 0x429e,
            [0xb4, 0xf8, 0x43, 0xb0, 0x0e, 0x8e, 0x11, 0x1c])
    }
}

unsafe impl RefCounted for nsIPresentationRespondingListener {
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
pub trait nsIPresentationRespondingListenerCoerce {
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self;
}

impl nsIPresentationRespondingListenerCoerce for nsIPresentationRespondingListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self {
        v
    }
}

impl nsIPresentationRespondingListener {
    #[inline]
    pub fn coerce<T: nsIPresentationRespondingListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationRespondingListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationRespondingListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationRespondingListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationRespondingListenerVTable {
    pub __base: nsISupportsVTable,

    /* void notifySessionConnect (in unsigned long long windowId, in DOMString sessionId); */
    pub notifySessionConnect: unsafe extern "C" fn (this: *const nsIPresentationRespondingListener, windowId: libc::uint64_t, sessionId: *const nsAString) -> nsresult,

}


impl nsIPresentationRespondingListener {
    /* void notifySessionConnect (in unsigned long long windowId, in DOMString sessionId); */
    #[inline]
    pub unsafe fn notifySessionConnect(&self, windowId: libc::uint64_t, sessionId: &[u16]) -> Result<(), nsresult> {
        let sessionId = nsString::from(sessionId);
        match ((*self.vtable).notifySessionConnect)(self as *const _, windowId, &*sessionId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


