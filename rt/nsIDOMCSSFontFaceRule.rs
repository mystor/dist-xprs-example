//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSFontFaceRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSFontFaceRule {
    vtable: *const nsIDOMCSSFontFaceRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSFontFaceRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdb971017, 0xfe0c, 0x4529,
            [0x97, 0x2c, 0x82, 0x17, 0xf2, 0xfe, 0xe2, 0x17])
    }
}

unsafe impl RefCounted for nsIDOMCSSFontFaceRule {
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
pub trait nsIDOMCSSFontFaceRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSFontFaceRule) -> &Self;
}

impl nsIDOMCSSFontFaceRuleCoerce for nsIDOMCSSFontFaceRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSFontFaceRule) -> &Self {
        v
    }
}

impl nsIDOMCSSFontFaceRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSFontFaceRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSFontFaceRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSFontFaceRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSFontFaceRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSFontFaceRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMCSSStyleDeclaration style; */
    pub get_style: unsafe extern "C" fn (this: *const nsIDOMCSSFontFaceRule, aStyle: *mut *const nsIDOMCSSStyleDeclaration) -> nsresult,

}


impl nsIDOMCSSFontFaceRule {
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


