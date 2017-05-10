//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocale.idl
//


#[repr(C)]
pub struct nsILocale {
    vtable: *const nsILocaleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocale {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x21035ee0, 0x4556, 0x11d3,
            [0x91, 0xcd, 0x00, 0x10, 0x5a, 0xa3, 0xf7, 0xdc])
    }
}

unsafe impl RefCounted for nsILocale {
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
pub trait nsILocaleCoerce {
    fn coerce_from(v: &nsILocale) -> &Self;
}

impl nsILocaleCoerce for nsILocale {
    #[inline]
    fn coerce_from(v: &nsILocale) -> &Self {
        v
    }
}

impl nsILocale {
    #[inline]
    pub fn coerce<T: nsILocaleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocale {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILocaleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocale) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocaleVTable {
    pub __base: nsISupportsVTable,

    /* AString getCategory (in AString category); */
    pub getCategory: unsafe extern "C" fn (this: *const nsILocale, category: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsILocale {
    /* AString getCategory (in AString category); */
    #[inline]
    pub unsafe fn getCategory(&self, category: &[u16]) -> Result<nsString, nsresult> {
        let category = nsString::from(category);
        let mut _retval = nsString::new();
        match ((*self.vtable).getCategory)(self as *const _, &*category, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


