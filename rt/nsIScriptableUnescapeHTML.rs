//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableUnescapeHTML.idl
//


#[repr(C)]
pub struct nsIScriptableUnescapeHTML {
    vtable: *const nsIScriptableUnescapeHTMLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableUnescapeHTML {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ab244a9, 0xf09d, 0x44da,
            [0x9e, 0x3f, 0xee, 0x4d, 0x67, 0x36, 0x7f, 0x2d])
    }
}

unsafe impl RefCounted for nsIScriptableUnescapeHTML {
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
pub trait nsIScriptableUnescapeHTMLCoerce {
    fn coerce_from(v: &nsIScriptableUnescapeHTML) -> &Self;
}

impl nsIScriptableUnescapeHTMLCoerce for nsIScriptableUnescapeHTML {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnescapeHTML) -> &Self {
        v
    }
}

impl nsIScriptableUnescapeHTML {
    #[inline]
    pub fn coerce<T: nsIScriptableUnescapeHTMLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableUnescapeHTML {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableUnescapeHTMLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnescapeHTML) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableUnescapeHTMLVTable {
    pub __base: nsISupportsVTable,

    /* AString unescape (in AString src); */
    pub unescape: unsafe extern "C" fn (this: *const nsIScriptableUnescapeHTML, src: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
    pub parseFragment: unsafe extern "C" fn (this: *const nsIScriptableUnescapeHTML, fragment: *const nsAString, isXML: bool, baseURI: *const nsIURI, element: *const nsIDOMElement, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

}


impl nsIScriptableUnescapeHTML {
    /* AString unescape (in AString src); */
    #[inline]
    pub unsafe fn unescape(&self, src: &[u16]) -> Result<nsString, nsresult> {
        let src = nsString::from(src);
        let mut _retval = nsString::new();
        match ((*self.vtable).unescape)(self as *const _, &*src, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMDocumentFragment parseFragment (in AString fragment, in boolean isXML, in nsIURI baseURI, in nsIDOMElement element); */
    #[inline]
    pub unsafe fn parseFragment(&self, fragment: &[u16], isXML: bool, baseURI: Option<&nsIURI>, element: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let fragment = nsString::from(fragment);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseFragment)(self as *const _, &*fragment, isXML, baseURI.map_or(::std::ptr::null(), |x| x as *const _), element.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


