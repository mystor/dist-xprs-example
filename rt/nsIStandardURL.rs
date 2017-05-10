//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStandardURL.idl
//


pub mod nsIStandardURL_consts {
    pub const URLTYPE_STANDARD: i64 = 1;
    pub const URLTYPE_AUTHORITY: i64 = 2;
    pub const URLTYPE_NO_AUTHORITY: i64 = 3;
}


#[repr(C)]
pub struct nsIStandardURL {
    vtable: *const nsIStandardURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStandardURL {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbabd6cca, 0xebe7, 0x4329,
            [0x96, 0x7c, 0xd6, 0xb9, 0xe3, 0x3c, 0xaa, 0x81])
    }
}

unsafe impl RefCounted for nsIStandardURL {
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
pub trait nsIStandardURLCoerce {
    fn coerce_from(v: &nsIStandardURL) -> &Self;
}

impl nsIStandardURLCoerce for nsIStandardURL {
    #[inline]
    fn coerce_from(v: &nsIStandardURL) -> &Self {
        v
    }
}

impl nsIStandardURL {
    #[inline]
    pub fn coerce<T: nsIStandardURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStandardURL {
    type Target = nsIMutable;
    #[inline]
    fn deref(&self) -> &nsIMutable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIMutableCoerce> nsIStandardURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStandardURL) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStandardURLVTable {
    pub __base: nsIMutableVTable,

    /* void init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
    pub init: unsafe extern "C" fn (this: *const nsIStandardURL, aUrlType: libc::uint32_t, aDefaultPort: libc::int32_t, aSpec: *const nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI) -> nsresult,

    /* void setDefaultPort (in long aNewDefaultPort); */
    pub setDefaultPort: unsafe extern "C" fn (this: *const nsIStandardURL, aNewDefaultPort: libc::int32_t) -> nsresult,

}


impl nsIStandardURL {
    /* void init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
    #[inline]
    pub unsafe fn init(&self, aUrlType: libc::uint32_t, aDefaultPort: libc::int32_t, aSpec: &[u8], aOriginCharset: *const libc::c_char, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {
        let aSpec = nsCString::from(aSpec);
        match ((*self.vtable).init)(self as *const _, aUrlType, aDefaultPort, &*aSpec, aOriginCharset, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDefaultPort (in long aNewDefaultPort); */
    #[inline]
    pub unsafe fn setDefaultPort(&self, aNewDefaultPort: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDefaultPort)(self as *const _, aNewDefaultPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


