//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedResultService.idl
//


#[repr(C)]
pub struct nsIFeedResultService {
    vtable: *const nsIFeedResultServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedResultService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x95309fd2, 0x7b3a, 0x47fb,
            [0x97, 0xf3, 0x5c, 0x46, 0x0d, 0x94, 0x73, 0xcd])
    }
}

unsafe impl RefCounted for nsIFeedResultService {
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
pub trait nsIFeedResultServiceCoerce {
    fn coerce_from(v: &nsIFeedResultService) -> &Self;
}

impl nsIFeedResultServiceCoerce for nsIFeedResultService {
    #[inline]
    fn coerce_from(v: &nsIFeedResultService) -> &Self {
        v
    }
}

impl nsIFeedResultService {
    #[inline]
    pub fn coerce<T: nsIFeedResultServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedResultService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFeedResultServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedResultService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedResultServiceVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean forcePreviewPage; */
    pub get_forcePreviewPage: unsafe extern "C" fn (this: *const nsIFeedResultService, aForcePreviewPage: *mut bool) -> nsresult,
    pub set_forcePreviewPage: unsafe extern "C" fn (this: *const nsIFeedResultService, aForcePreviewPage: bool) -> nsresult,

    /* void addToClientReader (in AUTF8String uri, in AString title, in AString subtitle, in unsigned long feedType, [optional] in AString feedReader); */
    pub addToClientReader: unsafe extern "C" fn (this: *const nsIFeedResultService, uri: *const nsACString, title: *const nsAString, subtitle: *const nsAString, feedType: libc::uint32_t, feedReader: *const nsAString) -> nsresult,

    /* void addFeedResult (in nsIFeedResult feedResult); */
    pub addFeedResult: unsafe extern "C" fn (this: *const nsIFeedResultService, feedResult: *const nsIFeedResult) -> nsresult,

    /* nsIFeedResult getFeedResult (in nsIURI uri); */
    pub getFeedResult: unsafe extern "C" fn (this: *const nsIFeedResultService, uri: *const nsIURI, _retval: *mut *const nsIFeedResult) -> nsresult,

    /* void removeFeedResult (in nsIURI uri); */
    pub removeFeedResult: unsafe extern "C" fn (this: *const nsIFeedResultService, uri: *const nsIURI) -> nsresult,

}


impl nsIFeedResultService {
    /* attribute boolean forcePreviewPage; */
    #[inline]
    pub unsafe fn get_forcePreviewPage(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forcePreviewPage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_forcePreviewPage(&self, aForcePreviewPage: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_forcePreviewPage)(self as *const _, aForcePreviewPage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addToClientReader (in AUTF8String uri, in AString title, in AString subtitle, in unsigned long feedType, [optional] in AString feedReader); */
    #[inline]
    pub unsafe fn addToClientReader(&self, uri: &[u8], title: &[u16], subtitle: &[u16], feedType: libc::uint32_t, feedReader: &[u16]) -> Result<(), nsresult> {
        let uri = nsCString::from(uri);
        let title = nsString::from(title);
        let subtitle = nsString::from(subtitle);
        let feedReader = nsString::from(feedReader);
        match ((*self.vtable).addToClientReader)(self as *const _, &*uri, &*title, &*subtitle, feedType, &*feedReader) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addFeedResult (in nsIFeedResult feedResult); */
    #[inline]
    pub unsafe fn addFeedResult(&self, feedResult: Option<&nsIFeedResult>) -> Result<(), nsresult> {

        match ((*self.vtable).addFeedResult)(self as *const _, feedResult.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIFeedResult getFeedResult (in nsIURI uri); */
    #[inline]
    pub unsafe fn getFeedResult(&self, uri: Option<&nsIURI>) -> Result<Option<RefPtr<nsIFeedResult>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFeedResult)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeFeedResult (in nsIURI uri); */
    #[inline]
    pub unsafe fn removeFeedResult(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).removeFeedResult)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


