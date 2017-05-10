//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMFontFace.idl
//


#[repr(C)]
pub struct nsIDOMFontFace {
    vtable: *const nsIDOMFontFaceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMFontFace {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9a3b1272, 0x6585, 0x4f41,
            [0xb0, 0x8f, 0xfd, 0xc5, 0xda, 0x44, 0x4c, 0xd0])
    }
}

unsafe impl RefCounted for nsIDOMFontFace {
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
pub trait nsIDOMFontFaceCoerce {
    fn coerce_from(v: &nsIDOMFontFace) -> &Self;
}

impl nsIDOMFontFaceCoerce for nsIDOMFontFace {
    #[inline]
    fn coerce_from(v: &nsIDOMFontFace) -> &Self {
        v
    }
}

impl nsIDOMFontFace {
    #[inline]
    pub fn coerce<T: nsIDOMFontFaceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMFontFace {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMFontFaceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMFontFace) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMFontFaceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean fromFontGroup; */
    pub get_fromFontGroup: unsafe extern "C" fn (this: *const nsIDOMFontFace, aFromFontGroup: *mut bool) -> nsresult,

    /* readonly attribute boolean fromLanguagePrefs; */
    pub get_fromLanguagePrefs: unsafe extern "C" fn (this: *const nsIDOMFontFace, aFromLanguagePrefs: *mut bool) -> nsresult,

    /* readonly attribute boolean fromSystemFallback; */
    pub get_fromSystemFallback: unsafe extern "C" fn (this: *const nsIDOMFontFace, aFromSystemFallback: *mut bool) -> nsresult,

    /* readonly attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMFontFace, aName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString CSSFamilyName; */
    pub get_CSSFamilyName: unsafe extern "C" fn (this: *const nsIDOMFontFace, aCSSFamilyName: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSFontFaceRule rule; */
    pub get_rule: unsafe extern "C" fn (this: *const nsIDOMFontFace, aRule: *mut *const nsIDOMCSSFontFaceRule) -> nsresult,

    /* readonly attribute long srcIndex; */
    pub get_srcIndex: unsafe extern "C" fn (this: *const nsIDOMFontFace, aSrcIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute DOMString URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIDOMFontFace, aURI: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString localName; */
    pub get_localName: unsafe extern "C" fn (this: *const nsIDOMFontFace, aLocalName: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString format; */
    pub get_format: unsafe extern "C" fn (this: *const nsIDOMFontFace, aFormat: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString metadata; */
    pub get_metadata: unsafe extern "C" fn (this: *const nsIDOMFontFace, aMetadata: *mut nsAString) -> nsresult,

}


impl nsIDOMFontFace {
    /* readonly attribute boolean fromFontGroup; */
    #[inline]
    pub unsafe fn get_fromFontGroup(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fromFontGroup)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean fromLanguagePrefs; */
    #[inline]
    pub unsafe fn get_fromLanguagePrefs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fromLanguagePrefs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean fromSystemFallback; */
    #[inline]
    pub unsafe fn get_fromSystemFallback(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fromSystemFallback)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString CSSFamilyName; */
    #[inline]
    pub unsafe fn get_CSSFamilyName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_CSSFamilyName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMCSSFontFaceRule rule; */
    #[inline]
    pub unsafe fn get_rule(&self, ) -> Result<Option<RefPtr<nsIDOMCSSFontFaceRule>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rule)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long srcIndex; */
    #[inline]
    pub unsafe fn get_srcIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_srcIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_URI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString localName; */
    #[inline]
    pub unsafe fn get_localName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_localName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString format; */
    #[inline]
    pub unsafe fn get_format(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_format)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString metadata; */
    #[inline]
    pub unsafe fn get_metadata(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_metadata)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


