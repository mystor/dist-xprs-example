//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509Cert.idl
//


pub mod nsIX509Cert_consts {
    pub const UNKNOWN_CERT: i64 = 0;
    pub const CA_CERT: i64 = 1;
    pub const USER_CERT: i64 = 2;
    pub const EMAIL_CERT: i64 = 4;
    pub const SERVER_CERT: i64 = 8;
    pub const ANY_CERT: i64 = 65535;
    pub const CMS_CHAIN_MODE_CertOnly: i64 = 1;
    pub const CMS_CHAIN_MODE_CertChain: i64 = 2;
    pub const CMS_CHAIN_MODE_CertChainWithRoot: i64 = 3;
}


#[repr(C)]
pub struct nsIX509Cert {
    vtable: *const nsIX509CertVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIX509Cert {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbdc3979a, 0x5422, 0x4cd5,
            [0x85, 0x89, 0x69, 0x6b, 0x6e, 0x96, 0xea, 0x83])
    }
}

unsafe impl RefCounted for nsIX509Cert {
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
pub trait nsIX509CertCoerce {
    fn coerce_from(v: &nsIX509Cert) -> &Self;
}

impl nsIX509CertCoerce for nsIX509Cert {
    #[inline]
    fn coerce_from(v: &nsIX509Cert) -> &Self {
        v
    }
}

impl nsIX509Cert {
    #[inline]
    pub fn coerce<T: nsIX509CertCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIX509Cert {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIX509CertCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509Cert) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIX509CertVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString emailAddress; */
    pub get_emailAddress: unsafe extern "C" fn (this: *const nsIX509Cert, aEmailAddress: *mut nsAString) -> nsresult,

    /* readonly attribute bool isBuiltInRoot; */
    pub get_isBuiltInRoot: unsafe extern "C" fn (this: *const nsIX509Cert, aIsBuiltInRoot: *mut bool) -> nsresult,

    /* void getEmailAddresses (out unsigned long length, [array, size_is (length), retval] out wstring addresses); */
    /// Unable to call function as its signature contains a non-rust type
    pub getEmailAddresses: *const ::libc::c_void,

    /* boolean containsEmailAddress (in AString aEmailAddress); */
    pub containsEmailAddress: unsafe extern "C" fn (this: *const nsIX509Cert, aEmailAddress: *const nsAString, _retval: *mut bool) -> nsresult,

    /* readonly attribute AString subjectName; */
    pub get_subjectName: unsafe extern "C" fn (this: *const nsIX509Cert, aSubjectName: *mut nsAString) -> nsresult,

    /* readonly attribute AString commonName; */
    pub get_commonName: unsafe extern "C" fn (this: *const nsIX509Cert, aCommonName: *mut nsAString) -> nsresult,

    /* readonly attribute AString organization; */
    pub get_organization: unsafe extern "C" fn (this: *const nsIX509Cert, aOrganization: *mut nsAString) -> nsresult,

    /* readonly attribute AString organizationalUnit; */
    pub get_organizationalUnit: unsafe extern "C" fn (this: *const nsIX509Cert, aOrganizationalUnit: *mut nsAString) -> nsresult,

    /* readonly attribute AString sha256Fingerprint; */
    pub get_sha256Fingerprint: unsafe extern "C" fn (this: *const nsIX509Cert, aSha256Fingerprint: *mut nsAString) -> nsresult,

    /* readonly attribute AString sha1Fingerprint; */
    pub get_sha1Fingerprint: unsafe extern "C" fn (this: *const nsIX509Cert, aSha1Fingerprint: *mut nsAString) -> nsresult,

    /* readonly attribute AString tokenName; */
    pub get_tokenName: unsafe extern "C" fn (this: *const nsIX509Cert, aTokenName: *mut nsAString) -> nsresult,

    /* readonly attribute AString issuerName; */
    pub get_issuerName: unsafe extern "C" fn (this: *const nsIX509Cert, aIssuerName: *mut nsAString) -> nsresult,

    /* readonly attribute AString serialNumber; */
    pub get_serialNumber: unsafe extern "C" fn (this: *const nsIX509Cert, aSerialNumber: *mut nsAString) -> nsresult,

    /* readonly attribute AString issuerCommonName; */
    pub get_issuerCommonName: unsafe extern "C" fn (this: *const nsIX509Cert, aIssuerCommonName: *mut nsAString) -> nsresult,

    /* readonly attribute AString issuerOrganization; */
    pub get_issuerOrganization: unsafe extern "C" fn (this: *const nsIX509Cert, aIssuerOrganization: *mut nsAString) -> nsresult,

    /* readonly attribute AString issuerOrganizationUnit; */
    pub get_issuerOrganizationUnit: unsafe extern "C" fn (this: *const nsIX509Cert, aIssuerOrganizationUnit: *mut nsAString) -> nsresult,

    /* readonly attribute nsIX509Cert issuer; */
    pub get_issuer: unsafe extern "C" fn (this: *const nsIX509Cert, aIssuer: *mut *const nsIX509Cert) -> nsresult,

    /* readonly attribute nsIX509CertValidity validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIX509Cert, aValidity: *mut *const nsIX509CertValidity) -> nsresult,

    /* readonly attribute ACString dbKey; */
    pub get_dbKey: unsafe extern "C" fn (this: *const nsIX509Cert, aDbKey: *mut nsACString) -> nsresult,

    /* readonly attribute AString displayName; */
    pub get_displayName: unsafe extern "C" fn (this: *const nsIX509Cert, aDisplayName: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long certType; */
    pub get_certType: unsafe extern "C" fn (this: *const nsIX509Cert, aCertType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isSelfSigned; */
    pub get_isSelfSigned: unsafe extern "C" fn (this: *const nsIX509Cert, aIsSelfSigned: *mut bool) -> nsresult,

    /* nsIArray getChain (); */
    pub getChain: unsafe extern "C" fn (this: *const nsIX509Cert, _retval: *mut *const nsIArray) -> nsresult,

    /* readonly attribute AString keyUsages; */
    pub get_keyUsages: unsafe extern "C" fn (this: *const nsIX509Cert, aKeyUsages: *mut nsAString) -> nsresult,

    /* readonly attribute nsIASN1Object ASN1Structure; */
    pub get_ASN1Structure: unsafe extern "C" fn (this: *const nsIX509Cert, aASN1Structure: *mut *const nsIASN1Object) -> nsresult,

    /* void getRawDER (out unsigned long length, [array, size_is (length), retval] out octet data); */
    /// Unable to call function as its signature contains a non-rust type
    pub getRawDER: *const ::libc::c_void,

    /* boolean equals (in nsIX509Cert other); */
    pub equals: unsafe extern "C" fn (this: *const nsIX509Cert, other: *const nsIX509Cert, _retval: *mut bool) -> nsresult,

    /* readonly attribute ACString sha256SubjectPublicKeyInfoDigest; */
    pub get_sha256SubjectPublicKeyInfoDigest: unsafe extern "C" fn (this: *const nsIX509Cert, aSha256SubjectPublicKeyInfoDigest: *mut nsACString) -> nsresult,

    /* void exportAsCMS (in unsigned long chainMode, out unsigned long length, [array, size_is (length), retval] out octet data); */
    /// Unable to call function as its signature contains a non-rust type
    pub exportAsCMS: *const ::libc::c_void,

    /* [noscript,notxpcom] CERTCertificatePtr getCert (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCert: *const ::libc::c_void,

    /* void markForPermDeletion (); */
    pub markForPermDeletion: unsafe extern "C" fn (this: *const nsIX509Cert) -> nsresult,

}


impl nsIX509Cert {
    /* readonly attribute AString emailAddress; */
    #[inline]
    pub unsafe fn get_emailAddress(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_emailAddress)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool isBuiltInRoot; */
    #[inline]
    pub unsafe fn get_isBuiltInRoot(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isBuiltInRoot)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getEmailAddresses (out unsigned long length, [array, size_is (length), retval] out wstring addresses); */


    /* boolean containsEmailAddress (in AString aEmailAddress); */
    #[inline]
    pub unsafe fn containsEmailAddress(&self, aEmailAddress: &[u16]) -> Result<bool, nsresult> {
        let aEmailAddress = nsString::from(aEmailAddress);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).containsEmailAddress)(self as *const _, &*aEmailAddress, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString subjectName; */
    #[inline]
    pub unsafe fn get_subjectName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_subjectName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString commonName; */
    #[inline]
    pub unsafe fn get_commonName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_commonName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString organization; */
    #[inline]
    pub unsafe fn get_organization(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_organization)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString organizationalUnit; */
    #[inline]
    pub unsafe fn get_organizationalUnit(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_organizationalUnit)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString sha256Fingerprint; */
    #[inline]
    pub unsafe fn get_sha256Fingerprint(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sha256Fingerprint)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString sha1Fingerprint; */
    #[inline]
    pub unsafe fn get_sha1Fingerprint(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sha1Fingerprint)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString tokenName; */
    #[inline]
    pub unsafe fn get_tokenName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_tokenName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString issuerName; */
    #[inline]
    pub unsafe fn get_issuerName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_issuerName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString serialNumber; */
    #[inline]
    pub unsafe fn get_serialNumber(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_serialNumber)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString issuerCommonName; */
    #[inline]
    pub unsafe fn get_issuerCommonName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_issuerCommonName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString issuerOrganization; */
    #[inline]
    pub unsafe fn get_issuerOrganization(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_issuerOrganization)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString issuerOrganizationUnit; */
    #[inline]
    pub unsafe fn get_issuerOrganizationUnit(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_issuerOrganizationUnit)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIX509Cert issuer; */
    #[inline]
    pub unsafe fn get_issuer(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_issuer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIX509CertValidity validity; */
    #[inline]
    pub unsafe fn get_validity(&self, ) -> Result<Option<RefPtr<nsIX509CertValidity>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_validity)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString dbKey; */
    #[inline]
    pub unsafe fn get_dbKey(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_dbKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString displayName; */
    #[inline]
    pub unsafe fn get_displayName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displayName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long certType; */
    #[inline]
    pub unsafe fn get_certType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_certType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSelfSigned; */
    #[inline]
    pub unsafe fn get_isSelfSigned(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSelfSigned)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIArray getChain (); */
    #[inline]
    pub unsafe fn getChain(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChain)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString keyUsages; */
    #[inline]
    pub unsafe fn get_keyUsages(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_keyUsages)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIASN1Object ASN1Structure; */
    #[inline]
    pub unsafe fn get_ASN1Structure(&self, ) -> Result<Option<RefPtr<nsIASN1Object>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ASN1Structure)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getRawDER (out unsigned long length, [array, size_is (length), retval] out octet data); */


    /* boolean equals (in nsIX509Cert other); */
    #[inline]
    pub unsafe fn equals(&self, other: Option<&nsIX509Cert>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString sha256SubjectPublicKeyInfoDigest; */
    #[inline]
    pub unsafe fn get_sha256SubjectPublicKeyInfoDigest(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sha256SubjectPublicKeyInfoDigest)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void exportAsCMS (in unsigned long chainMode, out unsigned long length, [array, size_is (length), retval] out octet data); */


    /* [noscript,notxpcom] CERTCertificatePtr getCert (); */


    /* void markForPermDeletion (); */
    #[inline]
    pub unsafe fn markForPermDeletion(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).markForPermDeletion)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


