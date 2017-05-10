//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChannel.idl
//


pub mod nsIChannel_consts {
    pub const LOAD_DOCUMENT_URI: i64 = 65536;
    pub const LOAD_RETARGETED_DOCUMENT_URI: i64 = 131072;
    pub const LOAD_REPLACE: i64 = 262144;
    pub const LOAD_INITIAL_DOCUMENT_URI: i64 = 524288;
    pub const LOAD_TARGETED: i64 = 1048576;
    pub const LOAD_CALL_CONTENT_SNIFFERS: i64 = 2097152;
    pub const LOAD_CLASSIFY_URI: i64 = 4194304;
    pub const LOAD_MEDIA_SNIFFER_OVERRIDES_CONTENT_TYPE: i64 = 8388608;
    pub const LOAD_EXPLICIT_CREDENTIALS: i64 = 16777216;
    pub const LOAD_BYPASS_SERVICE_WORKER: i64 = 33554432;
    pub const DISPOSITION_INLINE: i64 = 0;
    pub const DISPOSITION_ATTACHMENT: i64 = 1;
}


#[repr(C)]
pub struct nsIChannel {
    vtable: *const nsIChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2c389865, 0x23db, 0x4aa7,
            [0x9f, 0xe5, 0x60, 0xcc, 0x7b, 0x00, 0x69, 0x7e])
    }
}

unsafe impl RefCounted for nsIChannel {
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
pub trait nsIChannelCoerce {
    fn coerce_from(v: &nsIChannel) -> &Self;
}

impl nsIChannelCoerce for nsIChannel {
    #[inline]
    fn coerce_from(v: &nsIChannel) -> &Self {
        v
    }
}

impl nsIChannel {
    #[inline]
    pub fn coerce<T: nsIChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIChannel {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsIChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIChannelVTable {
    pub __base: nsIRequestVTable,

    /* attribute nsIURI originalURI; */
    pub get_originalURI: unsafe extern "C" fn (this: *const nsIChannel, aOriginalURI: *mut *const nsIURI) -> nsresult,
    pub set_originalURI: unsafe extern "C" fn (this: *const nsIChannel, aOriginalURI: *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIChannel, aURI: *mut *const nsIURI) -> nsresult,

    /* attribute nsISupports owner; */
    pub get_owner: unsafe extern "C" fn (this: *const nsIChannel, aOwner: *mut *const nsISupports) -> nsresult,
    pub set_owner: unsafe extern "C" fn (this: *const nsIChannel, aOwner: *const nsISupports) -> nsresult,

    /* attribute nsIInterfaceRequestor notificationCallbacks; */
    pub get_notificationCallbacks: unsafe extern "C" fn (this: *const nsIChannel, aNotificationCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,
    pub set_notificationCallbacks: unsafe extern "C" fn (this: *const nsIChannel, aNotificationCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* readonly attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsIChannel, aSecurityInfo: *mut *const nsISupports) -> nsresult,

    /* attribute ACString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIChannel, aContentType: *mut nsACString) -> nsresult,
    pub set_contentType: unsafe extern "C" fn (this: *const nsIChannel, aContentType: *const nsACString) -> nsresult,

    /* attribute ACString contentCharset; */
    pub get_contentCharset: unsafe extern "C" fn (this: *const nsIChannel, aContentCharset: *mut nsACString) -> nsresult,
    pub set_contentCharset: unsafe extern "C" fn (this: *const nsIChannel, aContentCharset: *const nsACString) -> nsresult,

    /* attribute int64_t contentLength; */
    pub get_contentLength: unsafe extern "C" fn (this: *const nsIChannel, aContentLength: *mut int64_t) -> nsresult,
    pub set_contentLength: unsafe extern "C" fn (this: *const nsIChannel, aContentLength: int64_t) -> nsresult,

    /* nsIInputStream open (); */
    pub open: unsafe extern "C" fn (this: *const nsIChannel, _retval: *mut *const nsIInputStream) -> nsresult,

    /* nsIInputStream open2 (); */
    pub open2: unsafe extern "C" fn (this: *const nsIChannel, _retval: *mut *const nsIInputStream) -> nsresult,

    /* void asyncOpen (in nsIStreamListener aListener, in nsISupports aContext); */
    pub asyncOpen: unsafe extern "C" fn (this: *const nsIChannel, aListener: *const nsIStreamListener, aContext: *const nsISupports) -> nsresult,

    /* void asyncOpen2 (in nsIStreamListener aListener); */
    pub asyncOpen2: unsafe extern "C" fn (this: *const nsIChannel, aListener: *const nsIStreamListener) -> nsresult,

    /* attribute unsigned long contentDisposition; */
    pub get_contentDisposition: unsafe extern "C" fn (this: *const nsIChannel, aContentDisposition: *mut libc::uint32_t) -> nsresult,
    pub set_contentDisposition: unsafe extern "C" fn (this: *const nsIChannel, aContentDisposition: libc::uint32_t) -> nsresult,

    /* attribute AString contentDispositionFilename; */
    pub get_contentDispositionFilename: unsafe extern "C" fn (this: *const nsIChannel, aContentDispositionFilename: *mut nsAString) -> nsresult,
    pub set_contentDispositionFilename: unsafe extern "C" fn (this: *const nsIChannel, aContentDispositionFilename: *const nsAString) -> nsresult,

    /* readonly attribute ACString contentDispositionHeader; */
    pub get_contentDispositionHeader: unsafe extern "C" fn (this: *const nsIChannel, aContentDispositionHeader: *mut nsACString) -> nsresult,

    /* attribute nsILoadInfo loadInfo; */
    pub get_loadInfo: unsafe extern "C" fn (this: *const nsIChannel, aLoadInfo: *mut *const nsILoadInfo) -> nsresult,
    pub set_loadInfo: unsafe extern "C" fn (this: *const nsIChannel, aLoadInfo: *const nsILoadInfo) -> nsresult,

    /* readonly attribute bool isDocument; */
    pub get_isDocument: unsafe extern "C" fn (this: *const nsIChannel, aIsDocument: *mut bool) -> nsresult,

}


impl nsIChannel {
    /* attribute nsIURI originalURI; */
    #[inline]
    pub unsafe fn get_originalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_originalURI(&self, aOriginalURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_originalURI)(self as *const _, aOriginalURI.map_or(::std::ptr::null(), |x| x as *const _)) {
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

    /* attribute nsISupports owner; */
    #[inline]
    pub unsafe fn get_owner(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_owner)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_owner(&self, aOwner: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_owner)(self as *const _, aOwner.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* readonly attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute ACString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentType(&self, aContentType: &[u8]) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).set_contentType)(self as *const _, &*aContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString contentCharset; */
    #[inline]
    pub unsafe fn get_contentCharset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentCharset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentCharset(&self, aContentCharset: &[u8]) -> Result<(), nsresult> {
        let aContentCharset = nsCString::from(aContentCharset);
        match ((*self.vtable).set_contentCharset)(self as *const _, &*aContentCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute int64_t contentLength; */
    #[inline]
    pub unsafe fn get_contentLength(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_contentLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentLength(&self, aContentLength: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentLength)(self as *const _, aContentLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIInputStream open (); */
    #[inline]
    pub unsafe fn open(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).open)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIInputStream open2 (); */
    #[inline]
    pub unsafe fn open2(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).open2)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void asyncOpen (in nsIStreamListener aListener, in nsISupports aContext); */
    #[inline]
    pub unsafe fn asyncOpen(&self, aListener: Option<&nsIStreamListener>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncOpen)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncOpen2 (in nsIStreamListener aListener); */
    #[inline]
    pub unsafe fn asyncOpen2(&self, aListener: Option<&nsIStreamListener>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncOpen2)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long contentDisposition; */
    #[inline]
    pub unsafe fn get_contentDisposition(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_contentDisposition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentDisposition(&self, aContentDisposition: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentDisposition)(self as *const _, aContentDisposition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString contentDispositionFilename; */
    #[inline]
    pub unsafe fn get_contentDispositionFilename(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_contentDispositionFilename)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentDispositionFilename(&self, aContentDispositionFilename: &[u16]) -> Result<(), nsresult> {
        let aContentDispositionFilename = nsString::from(aContentDispositionFilename);
        match ((*self.vtable).set_contentDispositionFilename)(self as *const _, &*aContentDispositionFilename) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute ACString contentDispositionHeader; */
    #[inline]
    pub unsafe fn get_contentDispositionHeader(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentDispositionHeader)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsILoadInfo loadInfo; */
    #[inline]
    pub unsafe fn get_loadInfo(&self, ) -> Result<Option<RefPtr<nsILoadInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_loadInfo(&self, aLoadInfo: Option<&nsILoadInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadInfo)(self as *const _, aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool isDocument; */
    #[inline]
    pub unsafe fn get_isDocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


