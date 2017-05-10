//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIX509CertDB.idl
//


pub type AppTrustedRoot = uint32_t;


#[repr(C)]
pub struct nsIOpenSignedAppFileCallback {
    vtable: *const nsIOpenSignedAppFileCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOpenSignedAppFileCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfc2b60e5, 0x9a07, 0x47c2,
            [0xa2, 0xcd, 0xb8, 0x3b, 0x68, 0xa6, 0x60, 0xac])
    }
}

unsafe impl RefCounted for nsIOpenSignedAppFileCallback {
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
pub trait nsIOpenSignedAppFileCallbackCoerce {
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self;
}

impl nsIOpenSignedAppFileCallbackCoerce for nsIOpenSignedAppFileCallback {
    #[inline]
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self {
        v
    }
}

impl nsIOpenSignedAppFileCallback {
    #[inline]
    pub fn coerce<T: nsIOpenSignedAppFileCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOpenSignedAppFileCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOpenSignedAppFileCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOpenSignedAppFileCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOpenSignedAppFileCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
    pub openSignedAppFileFinished: unsafe extern "C" fn (this: *const nsIOpenSignedAppFileCallback, rv: nsresult, aZipReader: *const nsIZipReader, aSignerCert: *const nsIX509Cert) -> nsresult,

}


impl nsIOpenSignedAppFileCallback {
    /* void openSignedAppFileFinished (in nsresult rv, in nsIZipReader aZipReader, in nsIX509Cert aSignerCert); */
    #[inline]
    pub unsafe fn openSignedAppFileFinished(&self, rv: nsresult, aZipReader: Option<&nsIZipReader>, aSignerCert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).openSignedAppFileFinished)(self as *const _, rv, aZipReader.map_or(::std::ptr::null(), |x| x as *const _), aSignerCert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIVerifySignedDirectoryCallback {
    vtable: *const nsIVerifySignedDirectoryCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIVerifySignedDirectoryCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5f97827, 0x622a, 0x488f,
            [0xbe, 0x08, 0xd8, 0x50, 0x43, 0x2a, 0xc8, 0xec])
    }
}

unsafe impl RefCounted for nsIVerifySignedDirectoryCallback {
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
pub trait nsIVerifySignedDirectoryCallbackCoerce {
    fn coerce_from(v: &nsIVerifySignedDirectoryCallback) -> &Self;
}

impl nsIVerifySignedDirectoryCallbackCoerce for nsIVerifySignedDirectoryCallback {
    #[inline]
    fn coerce_from(v: &nsIVerifySignedDirectoryCallback) -> &Self {
        v
    }
}

impl nsIVerifySignedDirectoryCallback {
    #[inline]
    pub fn coerce<T: nsIVerifySignedDirectoryCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIVerifySignedDirectoryCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIVerifySignedDirectoryCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIVerifySignedDirectoryCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIVerifySignedDirectoryCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void verifySignedDirectoryFinished (in nsresult rv, in nsIX509Cert aSignerCert); */
    pub verifySignedDirectoryFinished: unsafe extern "C" fn (this: *const nsIVerifySignedDirectoryCallback, rv: nsresult, aSignerCert: *const nsIX509Cert) -> nsresult,

}


impl nsIVerifySignedDirectoryCallback {
    /* void verifySignedDirectoryFinished (in nsresult rv, in nsIX509Cert aSignerCert); */
    #[inline]
    pub unsafe fn verifySignedDirectoryFinished(&self, rv: nsresult, aSignerCert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).verifySignedDirectoryFinished)(self as *const _, rv, aSignerCert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsICertVerificationCallback {
    vtable: *const nsICertVerificationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertVerificationCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x49e16fc8, 0xefac, 0x4f57,
            [0x83, 0x61, 0x95, 0x6e, 0xf6, 0xb9, 0x60, 0xa4])
    }
}

unsafe impl RefCounted for nsICertVerificationCallback {
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
pub trait nsICertVerificationCallbackCoerce {
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self;
}

impl nsICertVerificationCallbackCoerce for nsICertVerificationCallback {
    #[inline]
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self {
        v
    }
}

impl nsICertVerificationCallback {
    #[inline]
    pub fn coerce<T: nsICertVerificationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertVerificationCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICertVerificationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertVerificationCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertVerificationCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void verifyCertFinished (in int32_t aPRErrorCode, in nsIX509CertList aVerifiedChain, in bool aHasEVPolicy); */
    pub verifyCertFinished: unsafe extern "C" fn (this: *const nsICertVerificationCallback, aPRErrorCode: int32_t, aVerifiedChain: *const nsIX509CertList, aHasEVPolicy: bool) -> nsresult,

}


impl nsICertVerificationCallback {
    /* void verifyCertFinished (in int32_t aPRErrorCode, in nsIX509CertList aVerifiedChain, in bool aHasEVPolicy); */
    #[inline]
    pub unsafe fn verifyCertFinished(&self, aPRErrorCode: int32_t, aVerifiedChain: Option<&nsIX509CertList>, aHasEVPolicy: bool) -> Result<(), nsresult> {

        match ((*self.vtable).verifyCertFinished)(self as *const _, aPRErrorCode, aVerifiedChain.map_or(::std::ptr::null(), |x| x as *const _), aHasEVPolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIX509CertDB_consts {
    pub const UNTRUSTED: i64 = 0;
    pub const TRUSTED_SSL: i64 = 1;
    pub const TRUSTED_EMAIL: i64 = 2;
    pub const TRUSTED_OBJSIGN: i64 = 4;
    pub const AppXPCShellRoot: i64 = 6;
    pub const AddonsPublicRoot: i64 = 7;
    pub const AddonsStageRoot: i64 = 8;
    pub const PrivilegedPackageRoot: i64 = 9;
    pub const DeveloperImportedRoot: i64 = 10;
    pub const FLAG_LOCAL_ONLY: i64 = 1;
    pub const FLAG_MUST_BE_EV: i64 = 2;
}


#[repr(C)]
pub struct nsIX509CertDB {
    vtable: *const nsIX509CertDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIX509CertDB {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5c16cd9b, 0x5a73, 0x47f1,
            [0xab, 0x0f, 0x11, 0xed, 0xe7, 0x49, 0x5c, 0xce])
    }
}

unsafe impl RefCounted for nsIX509CertDB {
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
pub trait nsIX509CertDBCoerce {
    fn coerce_from(v: &nsIX509CertDB) -> &Self;
}

impl nsIX509CertDBCoerce for nsIX509CertDB {
    #[inline]
    fn coerce_from(v: &nsIX509CertDB) -> &Self {
        v
    }
}

impl nsIX509CertDB {
    #[inline]
    pub fn coerce<T: nsIX509CertDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIX509CertDB {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIX509CertDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIX509CertDB) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIX509CertDBVTable {
    pub __base: nsISupportsVTable,

    /* nsIX509Cert findCertByDBKey (in ACString aDBkey); */
    pub findCertByDBKey: unsafe extern "C" fn (this: *const nsIX509CertDB, aDBkey: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* nsIX509Cert findCertByEmailAddress (in ACString aEmailAddress); */
    pub findCertByEmailAddress: unsafe extern "C" fn (this: *const nsIX509CertDB, aEmailAddress: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx); */
    /// Unable to call function as its signature contains a non-rust type
    pub importCertificates: *const ::libc::c_void,

    /* void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
    /// Unable to call function as its signature contains a non-rust type
    pub importEmailCertificate: *const ::libc::c_void,

    /* void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */
    /// Unable to call function as its signature contains a non-rust type
    pub importUserCertificate: *const ::libc::c_void,

    /* void deleteCertificate (in nsIX509Cert aCert); */
    pub deleteCertificate: unsafe extern "C" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert) -> nsresult,

    /* void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust); */
    pub setCertTrust: unsafe extern "C" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, type_: libc::uint32_t, trust: libc::uint32_t) -> nsresult,

    /* void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString); */
    pub setCertTrustFromString: unsafe extern "C" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, trustString: *const nsACString) -> nsresult,

    /* boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType); */
    pub isCertTrusted: unsafe extern "C" fn (this: *const nsIX509CertDB, cert: *const nsIX509Cert, certType: libc::uint32_t, trustType: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void importCertsFromFile (in nsIFile aFile, in unsigned long aType); */
    pub importCertsFromFile: unsafe extern "C" fn (this: *const nsIX509CertDB, aFile: *const nsIFile, aType: libc::uint32_t) -> nsresult,

    /* void importPKCS12File (in nsIFile aFile); */
    pub importPKCS12File: unsafe extern "C" fn (this: *const nsIX509CertDB, aFile: *const nsIFile) -> nsresult,

    /* void exportPKCS12File (in nsIFile aFile, in unsigned long count, [array, size_is (count)] in nsIX509Cert aCerts); */
    /// Unable to call function as its signature contains a non-rust type
    pub exportPKCS12File: *const ::libc::c_void,

    /* nsIX509Cert constructX509FromBase64 (in ACString base64); */
    pub constructX509FromBase64: unsafe extern "C" fn (this: *const nsIX509CertDB, base64: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* nsIX509Cert constructX509 (in ACString certDER); */
    pub constructX509: unsafe extern "C" fn (this: *const nsIX509CertDB, certDER: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback); */
    pub openSignedAppFileAsync: unsafe extern "C" fn (this: *const nsIX509CertDB, trustedRoot: AppTrustedRoot, aJarFile: *const nsIFile, callback: *const nsIOpenSignedAppFileCallback) -> nsresult,

    /* void verifySignedDirectoryAsync (in AppTrustedRoot trustedRoot, in nsIFile aUnpackedDir, in nsIVerifySignedDirectoryCallback callback); */
    pub verifySignedDirectoryAsync: unsafe extern "C" fn (this: *const nsIX509CertDB, trustedRoot: AppTrustedRoot, aUnpackedDir: *const nsIFile, callback: *const nsIVerifySignedDirectoryCallback) -> nsresult,

    /* nsIX509Cert addCert (in ACString certDER, in ACString trust); */
    pub addCert: unsafe extern "C" fn (this: *const nsIX509CertDB, certDER: *const nsACString, trust: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* int32_t verifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, out nsIX509CertList aVerifiedChain, out bool aHasEVPolicy); */
    pub verifyCertAtTime: unsafe extern "C" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert, aUsage: int64_t, aFlags: uint32_t, aHostname: *const nsACString, aTime: uint64_t, aVerifiedChain: *mut *const nsIX509CertList, aHasEVPolicy: *mut bool, _retval: *mut int32_t) -> nsresult,

    /* int32_t verifyCertNow (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, out nsIX509CertList aVerifiedChain, out bool aHasEVPolicy); */
    pub verifyCertNow: unsafe extern "C" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert, aUsage: int64_t, aFlags: uint32_t, aHostname: *const nsACString, aVerifiedChain: *mut *const nsIX509CertList, aHasEVPolicy: *mut bool, _retval: *mut int32_t) -> nsresult,

    /* void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback); */
    pub asyncVerifyCertAtTime: unsafe extern "C" fn (this: *const nsIX509CertDB, aCert: *const nsIX509Cert, aUsage: int64_t, aFlags: uint32_t, aHostname: *const nsACString, aTime: uint64_t, aCallback: *const nsICertVerificationCallback) -> nsresult,

    /* void clearOCSPCache (); */
    pub clearOCSPCache: unsafe extern "C" fn (this: *const nsIX509CertDB) -> nsresult,

    /* nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust); */
    pub addCertFromBase64: unsafe extern "C" fn (this: *const nsIX509CertDB, base64: *const nsACString, trust: *const nsACString, _retval: *mut *const nsIX509Cert) -> nsresult,

    /* nsIX509CertList getCerts (); */
    pub getCerts: unsafe extern "C" fn (this: *const nsIX509CertDB, _retval: *mut *const nsIX509CertList) -> nsresult,

    /* nsIX509CertList getEnterpriseRoots (); */
    pub getEnterpriseRoots: unsafe extern "C" fn (this: *const nsIX509CertDB, _retval: *mut *const nsIX509CertList) -> nsresult,

}


impl nsIX509CertDB {
    /* nsIX509Cert findCertByDBKey (in ACString aDBkey); */
    #[inline]
    pub unsafe fn findCertByDBKey(&self, aDBkey: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let aDBkey = nsCString::from(aDBkey);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findCertByDBKey)(self as *const _, &*aDBkey, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIX509Cert findCertByEmailAddress (in ACString aEmailAddress); */
    #[inline]
    pub unsafe fn findCertByEmailAddress(&self, aEmailAddress: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let aEmailAddress = nsCString::from(aEmailAddress);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findCertByEmailAddress)(self as *const _, &*aEmailAddress, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void importCertificates ([array, size_is (length)] in octet data, in unsigned long length, in unsigned long type, in nsIInterfaceRequestor ctx); */


    /* void importEmailCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */


    /* void importUserCertificate ([array, size_is (length)] in octet data, in unsigned long length, in nsIInterfaceRequestor ctx); */


    /* void deleteCertificate (in nsIX509Cert aCert); */
    #[inline]
    pub unsafe fn deleteCertificate(&self, aCert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).deleteCertificate)(self as *const _, aCert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCertTrust (in nsIX509Cert cert, in unsigned long type, in unsigned long trust); */
    #[inline]
    pub unsafe fn setCertTrust(&self, cert: Option<&nsIX509Cert>, type_: libc::uint32_t, trust: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setCertTrust)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _), type_, trust) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCertTrustFromString (in nsIX509Cert cert, in ACString trustString); */
    #[inline]
    pub unsafe fn setCertTrustFromString(&self, cert: Option<&nsIX509Cert>, trustString: &[u8]) -> Result<(), nsresult> {
        let trustString = nsCString::from(trustString);
        match ((*self.vtable).setCertTrustFromString)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _), &*trustString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isCertTrusted (in nsIX509Cert cert, in unsigned long certType, in unsigned long trustType); */
    #[inline]
    pub unsafe fn isCertTrusted(&self, cert: Option<&nsIX509Cert>, certType: libc::uint32_t, trustType: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCertTrusted)(self as *const _, cert.map_or(::std::ptr::null(), |x| x as *const _), certType, trustType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void importCertsFromFile (in nsIFile aFile, in unsigned long aType); */
    #[inline]
    pub unsafe fn importCertsFromFile(&self, aFile: Option<&nsIFile>, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).importCertsFromFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _), aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void importPKCS12File (in nsIFile aFile); */
    #[inline]
    pub unsafe fn importPKCS12File(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).importPKCS12File)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void exportPKCS12File (in nsIFile aFile, in unsigned long count, [array, size_is (count)] in nsIX509Cert aCerts); */


    /* nsIX509Cert constructX509FromBase64 (in ACString base64); */
    #[inline]
    pub unsafe fn constructX509FromBase64(&self, base64: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let base64 = nsCString::from(base64);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).constructX509FromBase64)(self as *const _, &*base64, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIX509Cert constructX509 (in ACString certDER); */
    #[inline]
    pub unsafe fn constructX509(&self, certDER: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let certDER = nsCString::from(certDER);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).constructX509)(self as *const _, &*certDER, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void openSignedAppFileAsync (in AppTrustedRoot trustedRoot, in nsIFile aJarFile, in nsIOpenSignedAppFileCallback callback); */
    #[inline]
    pub unsafe fn openSignedAppFileAsync(&self, trustedRoot: AppTrustedRoot, aJarFile: Option<&nsIFile>, callback: Option<&nsIOpenSignedAppFileCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).openSignedAppFileAsync)(self as *const _, trustedRoot, aJarFile.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void verifySignedDirectoryAsync (in AppTrustedRoot trustedRoot, in nsIFile aUnpackedDir, in nsIVerifySignedDirectoryCallback callback); */
    #[inline]
    pub unsafe fn verifySignedDirectoryAsync(&self, trustedRoot: AppTrustedRoot, aUnpackedDir: Option<&nsIFile>, callback: Option<&nsIVerifySignedDirectoryCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).verifySignedDirectoryAsync)(self as *const _, trustedRoot, aUnpackedDir.map_or(::std::ptr::null(), |x| x as *const _), callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIX509Cert addCert (in ACString certDER, in ACString trust); */
    #[inline]
    pub unsafe fn addCert(&self, certDER: &[u8], trust: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let certDER = nsCString::from(certDER);
        let trust = nsCString::from(trust);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).addCert)(self as *const _, &*certDER, &*trust, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* int32_t verifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, out nsIX509CertList aVerifiedChain, out bool aHasEVPolicy); */
    #[inline]
    pub unsafe fn verifyCertAtTime(&self, aCert: Option<&nsIX509Cert>, aUsage: int64_t, aFlags: uint32_t, aHostname: &[u8], aTime: uint64_t) -> Result<(Option<RefPtr<nsIX509CertList>>, bool, int32_t), nsresult> {
        let aHostname = nsCString::from(aHostname);
        let mut aVerifiedChain = GetterAddrefs::new();
        let mut aHasEVPolicy: bool = ::std::mem::zeroed();
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).verifyCertAtTime)(self as *const _, aCert.map_or(::std::ptr::null(), |x| x as *const _), aUsage, aFlags, &*aHostname, aTime, aVerifiedChain.ptr(), &mut aHasEVPolicy as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aVerifiedChain.refptr(), aHasEVPolicy, _retval))
    }

    /* int32_t verifyCertNow (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, out nsIX509CertList aVerifiedChain, out bool aHasEVPolicy); */
    #[inline]
    pub unsafe fn verifyCertNow(&self, aCert: Option<&nsIX509Cert>, aUsage: int64_t, aFlags: uint32_t, aHostname: &[u8]) -> Result<(Option<RefPtr<nsIX509CertList>>, bool, int32_t), nsresult> {
        let aHostname = nsCString::from(aHostname);
        let mut aVerifiedChain = GetterAddrefs::new();
        let mut aHasEVPolicy: bool = ::std::mem::zeroed();
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).verifyCertNow)(self as *const _, aCert.map_or(::std::ptr::null(), |x| x as *const _), aUsage, aFlags, &*aHostname, aVerifiedChain.ptr(), &mut aHasEVPolicy as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aVerifiedChain.refptr(), aHasEVPolicy, _retval))
    }

    /* void asyncVerifyCertAtTime (in nsIX509Cert aCert, in int64_t aUsage, in uint32_t aFlags, in ACString aHostname, in uint64_t aTime, in nsICertVerificationCallback aCallback); */
    #[inline]
    pub unsafe fn asyncVerifyCertAtTime(&self, aCert: Option<&nsIX509Cert>, aUsage: int64_t, aFlags: uint32_t, aHostname: &[u8], aTime: uint64_t, aCallback: Option<&nsICertVerificationCallback>) -> Result<(), nsresult> {
        let aHostname = nsCString::from(aHostname);
        match ((*self.vtable).asyncVerifyCertAtTime)(self as *const _, aCert.map_or(::std::ptr::null(), |x| x as *const _), aUsage, aFlags, &*aHostname, aTime, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearOCSPCache (); */
    #[inline]
    pub unsafe fn clearOCSPCache(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearOCSPCache)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIX509Cert addCertFromBase64 (in ACString base64, in ACString trust); */
    #[inline]
    pub unsafe fn addCertFromBase64(&self, base64: &[u8], trust: &[u8]) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let base64 = nsCString::from(base64);
        let trust = nsCString::from(trust);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).addCertFromBase64)(self as *const _, &*base64, &*trust, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIX509CertList getCerts (); */
    #[inline]
    pub unsafe fn getCerts(&self, ) -> Result<Option<RefPtr<nsIX509CertList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCerts)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIX509CertList getEnterpriseRoots (); */
    #[inline]
    pub unsafe fn getEnterpriseRoots(&self, ) -> Result<Option<RefPtr<nsIX509CertList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEnterpriseRoots)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


