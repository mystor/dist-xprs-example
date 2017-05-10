//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSStyleRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSStyleRule {
    vtable: *const nsIDOMCSSStyleRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSStyleRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb5e9af48, 0xa7c2, 0x4f88,
            [0xaa, 0xe3, 0x58, 0x30, 0x7a, 0xf4, 0xb5, 0xa5])
    }
}

unsafe impl RefCounted for nsIDOMCSSStyleRule {
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
pub trait nsIDOMCSSStyleRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSStyleRule) -> &Self;
}

impl nsIDOMCSSStyleRuleCoerce for nsIDOMCSSStyleRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleRule) -> &Self {
        v
    }
}

impl nsIDOMCSSStyleRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSStyleRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSStyleRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSStyleRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSStyleRuleVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString selectorText; */
    pub get_selectorText: unsafe extern "C" fn (this: *const nsIDOMCSSStyleRule, aSelectorText: *mut nsAString) -> nsresult,
    pub set_selectorText: unsafe extern "C" fn (this: *const nsIDOMCSSStyleRule, aSelectorText: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    pub get_style: unsafe extern "C" fn (this: *const nsIDOMCSSStyleRule, aStyle: *mut *const nsIDOMCSSStyleDeclaration) -> nsresult,

}


impl nsIDOMCSSStyleRule {
    /* attribute DOMString selectorText; */
    #[inline]
    pub unsafe fn get_selectorText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_selectorText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectorText(&self, aSelectorText: &[u16]) -> Result<(), nsresult> {
        let aSelectorText = nsString::from(aSelectorText);
        match ((*self.vtable).set_selectorText)(self as *const _, &*aSelectorText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    #[inline]
    pub unsafe fn get_style(&self, ) -> Result<Option<RefPtr<nsIDOMCSSStyleDeclaration>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_style)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


