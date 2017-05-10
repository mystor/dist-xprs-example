//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadProgressListener.idl
//


#[repr(C)]
pub struct nsIDownloadProgressListener {
    vtable: *const nsIDownloadProgressListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadProgressListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7acb07ea, 0xcac2, 0x4c15,
            [0xa3, 0xad, 0x23, 0xaa, 0xa7, 0x89, 0xed, 0x51])
    }
}

unsafe impl RefCounted for nsIDownloadProgressListener {
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
pub trait nsIDownloadProgressListenerCoerce {
    fn coerce_from(v: &nsIDownloadProgressListener) -> &Self;
}

impl nsIDownloadProgressListenerCoerce for nsIDownloadProgressListener {
    #[inline]
    fn coerce_from(v: &nsIDownloadProgressListener) -> &Self {
        v
    }
}

impl nsIDownloadProgressListener {
    #[inline]
    pub fn coerce<T: nsIDownloadProgressListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadProgressListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadProgressListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadProgressListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadProgressListenerVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIDOMDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aDocument: *mut *const nsIDOMDocument) -> nsresult,
    pub set_document: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aDocument: *const nsIDOMDocument) -> nsresult,

    /* void onDownloadStateChange (in short aState, in nsIDownload aDownload); */
    pub onDownloadStateChange: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aState: libc::int16_t, aDownload: *const nsIDownload) -> nsresult,

    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus, in nsIDownload aDownload); */
    pub onStateChange: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aStateFlags: libc::uint32_t, aStatus: nsresult, aDownload: *const nsIDownload) -> nsresult,

    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress, in nsIDownload aDownload); */
    pub onProgressChange: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aCurSelfProgress: libc::int64_t, aMaxSelfProgress: libc::int64_t, aCurTotalProgress: libc::int64_t, aMaxTotalProgress: libc::int64_t, aDownload: *const nsIDownload) -> nsresult,

    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState, in nsIDownload aDownload); */
    pub onSecurityChange: unsafe extern "C" fn (this: *const nsIDownloadProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aState: libc::uint32_t, aDownload: *const nsIDownload) -> nsresult,

}


impl nsIDownloadProgressListener {
    /* attribute nsIDOMDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_document(&self, aDocument: Option<&nsIDOMDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).set_document)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onDownloadStateChange (in short aState, in nsIDownload aDownload); */
    #[inline]
    pub unsafe fn onDownloadStateChange(&self, aState: libc::int16_t, aDownload: Option<&nsIDownload>) -> Result<(), nsresult> {

        match ((*self.vtable).onDownloadStateChange)(self as *const _, aState, aDownload.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus, in nsIDownload aDownload); */
    #[inline]
    pub unsafe fn onStateChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aStateFlags: libc::uint32_t, aStatus: nsresult, aDownload: Option<&nsIDownload>) -> Result<(), nsresult> {

        match ((*self.vtable).onStateChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aStateFlags, aStatus, aDownload.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress, in nsIDownload aDownload); */
    #[inline]
    pub unsafe fn onProgressChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aCurSelfProgress: libc::int64_t, aMaxSelfProgress: libc::int64_t, aCurTotalProgress: libc::int64_t, aMaxTotalProgress: libc::int64_t, aDownload: Option<&nsIDownload>) -> Result<(), nsresult> {

        match ((*self.vtable).onProgressChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress, aDownload.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState, in nsIDownload aDownload); */
    #[inline]
    pub unsafe fn onSecurityChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aState: libc::uint32_t, aDownload: Option<&nsIDownload>) -> Result<(), nsresult> {

        match ((*self.vtable).onSecurityChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aState, aDownload.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


