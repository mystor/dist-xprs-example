//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULTemplateRuleFilter.idl
//


#[repr(C)]
pub struct nsIXULTemplateRuleFilter {
    vtable: *const nsIXULTemplateRuleFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULTemplateRuleFilter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x819cd1ed, 0x8010, 0x42e1,
            [0xa8, 0xb9, 0x77, 0x8b, 0x72, 0x6a, 0x1f, 0xf3])
    }
}

unsafe impl RefCounted for nsIXULTemplateRuleFilter {
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
pub trait nsIXULTemplateRuleFilterCoerce {
    fn coerce_from(v: &nsIXULTemplateRuleFilter) -> &Self;
}

impl nsIXULTemplateRuleFilterCoerce for nsIXULTemplateRuleFilter {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateRuleFilter) -> &Self {
        v
    }
}

impl nsIXULTemplateRuleFilter {
    #[inline]
    pub fn coerce<T: nsIXULTemplateRuleFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULTemplateRuleFilter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULTemplateRuleFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULTemplateRuleFilter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULTemplateRuleFilterVTable {
    pub __base: nsISupportsVTable,

    /* boolean match (in nsIXULTemplateResult aRef, in nsIDOMNode aRule); */
    pub match_: unsafe extern "C" fn (this: *const nsIXULTemplateRuleFilter, aRef: *const nsIXULTemplateResult, aRule: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

}


impl nsIXULTemplateRuleFilter {
    /* boolean match (in nsIXULTemplateResult aRef, in nsIDOMNode aRule); */
    #[inline]
    pub unsafe fn match_(&self, aRef: Option<&nsIXULTemplateResult>, aRule: Option<&nsIDOMNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).match_)(self as *const _, aRef.map_or(::std::ptr::null(), |x| x as *const _), aRule.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


