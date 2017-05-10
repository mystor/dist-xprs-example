//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSImportRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSImportRule {
    vtable: *const nsIDOMCSSImportRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSImportRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd3b2b914, 0x01ef, 0x4663,
            [0xbe, 0xda, 0xa6, 0x47, 0x5a, 0x26, 0xf4, 0x91])
    }
}

unsafe impl RefCounted for nsIDOMCSSImportRule {
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
pub trait nsIDOMCSSImportRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSImportRule) -> &Self;
}

impl nsIDOMCSSImportRuleCoerce for nsIDOMCSSImportRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSImportRule) -> &Self {
        v
    }
}

impl nsIDOMCSSImportRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSImportRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSImportRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSImportRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSImportRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSImportRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMCSSImportRule, aHref: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMMediaList media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMCSSImportRule, aMedia: *mut *const nsIDOMMediaList) -> nsresult,

    /* readonly attribute nsIDOMCSSStyleSheet styleSheet; */
    pub get_styleSheet: unsafe extern "C" fn (this: *const nsIDOMCSSImportRule, aStyleSheet: *mut *const nsIDOMCSSStyleSheet) -> nsresult,

}


impl nsIDOMCSSImportRule {
    /* readonly attribute DOMString href; */
    #[inline]
    pub unsafe fn get_href(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_href)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMMediaList media; */
    #[inline]
    pub unsafe fn get_media(&self, ) -> Result<Option<RefPtr<nsIDOMMediaList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_media)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSStyleSheet styleSheet; */
    #[inline]
    pub unsafe fn get_styleSheet(&self, ) -> Result<Option<RefPtr<nsIDOMCSSStyleSheet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_styleSheet)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


