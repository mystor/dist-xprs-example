//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURIClassifier.idl
//


#[repr(C)]
pub struct nsIURIClassifierCallback {
    vtable: *const nsIURIClassifierCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIClassifierCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8face46e, 0x0c96, 0x470f,
            [0xaf, 0x40, 0x00, 0x37, 0xdc, 0xd7, 0x97, 0xbd])
    }
}

unsafe impl RefCounted for nsIURIClassifierCallback {
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
pub trait nsIURIClassifierCallbackCoerce {
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self;
}

impl nsIURIClassifierCallbackCoerce for nsIURIClassifierCallback {
    #[inline]
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self {
        v
    }
}

impl nsIURIClassifierCallback {
    #[inline]
    pub fn coerce<T: nsIURIClassifierCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIClassifierCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIClassifierCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIClassifierCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIClassifierCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aPrefix); */
    pub onClassifyComplete: unsafe extern "C" fn (this: *const nsIURIClassifierCallback, aErrorCode: nsresult, aList: *const nsACString, aProvider: *const nsACString, aPrefix: *const nsACString) -> nsresult,

}


impl nsIURIClassifierCallback {
    /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aPrefix); */
    #[inline]
    pub unsafe fn onClassifyComplete(&self, aErrorCode: nsresult, aList: &[u8], aProvider: &[u8], aPrefix: &[u8]) -> Result<(), nsresult> {
        let aList = nsCString::from(aList);
        let aProvider = nsCString::from(aProvider);
        let aPrefix = nsCString::from(aPrefix);
        match ((*self.vtable).onClassifyComplete)(self as *const _, aErrorCode, &*aList, &*aProvider, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIURIClassifier {
    vtable: *const nsIURIClassifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURIClassifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x596620cc, 0x76e3, 0x4133,
            [0x9d, 0x90, 0x36, 0x0e, 0x59, 0xa7, 0x94, 0xcf])
    }
}

unsafe impl RefCounted for nsIURIClassifier {
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
pub trait nsIURIClassifierCoerce {
    fn coerce_from(v: &nsIURIClassifier) -> &Self;
}

impl nsIURIClassifierCoerce for nsIURIClassifier {
    #[inline]
    fn coerce_from(v: &nsIURIClassifier) -> &Self {
        v
    }
}

impl nsIURIClassifier {
    #[inline]
    pub fn coerce<T: nsIURIClassifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURIClassifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIURIClassifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURIClassifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURIClassifierVTable {
    pub __base: nsISupportsVTable,

    /* boolean classify (in nsIPrincipal aPrincipal, in nsIEventTarget aEventTarget, in boolean aTrackingProtectionEnabled, in nsIURIClassifierCallback aCallback); */
    pub classify: unsafe extern "C" fn (this: *const nsIURIClassifier, aPrincipal: *const nsIPrincipal, aEventTarget: *const nsIEventTarget, aTrackingProtectionEnabled: bool, aCallback: *const nsIURIClassifierCallback, _retval: *mut bool) -> nsresult,

    /* [noscript] StringArrayRef classifyLocalWithTables (in nsIURI aURI, in ACString aTables); */
    /// Unable to call function as its signature contains a non-rust type
    pub classifyLocalWithTables: *const ::libc::c_void,

    /* void asyncClassifyLocalWithTables (in nsIURI aURI, in ACString aTables, in nsIURIClassifierCallback aCallback); */
    pub asyncClassifyLocalWithTables: unsafe extern "C" fn (this: *const nsIURIClassifier, aURI: *const nsIURI, aTables: *const nsACString, aCallback: *const nsIURIClassifierCallback) -> nsresult,

    /* ACString classifyLocal (in nsIURI aURI, in ACString aTables); */
    pub classifyLocal: unsafe extern "C" fn (this: *const nsIURIClassifier, aURI: *const nsIURI, aTables: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIURIClassifier {
    /* boolean classify (in nsIPrincipal aPrincipal, in nsIEventTarget aEventTarget, in boolean aTrackingProtectionEnabled, in nsIURIClassifierCallback aCallback); */
    #[inline]
    pub unsafe fn classify(&self, aPrincipal: Option<&nsIPrincipal>, aEventTarget: Option<&nsIEventTarget>, aTrackingProtectionEnabled: bool, aCallback: Option<&nsIURIClassifierCallback>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).classify)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aEventTarget.map_or(::std::ptr::null(), |x| x as *const _), aTrackingProtectionEnabled, aCallback.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] StringArrayRef classifyLocalWithTables (in nsIURI aURI, in ACString aTables); */


    /* void asyncClassifyLocalWithTables (in nsIURI aURI, in ACString aTables, in nsIURIClassifierCallback aCallback); */
    #[inline]
    pub unsafe fn asyncClassifyLocalWithTables(&self, aURI: Option<&nsIURI>, aTables: &[u8], aCallback: Option<&nsIURIClassifierCallback>) -> Result<(), nsresult> {
        let aTables = nsCString::from(aTables);
        match ((*self.vtable).asyncClassifyLocalWithTables)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aTables, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString classifyLocal (in nsIURI aURI, in ACString aTables); */
    #[inline]
    pub unsafe fn classifyLocal(&self, aURI: Option<&nsIURI>, aTables: &[u8]) -> Result<nsCString, nsresult> {
        let aTables = nsCString::from(aTables);
        let mut _retval = nsCString::new();
        match ((*self.vtable).classifyLocal)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aTables, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


