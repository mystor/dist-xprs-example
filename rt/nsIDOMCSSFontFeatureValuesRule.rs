//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSFontFeatureValuesRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSFontFeatureValuesRule {
    vtable: *const nsIDOMCSSFontFeatureValuesRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSFontFeatureValuesRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa343d27f, 0x1da6, 0x4fc3,
            [0x93, 0x55, 0xd4, 0xca, 0x43, 0x4f, 0x95, 0x8e])
    }
}

unsafe impl RefCounted for nsIDOMCSSFontFeatureValuesRule {
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
pub trait nsIDOMCSSFontFeatureValuesRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSFontFeatureValuesRule) -> &Self;
}

impl nsIDOMCSSFontFeatureValuesRuleCoerce for nsIDOMCSSFontFeatureValuesRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSFontFeatureValuesRule) -> &Self {
        v
    }
}

impl nsIDOMCSSFontFeatureValuesRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSFontFeatureValuesRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSFontFeatureValuesRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSFontFeatureValuesRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSFontFeatureValuesRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSFontFeatureValuesRuleVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString fontFamily; */
    pub get_fontFamily: unsafe extern "C" fn (this: *const nsIDOMCSSFontFeatureValuesRule, aFontFamily: *mut nsAString) -> nsresult,
    pub set_fontFamily: unsafe extern "C" fn (this: *const nsIDOMCSSFontFeatureValuesRule, aFontFamily: *const nsAString) -> nsresult,

    /* attribute DOMString valueText; */
    pub get_valueText: unsafe extern "C" fn (this: *const nsIDOMCSSFontFeatureValuesRule, aValueText: *mut nsAString) -> nsresult,
    pub set_valueText: unsafe extern "C" fn (this: *const nsIDOMCSSFontFeatureValuesRule, aValueText: *const nsAString) -> nsresult,

}


impl nsIDOMCSSFontFeatureValuesRule {
    /* attribute DOMString fontFamily; */
    #[inline]
    pub unsafe fn get_fontFamily(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_fontFamily)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fontFamily(&self, aFontFamily: &[u16]) -> Result<(), nsresult> {
        let aFontFamily = nsString::from(aFontFamily);
        match ((*self.vtable).set_fontFamily)(self as *const _, &*aFontFamily) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString valueText; */
    #[inline]
    pub unsafe fn get_valueText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_valueText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_valueText(&self, aValueText: &[u16]) -> Result<(), nsresult> {
        let aValueText = nsString::from(aValueText);
        match ((*self.vtable).set_valueText)(self as *const _, &*aValueText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


