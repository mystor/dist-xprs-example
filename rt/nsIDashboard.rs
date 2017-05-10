//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDashboard.idl
//


#[repr(C)]
pub struct NetDashboardCallback {
    vtable: *const NetDashboardCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for NetDashboardCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x19d7f24f, 0xa95a, 0x4fd9,
            [0x87, 0xe2, 0xd9, 0x6e, 0x9e, 0x4b, 0x1f, 0x6d])
    }
}

unsafe impl RefCounted for NetDashboardCallback {
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
pub trait NetDashboardCallbackCoerce {
    fn coerce_from(v: &NetDashboardCallback) -> &Self;
}

impl NetDashboardCallbackCoerce for NetDashboardCallback {
    #[inline]
    fn coerce_from(v: &NetDashboardCallback) -> &Self {
        v
    }
}

impl NetDashboardCallback {
    #[inline]
    pub fn coerce<T: NetDashboardCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for NetDashboardCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> NetDashboardCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &NetDashboardCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct NetDashboardCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onDashboardDataAvailable (in jsval data); */
    /// Unable to call function as its signature contains a non-rust type
    pub onDashboardDataAvailable: *const ::libc::c_void,

}


impl NetDashboardCallback {
    /* void onDashboardDataAvailable (in jsval data); */


}


#[repr(C)]
pub struct nsIDashboard {
    vtable: *const nsIDashboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDashboard {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc79eb3c6, 0x091a, 0x45a6,
            [0x85, 0x44, 0x5a, 0x8d, 0x1a, 0xb7, 0x95, 0x37])
    }
}

unsafe impl RefCounted for nsIDashboard {
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
pub trait nsIDashboardCoerce {
    fn coerce_from(v: &nsIDashboard) -> &Self;
}

impl nsIDashboardCoerce for nsIDashboard {
    #[inline]
    fn coerce_from(v: &nsIDashboard) -> &Self {
        v
    }
}

impl nsIDashboard {
    #[inline]
    pub fn coerce<T: nsIDashboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDashboard {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDashboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDashboard) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDashboardVTable {
    pub __base: nsISupportsVTable,

    /* void requestSockets (in NetDashboardCallback cb); */
    pub requestSockets: unsafe extern "C" fn (this: *const nsIDashboard, cb: *const NetDashboardCallback) -> nsresult,

    /* void requestHttpConnections (in NetDashboardCallback cb); */
    pub requestHttpConnections: unsafe extern "C" fn (this: *const nsIDashboard, cb: *const NetDashboardCallback) -> nsresult,

    /* void requestWebsocketConnections (in NetDashboardCallback cb); */
    pub requestWebsocketConnections: unsafe extern "C" fn (this: *const nsIDashboard, cb: *const NetDashboardCallback) -> nsresult,

    /* void requestDNSInfo (in NetDashboardCallback cb); */
    pub requestDNSInfo: unsafe extern "C" fn (this: *const nsIDashboard, cb: *const NetDashboardCallback) -> nsresult,

    /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in NetDashboardCallback cb); */
    pub requestConnection: unsafe extern "C" fn (this: *const nsIDashboard, aHost: *const nsACString, aPort: libc::uint32_t, aProtocol: *const libc::c_char, aTimeout: libc::uint32_t, cb: *const NetDashboardCallback) -> nsresult,

    /* attribute boolean enableLogging; */
    pub get_enableLogging: unsafe extern "C" fn (this: *const nsIDashboard, aEnableLogging: *mut bool) -> nsresult,
    pub set_enableLogging: unsafe extern "C" fn (this: *const nsIDashboard, aEnableLogging: bool) -> nsresult,

    /* void requestDNSLookup (in ACString aHost, in NetDashboardCallback cb); */
    pub requestDNSLookup: unsafe extern "C" fn (this: *const nsIDashboard, aHost: *const nsACString, cb: *const NetDashboardCallback) -> nsresult,

    /* AUTF8String getLogPath (); */
    pub getLogPath: unsafe extern "C" fn (this: *const nsIDashboard, _retval: *mut nsACString) -> nsresult,

}


impl nsIDashboard {
    /* void requestSockets (in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestSockets(&self, cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).requestSockets)(self as *const _, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestHttpConnections (in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestHttpConnections(&self, cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).requestHttpConnections)(self as *const _, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestWebsocketConnections (in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestWebsocketConnections(&self, cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).requestWebsocketConnections)(self as *const _, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestDNSInfo (in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestDNSInfo(&self, cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).requestDNSInfo)(self as *const _, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestConnection (in ACString aHost, in unsigned long aPort, in string aProtocol, in unsigned long aTimeout, in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestConnection(&self, aHost: &[u8], aPort: libc::uint32_t, aProtocol: *const libc::c_char, aTimeout: libc::uint32_t, cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).requestConnection)(self as *const _, &*aHost, aPort, aProtocol, aTimeout, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean enableLogging; */
    #[inline]
    pub unsafe fn get_enableLogging(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enableLogging)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enableLogging(&self, aEnableLogging: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_enableLogging)(self as *const _, aEnableLogging) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestDNSLookup (in ACString aHost, in NetDashboardCallback cb); */
    #[inline]
    pub unsafe fn requestDNSLookup(&self, aHost: &[u8], cb: Option<&NetDashboardCallback>) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).requestDNSLookup)(self as *const _, &*aHost, cb.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AUTF8String getLogPath (); */
    #[inline]
    pub unsafe fn getLogPath(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getLogPath)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


