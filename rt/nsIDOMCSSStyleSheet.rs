//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSStyleSheet.idl
//


#[repr(C)]
pub struct nsIDOMCSSStyleSheet {
    vtable: *const nsIDOMCSSStyleSheetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSStyleSheet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa6cf90c2, 0x15b3, 0x11d2,
            [0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32])
    }
}

unsafe impl RefCounted for nsIDOMCSSStyleSheet {
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
pub trait nsIDOMCSSStyleSheetCoerce {
    fn coerce_from(v: &nsIDOMCSSStyleSheet) -> &Self;
}

impl nsIDOMCSSStyleSheetCoerce for nsIDOMCSSStyleSheet {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleSheet) -> &Self {
        v
    }
}

impl nsIDOMCSSStyleSheet {
    #[inline]
    pub fn coerce<T: nsIDOMCSSStyleSheetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSStyleSheet {
    type Target = nsIDOMStyleSheet;
    #[inline]
    fn deref(&self) -> &nsIDOMStyleSheet {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMStyleSheetCoerce> nsIDOMCSSStyleSheetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSStyleSheet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSStyleSheetVTable {
    pub __base: nsIDOMStyleSheetVTable,

    /* readonly attribute nsIDOMCSSRule ownerRule; */
    pub get_ownerRule: unsafe extern "C" fn (this: *const nsIDOMCSSStyleSheet, aOwnerRule: *mut *const nsIDOMCSSRule) -> nsresult,

    /* readonly attribute nsIDOMCSSRuleList cssRules; */
    pub get_cssRules: unsafe extern "C" fn (this: *const nsIDOMCSSStyleSheet, aCssRules: *mut *const nsIDOMCSSRuleList) -> nsresult,

    /* unsigned long insertRule (in DOMString rule, in unsigned long index) raises (DOMException); */
    pub insertRule: unsafe extern "C" fn (this: *const nsIDOMCSSStyleSheet, rule: *const nsAString, index: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void deleteRule (in unsigned long index) raises (DOMException); */
    pub deleteRule: unsafe extern "C" fn (this: *const nsIDOMCSSStyleSheet, index: libc::uint32_t) -> nsresult,

}


impl nsIDOMCSSStyleSheet {
    /* readonly attribute nsIDOMCSSRule ownerRule; */
    #[inline]
    pub unsafe fn get_ownerRule(&self, ) -> Result<Option<RefPtr<nsIDOMCSSRule>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerRule)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

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


