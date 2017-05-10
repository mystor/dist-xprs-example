//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIncrementalDownload.idl
//


#[repr(C)]
pub struct nsIIncrementalDownload {
    vtable: *const nsIIncrementalDownloadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIncrementalDownload {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6687823f, 0x56c4, 0x461d,
            [0x93, 0xa1, 0x7f, 0x6c, 0xb7, 0xdf, 0xbf, 0xba])
    }
}

unsafe impl RefCounted for nsIIncrementalDownload {
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
pub trait nsIIncrementalDownloadCoerce {
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self;
}

impl nsIIncrementalDownloadCoerce for nsIIncrementalDownload {
    #[inline]
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self {
        v
    }
}

impl nsIIncrementalDownload {
    #[inline]
    pub fn coerce<T: nsIIncrementalDownloadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIncrementalDownload {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsIIncrementalDownloadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIncrementalDownloadVTable {
    pub __base: nsIRequestVTable,

    /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
    pub init: unsafe extern "C" fn (this: *const nsIIncrementalDownload, uri: *const nsIURI, destination: *const nsIFile, chunkSize: libc::int32_t, intervalInSeconds: libc::int32_t) -> nsresult,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIIncrementalDownload, aURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI finalURI; */
    pub get_finalURI: unsafe extern "C" fn (this: *const nsIIncrementalDownload, aFinalURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIFile destination; */
    pub get_destination: unsafe extern "C" fn (this: *const nsIIncrementalDownload, aDestination: *mut *const nsIFile) -> nsresult,

    /* readonly attribute long long totalSize; */
    pub get_totalSize: unsafe extern "C" fn (this: *const nsIIncrementalDownload, aTotalSize: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long long currentSize; */
    pub get_currentSize: unsafe extern "C" fn (this: *const nsIIncrementalDownload, aCurrentSize: *mut libc::int64_t) -> nsresult,

    /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
    pub start: unsafe extern "C" fn (this: *const nsIIncrementalDownload, observer: *const nsIRequestObserver, ctxt: *const nsISupports) -> nsresult,

}


impl nsIIncrementalDownload {
    /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
    #[inline]
    pub unsafe fn init(&self, uri: Option<&nsIURI>, destination: Option<&nsIFile>, chunkSize: libc::int32_t, intervalInSeconds: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), destination.map_or(::std::ptr::null(), |x| x as *const _), chunkSize, intervalInSeconds) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIURI URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_URI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI finalURI; */
    #[inline]
    pub unsafe fn get_finalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_finalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIFile destination; */
    #[inline]
    pub unsafe fn get_destination(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_destination)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long long totalSize; */
    #[inline]
    pub unsafe fn get_totalSize(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_totalSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long currentSize; */
    #[inline]
    pub unsafe fn get_currentSize(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_currentSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
    #[inline]
    pub unsafe fn start(&self, observer: Option<&nsIRequestObserver>, ctxt: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).start)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), ctxt.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


