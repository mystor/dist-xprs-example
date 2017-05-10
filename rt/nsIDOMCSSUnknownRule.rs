//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSUnknownRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSUnknownRule {
    vtable: *const nsIDOMCSSUnknownRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSUnknownRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x98f4c27b, 0xfb35, 0x4355,
            [0x8f, 0xd9, 0x54, 0x6c, 0x46, 0x97, 0xd7, 0x1a])
    }
}

unsafe impl RefCounted for nsIDOMCSSUnknownRule {
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
pub trait nsIDOMCSSUnknownRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSUnknownRule) -> &Self;
}

impl nsIDOMCSSUnknownRuleCoerce for nsIDOMCSSUnknownRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSUnknownRule) -> &Self {
        v
    }
}

impl nsIDOMCSSUnknownRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSUnknownRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSUnknownRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSUnknownRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSUnknownRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSUnknownRuleVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMCSSUnknownRule {
}


