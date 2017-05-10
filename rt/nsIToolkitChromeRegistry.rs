//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitChromeRegistry.idl
//


#[repr(C)]
pub struct nsIToolkitChromeRegistry {
    vtable: *const nsIToolkitChromeRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIToolkitChromeRegistry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8727651c, 0x9530, 0x45a0,
            [0xb8, 0x1e, 0x0e, 0x06, 0x90, 0xc3, 0x0c, 0x50])
    }
}

unsafe impl RefCounted for nsIToolkitChromeRegistry {
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
pub trait nsIToolkitChromeRegistryCoerce {
    fn coerce_from(v: &nsIToolkitChromeRegistry) -> &Self;
}

impl nsIToolkitChromeRegistryCoerce for nsIToolkitChromeRegistry {
    #[inline]
    fn coerce_from(v: &nsIToolkitChromeRegistry) -> &Self {
        v
    }
}

impl nsIToolkitChromeRegistry {
    #[inline]
    pub fn coerce<T: nsIToolkitChromeRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIToolkitChromeRegistry {
    type Target = nsIXULChromeRegistry;
    #[inline]
    fn deref(&self) -> &nsIXULChromeRegistry {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIXULChromeRegistryCoerce> nsIToolkitChromeRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitChromeRegistry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIToolkitChromeRegistryVTable {
    pub __base: nsIXULChromeRegistryVTable,

    /* void checkForOSAccessibility (); */
    pub checkForOSAccessibility: unsafe extern "C" fn (this: *const nsIToolkitChromeRegistry) -> nsresult,

    /* nsIUTF8StringEnumerator getLocalesForPackage (in AUTF8String aPackage); */
    pub getLocalesForPackage: unsafe extern "C" fn (this: *const nsIToolkitChromeRegistry, aPackage: *const nsACString, _retval: *mut *const nsIUTF8StringEnumerator) -> nsresult,

}


impl nsIToolkitChromeRegistry {
    /* void checkForOSAccessibility (); */
    #[inline]
    pub unsafe fn checkForOSAccessibility(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkForOSAccessibility)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIUTF8StringEnumerator getLocalesForPackage (in AUTF8String aPackage); */
    #[inline]
    pub unsafe fn getLocalesForPackage(&self, aPackage: &[u8]) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let aPackage = nsCString::from(aPackage);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLocalesForPackage)(self as *const _, &*aPackage, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


