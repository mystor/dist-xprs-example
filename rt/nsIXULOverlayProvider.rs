//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULOverlayProvider.idl
//


#[repr(C)]
pub struct nsIXULOverlayProvider {
    vtable: *const nsIXULOverlayProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULOverlayProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1d5b5b94, 0xdc47, 0x4050,
            [0x93, 0xb7, 0xac, 0x09, 0x2e, 0x38, 0x3c, 0xad])
    }
}

unsafe impl RefCounted for nsIXULOverlayProvider {
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
pub trait nsIXULOverlayProviderCoerce {
    fn coerce_from(v: &nsIXULOverlayProvider) -> &Self;
}

impl nsIXULOverlayProviderCoerce for nsIXULOverlayProvider {
    #[inline]
    fn coerce_from(v: &nsIXULOverlayProvider) -> &Self {
        v
    }
}

impl nsIXULOverlayProvider {
    #[inline]
    pub fn coerce<T: nsIXULOverlayProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULOverlayProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULOverlayProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULOverlayProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULOverlayProviderVTable {
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator getXULOverlays (in nsIURI aURI); */
    pub getXULOverlays: unsafe extern "C" fn (this: *const nsIXULOverlayProvider, aURI: *const nsIURI, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsISimpleEnumerator getStyleOverlays (in nsIURI aURI); */
    pub getStyleOverlays: unsafe extern "C" fn (this: *const nsIXULOverlayProvider, aURI: *const nsIURI, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsIXULOverlayProvider {
    /* nsISimpleEnumerator getXULOverlays (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getXULOverlays(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getXULOverlays)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISimpleEnumerator getStyleOverlays (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getStyleOverlays(&self, aURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getStyleOverlays)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


