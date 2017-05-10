//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyFilter.idl
//


#[repr(C)]
pub struct nsIProtocolProxyFilter {
    vtable: *const nsIProtocolProxyFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolProxyFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf424abd3, 0x32b4, 0x456c,
            [0x9f, 0x45, 0xb7, 0xe3, 0x37, 0x6c, 0xb0, 0xd1])
    }
}

unsafe impl RefCounted for nsIProtocolProxyFilter {
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
pub trait nsIProtocolProxyFilterCoerce {
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self;
}

impl nsIProtocolProxyFilterCoerce for nsIProtocolProxyFilter {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self {
        v
    }
}

impl nsIProtocolProxyFilter {
    #[inline]
    pub fn coerce<T: nsIProtocolProxyFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolProxyFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolProxyFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolProxyFilterVTable {
    pub __base: nsISupportsVTable,

    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIURI aURI, in nsIProxyInfo aProxy); */
    pub applyFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyFilter, aProxyService: *const nsIProtocolProxyService, aURI: *const nsIURI, aProxy: *const nsIProxyInfo, _retval: *mut *const nsIProxyInfo) -> nsresult,

}


impl nsIProtocolProxyFilter {
    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIURI aURI, in nsIProxyInfo aProxy); */
    #[inline]
    pub unsafe fn applyFilter(&self, aProxyService: Option<&nsIProtocolProxyService>, aURI: Option<&nsIURI>, aProxy: Option<&nsIProxyInfo>) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).applyFilter)(self as *const _, aProxyService.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), aProxy.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIProtocolProxyChannelFilter {
    vtable: *const nsIProtocolProxyChannelFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolProxyChannelFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x245b0880, 0x82c5, 0x4e6e,
            [0xbe, 0x6d, 0xbc, 0x58, 0x6a, 0xa5, 0x5a, 0x90])
    }
}

unsafe impl RefCounted for nsIProtocolProxyChannelFilter {
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
pub trait nsIProtocolProxyChannelFilterCoerce {
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self;
}

impl nsIProtocolProxyChannelFilterCoerce for nsIProtocolProxyChannelFilter {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self {
        v
    }
}

impl nsIProtocolProxyChannelFilter {
    #[inline]
    pub fn coerce<T: nsIProtocolProxyChannelFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolProxyChannelFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolProxyChannelFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolProxyChannelFilterVTable {
    pub __base: nsISupportsVTable,

    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIChannel aChannel, in nsIProxyInfo aProxy); */
    pub applyFilter: unsafe extern "C" fn (this: *const nsIProtocolProxyChannelFilter, aProxyService: *const nsIProtocolProxyService, aChannel: *const nsIChannel, aProxy: *const nsIProxyInfo, _retval: *mut *const nsIProxyInfo) -> nsresult,

}


impl nsIProtocolProxyChannelFilter {
    /* nsIProxyInfo applyFilter (in nsIProtocolProxyService aProxyService, in nsIChannel aChannel, in nsIProxyInfo aProxy); */
    #[inline]
    pub unsafe fn applyFilter(&self, aProxyService: Option<&nsIProtocolProxyService>, aChannel: Option<&nsIChannel>, aProxy: Option<&nsIProxyInfo>) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).applyFilter)(self as *const _, aProxyService.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), aProxy.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


