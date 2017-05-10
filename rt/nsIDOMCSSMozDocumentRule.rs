//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSMozDocumentRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSMozDocumentRule {
    vtable: *const nsIDOMCSSMozDocumentRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSMozDocumentRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2d0cef9d, 0xc1b2, 0x4c6c,
            [0x90, 0x03, 0xfa, 0x83, 0x04, 0x06, 0x26, 0xd1])
    }
}

unsafe impl RefCounted for nsIDOMCSSMozDocumentRule {
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
pub trait nsIDOMCSSMozDocumentRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSMozDocumentRule) -> &Self;
}

impl nsIDOMCSSMozDocumentRuleCoerce for nsIDOMCSSMozDocumentRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSMozDocumentRule) -> &Self {
        v
    }
}

impl nsIDOMCSSMozDocumentRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSMozDocumentRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSMozDocumentRule {
    type Target = nsIDOMCSSConditionRule;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSConditionRule {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSConditionRuleCoerce> nsIDOMCSSMozDocumentRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSMozDocumentRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSMozDocumentRuleVTable {
    pub __base: nsIDOMCSSConditionRuleVTable,

}


impl nsIDOMCSSMozDocumentRule {
}


