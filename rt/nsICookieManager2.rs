//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICookieManager2.idl
//


#[repr(C)]
pub struct nsICookieManager2 {
    vtable: *const nsICookieManager2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICookieManager2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdaf0caa7, 0xb431, 0x4b4d,
            [0xba, 0x51, 0x08, 0xc1, 0x79, 0xbb, 0x9d, 0xfe])
    }
}

unsafe impl RefCounted for nsICookieManager2 {
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
pub trait nsICookieManager2Coerce {
    fn coerce_from(v: &nsICookieManager2) -> &Self;
}

impl nsICookieManager2Coerce for nsICookieManager2 {
    #[inline]
    fn coerce_from(v: &nsICookieManager2) -> &Self {
        v
    }
}

impl nsICookieManager2 {
    #[inline]
    pub fn coerce<T: nsICookieManager2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICookieManager2 {
    type Target = nsICookieManager;
    #[inline]
    fn deref(&self) -> &nsICookieManager {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICookieManagerCoerce> nsICookieManager2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieManager2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICookieManager2VTable {
    pub __base: nsICookieManagerVTable,

    /* [implicit_jscontext,optional_argc] void add (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in ACString aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub add: *const ::libc::c_void,

    /* [notxpcom] nsresult addNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in ACString aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in OriginAttributesPtr aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub addNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] boolean cookieExists (in nsICookie2 aCookie, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cookieExists: *const ::libc::c_void,

    /* [notxpcom] nsresult cookieExistsNative (in nsICookie2 aCookie, in OriginAttributesPtr aOriginAttributes, out boolean aExists); */
    /// Unable to call function as its signature contains a non-rust type
    pub cookieExistsNative: *const ::libc::c_void,

    /* unsigned long countCookiesFromHost (in AUTF8String aHost); */
    pub countCookiesFromHost: unsafe extern "C" fn (this: *const nsICookieManager2, aHost: *const nsACString, _retval: *mut libc::uint32_t) -> nsresult,

    /* [implicit_jscontext,optional_argc] nsISimpleEnumerator getCookiesFromHost (in AUTF8String aHost, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCookiesFromHost: *const ::libc::c_void,

    /* void importCookies (in nsIFile aCookieFile); */
    pub importCookies: unsafe extern "C" fn (this: *const nsICookieManager2, aCookieFile: *const nsIFile) -> nsresult,

    /* nsISimpleEnumerator getCookiesWithOriginAttributes (in DOMString aPattern, [optional] in AUTF8String aHost); */
    pub getCookiesWithOriginAttributes: unsafe extern "C" fn (this: *const nsICookieManager2, aPattern: *const nsAString, aHost: *const nsACString, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void removeCookiesWithOriginAttributes (in DOMString aPattern, [optional] in AUTF8String aHost); */
    pub removeCookiesWithOriginAttributes: unsafe extern "C" fn (this: *const nsICookieManager2, aPattern: *const nsAString, aHost: *const nsACString) -> nsresult,

}


impl nsICookieManager2 {
    /* [implicit_jscontext,optional_argc] void add (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in ACString aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult addNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in ACString aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in OriginAttributesPtr aOriginAttributes); */


    /* [implicit_jscontext,optional_argc] boolean cookieExists (in nsICookie2 aCookie, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult cookieExistsNative (in nsICookie2 aCookie, in OriginAttributesPtr aOriginAttributes, out boolean aExists); */


    /* unsigned long countCookiesFromHost (in AUTF8String aHost); */
    #[inline]
    pub unsafe fn countCookiesFromHost(&self, aHost: &[u8]) -> Result<libc::uint32_t, nsresult> {
        let aHost = nsCString::from(aHost);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).countCookiesFromHost)(self as *const _, &*aHost, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext,optional_argc] nsISimpleEnumerator getCookiesFromHost (in AUTF8String aHost, [optional] in jsval aOriginAttributes); */


    /* void importCookies (in nsIFile aCookieFile); */
    #[inline]
    pub unsafe fn importCookies(&self, aCookieFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).importCookies)(self as *const _, aCookieFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getCookiesWithOriginAttributes (in DOMString aPattern, [optional] in AUTF8String aHost); */
    #[inline]
    pub unsafe fn getCookiesWithOriginAttributes(&self, aPattern: &[u16], aHost: &[u8]) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let aPattern = nsString::from(aPattern);
        let aHost = nsCString::from(aHost);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCookiesWithOriginAttributes)(self as *const _, &*aPattern, &*aHost, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeCookiesWithOriginAttributes (in DOMString aPattern, [optional] in AUTF8String aHost); */
    #[inline]
    pub unsafe fn removeCookiesWithOriginAttributes(&self, aPattern: &[u16], aHost: &[u8]) -> Result<(), nsresult> {
        let aPattern = nsString::from(aPattern);
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).removeCookiesWithOriginAttributes)(self as *const _, &*aPattern, &*aHost) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


