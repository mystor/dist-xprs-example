//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSSupportsRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSSupportsRule {
    vtable: *const nsIDOMCSSSupportsRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSSupportsRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0b9e63a1, 0x1bd7, 0x4caf,
            [0x85, 0x0e, 0x14, 0x8b, 0x76, 0x2b, 0x14, 0xd2])
    }
}

unsafe impl RefCounted for nsIDOMCSSSupportsRule {
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
pub trait nsIDOMCSSSupportsRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSSupportsRule) -> &Self;
}

impl nsIDOMCSSSupportsRuleCoerce for nsIDOMCSSSupportsRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSSupportsRule) -> &Self {
        v
    }
}

impl nsIDOMCSSSupportsRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSSupportsRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSSupportsRule {
    type Target = nsIDOMCSSConditionRule;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSConditionRule {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSConditionRuleCoerce> nsIDOMCSSSupportsRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSSupportsRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSSupportsRuleVTable {
    pub __base: nsIDOMCSSConditionRuleVTable,

}


impl nsIDOMCSSSupportsRule {
}


