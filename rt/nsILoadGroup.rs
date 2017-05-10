//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadGroup.idl
//


#[repr(C)]
pub struct nsILoadGroup {
    vtable: *const nsILoadGroupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadGroup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf0c87725, 0x7a35, 0x463c,
            [0x9c, 0xeb, 0x2c, 0x07, 0xf2, 0x34, 0x06, 0xcc])
    }
}

unsafe impl RefCounted for nsILoadGroup {
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
pub trait nsILoadGroupCoerce {
    fn coerce_from(v: &nsILoadGroup) -> &Self;
}

impl nsILoadGroupCoerce for nsILoadGroup {
    #[inline]
    fn coerce_from(v: &nsILoadGroup) -> &Self {
        v
    }
}

impl nsILoadGroup {
    #[inline]
    pub fn coerce<T: nsILoadGroupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadGroup {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsILoadGroupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadGroup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadGroupVTable {
    pub __base: nsIRequestVTable,

    /* attribute nsIRequestObserver groupObserver; */
    pub get_groupObserver: unsafe extern "C" fn (this: *const nsILoadGroup, aGroupObserver: *mut *const nsIRequestObserver) -> nsresult,
    pub set_groupObserver: unsafe extern "C" fn (this: *const nsILoadGroup, aGroupObserver: *const nsIRequestObserver) -> nsresult,

    /* attribute nsIRequest defaultLoadRequest; */
    pub get_defaultLoadRequest: unsafe extern "C" fn (this: *const nsILoadGroup, aDefaultLoadRequest: *mut *const nsIRequest) -> nsresult,
    pub set_defaultLoadRequest: unsafe extern "C" fn (this: *const nsILoadGroup, aDefaultLoadRequest: *const nsIRequest) -> nsresult,

    /* void addRequest (in nsIRequest aRequest, in nsISupports aContext); */
    pub addRequest: unsafe extern "C" fn (this: *const nsILoadGroup, aRequest: *const nsIRequest, aContext: *const nsISupports) -> nsresult,

    /* void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus); */
    pub removeRequest: unsafe extern "C" fn (this: *const nsILoadGroup, aRequest: *const nsIRequest, aContext: *const nsISupports, aStatus: nsresult) -> nsresult,

    /* readonly attribute nsISimpleEnumerator requests; */
    pub get_requests: unsafe extern "C" fn (this: *const nsILoadGroup, aRequests: *mut *const nsISimpleEnumerator) -> nsresult,

    /* readonly attribute unsigned long activeCount; */
    pub get_activeCount: unsafe extern "C" fn (this: *const nsILoadGroup, aActiveCount: *mut libc::uint32_t) -> nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub get_notificationCallbacks: unsafe extern "C" fn (this: *const nsILoadGroup, aNotificationCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,
    pub set_notificationCallbacks: unsafe extern "C" fn (this: *const nsILoadGroup, aNotificationCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* [noscript] readonly attribute unsigned long long requestContextID; */
    pub get_requestContextID: unsafe extern "C" fn (this: *const nsILoadGroup, aRequestContextID: *mut libc::uint64_t) -> nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub get_defaultLoadFlags: unsafe extern "C" fn (this: *const nsILoadGroup, aDefaultLoadFlags: *mut nsLoadFlags) -> nsresult,
    pub set_defaultLoadFlags: unsafe extern "C" fn (this: *const nsILoadGroup, aDefaultLoadFlags: nsLoadFlags) -> nsresult,

    /* attribute ACString userAgentOverrideCache; */
    pub get_userAgentOverrideCache: unsafe extern "C" fn (this: *const nsILoadGroup, aUserAgentOverrideCache: *mut nsACString) -> nsresult,
    pub set_userAgentOverrideCache: unsafe extern "C" fn (this: *const nsILoadGroup, aUserAgentOverrideCache: *const nsACString) -> nsresult,

}


impl nsILoadGroup {
    /* attribute nsIRequestObserver groupObserver; */
    #[inline]
    pub unsafe fn get_groupObserver(&self, ) -> Result<Option<RefPtr<nsIRequestObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_groupObserver)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_groupObserver(&self, aGroupObserver: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).set_groupObserver)(self as *const _, aGroupObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIRequest defaultLoadRequest; */
    #[inline]
    pub unsafe fn get_defaultLoadRequest(&self, ) -> Result<Option<RefPtr<nsIRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultLoadRequest)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_defaultLoadRequest(&self, aDefaultLoadRequest: Option<&nsIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultLoadRequest)(self as *const _, aDefaultLoadRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addRequest (in nsIRequest aRequest, in nsISupports aContext); */
    #[inline]
    pub unsafe fn addRequest(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).addRequest)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus); */
    #[inline]
    pub unsafe fn removeRequest(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).removeRequest)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISimpleEnumerator requests; */
    #[inline]
    pub unsafe fn get_requests(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_requests)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long activeCount; */
    #[inline]
    pub unsafe fn get_activeCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_activeCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    #[inline]
    pub unsafe fn get_notificationCallbacks(&self, ) -> Result<Option<RefPtr<nsIInterfaceRequestor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_notificationCallbacks)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_notificationCallbacks(&self, aNotificationCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).set_notificationCallbacks)(self as *const _, aNotificationCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute unsigned long long requestContextID; */
    #[inline]
    pub unsafe fn get_requestContextID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_requestContextID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsLoadFlags defaultLoadFlags; */
    #[inline]
    pub unsafe fn get_defaultLoadFlags(&self, ) -> Result<nsLoadFlags, nsresult> {
        let mut _retval: nsLoadFlags = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultLoadFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultLoadFlags(&self, aDefaultLoadFlags: nsLoadFlags) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultLoadFlags)(self as *const _, aDefaultLoadFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString userAgentOverrideCache; */
    #[inline]
    pub unsafe fn get_userAgentOverrideCache(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_userAgentOverrideCache)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_userAgentOverrideCache(&self, aUserAgentOverrideCache: &[u8]) -> Result<(), nsresult> {
        let aUserAgentOverrideCache = nsCString::from(aUserAgentOverrideCache);
        match ((*self.vtable).set_userAgentOverrideCache)(self as *const _, &*aUserAgentOverrideCache) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


