//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSCounterStyleRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSCounterStyleRule {
    vtable: *const nsIDOMCSSCounterStyleRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSCounterStyleRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9b5e48ce, 0xd84c, 0x4e31,
            [0xaf, 0xf5, 0x34, 0xe9, 0xf4, 0x14, 0x13, 0x13])
    }
}

unsafe impl RefCounted for nsIDOMCSSCounterStyleRule {
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
pub trait nsIDOMCSSCounterStyleRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSCounterStyleRule) -> &Self;
}

impl nsIDOMCSSCounterStyleRuleCoerce for nsIDOMCSSCounterStyleRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSCounterStyleRule) -> &Self {
        v
    }
}

impl nsIDOMCSSCounterStyleRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSCounterStyleRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSCounterStyleRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSCounterStyleRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSCounterStyleRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSCounterStyleRuleVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aName: *const nsAString) -> nsresult,

    /* attribute DOMString system; */
    pub get_system: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSystem: *mut nsAString) -> nsresult,
    pub set_system: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSystem: *const nsAString) -> nsresult,

    /* attribute DOMString symbols; */
    pub get_symbols: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSymbols: *mut nsAString) -> nsresult,
    pub set_symbols: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSymbols: *const nsAString) -> nsresult,

    /* attribute DOMString additiveSymbols; */
    pub get_additiveSymbols: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aAdditiveSymbols: *mut nsAString) -> nsresult,
    pub set_additiveSymbols: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aAdditiveSymbols: *const nsAString) -> nsresult,

    /* attribute DOMString negative; */
    pub get_negative: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aNegative: *mut nsAString) -> nsresult,
    pub set_negative: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aNegative: *const nsAString) -> nsresult,

    /* attribute DOMString prefix; */
    pub get_prefix: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aPrefix: *mut nsAString) -> nsresult,
    pub set_prefix: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aPrefix: *const nsAString) -> nsresult,

    /* attribute DOMString suffix; */
    pub get_suffix: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSuffix: *mut nsAString) -> nsresult,
    pub set_suffix: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSuffix: *const nsAString) -> nsresult,

    /* attribute DOMString range; */
    pub get_range: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aRange: *mut nsAString) -> nsresult,
    pub set_range: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aRange: *const nsAString) -> nsresult,

    /* attribute DOMString pad; */
    pub get_pad: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aPad: *mut nsAString) -> nsresult,
    pub set_pad: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aPad: *const nsAString) -> nsresult,

    /* attribute DOMString speakAs; */
    pub get_speakAs: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSpeakAs: *mut nsAString) -> nsresult,
    pub set_speakAs: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aSpeakAs: *const nsAString) -> nsresult,

    /* attribute DOMString fallback; */
    pub get_fallback: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aFallback: *mut nsAString) -> nsresult,
    pub set_fallback: unsafe extern "C" fn (this: *const nsIDOMCSSCounterStyleRule, aFallback: *const nsAString) -> nsresult,

}


impl nsIDOMCSSCounterStyleRule {
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

    /* attribute DOMString system; */
    #[inline]
    pub unsafe fn get_system(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_system)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_system(&self, aSystem: &[u16]) -> Result<(), nsresult> {
        let aSystem = nsString::from(aSystem);
        match ((*self.vtable).set_system)(self as *const _, &*aSystem) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString symbols; */
    #[inline]
    pub unsafe fn get_symbols(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_symbols)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_symbols(&self, aSymbols: &[u16]) -> Result<(), nsresult> {
        let aSymbols = nsString::from(aSymbols);
        match ((*self.vtable).set_symbols)(self as *const _, &*aSymbols) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString additiveSymbols; */
    #[inline]
    pub unsafe fn get_additiveSymbols(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_additiveSymbols)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_additiveSymbols(&self, aAdditiveSymbols: &[u16]) -> Result<(), nsresult> {
        let aAdditiveSymbols = nsString::from(aAdditiveSymbols);
        match ((*self.vtable).set_additiveSymbols)(self as *const _, &*aAdditiveSymbols) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString negative; */
    #[inline]
    pub unsafe fn get_negative(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_negative)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_negative(&self, aNegative: &[u16]) -> Result<(), nsresult> {
        let aNegative = nsString::from(aNegative);
        match ((*self.vtable).set_negative)(self as *const _, &*aNegative) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString prefix; */
    #[inline]
    pub unsafe fn get_prefix(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_prefix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_prefix(&self, aPrefix: &[u16]) -> Result<(), nsresult> {
        let aPrefix = nsString::from(aPrefix);
        match ((*self.vtable).set_prefix)(self as *const _, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString suffix; */
    #[inline]
    pub unsafe fn get_suffix(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_suffix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_suffix(&self, aSuffix: &[u16]) -> Result<(), nsresult> {
        let aSuffix = nsString::from(aSuffix);
        match ((*self.vtable).set_suffix)(self as *const _, &*aSuffix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString range; */
    #[inline]
    pub unsafe fn get_range(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_range)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_range(&self, aRange: &[u16]) -> Result<(), nsresult> {
        let aRange = nsString::from(aRange);
        match ((*self.vtable).set_range)(self as *const _, &*aRange) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString pad; */
    #[inline]
    pub unsafe fn get_pad(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pad)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pad(&self, aPad: &[u16]) -> Result<(), nsresult> {
        let aPad = nsString::from(aPad);
        match ((*self.vtable).set_pad)(self as *const _, &*aPad) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString speakAs; */
    #[inline]
    pub unsafe fn get_speakAs(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_speakAs)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_speakAs(&self, aSpeakAs: &[u16]) -> Result<(), nsresult> {
        let aSpeakAs = nsString::from(aSpeakAs);
        match ((*self.vtable).set_speakAs)(self as *const _, &*aSpeakAs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString fallback; */
    #[inline]
    pub unsafe fn get_fallback(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_fallback)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fallback(&self, aFallback: &[u16]) -> Result<(), nsresult> {
        let aFallback = nsString::from(aFallback);
        match ((*self.vtable).set_fallback)(self as *const _, &*aFallback) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


