//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOService2.idl
//


#[repr(C)]
pub struct nsIIOService2 {
    vtable: *const nsIIOService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIOService2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x52c5804b, 0x0d3c, 0x4d4f,
            [0x86, 0x54, 0x1c, 0x36, 0xfd, 0x31, 0x0e, 0x69])
    }
}

unsafe impl RefCounted for nsIIOService2 {
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
pub trait nsIIOService2Coerce {
    fn coerce_from(v: &nsIIOService2) -> &Self;
}

impl nsIIOService2Coerce for nsIIOService2 {
    #[inline]
    fn coerce_from(v: &nsIIOService2) -> &Self {
        v
    }
}

impl nsIIOService2 {
    #[inline]
    pub fn coerce<T: nsIIOService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIOService2 {
    type Target = nsIIOService;
    #[inline]
    fn deref(&self) -> &nsIIOService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIIOServiceCoerce> nsIIOService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOService2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIOService2VTable {
    pub __base: nsIIOServiceVTable,

    /* attribute boolean manageOfflineStatus; */
    pub get_manageOfflineStatus: unsafe extern "C" fn (this: *const nsIIOService2, aManageOfflineStatus: *mut bool) -> nsresult,
    pub set_manageOfflineStatus: unsafe extern "C" fn (this: *const nsIIOService2, aManageOfflineStatus: bool) -> nsresult,

    /* nsIChannel newChannelFromURIWithProxyFlags2 (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    pub newChannelFromURIWithProxyFlags2: unsafe extern "C" fn (this: *const nsIIOService2, aURI: *const nsIURI, aProxyURI: *const nsIURI, aProxyFlags: libc::uint32_t, aLoadingNode: *const nsIDOMNode, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags); */
    pub newChannelFromURIWithProxyFlags: unsafe extern "C" fn (this: *const nsIIOService2, aURI: *const nsIURI, aProxyURI: *const nsIURI, aProxyFlags: libc::uint32_t, _retval: *mut *const nsIChannel) -> nsresult,

}


impl nsIIOService2 {
    /* attribute boolean manageOfflineStatus; */
    #[inline]
    pub unsafe fn get_manageOfflineStatus(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_manageOfflineStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_manageOfflineStatus(&self, aManageOfflineStatus: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_manageOfflineStatus)(self as *const _, aManageOfflineStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIChannel newChannelFromURIWithProxyFlags2 (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    #[inline]
    pub unsafe fn newChannelFromURIWithProxyFlags2(&self, aURI: Option<&nsIURI>, aProxyURI: Option<&nsIURI>, aProxyFlags: libc::uint32_t, aLoadingNode: Option<&nsIDOMNode>, aLoadingPrincipal: Option<&nsIPrincipal>, aTriggeringPrincipal: Option<&nsIPrincipal>, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannelFromURIWithProxyFlags2)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aProxyURI.map_or(::std::ptr::null(), |x| x as *const _), aProxyFlags, aLoadingNode.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aSecurityFlags, aContentPolicyType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags); */
    #[inline]
    pub unsafe fn newChannelFromURIWithProxyFlags(&self, aURI: Option<&nsIURI>, aProxyURI: Option<&nsIURI>, aProxyFlags: libc::uint32_t) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannelFromURIWithProxyFlags)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aProxyURI.map_or(::std::ptr::null(), |x| x as *const _), aProxyFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


