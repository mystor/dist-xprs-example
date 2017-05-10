//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStringBundleOverride.idl
//


#[repr(C)]
pub struct nsIStringBundleOverride {
    vtable: *const nsIStringBundleOverrideVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStringBundleOverride {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x965eb278, 0x5678, 0x456b,
            [0x82, 0xa7, 0x20, 0xa0, 0xc8, 0x6a, 0x80, 0x3c])
    }
}

unsafe impl RefCounted for nsIStringBundleOverride {
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
pub trait nsIStringBundleOverrideCoerce {
    fn coerce_from(v: &nsIStringBundleOverride) -> &Self;
}

impl nsIStringBundleOverrideCoerce for nsIStringBundleOverride {
    #[inline]
    fn coerce_from(v: &nsIStringBundleOverride) -> &Self {
        v
    }
}

impl nsIStringBundleOverride {
    #[inline]
    pub fn coerce<T: nsIStringBundleOverrideCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStringBundleOverride {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIStringBundleOverrideCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringBundleOverride) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStringBundleOverrideVTable {
    pub __base: nsISupportsVTable,

    /* AString getStringFromName (in AUTF8String url, in ACString key); */
    pub getStringFromName: unsafe extern "C" fn (this: *const nsIStringBundleOverride, url: *const nsACString, key: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* nsISimpleEnumerator enumerateKeysInBundle (in AUTF8String url); */
    pub enumerateKeysInBundle: unsafe extern "C" fn (this: *const nsIStringBundleOverride, url: *const nsACString, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIStringBundleOverride {
    /* AString getStringFromName (in AUTF8String url, in ACString key); */
    #[inline]
    pub unsafe fn getStringFromName(&self, url: &[u8], key: &[u8]) -> Result<nsString, nsresult> {
        let url = nsCString::from(url);
        let key = nsCString::from(key);
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringFromName)(self as *const _, &*url, &*key, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISimpleEnumerator enumerateKeysInBundle (in AUTF8String url); */
    #[inline]
    pub unsafe fn enumerateKeysInBundle(&self, url: &[u8]) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let url = nsCString::from(url);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerateKeysInBundle)(self as *const _, &*url, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


