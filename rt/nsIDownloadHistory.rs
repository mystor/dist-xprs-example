//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloadHistory.idl
//


#[repr(C)]
pub struct nsIDownloadHistory {
    vtable: *const nsIDownloadHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4dcd6a12, 0xa091, 0x4f38,
            [0x83, 0x60, 0x02, 0x29, 0x29, 0x63, 0x57, 0x46])
    }
}

unsafe impl RefCounted for nsIDownloadHistory {
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
pub trait nsIDownloadHistoryCoerce {
    fn coerce_from(v: &nsIDownloadHistory) -> &Self;
}

impl nsIDownloadHistoryCoerce for nsIDownloadHistory {
    #[inline]
    fn coerce_from(v: &nsIDownloadHistory) -> &Self {
        v
    }
}

impl nsIDownloadHistory {
    #[inline]
    pub fn coerce<T: nsIDownloadHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadHistoryVTable {
    pub __base: nsISupportsVTable,

    /* void addDownload (in nsIURI aSource, [optional] in nsIURI aReferrer, [optional] in PRTime aStartTime, [optional] in nsIURI aDestination); */
    pub addDownload: unsafe extern "C" fn (this: *const nsIDownloadHistory, aSource: *const nsIURI, aReferrer: *const nsIURI, aStartTime: PRTime, aDestination: *const nsIURI) -> nsresult,

    /* void removeAllDownloads (); */
    pub removeAllDownloads: unsafe extern "C" fn (this: *const nsIDownloadHistory) -> nsresult,

}


impl nsIDownloadHistory {
    /* void addDownload (in nsIURI aSource, [optional] in nsIURI aReferrer, [optional] in PRTime aStartTime, [optional] in nsIURI aDestination); */
    #[inline]
    pub unsafe fn addDownload(&self, aSource: Option<&nsIURI>, aReferrer: Option<&nsIURI>, aStartTime: PRTime, aDestination: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).addDownload)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aStartTime, aDestination.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllDownloads (); */
    #[inline]
    pub unsafe fn removeAllDownloads(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllDownloads)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


