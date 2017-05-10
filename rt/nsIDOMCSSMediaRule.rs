//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSMediaRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSMediaRule {
    vtable: *const nsIDOMCSSMediaRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSMediaRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6cf9c5b2, 0xfa0f, 0x43c0,
            [0xaa, 0x50, 0xef, 0x85, 0xb4, 0x75, 0x6e, 0x3a])
    }
}

unsafe impl RefCounted for nsIDOMCSSMediaRule {
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
pub trait nsIDOMCSSMediaRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSMediaRule) -> &Self;
}

impl nsIDOMCSSMediaRuleCoerce for nsIDOMCSSMediaRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSMediaRule) -> &Self {
        v
    }
}

impl nsIDOMCSSMediaRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSMediaRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSMediaRule {
    type Target = nsIDOMCSSConditionRule;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSConditionRule {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSConditionRuleCoerce> nsIDOMCSSMediaRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSMediaRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSMediaRuleVTable {
    pub __base: nsIDOMCSSConditionRuleVTable,

    /* readonly attribute nsIDOMMediaList media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMCSSMediaRule, aMedia: *mut *const nsIDOMMediaList) -> nsresult,

}


impl nsIDOMCSSMediaRule {
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

}


