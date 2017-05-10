//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpServer.idl
//


#[repr(C)]
pub struct nsIHttpServer {
    vtable: *const nsIHttpServerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpServer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcea8812e, 0xfaa6, 0x4013,
            [0x93, 0x96, 0xf9, 0x93, 0x6c, 0xbb, 0x74, 0xec])
    }
}

unsafe impl RefCounted for nsIHttpServer {
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
pub trait nsIHttpServerCoerce {
    fn coerce_from(v: &nsIHttpServer) -> &Self;
}

impl nsIHttpServerCoerce for nsIHttpServer {
    #[inline]
    fn coerce_from(v: &nsIHttpServer) -> &Self {
        v
    }
}

impl nsIHttpServer {
    #[inline]
    pub fn coerce<T: nsIHttpServerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpServer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpServerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpServerVTable {
    pub __base: nsISupportsVTable,

    /* void start (in long port); */
    pub start: unsafe extern "C" fn (this: *const nsIHttpServer, port: libc::int32_t) -> nsresult,

    /* void stop (in nsIHttpServerStoppedCallback callback); */
    pub stop: unsafe extern "C" fn (this: *const nsIHttpServer, callback: *const nsIHttpServerStoppedCallback) -> nsresult,

    /* void registerFile (in string path, in nsIFile file); */
    pub registerFile: unsafe extern "C" fn (this: *const nsIHttpServer, path: *const libc::c_char, file: *const nsIFile) -> nsresult,

    /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
    pub registerPathHandler: unsafe extern "C" fn (this: *const nsIHttpServer, path: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> nsresult,

    /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
    pub registerPrefixHandler: unsafe extern "C" fn (this: *const nsIHttpServer, prefix: *const libc::c_char, handler: *const nsIHttpRequestHandler) -> nsresult,

    /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
    pub registerErrorHandler: unsafe extern "C" fn (this: *const nsIHttpServer, code: libc::uint32_t, handler: *const nsIHttpRequestHandler) -> nsresult,

    /* void registerDirectory (in string path, in nsIFile dir); */
    pub registerDirectory: unsafe extern "C" fn (this: *const nsIHttpServer, path: *const libc::c_char, dir: *const nsIFile) -> nsresult,

    /* void registerContentType (in string extension, in string type); */
    pub registerContentType: unsafe extern "C" fn (this: *const nsIHttpServer, extension: *const libc::c_char, type_: *const libc::c_char) -> nsresult,

    /* void setIndexHandler (in nsIHttpRequestHandler handler); */
    pub setIndexHandler: unsafe extern "C" fn (this: *const nsIHttpServer, handler: *const nsIHttpRequestHandler) -> nsresult,

    /* readonly attribute nsIHttpServerIdentity identity; */
    pub get_identity: unsafe extern "C" fn (this: *const nsIHttpServer, aIdentity: *mut *const nsIHttpServerIdentity) -> nsresult,

    /* AString getState (in AString path, in AString key); */
    pub getState: unsafe extern "C" fn (this: *const nsIHttpServer, path: *const nsAString, key: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setState (in AString path, in AString key, in AString value); */
    pub setState: unsafe extern "C" fn (this: *const nsIHttpServer, path: *const nsAString, key: *const nsAString, value: *const nsAString) -> nsresult,

    /* AString getSharedState (in AString key); */
    pub getSharedState: unsafe extern "C" fn (this: *const nsIHttpServer, key: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setSharedState (in AString key, in AString value); */
    pub setSharedState: unsafe extern "C" fn (this: *const nsIHttpServer, key: *const nsAString, value: *const nsAString) -> nsresult,

    /* nsISupports getObjectState (in AString key); */
    pub getObjectState: unsafe extern "C" fn (this: *const nsIHttpServer, key: *const nsAString, _retval: *mut *const nsISupports) -> nsresult,

    /* void setObjectState (in AString key, in nsISupports value); */
    pub setObjectState: unsafe extern "C" fn (this: *const nsIHttpServer, key: *const nsAString, value: *const nsISupports) -> nsresult,

}


impl nsIHttpServer {
    /* void start (in long port); */
    #[inline]
    pub unsafe fn start(&self, port: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).start)(self as *const _, port) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stop (in nsIHttpServerStoppedCallback callback); */
    #[inline]
    pub unsafe fn stop(&self, callback: Option<&nsIHttpServerStoppedCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerFile (in string path, in nsIFile file); */
    #[inline]
    pub unsafe fn registerFile(&self, path: *const libc::c_char, file: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).registerFile)(self as *const _, path, file.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
    #[inline]
    pub unsafe fn registerPathHandler(&self, path: *const libc::c_char, handler: Option<&nsIHttpRequestHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).registerPathHandler)(self as *const _, path, handler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
    #[inline]
    pub unsafe fn registerPrefixHandler(&self, prefix: *const libc::c_char, handler: Option<&nsIHttpRequestHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).registerPrefixHandler)(self as *const _, prefix, handler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
    #[inline]
    pub unsafe fn registerErrorHandler(&self, code: libc::uint32_t, handler: Option<&nsIHttpRequestHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).registerErrorHandler)(self as *const _, code, handler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerDirectory (in string path, in nsIFile dir); */
    #[inline]
    pub unsafe fn registerDirectory(&self, path: *const libc::c_char, dir: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).registerDirectory)(self as *const _, path, dir.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerContentType (in string extension, in string type); */
    #[inline]
    pub unsafe fn registerContentType(&self, extension: *const libc::c_char, type_: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).registerContentType)(self as *const _, extension, type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setIndexHandler (in nsIHttpRequestHandler handler); */
    #[inline]
    pub unsafe fn setIndexHandler(&self, handler: Option<&nsIHttpRequestHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).setIndexHandler)(self as *const _, handler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIHttpServerIdentity identity; */
    #[inline]
    pub unsafe fn get_identity(&self, ) -> Result<Option<RefPtr<nsIHttpServerIdentity>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_identity)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getState (in AString path, in AString key); */
    #[inline]
    pub unsafe fn getState(&self, path: &[u16], key: &[u16]) -> Result<nsString, nsresult> {
        let path = nsString::from(path);
        let key = nsString::from(key);
        let mut _retval = nsString::new();
        match ((*self.vtable).getState)(self as *const _, &*path, &*key, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setState (in AString path, in AString key, in AString value); */
    #[inline]
    pub unsafe fn setState(&self, path: &[u16], key: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let path = nsString::from(path);
        let key = nsString::from(key);
        let value = nsString::from(value);
        match ((*self.vtable).setState)(self as *const _, &*path, &*key, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getSharedState (in AString key); */
    #[inline]
    pub unsafe fn getSharedState(&self, key: &[u16]) -> Result<nsString, nsresult> {
        let key = nsString::from(key);
        let mut _retval = nsString::new();
        match ((*self.vtable).getSharedState)(self as *const _, &*key, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setSharedState (in AString key, in AString value); */
    #[inline]
    pub unsafe fn setSharedState(&self, key: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let key = nsString::from(key);
        let value = nsString::from(value);
        match ((*self.vtable).setSharedState)(self as *const _, &*key, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISupports getObjectState (in AString key); */
    #[inline]
    pub unsafe fn getObjectState(&self, key: &[u16]) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let key = nsString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getObjectState)(self as *const _, &*key, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setObjectState (in AString key, in nsISupports value); */
    #[inline]
    pub unsafe fn setObjectState(&self, key: &[u16], value: Option<&nsISupports>) -> Result<(), nsresult> {
        let key = nsString::from(key);
        match ((*self.vtable).setObjectState)(self as *const _, &*key, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHttpServerStoppedCallback {
    vtable: *const nsIHttpServerStoppedCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpServerStoppedCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x925a6d33, 0x9937, 0x4c63,
            [0xab, 0xe1, 0xa1, 0xc5, 0x6a, 0x98, 0x64, 0x55])
    }
}

unsafe impl RefCounted for nsIHttpServerStoppedCallback {
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
pub trait nsIHttpServerStoppedCallbackCoerce {
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self;
}

impl nsIHttpServerStoppedCallbackCoerce for nsIHttpServerStoppedCallback {
    #[inline]
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self {
        v
    }
}

impl nsIHttpServerStoppedCallback {
    #[inline]
    pub fn coerce<T: nsIHttpServerStoppedCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpServerStoppedCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpServerStoppedCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServerStoppedCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpServerStoppedCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onStopped (); */
    pub onStopped: unsafe extern "C" fn (this: *const nsIHttpServerStoppedCallback) -> nsresult,

}


impl nsIHttpServerStoppedCallback {
    /* void onStopped (); */
    #[inline]
    pub unsafe fn onStopped(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onStopped)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHttpServerIdentity {
    vtable: *const nsIHttpServerIdentityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpServerIdentity {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa89de175, 0xae8e, 0x4c46,
            [0x91, 0xa5, 0x0d, 0xba, 0x99, 0xbb, 0xd2, 0x84])
    }
}

unsafe impl RefCounted for nsIHttpServerIdentity {
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
pub trait nsIHttpServerIdentityCoerce {
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self;
}

impl nsIHttpServerIdentityCoerce for nsIHttpServerIdentity {
    #[inline]
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self {
        v
    }
}

impl nsIHttpServerIdentity {
    #[inline]
    pub fn coerce<T: nsIHttpServerIdentityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpServerIdentity {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpServerIdentityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpServerIdentity) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpServerIdentityVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string primaryScheme; */
    pub get_primaryScheme: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, aPrimaryScheme: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string primaryHost; */
    pub get_primaryHost: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, aPrimaryHost: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute long primaryPort; */
    pub get_primaryPort: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, aPrimaryPort: *mut libc::int32_t) -> nsresult,

    /* void add (in string scheme, in string host, in long port); */
    pub add: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> nsresult,

    /* boolean remove (in string scheme, in string host, in long port); */
    pub remove: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean has (in string scheme, in string host, in long port); */
    pub has: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* string getScheme (in string host, in long port); */
    pub getScheme: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, host: *const libc::c_char, port: libc::int32_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* void setPrimary (in string scheme, in string host, in long port); */
    pub setPrimary: unsafe extern "C" fn (this: *const nsIHttpServerIdentity, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> nsresult,

}


impl nsIHttpServerIdentity {
    /* readonly attribute string primaryScheme; */
    #[inline]
    pub unsafe fn get_primaryScheme(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_primaryScheme)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string primaryHost; */
    #[inline]
    pub unsafe fn get_primaryHost(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_primaryHost)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long primaryPort; */
    #[inline]
    pub unsafe fn get_primaryPort(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_primaryPort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void add (in string scheme, in string host, in long port); */
    #[inline]
    pub unsafe fn add(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, scheme, host, port) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean remove (in string scheme, in string host, in long port); */
    #[inline]
    pub unsafe fn remove(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).remove)(self as *const _, scheme, host, port, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean has (in string scheme, in string host, in long port); */
    #[inline]
    pub unsafe fn has(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).has)(self as *const _, scheme, host, port, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string getScheme (in string host, in long port); */
    #[inline]
    pub unsafe fn getScheme(&self, host: *const libc::c_char, port: libc::int32_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getScheme)(self as *const _, host, port, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setPrimary (in string scheme, in string host, in long port); */
    #[inline]
    pub unsafe fn setPrimary(&self, scheme: *const libc::c_char, host: *const libc::c_char, port: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setPrimary)(self as *const _, scheme, host, port) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHttpRequestHandler {
    vtable: *const nsIHttpRequestHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpRequestHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2bbb4db7, 0xd285, 0x42b3,
            [0xa3, 0xce, 0x14, 0x2b, 0x8c, 0xc7, 0xe1, 0x39])
    }
}

unsafe impl RefCounted for nsIHttpRequestHandler {
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
pub trait nsIHttpRequestHandlerCoerce {
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self;
}

impl nsIHttpRequestHandlerCoerce for nsIHttpRequestHandler {
    #[inline]
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self {
        v
    }
}

impl nsIHttpRequestHandler {
    #[inline]
    pub fn coerce<T: nsIHttpRequestHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpRequestHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpRequestHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpRequestHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpRequestHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
    pub handle: unsafe extern "C" fn (this: *const nsIHttpRequestHandler, request: *const nsIHttpRequest, response: *const nsIHttpResponse) -> nsresult,

}


impl nsIHttpRequestHandler {
    /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
    #[inline]
    pub unsafe fn handle(&self, request: Option<&nsIHttpRequest>, response: Option<&nsIHttpResponse>) -> Result<(), nsresult> {

        match ((*self.vtable).handle)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _), response.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHttpRequest {
    vtable: *const nsIHttpRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x978cf30e, 0xad73, 0x42ee,
            [0x8f, 0x22, 0xfe, 0x0a, 0xaf, 0x1b, 0xf5, 0xd2])
    }
}

unsafe impl RefCounted for nsIHttpRequest {
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
pub trait nsIHttpRequestCoerce {
    fn coerce_from(v: &nsIHttpRequest) -> &Self;
}

impl nsIHttpRequestCoerce for nsIHttpRequest {
    #[inline]
    fn coerce_from(v: &nsIHttpRequest) -> &Self {
        v
    }
}

impl nsIHttpRequest {
    #[inline]
    pub fn coerce<T: nsIHttpRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpRequestVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string method; */
    pub get_method: unsafe extern "C" fn (this: *const nsIHttpRequest, aMethod: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string scheme; */
    pub get_scheme: unsafe extern "C" fn (this: *const nsIHttpRequest, aScheme: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string host; */
    pub get_host: unsafe extern "C" fn (this: *const nsIHttpRequest, aHost: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute unsigned long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIHttpRequest, aPort: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute string path; */
    pub get_path: unsafe extern "C" fn (this: *const nsIHttpRequest, aPath: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string queryString; */
    pub get_queryString: unsafe extern "C" fn (this: *const nsIHttpRequest, aQueryString: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute string httpVersion; */
    pub get_httpVersion: unsafe extern "C" fn (this: *const nsIHttpRequest, aHttpVersion: *mut *const libc::c_char) -> nsresult,

    /* string getHeader (in string fieldName); */
    pub getHeader: unsafe extern "C" fn (this: *const nsIHttpRequest, fieldName: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* boolean hasHeader (in string fieldName); */
    pub hasHeader: unsafe extern "C" fn (this: *const nsIHttpRequest, fieldName: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsISimpleEnumerator headers; */
    pub get_headers: unsafe extern "C" fn (this: *const nsIHttpRequest, aHeaders: *mut *const nsISimpleEnumerator) -> nsresult,

    /* readonly attribute nsIInputStream bodyInputStream; */
    pub get_bodyInputStream: unsafe extern "C" fn (this: *const nsIHttpRequest, aBodyInputStream: *mut *const nsIInputStream) -> nsresult,

}


impl nsIHttpRequest {
    /* readonly attribute string method; */
    #[inline]
    pub unsafe fn get_method(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_method)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string scheme; */
    #[inline]
    pub unsafe fn get_scheme(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_scheme)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_host)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string path; */
    #[inline]
    pub unsafe fn get_path(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_path)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string queryString; */
    #[inline]
    pub unsafe fn get_queryString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_queryString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute string httpVersion; */
    #[inline]
    pub unsafe fn get_httpVersion(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_httpVersion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string getHeader (in string fieldName); */
    #[inline]
    pub unsafe fn getHeader(&self, fieldName: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getHeader)(self as *const _, fieldName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasHeader (in string fieldName); */
    #[inline]
    pub unsafe fn hasHeader(&self, fieldName: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasHeader)(self as *const _, fieldName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsISimpleEnumerator headers; */
    #[inline]
    pub unsafe fn get_headers(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_headers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIInputStream bodyInputStream; */
    #[inline]
    pub unsafe fn get_bodyInputStream(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_bodyInputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIHttpResponse {
    vtable: *const nsIHttpResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpResponse {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1acd16c2, 0xdc59, 0x42fa,
            [0x91, 0x60, 0x4f, 0x26, 0xc4, 0x3c, 0x1c, 0x21])
    }
}

unsafe impl RefCounted for nsIHttpResponse {
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
pub trait nsIHttpResponseCoerce {
    fn coerce_from(v: &nsIHttpResponse) -> &Self;
}

impl nsIHttpResponseCoerce for nsIHttpResponse {
    #[inline]
    fn coerce_from(v: &nsIHttpResponse) -> &Self {
        v
    }
}

impl nsIHttpResponse {
    #[inline]
    pub fn coerce<T: nsIHttpResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpResponse {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpResponse) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpResponseVTable {
    pub __base: nsISupportsVTable,

    /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
    pub setStatusLine: unsafe extern "C" fn (this: *const nsIHttpResponse, httpVersion: *const libc::c_char, statusCode: libc::uint16_t, description: *const libc::c_char) -> nsresult,

    /* void setHeader (in string name, in string value, in boolean merge); */
    pub setHeader: unsafe extern "C" fn (this: *const nsIHttpResponse, name: *const libc::c_char, value: *const libc::c_char, merge: bool) -> nsresult,

    /* void setHeaderNoCheck (in string name, in string value); */
    pub setHeaderNoCheck: unsafe extern "C" fn (this: *const nsIHttpResponse, name: *const libc::c_char, value: *const libc::c_char) -> nsresult,

    /* readonly attribute nsIOutputStream bodyOutputStream; */
    pub get_bodyOutputStream: unsafe extern "C" fn (this: *const nsIHttpResponse, aBodyOutputStream: *mut *const nsIOutputStream) -> nsresult,

    /* void write (in string data); */
    pub write: unsafe extern "C" fn (this: *const nsIHttpResponse, data: *const libc::c_char) -> nsresult,

    /* void processAsync (); */
    pub processAsync: unsafe extern "C" fn (this: *const nsIHttpResponse) -> nsresult,

    /* void seizePower (); */
    pub seizePower: unsafe extern "C" fn (this: *const nsIHttpResponse) -> nsresult,

    /* void finish (); */
    pub finish: unsafe extern "C" fn (this: *const nsIHttpResponse) -> nsresult,

}


impl nsIHttpResponse {
    /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
    #[inline]
    pub unsafe fn setStatusLine(&self, httpVersion: *const libc::c_char, statusCode: libc::uint16_t, description: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setStatusLine)(self as *const _, httpVersion, statusCode, description) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setHeader (in string name, in string value, in boolean merge); */
    #[inline]
    pub unsafe fn setHeader(&self, name: *const libc::c_char, value: *const libc::c_char, merge: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setHeader)(self as *const _, name, value, merge) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setHeaderNoCheck (in string name, in string value); */
    #[inline]
    pub unsafe fn setHeaderNoCheck(&self, name: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setHeaderNoCheck)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIOutputStream bodyOutputStream; */
    #[inline]
    pub unsafe fn get_bodyOutputStream(&self, ) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_bodyOutputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void write (in string data); */
    #[inline]
    pub unsafe fn write(&self, data: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).write)(self as *const _, data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void processAsync (); */
    #[inline]
    pub unsafe fn processAsync(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).processAsync)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void seizePower (); */
    #[inline]
    pub unsafe fn seizePower(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).seizePower)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finish (); */
    #[inline]
    pub unsafe fn finish(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finish)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


