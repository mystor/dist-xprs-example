//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDownloader.idl
//


#[repr(C)]
pub struct nsIDownloader {
    vtable: *const nsIDownloaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfafe41a9, 0xa531, 0x4d6d,
            [0x89, 0xbc, 0x58, 0x8a, 0x65, 0x22, 0xfb, 0x4e])
    }
}

unsafe impl RefCounted for nsIDownloader {
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
pub trait nsIDownloaderCoerce {
    fn coerce_from(v: &nsIDownloader) -> &Self;
}

impl nsIDownloaderCoerce for nsIDownloader {
    #[inline]
    fn coerce_from(v: &nsIDownloader) -> &Self {
        v
    }
}

impl nsIDownloader {
    #[inline]
    pub fn coerce<T: nsIDownloaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIDownloaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloaderVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIDownloadObserver observer, in nsIFile downloadLocation); */
    pub init: unsafe extern "C" fn (this: *const nsIDownloader, observer: *const nsIDownloadObserver, downloadLocation: *const nsIFile) -> nsresult,

}


impl nsIDownloader {
    /* void init (in nsIDownloadObserver observer, in nsIFile downloadLocation); */
    #[inline]
    pub unsafe fn init(&self, observer: Option<&nsIDownloadObserver>, downloadLocation: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), downloadLocation.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIDownloadObserver {
    vtable: *const nsIDownloadObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDownloadObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44b3153e, 0xa54e, 0x4077,
            [0xa5, 0x27, 0xb0, 0x32, 0x5e, 0x40, 0x92, 0x4e])
    }
}

unsafe impl RefCounted for nsIDownloadObserver {
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
pub trait nsIDownloadObserverCoerce {
    fn coerce_from(v: &nsIDownloadObserver) -> &Self;
}

impl nsIDownloadObserverCoerce for nsIDownloadObserver {
    #[inline]
    fn coerce_from(v: &nsIDownloadObserver) -> &Self {
        v
    }
}

impl nsIDownloadObserver {
    #[inline]
    pub fn coerce<T: nsIDownloadObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDownloadObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDownloadObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDownloadObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result); */
    pub onDownloadComplete: unsafe extern "C" fn (this: *const nsIDownloadObserver, downloader: *const nsIDownloader, request: *const nsIRequest, ctxt: *const nsISupports, status: nsresult, result: *const nsIFile) -> nsresult,

}


impl nsIDownloadObserver {
    /* void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result); */
    #[inline]
    pub unsafe fn onDownloadComplete(&self, downloader: Option<&nsIDownloader>, request: Option<&nsIRequest>, ctxt: Option<&nsISupports>, status: nsresult, result: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).onDownloadComplete)(self as *const _, downloader.map_or(::std::ptr::null(), |x| x as *const _), request.map_or(::std::ptr::null(), |x| x as *const _), ctxt.map_or(::std::ptr::null(), |x| x as *const _), status, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


