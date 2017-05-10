//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSRule.idl
//


pub mod nsIDOMCSSRule_consts {
    pub const UNKNOWN_RULE: i64 = 0;
    pub const STYLE_RULE: i64 = 1;
    pub const CHARSET_RULE: i64 = 2;
    pub const IMPORT_RULE: i64 = 3;
    pub const MEDIA_RULE: i64 = 4;
    pub const FONT_FACE_RULE: i64 = 5;
    pub const PAGE_RULE: i64 = 6;
    pub const KEYFRAMES_RULE: i64 = 7;
    pub const KEYFRAME_RULE: i64 = 8;
    pub const MOZ_KEYFRAMES_RULE: i64 = 7;
    pub const MOZ_KEYFRAME_RULE: i64 = 8;
    pub const NAMESPACE_RULE: i64 = 10;
    pub const COUNTER_STYLE_RULE: i64 = 11;
    pub const SUPPORTS_RULE: i64 = 12;
    pub const FONT_FEATURE_VALUES_RULE: i64 = 14;
}


#[repr(C)]
pub struct nsIDOMCSSRule {
    vtable: *const nsIDOMCSSRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d6b3bad, 0xf53c, 0x4585,
            [0x82, 0xf6, 0x62, 0x98, 0x2e, 0x27, 0xed, 0xe8])
    }
}

unsafe impl RefCounted for nsIDOMCSSRule {
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
pub trait nsIDOMCSSRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSRule) -> &Self;
}

impl nsIDOMCSSRuleCoerce for nsIDOMCSSRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSRule) -> &Self {
        v
    }
}

impl nsIDOMCSSRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMCSSRule, aType: *mut libc::uint16_t) -> nsresult,

    /* attribute DOMString cssText; */
    pub get_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSRule, aCssText: *mut nsAString) -> nsresult,
    pub set_cssText: unsafe extern "C" fn (this: *const nsIDOMCSSRule, aCssText: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMCSSStyleSheet parentStyleSheet; */
    pub get_parentStyleSheet: unsafe extern "C" fn (this: *const nsIDOMCSSRule, aParentStyleSheet: *mut *const nsIDOMCSSStyleSheet) -> nsresult,

    /* readonly attribute nsIDOMCSSRule parentRule; */
    pub get_parentRule: unsafe extern "C" fn (this: *const nsIDOMCSSRule, aParentRule: *mut *const nsIDOMCSSRule) -> nsresult,

    /* [noscript,nostdcall,notxpcom] Rule getCSSRule (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCSSRule: *const ::libc::c_void,

}


impl nsIDOMCSSRule {
    /* readonly attribute unsigned short type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString cssText; */
    #[inline]
    pub unsafe fn get_cssText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cssText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cssText(&self, aCssText: &[u16]) -> Result<(), nsresult> {
        let aCssText = nsString::from(aCssText);
        match ((*self.vtable).set_cssText)(self as *const _, &*aCssText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMCSSStyleSheet parentStyleSheet; */
    #[inline]
    pub unsafe fn get_parentStyleSheet(&self, ) -> Result<Option<RefPtr<nsIDOMCSSStyleSheet>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentStyleSheet)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMCSSRule parentRule; */
    #[inline]
    pub unsafe fn get_parentRule(&self, ) -> Result<Option<RefPtr<nsIDOMCSSRule>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentRule)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,nostdcall,notxpcom] Rule getCSSRule (); */


}


