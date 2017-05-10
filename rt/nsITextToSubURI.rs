//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITextToSubURI.idl
//


#[repr(C)]
pub struct nsITextToSubURI {
    vtable: *const nsITextToSubURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITextToSubURI {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b042e24, 0x6f87, 0x11d3,
            [0xb3, 0xc8, 0x00, 0x80, 0x5f, 0x8a, 0x66, 0x70])
    }
}

unsafe impl RefCounted for nsITextToSubURI {
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
pub trait nsITextToSubURICoerce {
    fn coerce_from(v: &nsITextToSubURI) -> &Self;
}

impl nsITextToSubURICoerce for nsITextToSubURI {
    #[inline]
    fn coerce_from(v: &nsITextToSubURI) -> &Self {
        v
    }
}

impl nsITextToSubURI {
    #[inline]
    pub fn coerce<T: nsITextToSubURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITextToSubURI {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITextToSubURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextToSubURI) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITextToSubURIVTable {
    pub __base: nsISupportsVTable,

    /* string ConvertAndEscape (in string charset, in wstring text); */
    pub ConvertAndEscape: unsafe extern "C" fn (this: *const nsITextToSubURI, charset: *const libc::c_char, text: *const libc::int16_t, _retval: *mut *const libc::c_char) -> nsresult,

    /* wstring UnEscapeAndConvert (in string charset, in string text); */
    pub UnEscapeAndConvert: unsafe extern "C" fn (this: *const nsITextToSubURI, charset: *const libc::c_char, text: *const libc::c_char, _retval: *mut *const libc::int16_t) -> nsresult,

    /* AString unEscapeURIForUI (in ACString aCharset, in AUTF8String aURIFragment); */
    pub unEscapeURIForUI: unsafe extern "C" fn (this: *const nsITextToSubURI, aCharset: *const nsACString, aURIFragment: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
    pub unEscapeNonAsciiURI: unsafe extern "C" fn (this: *const nsITextToSubURI, aCharset: *const nsACString, aURIFragment: *const nsACString, _retval: *mut nsAString) -> nsresult,

}


impl nsITextToSubURI {
    /* string ConvertAndEscape (in string charset, in wstring text); */
    #[inline]
    pub unsafe fn ConvertAndEscape(&self, charset: *const libc::c_char, text: *const libc::int16_t) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).ConvertAndEscape)(self as *const _, charset, text, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring UnEscapeAndConvert (in string charset, in string text); */
    #[inline]
    pub unsafe fn UnEscapeAndConvert(&self, charset: *const libc::c_char, text: *const libc::c_char) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).UnEscapeAndConvert)(self as *const _, charset, text, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString unEscapeURIForUI (in ACString aCharset, in AUTF8String aURIFragment); */
    #[inline]
    pub unsafe fn unEscapeURIForUI(&self, aCharset: &[u8], aURIFragment: &[u8]) -> Result<nsString, nsresult> {
        let aCharset = nsCString::from(aCharset);
        let aURIFragment = nsCString::from(aURIFragment);
        let mut _retval = nsString::new();
        match ((*self.vtable).unEscapeURIForUI)(self as *const _, &*aCharset, &*aURIFragment, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString unEscapeNonAsciiURI (in ACString aCharset, in AUTF8String aURIFragment); */
    #[inline]
    pub unsafe fn unEscapeNonAsciiURI(&self, aCharset: &[u8], aURIFragment: &[u8]) -> Result<nsString, nsresult> {
        let aCharset = nsCString::from(aCharset);
        let aURIFragment = nsCString::from(aURIFragment);
        let mut _retval = nsString::new();
        match ((*self.vtable).unEscapeNonAsciiURI)(self as *const _, &*aCharset, &*aURIFragment, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


