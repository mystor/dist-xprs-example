//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolHandler.idl
//


#[repr(C)]
pub struct nsIProtocolHandlerWithDynamicFlags {
    vtable: *const nsIProtocolHandlerWithDynamicFlagsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolHandlerWithDynamicFlags {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x65a8e823, 0x0591, 0x4fc0,
            [0xa5, 0x6a, 0x03, 0x26, 0x5e, 0x0a, 0x4c, 0xe8])
    }
}

unsafe impl RefCounted for nsIProtocolHandlerWithDynamicFlags {
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
pub trait nsIProtocolHandlerWithDynamicFlagsCoerce {
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self;
}

impl nsIProtocolHandlerWithDynamicFlagsCoerce for nsIProtocolHandlerWithDynamicFlags {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self {
        v
    }
}

impl nsIProtocolHandlerWithDynamicFlags {
    #[inline]
    pub fn coerce<T: nsIProtocolHandlerWithDynamicFlagsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolHandlerWithDynamicFlags {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolHandlerWithDynamicFlagsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolHandlerWithDynamicFlagsVTable {
    pub __base: nsISupportsVTable,

    /* unsigned long getFlagsForURI (in nsIURI aURI); */
    pub getFlagsForURI: unsafe extern "C" fn (this: *const nsIProtocolHandlerWithDynamicFlags, aURI: *const nsIURI, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsIProtocolHandlerWithDynamicFlags {
    /* unsigned long getFlagsForURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getFlagsForURI(&self, aURI: Option<&nsIURI>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getFlagsForURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsIProtocolHandler_consts {
    pub const URI_STD: i64 = 0;
    pub const URI_NORELATIVE: i64 = 1;
    pub const URI_NOAUTH: i64 = 2;
    pub const ALLOWS_PROXY: i64 = 4;
    pub const ALLOWS_PROXY_HTTP: i64 = 8;
    pub const URI_INHERITS_SECURITY_CONTEXT: i64 = 16;
    pub const URI_FORBIDS_AUTOMATIC_DOCUMENT_REPLACEMENT: i64 = 32;
    pub const URI_LOADABLE_BY_ANYONE: i64 = 64;
    pub const URI_DANGEROUS_TO_LOAD: i64 = 128;
    pub const URI_IS_UI_RESOURCE: i64 = 256;
    pub const URI_IS_LOCAL_FILE: i64 = 512;
    pub const URI_LOADABLE_BY_SUBSUMERS: i64 = 1024;
    pub const URI_DOES_NOT_RETURN_DATA: i64 = 2048;
    pub const URI_IS_LOCAL_RESOURCE: i64 = 4096;
    pub const URI_OPENING_EXECUTES_SCRIPT: i64 = 8192;
    pub const URI_NON_PERSISTABLE: i64 = 16384;
    pub const URI_FORBIDS_COOKIE_ACCESS: i64 = 32768;
    pub const URI_CROSS_ORIGIN_NEEDS_WEBAPPS_PERM: i64 = 65536;
    pub const URI_SYNC_LOAD_IS_OK: i64 = 131072;
    pub const URI_SAFE_TO_LOAD_IN_SECURE_CONTEXT: i64 = 262144;
    pub const URI_FETCHABLE_BY_ANYONE: i64 = 524288;
    pub const ORIGIN_IS_FULL_SPEC: i64 = 1048576;
    pub const URI_SCHEME_NOT_SELF_LINKABLE: i64 = 2097152;
}


#[repr(C)]
pub struct nsIProtocolHandler {
    vtable: *const nsIProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa87210e6, 0x7c8c, 0x41f7,
            [0x86, 0x4d, 0xdf, 0x80, 0x90, 0x15, 0x19, 0x3e])
    }
}

unsafe impl RefCounted for nsIProtocolHandler {
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
pub trait nsIProtocolHandlerCoerce {
    fn coerce_from(v: &nsIProtocolHandler) -> &Self;
}

impl nsIProtocolHandlerCoerce for nsIProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandler) -> &Self {
        v
    }
}

impl nsIProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolHandlerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString scheme; */
    pub get_scheme: unsafe extern "C" fn (this: *const nsIProtocolHandler, aScheme: *mut nsACString) -> nsresult,

    /* readonly attribute long defaultPort; */
    pub get_defaultPort: unsafe extern "C" fn (this: *const nsIProtocolHandler, aDefaultPort: *mut libc::int32_t) -> nsresult,

    /* readonly attribute unsigned long protocolFlags; */
    pub get_protocolFlags: unsafe extern "C" fn (this: *const nsIProtocolHandler, aProtocolFlags: *mut libc::uint32_t) -> nsresult,

    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
    pub newURI: unsafe extern "C" fn (this: *const nsIProtocolHandler, aSpec: *const nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIChannel newChannel2 (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
    pub newChannel2: unsafe extern "C" fn (this: *const nsIProtocolHandler, aURI: *const nsIURI, aLoadinfo: *const nsILoadInfo, _retval: *mut *const nsIChannel) -> nsresult,

    /* nsIChannel newChannel (in nsIURI aURI); */
    pub newChannel: unsafe extern "C" fn (this: *const nsIProtocolHandler, aURI: *const nsIURI, _retval: *mut *const nsIChannel) -> nsresult,

    /* boolean allowPort (in long port, in string scheme); */
    pub allowPort: unsafe extern "C" fn (this: *const nsIProtocolHandler, port: libc::int32_t, scheme: *const libc::c_char, _retval: *mut bool) -> nsresult,

}


impl nsIProtocolHandler {
    /* readonly attribute ACString scheme; */
    #[inline]
    pub unsafe fn get_scheme(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_scheme)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long defaultPort; */
    #[inline]
    pub unsafe fn get_defaultPort(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long protocolFlags; */
    #[inline]
    pub unsafe fn get_protocolFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_protocolFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
    #[inline]
    pub unsafe fn newURI(&self, aSpec: &[u8], aOriginCharset: *const libc::c_char, aBaseURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let aSpec = nsCString::from(aSpec);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newURI)(self as *const _, &*aSpec, aOriginCharset, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannel2 (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
    #[inline]
    pub unsafe fn newChannel2(&self, aURI: Option<&nsIURI>, aLoadinfo: Option<&nsILoadInfo>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannel2)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadinfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIChannel newChannel (in nsIURI aURI); */
    #[inline]
    pub unsafe fn newChannel(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newChannel)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean allowPort (in long port, in string scheme); */
    #[inline]
    pub unsafe fn allowPort(&self, port: libc::int32_t, scheme: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowPort)(self as *const _, port, scheme, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


