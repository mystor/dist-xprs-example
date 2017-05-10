//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIOService.idl
//


#[repr(C)]
pub struct nsIIOService {
    vtable: *const nsIIOServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIOService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4286de5a, 0xb2ea, 0x446f,
            [0x8f, 0x70, 0xe2, 0xa4, 0x61, 0xf4, 0x26, 0x94])
    }
}

unsafe impl RefCounted for nsIIOService {
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
pub trait nsIIOServiceCoerce {
    fn coerce_from(v: &nsIIOService) -> &Self;
}

impl nsIIOServiceCoerce for nsIIOService {
    #[inline]
    fn coerce_from(v: &nsIIOService) -> &Self {
        v
    }
}

impl nsIIOService {
    #[inline]
    pub fn coerce<T: nsIIOServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIOService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIOServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIOServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIProtocolHandler getProtocolHandler (in string aScheme); */
    pub getProtocolHandler: unsafe extern "C" fn (this: *const nsIIOService, aScheme: *const libc::c_char, _retval: *mut *const nsIProtocolHandler) -> nsresult,

    /* unsigned long getProtocolFlags (in string aScheme); */
    pub getProtocolFlags: unsafe extern "C" fn (this: *const nsIIOService, aScheme: *const libc::c_char, _retval: *mut libc::uint32_t) -> nsresult,

    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
    pub newURI: unsafe extern "C" fn (this: *const nsIIOService, aSpec: *const nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIURI newFileURI (in nsIFile aFile); */
    pub newFileURI: unsafe extern "C" fn (this: *const nsIIOService, aFile: *const nsIFile, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIChannel newChannelFromURI2 (in nsIURI aURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    pub newChannelFromURI2: unsafe extern "C" fn (this: *const nsIIOService, aURI: *const nsIURI, aLoadingNode: *const nsIDOMNode, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    pub newChannelFromURIWithLoadInfo: unsafe extern "C" fn (this: *const nsIIOService, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannel2 (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    pub newChannel2: unsafe extern "C" fn (this: *const nsIIOService, aSpec: *const nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, aLoadingNode: *const nsIDOMNode, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannelFromURI (in nsIURI aURI); */
    pub newChannelFromURI: unsafe extern "C" fn (this: *const nsIIOService, aURI: *const nsIURI, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
    pub newChannel: unsafe extern "C" fn (this: *const nsIIOService, aSpec: *const nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut *const nsIChannel) -> nsresult,

    /* attribute boolean offline; */
    pub get_offline: unsafe extern "C" fn (this: *const nsIIOService, aOffline: *mut bool) -> nsresult,
    pub set_offline: unsafe extern "C" fn (this: *const nsIIOService, aOffline: bool) -> nsresult,

    /* readonly attribute boolean connectivity; */
    pub get_connectivity: unsafe extern "C" fn (this: *const nsIIOService, aConnectivity: *mut bool) -> nsresult,

    /* boolean allowPort (in long aPort, in string aScheme); */
    pub allowPort: unsafe extern "C" fn (this: *const nsIIOService, aPort: libc::int32_t, aScheme: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* ACString extractScheme (in AUTF8String urlString); */
    pub extractScheme: unsafe extern "C" fn (this: *const nsIIOService, urlString: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIIOService {
    /* nsIProtocolHandler getProtocolHandler (in string aScheme); */
    #[inline]
    pub unsafe fn getProtocolHandler(&self, aScheme: *const libc::c_char) -> Result<Option<RefPtr<nsIProtocolHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProtocolHandler)(self as *const _, aScheme, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getProtocolFlags (in string aScheme); */
    #[inline]
    pub unsafe fn getProtocolFlags(&self, aScheme: *const libc::c_char) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getProtocolFlags)(self as *const _, aScheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
    #[inline]
    pub unsafe fn newURI(&self, aSpec: &[u8], aOriginCharset: *const libc::c_char, aBaseURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let aSpec = nsCString::from(aSpec);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newURI)(self as *const _, &*aSpec, aOriginCharset, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI newFileURI (in nsIFile aFile); */
    #[inline]
    pub unsafe fn newFileURI(&self, aFile: Option<&nsIFile>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newFileURI)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannelFromURI2 (in nsIURI aURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    #[inline]
    pub unsafe fn newChannelFromURI2(&self, aURI: Option<&nsIURI>, aLoadingNode: Option<&nsIDOMNode>, aLoadingPrincipal: Option<&nsIPrincipal>, aTriggeringPrincipal: Option<&nsIPrincipal>, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannelFromURI2)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingNode.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aSecurityFlags, aContentPolicyType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    #[inline]
    pub unsafe fn newChannelFromURIWithLoadInfo(&self, aURI: Option<&nsIURI>, aLoadInfo: Option<&nsILoadInfo>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannelFromURIWithLoadInfo)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannel2 (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in nsIDOMNode aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in unsigned long aContentPolicyType); */
    #[inline]
    pub unsafe fn newChannel2(&self, aSpec: &[u8], aOriginCharset: *const libc::c_char, aBaseURI: Option<&nsIURI>, aLoadingNode: Option<&nsIDOMNode>, aLoadingPrincipal: Option<&nsIPrincipal>, aTriggeringPrincipal: Option<&nsIPrincipal>, aSecurityFlags: libc::uint32_t, aContentPolicyType: libc::uint32_t) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let aSpec = nsCString::from(aSpec);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannel2)(self as *const _, &*aSpec, aOriginCharset, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadingNode.map_or(::std::ptr::null(), |x| x as *const _), aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aSecurityFlags, aContentPolicyType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannelFromURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn newChannelFromURI(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannelFromURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
    #[inline]
    pub unsafe fn newChannel(&self, aSpec: &[u8], aOriginCharset: *const libc::c_char, aBaseURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let aSpec = nsCString::from(aSpec);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannel)(self as *const _, &*aSpec, aOriginCharset, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean offline; */
    #[inline]
    pub unsafe fn get_offline(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_offline)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_offline(&self, aOffline: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_offline)(self as *const _, aOffline) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean connectivity; */
    #[inline]
    pub unsafe fn get_connectivity(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_connectivity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean allowPort (in long aPort, in string aScheme); */
    #[inline]
    pub unsafe fn allowPort(&self, aPort: libc::int32_t, aScheme: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowPort)(self as *const _, aPort, aScheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString extractScheme (in AUTF8String urlString); */
    #[inline]
    pub unsafe fn extractScheme(&self, urlString: &[u8]) -> Result<nsCString, nsresult> {
        let urlString = nsCString::from(urlString);
        let mut _retval = nsCString::new();
        match ((*self.vtable).extractScheme)(self as *const _, &*urlString, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIIOServiceInternal {
    vtable: *const nsIIOServiceInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIOServiceInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6633c0bf, 0xd97a, 0x428f,
            [0x8e, 0xce, 0xcb, 0x6a, 0x65, 0x5f, 0xb9, 0x5a])
    }
}

unsafe impl RefCounted for nsIIOServiceInternal {
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
pub trait nsIIOServiceInternalCoerce {
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self;
}

impl nsIIOServiceInternalCoerce for nsIIOServiceInternal {
    #[inline]
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self {
        v
    }
}

impl nsIIOServiceInternal {
    #[inline]
    pub fn coerce<T: nsIIOServiceInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIOServiceInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIOServiceInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIOServiceInternalVTable {
    pub __base: nsISupportsVTable,

    /* void SetConnectivity (in boolean connectivity); */
    pub SetConnectivity: unsafe extern "C" fn (this: *const nsIIOServiceInternal, connectivity: bool) -> nsresult,

    /* void NotifyWakeup (); */
    pub NotifyWakeup: unsafe extern "C" fn (this: *const nsIIOServiceInternal) -> nsresult,

}


impl nsIIOServiceInternal {
    /* void SetConnectivity (in boolean connectivity); */
    #[inline]
    pub unsafe fn SetConnectivity(&self, connectivity: bool) -> Result<(), nsresult> {

        match ((*self.vtable).SetConnectivity)(self as *const _, connectivity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void NotifyWakeup (); */
    #[inline]
    pub unsafe fn NotifyWakeup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).NotifyWakeup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


