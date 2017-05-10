//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDataSignatureVerifier.idl
//


pub mod nsIDataSignatureVerifier_consts {
    pub const VERIFY_OK: i64 = 0;
    pub const VERIFY_ERROR_UNKNOWN_ISSUER: i64 = 1;
    pub const VERIFY_ERROR_OTHER: i64 = 2;
}


#[repr(C)]
pub struct nsIDataSignatureVerifier {
    vtable: *const nsIDataSignatureVerifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDataSignatureVerifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x94066a00, 0x37c9, 0x11e4,
            [0x91, 0x6c, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIDataSignatureVerifier {
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
pub trait nsIDataSignatureVerifierCoerce {
    fn coerce_from(v: &nsIDataSignatureVerifier) -> &Self;
}

impl nsIDataSignatureVerifierCoerce for nsIDataSignatureVerifier {
    #[inline]
    fn coerce_from(v: &nsIDataSignatureVerifier) -> &Self {
        v
    }
}

impl nsIDataSignatureVerifier {
    #[inline]
    pub fn coerce<T: nsIDataSignatureVerifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDataSignatureVerifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDataSignatureVerifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDataSignatureVerifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDataSignatureVerifierVTable {
    pub __base: nsISupportsVTable,

    /* boolean verifyData (in ACString aData, in ACString aSignature, in ACString aPublicKey); */
    pub verifyData: unsafe extern "C" fn (this: *const nsIDataSignatureVerifier, aData: *const nsACString, aSignature: *const nsACString, aPublicKey: *const nsACString, _retval: *mut bool) -> nsresult,

    /* nsIX509Cert verifySignature (in ACString signature, in ACString plaintext, out long errorCode); */
    pub verifySignature: unsafe extern "C" fn (this: *const nsIDataSignatureVerifier, signature: *const nsACString, plaintext: *const nsACString, errorCode: *mut libc::int32_t, _retval: *mut *const nsIX509Cert) -> nsresult,

}


impl nsIDataSignatureVerifier {
    /* boolean verifyData (in ACString aData, in ACString aSignature, in ACString aPublicKey); */
    #[inline]
    pub unsafe fn verifyData(&self, aData: &[u8], aSignature: &[u8], aPublicKey: &[u8]) -> Result<bool, nsresult> {
        let aData = nsCString::from(aData);
        let aSignature = nsCString::from(aSignature);
        let aPublicKey = nsCString::from(aPublicKey);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).verifyData)(self as *const _, &*aData, &*aSignature, &*aPublicKey, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIX509Cert verifySignature (in ACString signature, in ACString plaintext, out long errorCode); */
    #[inline]
    pub unsafe fn verifySignature(&self, signature: &[u8], plaintext: &[u8]) -> Result<(libc::int32_t, Option<RefPtr<nsIX509Cert>>), nsresult> {
        let signature = nsCString::from(signature);
        let plaintext = nsCString::from(plaintext);
        let mut errorCode: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).verifySignature)(self as *const _, &*signature, &*plaintext, &mut errorCode as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((errorCode, _retval.refptr()))
    }

}


