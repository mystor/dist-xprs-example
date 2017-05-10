//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSConditionRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSConditionRule {
    vtable: *const nsIDOMCSSConditionRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSConditionRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44da41b2, 0x5660, 0x415d,
            [0x86, 0x92, 0xea, 0xe8, 0x05, 0x77, 0x61, 0x03])
    }
}

unsafe impl RefCounted for nsIDOMCSSConditionRule {
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
pub trait nsIDOMCSSConditionRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSConditionRule) -> &Self;
}

impl nsIDOMCSSConditionRuleCoerce for nsIDOMCSSConditionRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSConditionRule) -> &Self {
        v
    }
}

impl nsIDOMCSSConditionRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSConditionRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSConditionRule {
    type Target = nsIDOMCSSGroupingRule;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSGroupingRule {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSGroupingRuleCoerce> nsIDOMCSSConditionRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSConditionRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSConditionRuleVTable {
    pub __base: nsIDOMCSSGroupingRuleVTable,

    /* attribute DOMString conditionText; */
    pub get_conditionText: unsafe extern "C" fn (this: *const nsIDOMCSSConditionRule, aConditionText: *mut nsAString) -> nsresult,
    pub set_conditionText: unsafe extern "C" fn (this: *const nsIDOMCSSConditionRule, aConditionText: *const nsAString) -> nsresult,

}


impl nsIDOMCSSConditionRule {
    /* attribute DOMString conditionText; */
    #[inline]
    pub unsafe fn get_conditionText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_conditionText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_conditionText(&self, aConditionText: &[u16]) -> Result<(), nsresult> {
        let aConditionText = nsString::from(aConditionText);
        match ((*self.vtable).set_conditionText)(self as *const _, &*aConditionText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


