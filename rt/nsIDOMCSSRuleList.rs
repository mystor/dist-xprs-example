//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSRuleList.idl
//


#[repr(C)]
pub struct nsIDOMCSSRuleList {
    vtable: *const nsIDOMCSSRuleListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSRuleList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa6cf90c0, 0x15b3, 0x11d2,
            [0x93, 0x2e, 0x00, 0x80, 0x5f, 0x8a, 0xdd, 0x32])
    }
}

unsafe impl RefCounted for nsIDOMCSSRuleList {
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
pub trait nsIDOMCSSRuleListCoerce {
    fn coerce_from(v: &nsIDOMCSSRuleList) -> &Self;
}

impl nsIDOMCSSRuleListCoerce for nsIDOMCSSRuleList {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSRuleList) -> &Self {
        v
    }
}

impl nsIDOMCSSRuleList {
    #[inline]
    pub fn coerce<T: nsIDOMCSSRuleListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSRuleList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMCSSRuleListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSRuleList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSRuleListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMCSSRuleList, aLength: *mut libc::uint32_t) -> nsresult,

    /* nsIDOMCSSRule item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMCSSRuleList, index: libc::uint32_t, _retval: *mut *const nsIDOMCSSRule) -> nsresult,

}


impl nsIDOMCSSRuleList {
    /* readonly attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMCSSRule item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMCSSRule>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


