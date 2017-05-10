//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISpeculativeConnect.idl
//


#[repr(C)]
pub struct nsISpeculativeConnect {
    vtable: *const nsISpeculativeConnectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeculativeConnect {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd74a17ac, 0x5b8a, 0x4824,
            [0xa3, 0x09, 0xb1, 0xf0, 0x4a, 0x3c, 0x4a, 0xed])
    }
}

unsafe impl RefCounted for nsISpeculativeConnect {
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
pub trait nsISpeculativeConnectCoerce {
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self;
}

impl nsISpeculativeConnectCoerce for nsISpeculativeConnect {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self {
        v
    }
}

impl nsISpeculativeConnect {
    #[inline]
    pub fn coerce<T: nsISpeculativeConnectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeculativeConnect {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeculativeConnectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeculativeConnectVTable {
    pub __base: nsISupportsVTable,

    /* void speculativeConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
    pub speculativeConnect: unsafe extern "C" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* void speculativeConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    pub speculativeConnect2: unsafe extern "C" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
    pub speculativeAnonymousConnect: unsafe extern "C" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* void speculativeAnonymousConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    pub speculativeAnonymousConnect2: unsafe extern "C" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> nsresult,

}


impl nsISpeculativeConnect {
    /* void speculativeConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
    #[inline]
    pub unsafe fn speculativeConnect(&self, aURI: Option<&nsIURI>, aCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).speculativeConnect)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void speculativeConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    #[inline]
    pub unsafe fn speculativeConnect2(&self, aURI: Option<&nsIURI>, aPrincipal: Option<&nsIPrincipal>, aCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).speculativeConnect2)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIInterfaceRequestor aCallbacks); */
    #[inline]
    pub unsafe fn speculativeAnonymousConnect(&self, aURI: Option<&nsIURI>, aCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).speculativeAnonymousConnect)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void speculativeAnonymousConnect2 (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    #[inline]
    pub unsafe fn speculativeAnonymousConnect2(&self, aURI: Option<&nsIURI>, aPrincipal: Option<&nsIPrincipal>, aCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).speculativeAnonymousConnect2)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISpeculativeConnectionOverrider {
    vtable: *const nsISpeculativeConnectionOverriderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISpeculativeConnectionOverrider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1040ebe3, 0x6ed1, 0x45a6,
            [0x85, 0x87, 0x99, 0x5e, 0x08, 0x25, 0x18, 0xd7])
    }
}

unsafe impl RefCounted for nsISpeculativeConnectionOverrider {
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
pub trait nsISpeculativeConnectionOverriderCoerce {
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self;
}

impl nsISpeculativeConnectionOverriderCoerce for nsISpeculativeConnectionOverrider {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self {
        v
    }
}

impl nsISpeculativeConnectionOverrider {
    #[inline]
    pub fn coerce<T: nsISpeculativeConnectionOverriderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISpeculativeConnectionOverrider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISpeculativeConnectionOverriderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISpeculativeConnectionOverriderVTable {
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
    pub get_parallelSpeculativeConnectLimit: unsafe extern "C" fn (this: *const nsISpeculativeConnectionOverrider, aParallelSpeculativeConnectLimit: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean ignoreIdle; */
    pub get_ignoreIdle: unsafe extern "C" fn (this: *const nsISpeculativeConnectionOverrider, aIgnoreIdle: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isFromPredictor; */
    pub get_isFromPredictor: unsafe extern "C" fn (this: *const nsISpeculativeConnectionOverrider, aIsFromPredictor: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean allow1918; */
    pub get_allow1918: unsafe extern "C" fn (this: *const nsISpeculativeConnectionOverrider, aAllow1918: *mut bool) -> nsresult,

}


impl nsISpeculativeConnectionOverrider {
    /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
    #[inline]
    pub unsafe fn get_parallelSpeculativeConnectLimit(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parallelSpeculativeConnectLimit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean ignoreIdle; */
    #[inline]
    pub unsafe fn get_ignoreIdle(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ignoreIdle)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isFromPredictor; */
    #[inline]
    pub unsafe fn get_isFromPredictor(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFromPredictor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean allow1918; */
    #[inline]
    pub unsafe fn get_allow1918(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allow1918)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


