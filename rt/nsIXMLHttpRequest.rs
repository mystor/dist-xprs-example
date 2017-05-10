//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXMLHttpRequest.idl
//


#[repr(C)]
pub struct nsIXMLHttpRequestEventTarget {
    vtable: *const nsIXMLHttpRequestEventTargetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXMLHttpRequestEventTarget {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x88e7d2a0, 0x2e5b, 0x4f65,
            [0x96, 0x24, 0xa6, 0x1e, 0x60, 0x7a, 0x99, 0x48])
    }
}

unsafe impl RefCounted for nsIXMLHttpRequestEventTarget {
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
pub trait nsIXMLHttpRequestEventTargetCoerce {
    fn coerce_from(v: &nsIXMLHttpRequestEventTarget) -> &Self;
}

impl nsIXMLHttpRequestEventTargetCoerce for nsIXMLHttpRequestEventTarget {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequestEventTarget) -> &Self {
        v
    }
}

impl nsIXMLHttpRequestEventTarget {
    #[inline]
    pub fn coerce<T: nsIXMLHttpRequestEventTargetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXMLHttpRequestEventTarget {
    type Target = nsIDOMEventTarget;
    #[inline]
    fn deref(&self) -> &nsIDOMEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMEventTargetCoerce> nsIXMLHttpRequestEventTargetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequestEventTarget) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXMLHttpRequestEventTargetVTable {
    pub __base: nsIDOMEventTargetVTable,

}


impl nsIXMLHttpRequestEventTarget {
}


#[repr(C)]
pub struct nsIXMLHttpRequestUpload {
    vtable: *const nsIXMLHttpRequestUploadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXMLHttpRequestUpload {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd74c4dc4, 0xbc8c, 0x4f5d,
            [0xb7, 0xf1, 0x12, 0x1a, 0x48, 0x75, 0x0a, 0xbe])
    }
}

unsafe impl RefCounted for nsIXMLHttpRequestUpload {
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
pub trait nsIXMLHttpRequestUploadCoerce {
    fn coerce_from(v: &nsIXMLHttpRequestUpload) -> &Self;
}

impl nsIXMLHttpRequestUploadCoerce for nsIXMLHttpRequestUpload {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequestUpload) -> &Self {
        v
    }
}

impl nsIXMLHttpRequestUpload {
    #[inline]
    pub fn coerce<T: nsIXMLHttpRequestUploadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXMLHttpRequestUpload {
    type Target = nsIXMLHttpRequestEventTarget;
    #[inline]
    fn deref(&self) -> &nsIXMLHttpRequestEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXMLHttpRequestEventTargetCoerce> nsIXMLHttpRequestUploadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequestUpload) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXMLHttpRequestUploadVTable {
    pub __base: nsIXMLHttpRequestEventTargetVTable,

}


impl nsIXMLHttpRequestUpload {
}


pub mod nsIXMLHttpRequest_consts {
    pub const UNSENT: i64 = 0;
    pub const OPENED: i64 = 1;
    pub const HEADERS_RECEIVED: i64 = 2;
    pub const LOADING: i64 = 3;
    pub const DONE: i64 = 4;
}


#[repr(C)]
pub struct nsIXMLHttpRequest {
    vtable: *const nsIXMLHttpRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXMLHttpRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6f54214c, 0x7175, 0x498d,
            [0x9d, 0x2d, 0x04, 0x29, 0xe3, 0x8c, 0x28, 0x69])
    }
}

unsafe impl RefCounted for nsIXMLHttpRequest {
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
pub trait nsIXMLHttpRequestCoerce {
    fn coerce_from(v: &nsIXMLHttpRequest) -> &Self;
}

impl nsIXMLHttpRequestCoerce for nsIXMLHttpRequest {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequest) -> &Self {
        v
    }
}

impl nsIXMLHttpRequest {
    #[inline]
    pub fn coerce<T: nsIXMLHttpRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXMLHttpRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXMLHttpRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXMLHttpRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXMLHttpRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIChannel channel; */
    pub get_channel: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aChannel: *mut *const nsIChannel) -> nsresult,

    /* readonly attribute nsIDOMDocument responseXML; */
    pub get_responseXML: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aResponseXML: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute AString responseText; */
    pub get_responseText: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aResponseText: *mut nsAString) -> nsresult,

    /* attribute AString responseType; */
    pub get_responseType: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aResponseType: *mut nsAString) -> nsresult,
    pub set_responseType: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aResponseType: *const nsAString) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval response; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_response: *const ::libc::c_void,

    /* readonly attribute unsigned long status; */
    pub get_status: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aStatus: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute ACString statusText; */
    pub get_statusText: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aStatusText: *mut nsACString) -> nsresult,

    /* [binaryname(SlowAbort)] void abort (); */
    pub SlowAbort: unsafe extern "C" fn (this: *const nsIXMLHttpRequest) -> nsresult,

    /* ACString getAllResponseHeaders (); */
    pub getAllResponseHeaders: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, _retval: *mut nsACString) -> nsresult,

    /* ACString getResponseHeader (in ACString header); */
    pub getResponseHeader: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, header: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* [optional_argc] void open (in ACString method, in AUTF8String url, [optional] in boolean async, [optional, Undefined (Empty)] in DOMString user, [optional, Undefined (Empty)] in DOMString password); */
    /// Unable to call function as its signature contains a non-rust type
    pub open: *const ::libc::c_void,

    /* void send ([optional] in nsIVariant body); */
    pub send: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, body: *const nsIVariant) -> nsresult,

    /* void setRequestHeader (in ACString header, in ACString value); */
    pub setRequestHeader: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, header: *const nsACString, value: *const nsACString) -> nsresult,

    /* attribute unsigned long timeout; */
    pub get_timeout: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aTimeout: *mut libc::uint32_t) -> nsresult,
    pub set_timeout: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aTimeout: libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned short readyState; */
    pub get_readyState: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aReadyState: *mut libc::uint16_t) -> nsresult,

    /* [binaryname(SlowOverrideMimeType)] void overrideMimeType (in DOMString mimetype); */
    pub SlowOverrideMimeType: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, mimetype: *const nsAString) -> nsresult,

    /* attribute boolean mozBackgroundRequest; */
    pub get_mozBackgroundRequest: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aMozBackgroundRequest: *mut bool) -> nsresult,
    pub set_mozBackgroundRequest: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aMozBackgroundRequest: bool) -> nsresult,

    /* attribute boolean withCredentials; */
    pub get_withCredentials: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aWithCredentials: *mut bool) -> nsresult,
    pub set_withCredentials: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aWithCredentials: bool) -> nsresult,

    /* [noscript] void init (in nsIPrincipal principal, in nsIGlobalObject globalObject, in nsIURI baseURI, [optional] in nsILoadGroup loadGroup); */
    pub init: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, principal: *const nsIPrincipal, globalObject: *const nsIGlobalObject, baseURI: *const nsIURI, loadGroup: *const nsILoadGroup) -> nsresult,

    /* readonly attribute nsIXMLHttpRequestUpload upload; */
    pub get_upload: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aUpload: *mut *const nsIXMLHttpRequestUpload) -> nsresult,

    /* [implicit_jscontext] attribute jsval onreadystatechange; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_onreadystatechange: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_onreadystatechange: *const ::libc::c_void,

    /* readonly attribute boolean mozAnon; */
    pub get_mozAnon: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aMozAnon: *mut bool) -> nsresult,

    /* readonly attribute boolean mozSystem; */
    pub get_mozSystem: unsafe extern "C" fn (this: *const nsIXMLHttpRequest, aMozSystem: *mut bool) -> nsresult,

}


impl nsIXMLHttpRequest {
    /* readonly attribute nsIChannel channel; */
    #[inline]
    pub unsafe fn get_channel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_channel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMDocument responseXML; */
    #[inline]
    pub unsafe fn get_responseXML(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_responseXML)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString responseText; */
    #[inline]
    pub unsafe fn get_responseText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_responseText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString responseType; */
    #[inline]
    pub unsafe fn get_responseType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_responseType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_responseType(&self, aResponseType: &[u16]) -> Result<(), nsresult> {
        let aResponseType = nsString::from(aResponseType);
        match ((*self.vtable).set_responseType)(self as *const _, &*aResponseType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] readonly attribute jsval response; */


    /* readonly attribute unsigned long status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_status)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString statusText; */
    #[inline]
    pub unsafe fn get_statusText(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_statusText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(SlowAbort)] void abort (); */
    #[inline]
    pub unsafe fn SlowAbort(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SlowAbort)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString getAllResponseHeaders (); */
    #[inline]
    pub unsafe fn getAllResponseHeaders(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAllResponseHeaders)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getResponseHeader (in ACString header); */
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

    /* [optional_argc] void open (in ACString method, in AUTF8String url, [optional] in boolean async, [optional, Undefined (Empty)] in DOMString user, [optional, Undefined (Empty)] in DOMString password); */


    /* void send ([optional] in nsIVariant body); */
    #[inline]
    pub unsafe fn send(&self, body: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).send)(self as *const _, body.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRequestHeader (in ACString header, in ACString value); */
    #[inline]
    pub unsafe fn setRequestHeader(&self, header: &[u8], value: &[u8]) -> Result<(), nsresult> {
        let header = nsCString::from(header);
        let value = nsCString::from(value);
        match ((*self.vtable).setRequestHeader)(self as *const _, &*header, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long timeout; */
    #[inline]
    pub unsafe fn get_timeout(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeout(&self, aTimeout: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeout)(self as *const _, aTimeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short readyState; */
    #[inline]
    pub unsafe fn get_readyState(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_readyState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(SlowOverrideMimeType)] void overrideMimeType (in DOMString mimetype); */
    #[inline]
    pub unsafe fn SlowOverrideMimeType(&self, mimetype: &[u16]) -> Result<(), nsresult> {
        let mimetype = nsString::from(mimetype);
        match ((*self.vtable).SlowOverrideMimeType)(self as *const _, &*mimetype) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean mozBackgroundRequest; */
    #[inline]
    pub unsafe fn get_mozBackgroundRequest(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozBackgroundRequest)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_mozBackgroundRequest(&self, aMozBackgroundRequest: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_mozBackgroundRequest)(self as *const _, aMozBackgroundRequest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean withCredentials; */
    #[inline]
    pub unsafe fn get_withCredentials(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_withCredentials)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_withCredentials(&self, aWithCredentials: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_withCredentials)(self as *const _, aWithCredentials) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void init (in nsIPrincipal principal, in nsIGlobalObject globalObject, in nsIURI baseURI, [optional] in nsILoadGroup loadGroup); */
    #[inline]
    pub unsafe fn init(&self, principal: Option<&nsIPrincipal>, globalObject: Option<&nsIGlobalObject>, baseURI: Option<&nsIURI>, loadGroup: Option<&nsILoadGroup>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), globalObject.map_or(::std::ptr::null(), |x| x as *const _), baseURI.map_or(::std::ptr::null(), |x| x as *const _), loadGroup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIXMLHttpRequestUpload upload; */
    #[inline]
    pub unsafe fn get_upload(&self, ) -> Result<Option<RefPtr<nsIXMLHttpRequestUpload>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_upload)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] attribute jsval onreadystatechange; */



    /* readonly attribute boolean mozAnon; */
    #[inline]
    pub unsafe fn get_mozAnon(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozAnon)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean mozSystem; */
    #[inline]
    pub unsafe fn get_mozSystem(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozSystem)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXHRSendable {
    vtable: *const nsIXHRSendableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXHRSendable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x840d0d00, 0xe83e, 0x4a29,
            [0xb3, 0xc7, 0x67, 0xe9, 0x6e, 0x90, 0xa4, 0x99])
    }
}

unsafe impl RefCounted for nsIXHRSendable {
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
pub trait nsIXHRSendableCoerce {
    fn coerce_from(v: &nsIXHRSendable) -> &Self;
}

impl nsIXHRSendableCoerce for nsIXHRSendable {
    #[inline]
    fn coerce_from(v: &nsIXHRSendable) -> &Self {
        v
    }
}

impl nsIXHRSendable {
    #[inline]
    pub fn coerce<T: nsIXHRSendableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXHRSendable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXHRSendableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXHRSendable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXHRSendableVTable {
    pub __base: nsISupportsVTable,

    /* void getSendInfo (out nsIInputStream body, out uint64_t contentLength, out ACString contentTypeWithCharset, out ACString charset); */
    pub getSendInfo: unsafe extern "C" fn (this: *const nsIXHRSendable, body: *mut *const nsIInputStream, contentLength: *mut uint64_t, contentTypeWithCharset: *mut nsACString, charset: *mut nsACString) -> nsresult,

}


impl nsIXHRSendable {
    /* void getSendInfo (out nsIInputStream body, out uint64_t contentLength, out ACString contentTypeWithCharset, out ACString charset); */
    #[inline]
    pub unsafe fn getSendInfo(&self, ) -> Result<(Option<RefPtr<nsIInputStream>>, uint64_t, nsCString, nsCString), nsresult> {
        let mut body = GetterAddrefs::new();
        let mut contentLength: uint64_t = ::std::mem::zeroed();
        let mut contentTypeWithCharset = nsCString::new();
        let mut charset = nsCString::new();
        match ((*self.vtable).getSendInfo)(self as *const _, body.ptr(), &mut contentLength as *mut _, &mut *contentTypeWithCharset, &mut *charset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((body.refptr(), contentLength, contentTypeWithCharset, charset))
    }

}


#[repr(C)]
pub struct nsIJSXMLHttpRequest {
    vtable: *const nsIJSXMLHttpRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIJSXMLHttpRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8ae70a39, 0xedf1, 0x40b4,
            [0xa9, 0x92, 0x47, 0x2d, 0x23, 0x42, 0x1c, 0x25])
    }
}

unsafe impl RefCounted for nsIJSXMLHttpRequest {
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
pub trait nsIJSXMLHttpRequestCoerce {
    fn coerce_from(v: &nsIJSXMLHttpRequest) -> &Self;
}

impl nsIJSXMLHttpRequestCoerce for nsIJSXMLHttpRequest {
    #[inline]
    fn coerce_from(v: &nsIJSXMLHttpRequest) -> &Self {
        v
    }
}

impl nsIJSXMLHttpRequest {
    #[inline]
    pub fn coerce<T: nsIJSXMLHttpRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIJSXMLHttpRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIJSXMLHttpRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSXMLHttpRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIJSXMLHttpRequestVTable {
    pub __base: nsISupportsVTable,

}


impl nsIJSXMLHttpRequest {
}


