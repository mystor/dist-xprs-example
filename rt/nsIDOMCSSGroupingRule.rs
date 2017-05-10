//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSGroupingRule.idl
//


#[repr(C)]
pub struct nsIDOMCSSGroupingRule {
    vtable: *const nsIDOMCSSGroupingRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSGroupingRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa0e3324a, 0xf911, 0x4baf,
            [0x95, 0x91, 0x53, 0x22, 0xc7, 0x6c, 0xbb, 0x0d])
    }
}

unsafe impl RefCounted for nsIDOMCSSGroupingRule {
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
pub trait nsIDOMCSSGroupingRuleCoerce {
    fn coerce_from(v: &nsIDOMCSSGroupingRule) -> &Self;
}

impl nsIDOMCSSGroupingRuleCoerce for nsIDOMCSSGroupingRule {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSGroupingRule) -> &Self {
        v
    }
}

impl nsIDOMCSSGroupingRule {
    #[inline]
    pub fn coerce<T: nsIDOMCSSGroupingRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSGroupingRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSGroupingRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSGroupingRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSGroupingRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMCSSRuleList cssRules; */
    pub get_cssRules: unsafe extern "C" fn (this: *const nsIDOMCSSGroupingRule, aCssRules: *mut *const nsIDOMCSSRuleList) -> nsresult,

    /* unsigned long insertRule (in DOMString rule, in unsigned long index) raises (DOMException); */
    pub insertRule: unsafe extern "C" fn (this: *const nsIDOMCSSGroupingRule, rule: *const nsAString, index: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void deleteRule (in unsigned long index) raises (DOMException); */
    pub deleteRule: unsafe extern "C" fn (this: *const nsIDOMCSSGroupingRule, index: libc::uint32_t) -> nsresult,

}


impl nsIDOMCSSGroupingRule {
    /* readonly attribute nsIDOMCSSRuleList cssRules; */
    #[inline]
    pub unsafe fn get_cssRules(&self, ) -> Result<Option<RefPtr<nsIDOMCSSRuleList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cssRules)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long insertRule (in DOMString rule, in unsigned long index) raises (DOMException); */
    #[inline]
    pub unsafe fn insertRule(&self, rule: &[u16], index: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let rule = nsString::from(rule);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).insertRule)(self as *const _, &*rule, index, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void deleteRule (in unsigned long index) raises (DOMException); */
    #[inline]
    pub unsafe fn deleteRule(&self, index: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteRule)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


