//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierStreamUpdater.idl
//


#[repr(C)]
pub struct nsIUrlClassifierStreamUpdater {
    vtable: *const nsIUrlClassifierStreamUpdaterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierStreamUpdater {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe1797597, 0xf4d6, 0x4dd3,
            [0xa1, 0xe1, 0x74, 0x5a, 0xd3, 0x52, 0xcd, 0x80])
    }
}

unsafe impl RefCounted for nsIUrlClassifierStreamUpdater {
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
pub trait nsIUrlClassifierStreamUpdaterCoerce {
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self;
}

impl nsIUrlClassifierStreamUpdaterCoerce for nsIUrlClassifierStreamUpdater {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self {
        v
    }
}

impl nsIUrlClassifierStreamUpdater {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierStreamUpdaterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierStreamUpdater {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierStreamUpdaterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierStreamUpdaterVTable {
    pub __base: nsISupportsVTable,

    /* boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback); */
    pub downloadUpdates: unsafe extern "C" fn (this: *const nsIUrlClassifierStreamUpdater, aRequestTables: *const nsACString, aRequestPayload: *const nsACString, aIsPostRequest: bool, aUpdateUrl: *const nsACString, aSuccessCallback: *const nsIUrlClassifierCallback, aUpdateErrorCallback: *const nsIUrlClassifierCallback, aDownloadErrorCallback: *const nsIUrlClassifierCallback, _retval: *mut bool) -> nsresult,

}


impl nsIUrlClassifierStreamUpdater {
    /* boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback); */
    #[inline]
    pub unsafe fn downloadUpdates(&self, aRequestTables: &[u8], aRequestPayload: &[u8], aIsPostRequest: bool, aUpdateUrl: &[u8], aSuccessCallback: Option<&nsIUrlClassifierCallback>, aUpdateErrorCallback: Option<&nsIUrlClassifierCallback>, aDownloadErrorCallback: Option<&nsIUrlClassifierCallback>) -> Result<bool, nsresult> {
        let aRequestTables = nsCString::from(aRequestTables);
        let aRequestPayload = nsCString::from(aRequestPayload);
        let aUpdateUrl = nsCString::from(aUpdateUrl);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).downloadUpdates)(self as *const _, &*aRequestTables, &*aRequestPayload, aIsPostRequest, &*aUpdateUrl, aSuccessCallback.map_or(::std::ptr::null(), |x| x as *const _), aUpdateErrorCallback.map_or(::std::ptr::null(), |x| x as *const _), aDownloadErrorCallback.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


