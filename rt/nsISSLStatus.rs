//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISSLStatus.idl
//


pub mod nsISSLStatus_consts {
    pub const SSL_VERSION_3: i64 = 0;
    pub const TLS_VERSION_1: i64 = 1;
    pub const TLS_VERSION_1_1: i64 = 2;
    pub const TLS_VERSION_1_2: i64 = 3;
    pub const TLS_VERSION_1_3: i64 = 4;
    pub const CERTIFICATE_TRANSPARENCY_NOT_APPLICABLE: i64 = 0;
    pub const CERTIFICATE_TRANSPARENCY_POLICY_COMPLIANT: i64 = 5;
    pub const CERTIFICATE_TRANSPARENCY_POLICY_NOT_ENOUGH_SCTS: i64 = 6;
    pub const CERTIFICATE_TRANSPARENCY_POLICY_NOT_DIVERSE_SCTS: i64 = 7;
}


#[repr(C)]
pub struct nsISSLStatus {
    vtable: *const nsISSLStatusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISSLStatus {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfa9ba95b, 0xca3b, 0x498a,
            [0xb8, 0x89, 0x7c, 0x79, 0xcf, 0x28, 0xfe, 0xe8])
    }
}

unsafe impl RefCounted for nsISSLStatus {
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
pub trait nsISSLStatusCoerce {
    fn coerce_from(v: &nsISSLStatus) -> &Self;
}

impl nsISSLStatusCoerce for nsISSLStatus {
    #[inline]
    fn coerce_from(v: &nsISSLStatus) -> &Self {
        v
    }
}

impl nsISSLStatus {
    #[inline]
    pub fn coerce<T: nsISSLStatusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISSLStatus {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISSLStatusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISSLStatus) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISSLStatusVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIX509Cert serverCert; */
    pub get_serverCert: unsafe extern "C" fn (this: *const nsISSLStatus, aServerCert: *mut *const nsIX509Cert) -> nsresult,

    /* readonly attribute ACString cipherName; */
    pub get_cipherName: unsafe extern "C" fn (this: *const nsISSLStatus, aCipherName: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long keyLength; */
    pub get_keyLength: unsafe extern "C" fn (this: *const nsISSLStatus, aKeyLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long secretKeyLength; */
    pub get_secretKeyLength: unsafe extern "C" fn (this: *const nsISSLStatus, aSecretKeyLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned short protocolVersion; */
    pub get_protocolVersion: unsafe extern "C" fn (this: *const nsISSLStatus, aProtocolVersion: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute unsigned short certificateTransparencyStatus; */
    pub get_certificateTransparencyStatus: unsafe extern "C" fn (this: *const nsISSLStatus, aCertificateTransparencyStatus: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean isDomainMismatch; */
    pub get_isDomainMismatch: unsafe extern "C" fn (this: *const nsISSLStatus, aIsDomainMismatch: *mut bool) -> nsresult,

    /* readonly attribute boolean isNotValidAtThisTime; */
    pub get_isNotValidAtThisTime: unsafe extern "C" fn (this: *const nsISSLStatus, aIsNotValidAtThisTime: *mut bool) -> nsresult,

    /* readonly attribute boolean isUntrusted; */
    pub get_isUntrusted: unsafe extern "C" fn (this: *const nsISSLStatus, aIsUntrusted: *mut bool) -> nsresult,

    /* readonly attribute boolean isExtendedValidation; */
    pub get_isExtendedValidation: unsafe extern "C" fn (this: *const nsISSLStatus, aIsExtendedValidation: *mut bool) -> nsresult,

}


impl nsISSLStatus {
    /* readonly attribute nsIX509Cert serverCert; */
    #[inline]
    pub unsafe fn get_serverCert(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_serverCert)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString cipherName; */
    #[inline]
    pub unsafe fn get_cipherName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_cipherName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long keyLength; */
    #[inline]
    pub unsafe fn get_keyLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keyLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long secretKeyLength; */
    #[inline]
    pub unsafe fn get_secretKeyLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_secretKeyLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short protocolVersion; */
    #[inline]
    pub unsafe fn get_protocolVersion(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_protocolVersion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short certificateTransparencyStatus; */
    #[inline]
    pub unsafe fn get_certificateTransparencyStatus(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_certificateTransparencyStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isDomainMismatch; */
    #[inline]
    pub unsafe fn get_isDomainMismatch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDomainMismatch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isNotValidAtThisTime; */
    #[inline]
    pub unsafe fn get_isNotValidAtThisTime(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isNotValidAtThisTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isUntrusted; */
    #[inline]
    pub unsafe fn get_isUntrusted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isUntrusted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isExtendedValidation; */
    #[inline]
    pub unsafe fn get_isExtendedValidation(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isExtendedValidation)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


