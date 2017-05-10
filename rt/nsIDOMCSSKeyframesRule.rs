//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSKeyframesRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSKeyframesRule {
    vtable: *const nsIDOMCSSKeyframesRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSKeyframesRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x400f4b70, 0xad0a, 0x4047,
            [0xab, 0xa4, 0xee, 0x80, 0x19, 0xf6, 0xb9, 0x07])
    }
}

unsafe impl RefCounted for nsIDOMCSSKeyframesRule {
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
pub trait nsIDOMCSSKeyframesRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSKeyframesRule) -> &Self;
}

impl nsIDOMCSSKeyframesRuleCoerce for nsIDOMCSSKeyframesRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSKeyframesRule) -> &Self {
        v
    }
}

impl nsIDOMCSSKeyframesRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSKeyframesRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSKeyframesRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSKeyframesRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSKeyframesRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSKeyframesRuleVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, aName: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSRuleList cssRules; */
    pub get_cssRules: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, aCssRules: *mut *const nsIDOMCSSRuleList) -> nsresult,

    /* void appendRule (in DOMString rule); */
    pub appendRule: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, rule: *const nsAString) -> nsresult,

    /* void deleteRule (in DOMString key); */
    pub deleteRule: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, key: *const nsAString) -> nsresult,

    /* nsIDOMCSSKeyframeRule findRule (in DOMString key); */
    pub findRule: unsafe extern "C" fn (this: *const nsIDOMCSSKeyframesRule, key: *const nsAString, _retval: *mut *const nsIDOMCSSKeyframeRule) -> nsresult,

}


impl nsIDOMCSSKeyframesRule {
    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMCSSRuleList cssRules; */
    #[inline]
    pub unsafe fn get_cssRules(&self, ) -> Result<Option<RefPtr<nsIDOMCSSRuleList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cssRules)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void appendRule (in DOMString rule); */
    #[inline]
    pub unsafe fn appendRule(&self, rule: &[u16]) -> Result<(), nsresult> {
        let rule = nsString::from(rule);
        match ((*self.vtable).appendRule)(self as *const _, &*rule) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteRule (in DOMString key); */
    #[inline]
    pub unsafe fn deleteRule(&self, key: &[u16]) -> Result<(), nsresult> {
        let key = nsString::from(key);
        match ((*self.vtable).deleteRule)(self as *const _, &*key) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMCSSKeyframeRule findRule (in DOMString key); */
    #[inline]
    pub unsafe fn findRule(&self, key: &[u16]) -> Result<Option<RefPtr<nsIDOMCSSKeyframeRule>>, nsresult> {
        let key = nsString::from(key);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findRule)(self as *const _, &*key, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


