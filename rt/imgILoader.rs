//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/imgILoader.idl
//


pub mod imgILoader_consts {
    pub const LOAD_CORS_ANONYMOUS: i64 = 65536;
    pub const LOAD_CORS_USE_CREDENTIALS: i64 = 131072;
}


#[repr(C)]
pub struct imgILoader {
    vtable: *const imgILoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for imgILoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe61377d2, 0x910e, 0x4c65,
            [0xa6, 0x4b, 0x42, 0x8d, 0x15, 0x0e, 0x1f, 0xd1])
    }
}

unsafe impl RefCounted for imgILoader {
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
pub trait imgILoaderCoerce {
    fn coerce_from(v: &imgILoader) -> &Self;
}

impl imgILoaderCoerce for imgILoader {
    #[inline]
    fn coerce_from(v: &imgILoader) -> &Self {
        v
    }
}

impl imgILoader {
    #[inline]
    pub fn coerce<T: imgILoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for imgILoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> imgILoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &imgILoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct imgILoaderVTable {
    pub __base: nsISupportsVTable,

    /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIURI aReferrerURI, in AString aReferrerPolicy, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in nsISupports aCX, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
    pub loadImageXPCOM: unsafe extern "C" fn (this: *const imgILoader, aURI: *const nsIURI, aInitialDocumentURL: *const nsIURI, aReferrerURI: *const nsIURI, aReferrerPolicy: *const nsAString, aLoadingPrincipal: *const nsIPrincipal, aLoadGroup: *const nsILoadGroup, aObserver: *const imgINotificationObserver, aCX: *const nsISupports, aLoadFlags: nsLoadFlags, cacheKey: *const nsISupports, aContentPolicyType: nsContentPolicyType, _retval: *mut *const imgIRequest) -> nsresult,

    /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in nsISupports cx, out nsIStreamListener aListener); */
    pub loadImageWithChannelXPCOM: unsafe extern "C" fn (this: *const imgILoader, aChannel: *const nsIChannel, aObserver: *const imgINotificationObserver, cx: *const nsISupports, aListener: *mut *const nsIStreamListener, _retval: *mut *const imgIRequest) -> nsresult,

}


impl imgILoader {
    /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIURI aReferrerURI, in AString aReferrerPolicy, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in nsISupports aCX, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
    #[inline]
    pub unsafe fn loadImageXPCOM(&self, aURI: Option<&nsIURI>, aInitialDocumentURL: Option<&nsIURI>, aReferrerURI: Option<&nsIURI>, aReferrerPolicy: &[u16], aLoadingPrincipal: Option<&nsIPrincipal>, aLoadGroup: Option<&nsILoadGroup>, aObserver: Option<&imgINotificationObserver>, aCX: Option<&nsISupports>, aLoadFlags: nsLoadFlags, cacheKey: Option<&nsISupports>, aContentPolicyType: nsContentPolicyType) -> Result<Option<RefPtr<imgIRequest>>, nsresult> {
        let aReferrerPolicy = nsString::from(aReferrerPolicy);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).loadImageXPCOM)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aInitialDocumentURL.map_or(::std::ptr::null(), |x| x as *const _), aReferrerURI.map_or(::std::ptr::null(), |x| x as *const _), &*aReferrerPolicy, aLoadingPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aLoadGroup.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _), aCX.map_or(::std::ptr::null(), |x| x as *const _), aLoadFlags, cacheKey.map_or(::std::ptr::null(), |x| x as *const _), aContentPolicyType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in nsISupports cx, out nsIStreamListener aListener); */
    #[inline]
    pub unsafe fn loadImageWithChannelXPCOM(&self, aChannel: Option<&nsIChannel>, aObserver: Option<&imgINotificationObserver>, cx: Option<&nsISupports>) -> Result<(Option<RefPtr<nsIStreamListener>>, Option<RefPtr<imgIRequest>>), nsresult> {
        let mut aListener = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).loadImageWithChannelXPCOM)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _), cx.map_or(::std::ptr::null(), |x| x as *const _), aListener.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aListener.refptr(), _retval.refptr()))
    }

}


