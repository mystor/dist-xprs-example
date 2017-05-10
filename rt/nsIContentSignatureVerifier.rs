//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSignatureVerifier.idl
//


#[repr(C)]
pub struct nsIContentSignatureVerifier {
    vtable: *const nsIContentSignatureVerifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentSignatureVerifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x45a5fe2f, 0xc350, 0x4b86,
            [0x96, 0x2d, 0x02, 0xd5, 0xaa, 0xaa, 0x95, 0x5a])
    }
}

unsafe impl RefCounted for nsIContentSignatureVerifier {
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
pub trait nsIContentSignatureVerifierCoerce {
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self;
}

impl nsIContentSignatureVerifierCoerce for nsIContentSignatureVerifier {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self {
        v
    }
}

impl nsIContentSignatureVerifier {
    #[inline]
    pub fn coerce<T: nsIContentSignatureVerifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentSignatureVerifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentSignatureVerifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureVerifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentSignatureVerifierVTable {
    pub __base: nsISupportsVTable,

    /* boolean verifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
    pub verifyContentSignature: unsafe extern "C" fn (this: *const nsIContentSignatureVerifier, aData: *const nsACString, aContentSignatureHeader: *const nsACString, aCertificateChain: *const nsACString, aName: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void createContext (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
    pub createContext: unsafe extern "C" fn (this: *const nsIContentSignatureVerifier, aData: *const nsACString, aContentSignatureHeader: *const nsACString, aCertificateChain: *const nsACString, aName: *const nsACString) -> nsresult,

    /* void createContextWithoutCertChain (in nsIContentSignatureReceiverCallback aCallback, in ACString aContentSignatureHeader, in ACString aName); */
    pub createContextWithoutCertChain: unsafe extern "C" fn (this: *const nsIContentSignatureVerifier, aCallback: *const nsIContentSignatureReceiverCallback, aContentSignatureHeader: *const nsACString, aName: *const nsACString) -> nsresult,

    /* void update (in ACString aData); */
    pub update: unsafe extern "C" fn (this: *const nsIContentSignatureVerifier, aData: *const nsACString) -> nsresult,

    /* boolean end (); */
    pub end: unsafe extern "C" fn (this: *const nsIContentSignatureVerifier, _retval: *mut bool) -> nsresult,

}


impl nsIContentSignatureVerifier {
    /* boolean verifyContentSignature (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
    #[inline]
    pub unsafe fn verifyContentSignature(&self, aData: &[u8], aContentSignatureHeader: &[u8], aCertificateChain: &[u8], aName: &[u8]) -> Result<bool, nsresult> {
        let aData = nsCString::from(aData);
        let aContentSignatureHeader = nsCString::from(aContentSignatureHeader);
        let aCertificateChain = nsCString::from(aCertificateChain);
        let aName = nsCString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).verifyContentSignature)(self as *const _, &*aData, &*aContentSignatureHeader, &*aCertificateChain, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void createContext (in ACString aData, in ACString aContentSignatureHeader, in ACString aCertificateChain, in ACString aName); */
    #[inline]
    pub unsafe fn createContext(&self, aData: &[u8], aContentSignatureHeader: &[u8], aCertificateChain: &[u8], aName: &[u8]) -> Result<(), nsresult> {
        let aData = nsCString::from(aData);
        let aContentSignatureHeader = nsCString::from(aContentSignatureHeader);
        let aCertificateChain = nsCString::from(aCertificateChain);
        let aName = nsCString::from(aName);
        match ((*self.vtable).createContext)(self as *const _, &*aData, &*aContentSignatureHeader, &*aCertificateChain, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createContextWithoutCertChain (in nsIContentSignatureReceiverCallback aCallback, in ACString aContentSignatureHeader, in ACString aName); */
    #[inline]
    pub unsafe fn createContextWithoutCertChain(&self, aCallback: Option<&nsIContentSignatureReceiverCallback>, aContentSignatureHeader: &[u8], aName: &[u8]) -> Result<(), nsresult> {
        let aContentSignatureHeader = nsCString::from(aContentSignatureHeader);
        let aName = nsCString::from(aName);
        match ((*self.vtable).createContextWithoutCertChain)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), &*aContentSignatureHeader, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void update (in ACString aData); */
    #[inline]
    pub unsafe fn update(&self, aData: &[u8]) -> Result<(), nsresult> {
        let aData = nsCString::from(aData);
        match ((*self.vtable).update)(self as *const _, &*aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean end (); */
    #[inline]
    pub unsafe fn end(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).end)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIContentSignatureReceiverCallback {
    vtable: *const nsIContentSignatureReceiverCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentSignatureReceiverCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1eb90707, 0xdf59, 0x48b7,
            [0x9d, 0x42, 0xd8, 0xbf, 0x63, 0x0a, 0xe7, 0x44])
    }
}

unsafe impl RefCounted for nsIContentSignatureReceiverCallback {
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
pub trait nsIContentSignatureReceiverCallbackCoerce {
    fn coerce_from(v: &nsIContentSignatureReceiverCallback) -> &Self;
}

impl nsIContentSignatureReceiverCallbackCoerce for nsIContentSignatureReceiverCallback {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureReceiverCallback) -> &Self {
        v
    }
}

impl nsIContentSignatureReceiverCallback {
    #[inline]
    pub fn coerce<T: nsIContentSignatureReceiverCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentSignatureReceiverCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentSignatureReceiverCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSignatureReceiverCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentSignatureReceiverCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void contextCreated (in boolean successful); */
    pub contextCreated: unsafe extern "C" fn (this: *const nsIContentSignatureReceiverCallback, successful: bool) -> nsresult,

}


impl nsIContentSignatureReceiverCallback {
    /* void contextCreated (in boolean successful); */
    #[inline]
    pub unsafe fn contextCreated(&self, successful: bool) -> Result<(), nsresult> {

        match ((*self.vtable).contextCreated)(self as *const _, successful) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


