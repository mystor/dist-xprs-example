//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProxiedProtocolHandler.idl
//


#[repr(C)]
pub struct nsIProxiedProtocolHandler {
    vtable: *const nsIProxiedProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProxiedProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3756047a, 0xfa2b, 0x4b45,
            [0x99, 0x48, 0x3b, 0x5f, 0x8f, 0xc3, 0x75, 0xe7])
    }
}

unsafe impl RefCounted for nsIProxiedProtocolHandler {
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
pub trait nsIProxiedProtocolHandlerCoerce {
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self;
}

impl nsIProxiedProtocolHandlerCoerce for nsIProxiedProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self {
        v
    }
}

impl nsIProxiedProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIProxiedProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProxiedProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolHandlerCoerce> nsIProxiedProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxiedProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProxiedProtocolHandlerVTable {
    pub __base: nsIProtocolHandlerVTable,

    /* nsIChannel newProxiedChannel2 (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo); */
    pub newProxiedChannel2: unsafe extern "C" fn (this: *const nsIProxiedProtocolHandler, uri: *const nsIURI, proxyInfo: *const nsIProxyInfo, proxyResolveFlags: libc::uint32_t, proxyURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI); */
    pub newProxiedChannel: unsafe extern "C" fn (this: *const nsIProxiedProtocolHandler, uri: *const nsIURI, proxyInfo: *const nsIProxyInfo, proxyResolveFlags: libc::uint32_t, proxyURI: *const nsIURI, _retval: *mut *const nsIChannel) -> nsresult,

}


impl nsIProxiedProtocolHandler {
    /* nsIChannel newProxiedChannel2 (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI, in nsILoadInfo aLoadInfo); */
    #[inline]
    pub unsafe fn newProxiedChannel2(&self, uri: Option<&nsIURI>, proxyInfo: Option<&nsIProxyInfo>, proxyResolveFlags: libc::uint32_t, proxyURI: Option<&nsIURI>, aLoadInfo: Option<&nsILoadInfo>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newProxiedChannel2)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), proxyInfo.map_or(::std::ptr::null(), |x| x as *const _), proxyResolveFlags, proxyURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newProxiedChannel (in nsIURI uri, in nsIProxyInfo proxyInfo, in unsigned long proxyResolveFlags, in nsIURI proxyURI); */
    #[inline]
    pub unsafe fn newProxiedChannel(&self, uri: Option<&nsIURI>, proxyInfo: Option<&nsIProxyInfo>, proxyResolveFlags: libc::uint32_t, proxyURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newProxiedChannel)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), proxyInfo.map_or(::std::ptr::null(), |x| x as *const _), proxyResolveFlags, proxyURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


