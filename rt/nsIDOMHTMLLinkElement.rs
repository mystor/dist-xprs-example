//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLinkElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLLinkElement {
    vtable: *const nsIDOMHTMLLinkElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLLinkElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xee50b7ab, 0x0015, 0x4fbe,
            [0x89, 0xe0, 0xe3, 0xfe, 0xac, 0xd4, 0xff, 0xde])
    }
}

unsafe impl RefCounted for nsIDOMHTMLLinkElement {
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
pub trait nsIDOMHTMLLinkElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLLinkElement) -> &Self;
}

impl nsIDOMHTMLLinkElementCoerce for nsIDOMHTMLLinkElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLinkElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLLinkElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLLinkElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLLinkElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLLinkElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLinkElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLLinkElementVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(MozDisabled)] attribute boolean disabled; */
    pub get_MozDisabled: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aDisabled: *mut bool) -> nsresult,
    pub set_MozDisabled: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aDisabled: bool) -> nsresult,

    /* attribute DOMString charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aCharset: *mut nsAString) -> nsresult,
    pub set_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aCharset: *const nsAString) -> nsresult,

    /* attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aHref: *mut nsAString) -> nsresult,
    pub set_href: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aHref: *const nsAString) -> nsresult,

    /* attribute DOMString hreflang; */
    pub get_hreflang: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aHreflang: *mut nsAString) -> nsresult,
    pub set_hreflang: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aHreflang: *const nsAString) -> nsresult,

    /* attribute DOMString media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aMedia: *mut nsAString) -> nsresult,
    pub set_media: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aMedia: *const nsAString) -> nsresult,

    /* attribute DOMString rel; */
    pub get_rel: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aRel: *mut nsAString) -> nsresult,
    pub set_rel: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aRel: *const nsAString) -> nsresult,

    /* attribute DOMString rev; */
    pub get_rev: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aRev: *mut nsAString) -> nsresult,
    pub set_rev: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aRev: *const nsAString) -> nsresult,

    /* attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aTarget: *mut nsAString) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aTarget: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLLinkElement, aType: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLLinkElement {
    /* [binaryname(MozDisabled)] attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_MozDisabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_MozDisabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_MozDisabled(&self, aDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_MozDisabled)(self as *const _, aDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charset(&self, aCharset: &[u16]) -> Result<(), nsresult> {
        let aCharset = nsString::from(aCharset);
        match ((*self.vtable).set_charset)(self as *const _, &*aCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString href; */
    #[inline]
    pub unsafe fn get_href(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_href)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_href(&self, aHref: &[u16]) -> Result<(), nsresult> {
        let aHref = nsString::from(aHref);
        match ((*self.vtable).set_href)(self as *const _, &*aHref) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString hreflang; */
    #[inline]
    pub unsafe fn get_hreflang(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_hreflang)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hreflang(&self, aHreflang: &[u16]) -> Result<(), nsresult> {
        let aHreflang = nsString::from(aHreflang);
        match ((*self.vtable).set_hreflang)(self as *const _, &*aHreflang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString media; */
    #[inline]
    pub unsafe fn get_media(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_media)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_media(&self, aMedia: &[u16]) -> Result<(), nsresult> {
        let aMedia = nsString::from(aMedia);
        match ((*self.vtable).set_media)(self as *const _, &*aMedia) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString rel; */
    #[inline]
    pub unsafe fn get_rel(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rel(&self, aRel: &[u16]) -> Result<(), nsresult> {
        let aRel = nsString::from(aRel);
        match ((*self.vtable).set_rel)(self as *const _, &*aRel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString rev; */
    #[inline]
    pub unsafe fn get_rev(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rev)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rev(&self, aRev: &[u16]) -> Result<(), nsresult> {
        let aRev = nsString::from(aRev);
        match ((*self.vtable).set_rev)(self as *const _, &*aRev) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: &[u16]) -> Result<(), nsresult> {
        let aTarget = nsString::from(aTarget);
        match ((*self.vtable).set_target)(self as *const _, &*aTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


