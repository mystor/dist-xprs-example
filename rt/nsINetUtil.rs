//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetUtil.idl
//


pub mod nsINetUtil_consts {
    pub const ESCAPE_ALL: i64 = 0;
    pub const ESCAPE_XALPHAS: i64 = 1;
    pub const ESCAPE_XPALPHAS: i64 = 2;
    pub const ESCAPE_URL_PATH: i64 = 4;
    pub const ESCAPE_URL_SCHEME: i64 = 1;
    pub const ESCAPE_URL_USERNAME: i64 = 2;
    pub const ESCAPE_URL_PASSWORD: i64 = 4;
    pub const ESCAPE_URL_HOST: i64 = 8;
    pub const ESCAPE_URL_DIRECTORY: i64 = 16;
    pub const ESCAPE_URL_FILE_BASENAME: i64 = 32;
    pub const ESCAPE_URL_FILE_EXTENSION: i64 = 64;
    pub const ESCAPE_URL_PARAM: i64 = 128;
    pub const ESCAPE_URL_QUERY: i64 = 256;
    pub const ESCAPE_URL_REF: i64 = 512;
    pub const ESCAPE_URL_FILEPATH: i64 = 112;
    pub const ESCAPE_URL_MINIMAL: i64 = 1023;
    pub const ESCAPE_URL_FORCED: i64 = 1024;
    pub const ESCAPE_URL_ONLY_ASCII: i64 = 2048;
    pub const ESCAPE_URL_ONLY_NONASCII: i64 = 4096;
    pub const ESCAPE_URL_COLON: i64 = 16384;
    pub const ESCAPE_URL_SKIP_CONTROL: i64 = 32768;
}


#[repr(C)]
pub struct nsINetUtil {
    vtable: *const nsINetUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetUtil {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfe2625ec, 0xb884, 0x4df1,
            [0xb3, 0x9c, 0x9e, 0x83, 0x0e, 0x47, 0xaa, 0x94])
    }
}

unsafe impl RefCounted for nsINetUtil {
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
pub trait nsINetUtilCoerce {
    fn coerce_from(v: &nsINetUtil) -> &Self;
}

impl nsINetUtilCoerce for nsINetUtil {
    #[inline]
    fn coerce_from(v: &nsINetUtil) -> &Self {
        v
    }
}

impl nsINetUtil {
    #[inline]
    pub fn coerce<T: nsINetUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetUtil {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetUtil) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetUtilVTable {
    pub __base: nsISupportsVTable,

    /* AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    pub parseRequestContentType: unsafe extern "C" fn (this: *const nsINetUtil, aTypeHeader: *const nsACString, aCharset: *mut nsACString, aHadCharset: *mut bool, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    pub parseResponseContentType: unsafe extern "C" fn (this: *const nsINetUtil, aTypeHeader: *const nsACString, aCharset: *mut nsACString, aHadCharset: *mut bool, _retval: *mut nsACString) -> nsresult,

    /* boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag); */
    pub protocolHasFlags: unsafe extern "C" fn (this: *const nsINetUtil, aURI: *const nsIURI, aFlag: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags); */
    pub URIChainHasFlags: unsafe extern "C" fn (this: *const nsINetUtil, aURI: *const nsIURI, aFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* nsIURI toImmutableURI (in nsIURI aURI); */
    pub toImmutableURI: unsafe extern "C" fn (this: *const nsINetUtil, aURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* nsIURI newSimpleNestedURI (in nsIURI aURI); */
    pub newSimpleNestedURI: unsafe extern "C" fn (this: *const nsINetUtil, aURI: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* ACString escapeString (in ACString aString, in unsigned long aEscapeType); */
    pub escapeString: unsafe extern "C" fn (this: *const nsINetUtil, aString: *const nsACString, aEscapeType: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* ACString escapeURL (in ACString aStr, in unsigned long aFlags); */
    pub escapeURL: unsafe extern "C" fn (this: *const nsINetUtil, aStr: *const nsACString, aFlags: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags); */
    pub unescapeString: unsafe extern "C" fn (this: *const nsINetUtil, aStr: *const nsACString, aFlags: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd); */
    pub extractCharsetFromContentType: unsafe extern "C" fn (this: *const nsINetUtil, aTypeHeader: *const nsACString, aCharset: *mut nsACString, aCharsetStart: *mut libc::int32_t, aCharsetEnd: *mut libc::int32_t, _retval: *mut bool) -> nsresult,

    /* unsigned long parseAttributePolicyString (in AString aPolicyString); */
    pub parseAttributePolicyString: unsafe extern "C" fn (this: *const nsINetUtil, aPolicyString: *const nsAString, _retval: *mut libc::uint32_t) -> nsresult,

}


impl nsINetUtil {
    /* AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    #[inline]
    pub unsafe fn parseRequestContentType(&self, aTypeHeader: &[u8]) -> Result<(nsCString, bool, nsCString), nsresult> {
        let aTypeHeader = nsCString::from(aTypeHeader);
        let mut aCharset = nsCString::new();
        let mut aHadCharset: bool = ::std::mem::zeroed();
        let mut _retval = nsCString::new();
        match ((*self.vtable).parseRequestContentType)(self as *const _, &*aTypeHeader, &mut *aCharset, &mut aHadCharset as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCharset, aHadCharset, _retval))
    }

    /* AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
    #[inline]
    pub unsafe fn parseResponseContentType(&self, aTypeHeader: &[u8]) -> Result<(nsCString, bool, nsCString), nsresult> {
        let aTypeHeader = nsCString::from(aTypeHeader);
        let mut aCharset = nsCString::new();
        let mut aHadCharset: bool = ::std::mem::zeroed();
        let mut _retval = nsCString::new();
        match ((*self.vtable).parseResponseContentType)(self as *const _, &*aTypeHeader, &mut *aCharset, &mut aHadCharset as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCharset, aHadCharset, _retval))
    }

    /* boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag); */
    #[inline]
    pub unsafe fn protocolHasFlags(&self, aURI: Option<&nsIURI>, aFlag: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).protocolHasFlags)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aFlag, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn URIChainHasFlags(&self, aURI: Option<&nsIURI>, aFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).URIChainHasFlags)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIURI toImmutableURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn toImmutableURI(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).toImmutableURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI newSimpleNestedURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn newSimpleNestedURI(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newSimpleNestedURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* ACString escapeString (in ACString aString, in unsigned long aEscapeType); */
    #[inline]
    pub unsafe fn escapeString(&self, aString: &[u8], aEscapeType: libc::uint32_t) -> Result<nsCString, nsresult> {
        let aString = nsCString::from(aString);
        let mut _retval = nsCString::new();
        match ((*self.vtable).escapeString)(self as *const _, &*aString, aEscapeType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString escapeURL (in ACString aStr, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn escapeURL(&self, aStr: &[u8], aFlags: libc::uint32_t) -> Result<nsCString, nsresult> {
        let aStr = nsCString::from(aStr);
        let mut _retval = nsCString::new();
        match ((*self.vtable).escapeURL)(self as *const _, &*aStr, aFlags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn unescapeString(&self, aStr: &[u8], aFlags: libc::uint32_t) -> Result<nsCString, nsresult> {
        let aStr = nsCString::from(aStr);
        let mut _retval = nsCString::new();
        match ((*self.vtable).unescapeString)(self as *const _, &*aStr, aFlags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd); */
    #[inline]
    pub unsafe fn extractCharsetFromContentType(&self, aTypeHeader: &[u8]) -> Result<(nsCString, libc::int32_t, libc::int32_t, bool), nsresult> {
        let aTypeHeader = nsCString::from(aTypeHeader);
        let mut aCharset = nsCString::new();
        let mut aCharsetStart: libc::int32_t = ::std::mem::zeroed();
        let mut aCharsetEnd: libc::int32_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).extractCharsetFromContentType)(self as *const _, &*aTypeHeader, &mut *aCharset, &mut aCharsetStart as *mut _, &mut aCharsetEnd as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCharset, aCharsetStart, aCharsetEnd, _retval))
    }

    /* unsigned long parseAttributePolicyString (in AString aPolicyString); */
    #[inline]
    pub unsafe fn parseAttributePolicyString(&self, aPolicyString: &[u16]) -> Result<libc::uint32_t, nsresult> {
        let aPolicyString = nsString::from(aPolicyString);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).parseAttributePolicyString)(self as *const _, &*aPolicyString, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


