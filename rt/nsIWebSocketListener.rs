//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebSocketListener.idl
//


#[repr(C)]
pub struct nsIWebSocketListener {
    vtable: *const nsIWebSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebSocketListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd74c96b2, 0x65b3, 0x4e39,
            [0x9e, 0x39, 0xc5, 0x77, 0xde, 0x5d, 0x7a, 0x73])
    }
}

unsafe impl RefCounted for nsIWebSocketListener {
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
pub trait nsIWebSocketListenerCoerce {
    fn coerce_from(v: &nsIWebSocketListener) -> &Self;
}

impl nsIWebSocketListenerCoerce for nsIWebSocketListener {
    #[inline]
    fn coerce_from(v: &nsIWebSocketListener) -> &Self {
        v
    }
}

impl nsIWebSocketListener {
    #[inline]
    pub fn coerce<T: nsIWebSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebSocketListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebSocketListenerVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void onStart (in nsISupports aContext); */
    pub onStart: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports) -> nsresult,

    /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
    pub onStop: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aStatusCode: nsresult) -> nsresult,

    /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
    pub onMessageAvailable: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aMsg: *const nsACString) -> nsresult,

    /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
    pub onBinaryMessageAvailable: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aMsg: *const nsACString) -> nsresult,

    /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
    pub onAcknowledge: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aSize: uint32_t) -> nsresult,

    /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
    pub onServerClose: unsafe extern "C" fn (this: *const nsIWebSocketListener, aContext: *const nsISupports, aCode: libc::uint16_t, aReason: *const nsACString) -> nsresult,

}


impl nsIWebSocketListener {
    /* [must_use] void onStart (in nsISupports aContext); */
    #[inline]
    pub unsafe fn onStart(&self, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onStart)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onStop (in nsISupports aContext, in nsresult aStatusCode); */
    #[inline]
    pub unsafe fn onStop(&self, aContext: Option<&nsISupports>, aStatusCode: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onStop)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), aStatusCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onMessageAvailable (in nsISupports aContext, in AUTF8String aMsg); */
    #[inline]
    pub unsafe fn onMessageAvailable(&self, aContext: Option<&nsISupports>, aMsg: &[u8]) -> Result<(), nsresult> {
        let aMsg = nsCString::from(aMsg);
        match ((*self.vtable).onMessageAvailable)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onBinaryMessageAvailable (in nsISupports aContext, in ACString aMsg); */
    #[inline]
    pub unsafe fn onBinaryMessageAvailable(&self, aContext: Option<&nsISupports>, aMsg: &[u8]) -> Result<(), nsresult> {
        let aMsg = nsCString::from(aMsg);
        match ((*self.vtable).onBinaryMessageAvailable)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aMsg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onAcknowledge (in nsISupports aContext, in uint32_t aSize); */
    #[inline]
    pub unsafe fn onAcknowledge(&self, aContext: Option<&nsISupports>, aSize: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onAcknowledge)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void onServerClose (in nsISupports aContext, in unsigned short aCode, in AUTF8String aReason); */
    #[inline]
    pub unsafe fn onServerClose(&self, aContext: Option<&nsISupports>, aCode: libc::uint16_t, aReason: &[u8]) -> Result<(), nsresult> {
        let aReason = nsCString::from(aReason);
        match ((*self.vtable).onServerClose)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), aCode, &*aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


