//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowProvider.idl
//


#[repr(C)]
pub struct nsIWindowProvider {
    vtable: *const nsIWindowProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe97a3830, 0x15ef, 0x499b,
            [0x83, 0x72, 0xc2, 0x2d, 0x12, 0x80, 0x91, 0xc1])
    }
}

unsafe impl RefCounted for nsIWindowProvider {
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
pub trait nsIWindowProviderCoerce {
    fn coerce_from(v: &nsIWindowProvider) -> &Self;
}

impl nsIWindowProviderCoerce for nsIWindowProvider {
    #[inline]
    fn coerce_from(v: &nsIWindowProvider) -> &Self {
        v
    }
}

impl nsIWindowProvider {
    #[inline]
    pub fn coerce<T: nsIWindowProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowProviderVTable {
    pub __base: nsISupportsVTable,

    /* mozIDOMWindowProxy provideWindow (in mozIDOMWindowProxy aParent, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aPositionSpecified, in boolean aSizeSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, out boolean aWindowIsNew); */
    pub provideWindow: unsafe extern "C" fn (this: *const nsIWindowProvider, aParent: *const mozIDOMWindowProxy, aChromeFlags: libc::uint32_t, aCalledFromJS: bool, aPositionSpecified: bool, aSizeSpecified: bool, aURI: *const nsIURI, aName: *const nsAString, aFeatures: *const nsACString, aForceNoOpener: bool, aWindowIsNew: *mut bool, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

}


impl nsIWindowProvider {
    /* mozIDOMWindowProxy provideWindow (in mozIDOMWindowProxy aParent, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aPositionSpecified, in boolean aSizeSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, out boolean aWindowIsNew); */
    #[inline]
    pub unsafe fn provideWindow(&self, aParent: Option<&mozIDOMWindowProxy>, aChromeFlags: libc::uint32_t, aCalledFromJS: bool, aPositionSpecified: bool, aSizeSpecified: bool, aURI: Option<&nsIURI>, aName: &[u16], aFeatures: &[u8], aForceNoOpener: bool) -> Result<(bool, Option<RefPtr<mozIDOMWindowProxy>>), nsresult> {
        let aName = nsString::from(aName);
        let aFeatures = nsCString::from(aFeatures);
        let mut aWindowIsNew: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).provideWindow)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aChromeFlags, aCalledFromJS, aPositionSpecified, aSizeSpecified, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &*aFeatures, aForceNoOpener, &mut aWindowIsNew as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aWindowIsNew, _retval.refptr()))
    }

}


