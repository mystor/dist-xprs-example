//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParserUtils.idl
//


pub mod nsIParserUtils_consts {
    pub const SanitizerAllowComments: i64 = 1;
    pub const SanitizerAllowStyle: i64 = 2;
    pub const SanitizerCidEmbedsOnly: i64 = 4;
    pub const SanitizerDropNonCSSPresentation: i64 = 8;
    pub const SanitizerDropForms: i64 = 16;
    pub const SanitizerDropMedia: i64 = 32;
}


#[repr(C)]
pub struct nsIParserUtils {
    vtable: *const nsIParserUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIParserUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa1101145, 0x0025, 0x411e,
            [0x88, 0x73, 0xfd, 0xf5, 0x7b, 0xf2, 0x81, 0x28])
    }
}

unsafe impl RefCounted for nsIParserUtils {
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
pub trait nsIParserUtilsCoerce {
    fn coerce_from(v: &nsIParserUtils) -> &Self;
}

impl nsIParserUtilsCoerce for nsIParserUtils {
    #[inline]
    fn coerce_from(v: &nsIParserUtils) -> &Self {
        v
    }
}

impl nsIParserUtils {
    #[inline]
    pub fn coerce<T: nsIParserUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIParserUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIParserUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParserUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIParserUtilsVTable {
    pub __base: nsISupportsVTable,

    /* AString sanitize (in AString src, in unsigned long flags); */
    pub sanitize: unsafe extern "C" fn (this: *const nsIParserUtils, src: *const nsAString, flags: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
    pub convertToPlainText: unsafe extern "C" fn (this: *const nsIParserUtils, src: *const nsAString, flags: libc::uint32_t, wrapCol: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
    pub parseFragment: unsafe extern "C" fn (this: *const nsIParserUtils, fragment: *const nsAString, flags: libc::uint32_t, isXML: bool, baseURI: *const nsIURI, element: *const nsIDOMElement, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

}


impl nsIParserUtils {
    /* AString sanitize (in AString src, in unsigned long flags); */
    #[inline]
    pub unsafe fn sanitize(&self, src: &[u16], flags: libc::uint32_t) -> Result<nsString, nsresult> {
        let src = nsString::from(src);
        let mut _retval = nsString::new();
        match ((*self.vtable).sanitize)(self as *const _, &*src, flags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
    #[inline]
    pub unsafe fn convertToPlainText(&self, src: &[u16], flags: libc::uint32_t, wrapCol: libc::uint32_t) -> Result<nsString, nsresult> {
        let src = nsString::from(src);
        let mut _retval = nsString::new();
        match ((*self.vtable).convertToPlainText)(self as *const _, &*src, flags, wrapCol, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
    #[inline]
    pub unsafe fn parseFragment(&self, fragment: &[u16], flags: libc::uint32_t, isXML: bool, baseURI: Option<&nsIURI>, element: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let fragment = nsString::from(fragment);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseFragment)(self as *const _, &*fragment, flags, isXML, baseURI.map_or(::std::ptr::null(), |x| x as *const _), element.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


