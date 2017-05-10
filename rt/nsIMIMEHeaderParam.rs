//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEHeaderParam.idl
//


#[repr(C)]
pub struct nsIMIMEHeaderParam {
    vtable: *const nsIMIMEHeaderParamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMIMEHeaderParam {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9c9252a1, 0xfdaf, 0x40a2,
            [0x9c, 0x2b, 0xa3, 0xdc, 0x45, 0xe2, 0x8d, 0xde])
    }
}

unsafe impl RefCounted for nsIMIMEHeaderParam {
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
pub trait nsIMIMEHeaderParamCoerce {
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self;
}

impl nsIMIMEHeaderParamCoerce for nsIMIMEHeaderParam {
    #[inline]
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self {
        v
    }
}

impl nsIMIMEHeaderParam {
    #[inline]
    pub fn coerce<T: nsIMIMEHeaderParamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMIMEHeaderParam {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMIMEHeaderParamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEHeaderParam) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMIMEHeaderParamVTable {
    pub __base: nsISupportsVTable,

    /* AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    pub getParameter: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut nsAString) -> nsresult,

    /* AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    pub getParameterHTTP: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const nsACString, aParamName: *const libc::c_char, aFallbackCharset: *const nsACString, aTryLocaleCharset: bool, aLang: *mut *const libc::c_char, _retval: *mut nsAString) -> nsresult,

    /* AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang); */
    pub decodeRFC5987Param: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aParamVal: *const nsACString, aLang: *mut nsACString, _retval: *mut nsAString) -> nsresult,

    /* [noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang); */
    pub getParameterInternal: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const libc::c_char, aParamName: *const libc::c_char, aCharset: *mut *const libc::c_char, aLang: *mut *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* [noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation); */
    pub decodeRFC2047Header: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aHeaderVal: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, aEatContinuation: bool, _retval: *mut nsACString) -> nsresult,

    /* [noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset); */
    pub decodeParameter: unsafe extern "C" fn (this: *const nsIMIMEHeaderParam, aParamValue: *const nsACString, aCharset: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, _retval: *mut nsACString) -> nsresult,

}


impl nsIMIMEHeaderParam {
    /* AString getParameter (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    #[inline]
    pub unsafe fn getParameter(&self, aHeaderVal: &[u8], aParamName: *const libc::c_char, aFallbackCharset: &[u8], aTryLocaleCharset: bool) -> Result<(*const libc::c_char, nsString), nsresult> {
        let aHeaderVal = nsCString::from(aHeaderVal);
        let aFallbackCharset = nsCString::from(aFallbackCharset);
        let mut aLang: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getParameter)(self as *const _, &*aHeaderVal, aParamName, &*aFallbackCharset, aTryLocaleCharset, &mut aLang as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLang, _retval))
    }

    /* AString getParameterHTTP (in ACString aHeaderVal, in string aParamName, in ACString aFallbackCharset, in boolean aTryLocaleCharset, out string aLang); */
    #[inline]
    pub unsafe fn getParameterHTTP(&self, aHeaderVal: &[u8], aParamName: *const libc::c_char, aFallbackCharset: &[u8], aTryLocaleCharset: bool) -> Result<(*const libc::c_char, nsString), nsresult> {
        let aHeaderVal = nsCString::from(aHeaderVal);
        let aFallbackCharset = nsCString::from(aFallbackCharset);
        let mut aLang: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getParameterHTTP)(self as *const _, &*aHeaderVal, aParamName, &*aFallbackCharset, aTryLocaleCharset, &mut aLang as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLang, _retval))
    }

    /* AString decodeRFC5987Param (in ACString aParamVal, out ACString aLang); */
    #[inline]
    pub unsafe fn decodeRFC5987Param(&self, aParamVal: &[u8]) -> Result<(nsCString, nsString), nsresult> {
        let aParamVal = nsCString::from(aParamVal);
        let mut aLang = nsCString::new();
        let mut _retval = nsString::new();
        match ((*self.vtable).decodeRFC5987Param)(self as *const _, &*aParamVal, &mut *aLang, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLang, _retval))
    }

    /* [noscript] string getParameterInternal (in string aHeaderVal, in string aParamName, out string aCharset, out string aLang); */
    #[inline]
    pub unsafe fn getParameterInternal(&self, aHeaderVal: *const libc::c_char, aParamName: *const libc::c_char) -> Result<(*const libc::c_char, *const libc::c_char, *const libc::c_char), nsresult> {
        let mut aCharset: *const libc::c_char = ::std::mem::zeroed();
        let mut aLang: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getParameterInternal)(self as *const _, aHeaderVal, aParamName, &mut aCharset as *mut _, &mut aLang as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCharset, aLang, _retval))
    }

    /* [noscript] ACString decodeRFC2047Header (in string aHeaderVal, in string aDefaultCharset, in boolean aOverrideCharset, in boolean aEatContinuation); */
    #[inline]
    pub unsafe fn decodeRFC2047Header(&self, aHeaderVal: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool, aEatContinuation: bool) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).decodeRFC2047Header)(self as *const _, aHeaderVal, aDefaultCharset, aOverrideCharset, aEatContinuation, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] ACString decodeParameter (in ACString aParamValue, in string aCharset, in string aDefaultCharset, in boolean aOverrideCharset); */
    #[inline]
    pub unsafe fn decodeParameter(&self, aParamValue: &[u8], aCharset: *const libc::c_char, aDefaultCharset: *const libc::c_char, aOverrideCharset: bool) -> Result<nsCString, nsresult> {
        let aParamValue = nsCString::from(aParamValue);
        let mut _retval = nsCString::new();
        match ((*self.vtable).decodeParameter)(self as *const _, &*aParamValue, aCharset, aDefaultCharset, aOverrideCharset, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


