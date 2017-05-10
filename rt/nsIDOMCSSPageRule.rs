//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSPageRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSPageRule {
    vtable: *const nsIDOMCSSPageRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSPageRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc119072b, 0x7d2f, 0x4aeb,
            [0xa9, 0x0d, 0xe2, 0xd6, 0xb6, 0x06, 0xc3, 0x2a])
    }
}

unsafe impl RefCounted for nsIDOMCSSPageRule {
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
pub trait nsIDOMCSSPageRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSPageRule) -> &Self;
}

impl nsIDOMCSSPageRuleCoerce for nsIDOMCSSPageRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSPageRule) -> &Self {
        v
    }
}

impl nsIDOMCSSPageRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSPageRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSPageRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSPageRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSPageRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSPageRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    pub get_style: unsafe extern "C" fn (this: *const nsIDOMCSSPageRule, aStyle: *mut *const nsIDOMCSSStyleDeclaration) -> nsresult,

}


impl nsIDOMCSSPageRule {
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


