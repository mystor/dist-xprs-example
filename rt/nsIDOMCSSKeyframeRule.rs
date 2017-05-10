//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSKeyframeRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSKeyframeRule {
    vtable: *const nsIDOMCSSKeyframeRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSKeyframeRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa281a8b4, 0xeaa2, 0x49a8,
            [0x8b, 0x97, 0xac, 0xc2, 0x81, 0x4a, 0x57, 0xc9])
    }
}

unsafe impl RefCounted for nsIDOMCSSKeyframeRule {
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
pub trait nsIDOMCSSKeyframeRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSKeyframeRule) -> &Self;
}

impl nsIDOMCSSKeyframeRuleCoerce for nsIDOMCSSKeyframeRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSKeyframeRule) -> &Self {
        v
    }
}

impl nsIDOMCSSKeyframeRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSKeyframeRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSKeyframeRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSKeyframeRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSKeyframeRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSKeyframeRuleVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString keyText; */
    pub get_keyText: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframeRule, aKeyText: *mut nsAString) -> nsresult,
    pub set_keyText: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframeRule, aKeyText: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    pub get_style: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframeRule, aStyle: *mut *const nsIDOMCSSStyleDeclaration) -> nsresult,

}


impl nsIDOMCSSKeyframeRule {
    /* attribute DOMString keyText; */
    #[inline]
    pub unsafe fn get_keyText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_keyText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_keyText(&self, aKeyText: &[u16]) -> Result<(), nsresult> {
        let aKeyText = nsString::from(aKeyText);
        match ((*self.vtable).set_keyText)(self as *const _, &*aKeyText) {
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


