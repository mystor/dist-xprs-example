//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannel.idl
//


pub mod nsIHttpChannel_consts {
    pub const REFERRER_POLICY_UNSET: i64 = 0;
    pub const REFERRER_POLICY_NO_REFERRER_WHEN_DOWNGRADE: i64 = 1;
    pub const REFERRER_POLICY_NO_REFERRER: i64 = 2;
    pub const REFERRER_POLICY_ORIGIN: i64 = 3;
    pub const REFERRER_POLICY_ORIGIN_WHEN_XORIGIN: i64 = 4;
    pub const REFERRER_POLICY_UNSAFE_URL: i64 = 5;
    pub const REFERRER_POLICY_SAME_ORIGIN: i64 = 6;
    pub const REFERRER_POLICY_STRICT_ORIGIN: i64 = 7;
    pub const REFERRER_POLICY_STRICT_ORIGIN_WHEN_XORIGIN: i64 = 8;
}


#[repr(C)]
pub struct nsIHttpChannel {
    vtable: *const nsIHttpChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc5a4a073, 0x4539, 0x49c7,
            [0xa3, 0xf2, 0xce, 0xc3, 0xf0, 0x61, 0x9c, 0x6c])
    }
}

unsafe impl RefCounted for nsIHttpChannel {
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
pub trait nsIHttpChannelCoerce {
    fn coerce_from(v: &nsIHttpChannel) -> &Self;
}

impl nsIHttpChannelCoerce for nsIHttpChannel {
    #[inline]
    fn coerce_from(v: &nsIHttpChannel) -> &Self {
        v
    }
}

impl nsIHttpChannel {
    #[inline]
    pub fn coerce<T: nsIHttpChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpChannel {
    type Target = nsIChannel;
    #[inline]
    fn deref(&self) -> &nsIChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIChannelCoerce> nsIHttpChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpChannelVTable {
    pub __base: nsIChannelVTable,

    /* [must_use] attribute ACString requestMethod; */
    pub get_requestMethod: unsafe extern "C" fn (this: *const nsIHttpChannel, aRequestMethod: *mut nsACString) -> nsresult,
    pub set_requestMethod: unsafe extern "C" fn (this: *const nsIHttpChannel, aRequestMethod: *const nsACString) -> nsresult,

    /* [must_use] attribute nsIURI referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIHttpChannel, aReferrer: *mut *const nsIURI) -> nsresult,
    pub set_referrer: unsafe extern "C" fn (this: *const nsIHttpChannel, aReferrer: *const nsIURI) -> nsresult,

    /* [must_use] readonly attribute unsigned long referrerPolicy; */
    pub get_referrerPolicy: unsafe extern "C" fn (this: *const nsIHttpChannel, aReferrerPolicy: *mut libc::uint32_t) -> nsresult,

    /* [must_use] void setReferrerWithPolicy (in nsIURI referrer, in unsigned long referrerPolicy); */
    pub setReferrerWithPolicy: unsafe extern "C" fn (this: *const nsIHttpChannel, referrer: *const nsIURI, referrerPolicy: libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute ACString protocolVersion; */
    pub get_protocolVersion: unsafe extern "C" fn (this: *const nsIHttpChannel, aProtocolVersion: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute uint64_t transferSize; */
    pub get_transferSize: unsafe extern "C" fn (this: *const nsIHttpChannel, aTransferSize: *mut uint64_t) -> nsresult,

    /* [must_use] readonly attribute uint64_t decodedBodySize; */
    pub get_decodedBodySize: unsafe extern "C" fn (this: *const nsIHttpChannel, aDecodedBodySize: *mut uint64_t) -> nsresult,

    /* [must_use] readonly attribute uint64_t encodedBodySize; */
    pub get_encodedBodySize: unsafe extern "C" fn (this: *const nsIHttpChannel, aEncodedBodySize: *mut uint64_t) -> nsresult,

    /* [must_use] ACString getRequestHeader (in ACString aHeader); */
    pub getRequestHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, aHeader: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
    pub setRequestHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, aHeader: *const nsACString, aValue: *const nsACString, aMerge: bool) -> nsresult,

    /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
    pub setEmptyRequestHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, aHeader: *const nsACString) -> nsresult,

    /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub visitRequestHeaders: unsafe extern "C" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub visitNonDefaultRequestHeaders: unsafe extern "C" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* [must_use] attribute boolean allowPipelining; */
    pub get_allowPipelining: unsafe extern "C" fn (this: *const nsIHttpChannel, aAllowPipelining: *mut bool) -> nsresult,
    pub set_allowPipelining: unsafe extern "C" fn (this: *const nsIHttpChannel, aAllowPipelining: bool) -> nsresult,

    /* [must_use] attribute boolean allowSTS; */
    pub get_allowSTS: unsafe extern "C" fn (this: *const nsIHttpChannel, aAllowSTS: *mut bool) -> nsresult,
    pub set_allowSTS: unsafe extern "C" fn (this: *const nsIHttpChannel, aAllowSTS: bool) -> nsresult,

    /* [must_use] attribute unsigned long redirectionLimit; */
    pub get_redirectionLimit: unsafe extern "C" fn (this: *const nsIHttpChannel, aRedirectionLimit: *mut libc::uint32_t) -> nsresult,
    pub set_redirectionLimit: unsafe extern "C" fn (this: *const nsIHttpChannel, aRedirectionLimit: libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute unsigned long responseStatus; */
    pub get_responseStatus: unsafe extern "C" fn (this: *const nsIHttpChannel, aResponseStatus: *mut libc::uint32_t) -> nsresult,

    /* [must_use] readonly attribute ACString responseStatusText; */
    pub get_responseStatusText: unsafe extern "C" fn (this: *const nsIHttpChannel, aResponseStatusText: *mut nsACString) -> nsresult,

    /* [must_use] readonly attribute boolean requestSucceeded; */
    pub get_requestSucceeded: unsafe extern "C" fn (this: *const nsIHttpChannel, aRequestSucceeded: *mut bool) -> nsresult,

    /* [must_use] attribute boolean isMainDocumentChannel; */
    pub get_isMainDocumentChannel: unsafe extern "C" fn (this: *const nsIHttpChannel, aIsMainDocumentChannel: *mut bool) -> nsresult,
    pub set_isMainDocumentChannel: unsafe extern "C" fn (this: *const nsIHttpChannel, aIsMainDocumentChannel: bool) -> nsresult,

    /* [must_use] ACString getResponseHeader (in ACString header); */
    pub getResponseHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, header: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
    pub setResponseHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, header: *const nsACString, value: *const nsACString, merge: bool) -> nsresult,

    /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub visitResponseHeaders: unsafe extern "C" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
    pub getOriginalResponseHeader: unsafe extern "C" fn (this: *const nsIHttpChannel, aHeader: *const nsACString, aVisitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    pub visitOriginalResponseHeaders: unsafe extern "C" fn (this: *const nsIHttpChannel, aVisitor: *const nsIHttpHeaderVisitor) -> nsresult,

    /* [must_use] boolean isNoStoreResponse (); */
    pub isNoStoreResponse: unsafe extern "C" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> nsresult,

    /* [must_use] boolean isNoCacheResponse (); */
    pub isNoCacheResponse: unsafe extern "C" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> nsresult,

    /* [must_use] boolean isPrivateResponse (); */
    pub isPrivateResponse: unsafe extern "C" fn (this: *const nsIHttpChannel, _retval: *mut bool) -> nsresult,

    /* [must_use] void redirectTo (in nsIURI aTargetURI); */
    pub redirectTo: unsafe extern "C" fn (this: *const nsIHttpChannel, aTargetURI: *const nsIURI) -> nsresult,

    /* [must_use,noscript] attribute uint64_t requestContextID; */
    pub get_requestContextID: unsafe extern "C" fn (this: *const nsIHttpChannel, aRequestContextID: *mut uint64_t) -> nsresult,
    pub set_requestContextID: unsafe extern "C" fn (this: *const nsIHttpChannel, aRequestContextID: uint64_t) -> nsresult,

    /* [must_use] attribute uint64_t channelId; */
    pub get_channelId: unsafe extern "C" fn (this: *const nsIHttpChannel, aChannelId: *mut uint64_t) -> nsresult,
    pub set_channelId: unsafe extern "C" fn (this: *const nsIHttpChannel, aChannelId: uint64_t) -> nsresult,

    /* [must_use] attribute uint64_t topLevelContentWindowId; */
    pub get_topLevelContentWindowId: unsafe extern "C" fn (this: *const nsIHttpChannel, aTopLevelContentWindowId: *mut uint64_t) -> nsresult,
    pub set_topLevelContentWindowId: unsafe extern "C" fn (this: *const nsIHttpChannel, aTopLevelContentWindowId: uint64_t) -> nsresult,

    /* [infallible] readonly attribute boolean isTrackingResource; */
    pub get_isTrackingResource: unsafe extern "C" fn (this: *const nsIHttpChannel, aIsTrackingResource: *mut bool) -> nsresult,

    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
    pub get_topLevelOuterContentWindowId: unsafe extern "C" fn (this: *const nsIHttpChannel, aTopLevelOuterContentWindowId: *mut uint64_t) -> nsresult,
    pub set_topLevelOuterContentWindowId: unsafe extern "C" fn (this: *const nsIHttpChannel, aTopLevelOuterContentWindowId: uint64_t) -> nsresult,

}


impl nsIHttpChannel {
    /* [must_use] attribute ACString requestMethod; */
    #[inline]
    pub unsafe fn get_requestMethod(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_requestMethod)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_requestMethod(&self, aRequestMethod: &[u8]) -> Result<(), nsresult> {
        let aRequestMethod = nsCString::from(aRequestMethod);
        match ((*self.vtable).set_requestMethod)(self as *const _, &*aRequestMethod) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute nsIURI referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_referrer(&self, aReferrer: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_referrer)(self as *const _, aReferrer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute unsigned long referrerPolicy; */
    #[inline]
    pub unsafe fn get_referrerPolicy(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_referrerPolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setReferrerWithPolicy (in nsIURI referrer, in unsigned long referrerPolicy); */
    #[inline]
    pub unsafe fn setReferrerWithPolicy(&self, referrer: Option<&nsIURI>, referrerPolicy: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setReferrerWithPolicy)(self as *const _, referrer.map_or(::std::ptr::null(), |x| x as *const _), referrerPolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute ACString protocolVersion; */
    #[inline]
    pub unsafe fn get_protocolVersion(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_protocolVersion)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute uint64_t transferSize; */
    #[inline]
    pub unsafe fn get_transferSize(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_transferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute uint64_t decodedBodySize; */
    #[inline]
    pub unsafe fn get_decodedBodySize(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_decodedBodySize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute uint64_t encodedBodySize; */
    #[inline]
    pub unsafe fn get_encodedBodySize(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_encodedBodySize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] ACString getRequestHeader (in ACString aHeader); */
    #[inline]
    pub unsafe fn getRequestHeader(&self, aHeader: &[u8]) -> Result<nsCString, nsresult> {
        let aHeader = nsCString::from(aHeader);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getRequestHeader)(self as *const _, &*aHeader, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
    #[inline]
    pub unsafe fn setRequestHeader(&self, aHeader: &[u8], aValue: &[u8], aMerge: bool) -> Result<(), nsresult> {
        let aHeader = nsCString::from(aHeader);
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).setRequestHeader)(self as *const _, &*aHeader, &*aValue, aMerge) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
    #[inline]
    pub unsafe fn setEmptyRequestHeader(&self, aHeader: &[u8]) -> Result<(), nsresult> {
        let aHeader = nsCString::from(aHeader);
        match ((*self.vtable).setEmptyRequestHeader)(self as *const _, &*aHeader) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitRequestHeaders(&self, aVisitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitRequestHeaders)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitNonDefaultRequestHeaders(&self, aVisitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitNonDefaultRequestHeaders)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean allowPipelining; */
    #[inline]
    pub unsafe fn get_allowPipelining(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowPipelining)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowPipelining(&self, aAllowPipelining: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowPipelining)(self as *const _, aAllowPipelining) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute boolean allowSTS; */
    #[inline]
    pub unsafe fn get_allowSTS(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowSTS)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowSTS(&self, aAllowSTS: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowSTS)(self as *const _, aAllowSTS) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute unsigned long redirectionLimit; */
    #[inline]
    pub unsafe fn get_redirectionLimit(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_redirectionLimit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_redirectionLimit(&self, aRedirectionLimit: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_redirectionLimit)(self as *const _, aRedirectionLimit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute unsigned long responseStatus; */
    #[inline]
    pub unsafe fn get_responseStatus(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_responseStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute ACString responseStatusText; */
    #[inline]
    pub unsafe fn get_responseStatusText(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_responseStatusText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] readonly attribute boolean requestSucceeded; */
    #[inline]
    pub unsafe fn get_requestSucceeded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_requestSucceeded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] attribute boolean isMainDocumentChannel; */
    #[inline]
    pub unsafe fn get_isMainDocumentChannel(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isMainDocumentChannel)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isMainDocumentChannel(&self, aIsMainDocumentChannel: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isMainDocumentChannel)(self as *const _, aIsMainDocumentChannel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] ACString getResponseHeader (in ACString header); */
    #[inline]
    pub unsafe fn getResponseHeader(&self, header: &[u8]) -> Result<nsCString, nsresult> {
        let header = nsCString::from(header);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getResponseHeader)(self as *const _, &*header, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
    #[inline]
    pub unsafe fn setResponseHeader(&self, header: &[u8], value: &[u8], merge: bool) -> Result<(), nsresult> {
        let header = nsCString::from(header);
        let value = nsCString::from(value);
        match ((*self.vtable).setResponseHeader)(self as *const _, &*header, &*value, merge) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitResponseHeaders(&self, aVisitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitResponseHeaders)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
    #[inline]
    pub unsafe fn getOriginalResponseHeader(&self, aHeader: &[u8], aVisitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {
        let aHeader = nsCString::from(aHeader);
        match ((*self.vtable).getOriginalResponseHeader)(self as *const _, &*aHeader, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
    #[inline]
    pub unsafe fn visitOriginalResponseHeaders(&self, aVisitor: Option<&nsIHttpHeaderVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitOriginalResponseHeaders)(self as *const _, aVisitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] boolean isNoStoreResponse (); */
    #[inline]
    pub unsafe fn isNoStoreResponse(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isNoStoreResponse)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] boolean isNoCacheResponse (); */
    #[inline]
    pub unsafe fn isNoCacheResponse(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isNoCacheResponse)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] boolean isPrivateResponse (); */
    #[inline]
    pub unsafe fn isPrivateResponse(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPrivateResponse)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] void redirectTo (in nsIURI aTargetURI); */
    #[inline]
    pub unsafe fn redirectTo(&self, aTargetURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).redirectTo)(self as *const _, aTargetURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use,noscript] attribute uint64_t requestContextID; */
    #[inline]
    pub unsafe fn get_requestContextID(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_requestContextID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_requestContextID(&self, aRequestContextID: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_requestContextID)(self as *const _, aRequestContextID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute uint64_t channelId; */
    #[inline]
    pub unsafe fn get_channelId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_channelId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_channelId(&self, aChannelId: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_channelId)(self as *const _, aChannelId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] attribute uint64_t topLevelContentWindowId; */
    #[inline]
    pub unsafe fn get_topLevelContentWindowId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_topLevelContentWindowId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_topLevelContentWindowId(&self, aTopLevelContentWindowId: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_topLevelContentWindowId)(self as *const _, aTopLevelContentWindowId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isTrackingResource; */
    #[inline]
    pub unsafe fn get_isTrackingResource(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTrackingResource)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
    #[inline]
    pub unsafe fn get_topLevelOuterContentWindowId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_topLevelOuterContentWindowId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_topLevelOuterContentWindowId(&self, aTopLevelOuterContentWindowId: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_topLevelOuterContentWindowId)(self as *const _, aTopLevelOuterContentWindowId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


