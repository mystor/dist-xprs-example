//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISaveAsCharset.idl
//


pub mod nsISaveAsCharset_consts {
    pub const mask_Fallback: i64 = 255;
    pub const mask_Entity: i64 = 768;
    pub const mask_CharsetFallback: i64 = 1024;
    pub const attr_FallbackNone: i64 = 0;
    pub const attr_FallbackQuestionMark: i64 = 1;
    pub const attr_FallbackEscapeU: i64 = 2;
    pub const attr_FallbackDecimalNCR: i64 = 3;
    pub const attr_FallbackHexNCR: i64 = 4;
    pub const attr_EntityNone: i64 = 0;
    pub const attr_EntityBeforeCharsetConv: i64 = 256;
    pub const attr_EntityAfterCharsetConv: i64 = 512;
    pub const attr_CharsetFallback: i64 = 1024;
    pub const attr_plainTextDefault: i64 = 0;
    pub const attr_htmlTextDefault: i64 = 259;
}


#[repr(C)]
pub struct nsISaveAsCharset {
    vtable: *const nsISaveAsCharsetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISaveAsCharset {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb3b8124f, 0x0abb, 0x460e,
            [0x88, 0xac, 0x3c, 0xf1, 0xa0, 0x13, 0x4b, 0x2d])
    }
}

unsafe impl RefCounted for nsISaveAsCharset {
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
pub trait nsISaveAsCharsetCoerce {
    fn coerce_from(v: &nsISaveAsCharset) -> &Self;
}

impl nsISaveAsCharsetCoerce for nsISaveAsCharset {
    #[inline]
    fn coerce_from(v: &nsISaveAsCharset) -> &Self {
        v
    }
}

impl nsISaveAsCharset {
    #[inline]
    pub fn coerce<T: nsISaveAsCharsetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISaveAsCharset {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISaveAsCharsetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISaveAsCharset) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISaveAsCharsetVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsISaveAsCharset, aCharset: *mut nsACString) -> nsresult,

    /* void Init (in AUTF8String charset, in unsigned long ignored, in unsigned long alsoIgnored); */
    pub Init: unsafe extern "C" fn (this: *const nsISaveAsCharset, charset: *const nsACString, ignored: libc::uint32_t, alsoIgnored: libc::uint32_t) -> nsresult,

    /* ACString Convert (in AString inString); */
    pub Convert: unsafe extern "C" fn (this: *const nsISaveAsCharset, inString: *const nsAString, _retval: *mut nsACString) -> nsresult,

}


impl nsISaveAsCharset {
    /* readonly attribute AUTF8String charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void Init (in AUTF8String charset, in unsigned long ignored, in unsigned long alsoIgnored); */
    #[inline]
    pub unsafe fn Init(&self, charset: &[u8], ignored: libc::uint32_t, alsoIgnored: libc::uint32_t) -> Result<(), nsresult> {
        let charset = nsCString::from(charset);
        match ((*self.vtable).Init)(self as *const _, &*charset, ignored, alsoIgnored) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString Convert (in AString inString); */
    #[inline]
    pub unsafe fn Convert(&self, inString: &[u16]) -> Result<nsCString, nsresult> {
        let inString = nsString::from(inString);
        let mut _retval = nsCString::new();
        match ((*self.vtable).Convert)(self as *const _, &*inString, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


