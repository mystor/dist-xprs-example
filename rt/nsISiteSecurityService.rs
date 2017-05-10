//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISiteSecurityService.idl
//


pub mod nsISiteSecurityState_consts {
    pub const SECURITY_PROPERTY_UNSET: i64 = 0;
    pub const SECURITY_PROPERTY_SET: i64 = 1;
    pub const SECURITY_PROPERTY_KNOCKOUT: i64 = 2;
    pub const SECURITY_PROPERTY_NEGATIVE: i64 = 3;
}


#[repr(C)]
pub struct nsISiteSecurityState {
    vtable: *const nsISiteSecurityStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISiteSecurityState {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x31313372, 0x842c, 0x4110,
            [0xbd, 0xf1, 0x6a, 0xea, 0x17, 0xc8, 0x45, 0xad])
    }
}

unsafe impl RefCounted for nsISiteSecurityState {
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
pub trait nsISiteSecurityStateCoerce {
    fn coerce_from(v: &nsISiteSecurityState) -> &Self;
}

impl nsISiteSecurityStateCoerce for nsISiteSecurityState {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityState) -> &Self {
        v
    }
}

impl nsISiteSecurityState {
    #[inline]
    pub fn coerce<T: nsISiteSecurityStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISiteSecurityState {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISiteSecurityStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityState) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISiteSecurityStateVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString hostname; */
    pub get_hostname: unsafe extern "C" fn (this: *const nsISiteSecurityState, aHostname: *mut nsACString) -> nsresult,

    /* [infallible] readonly attribute long long expireTime; */
    pub get_expireTime: unsafe extern "C" fn (this: *const nsISiteSecurityState, aExpireTime: *mut libc::int64_t) -> nsresult,

    /* [infallible] readonly attribute short securityPropertyState; */
    pub get_securityPropertyState: unsafe extern "C" fn (this: *const nsISiteSecurityState, aSecurityPropertyState: *mut libc::int16_t) -> nsresult,

    /* [infallible] readonly attribute boolean includeSubdomains; */
    pub get_includeSubdomains: unsafe extern "C" fn (this: *const nsISiteSecurityState, aIncludeSubdomains: *mut bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_originAttributes: *const ::libc::c_void,

}


impl nsISiteSecurityState {
    /* readonly attribute ACString hostname; */
    #[inline]
    pub unsafe fn get_hostname(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hostname)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute long long expireTime; */
    #[inline]
    pub unsafe fn get_expireTime(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expireTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute short securityPropertyState; */
    #[inline]
    pub unsafe fn get_securityPropertyState(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_securityPropertyState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean includeSubdomains; */
    #[inline]
    pub unsafe fn get_includeSubdomains(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_includeSubdomains)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */


}


#[repr(C)]
pub struct nsISiteHSTSState {
    vtable: *const nsISiteHSTSStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISiteHSTSState {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9ff16e40, 0x1029, 0x496c,
            [0x95, 0xc2, 0xbc, 0x81, 0x98, 0x72, 0xb2, 0x16])
    }
}

unsafe impl RefCounted for nsISiteHSTSState {
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
pub trait nsISiteHSTSStateCoerce {
    fn coerce_from(v: &nsISiteHSTSState) -> &Self;
}

impl nsISiteHSTSStateCoerce for nsISiteHSTSState {
    #[inline]
    fn coerce_from(v: &nsISiteHSTSState) -> &Self {
        v
    }
}

impl nsISiteHSTSState {
    #[inline]
    pub fn coerce<T: nsISiteHSTSStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISiteHSTSState {
    type Target = nsISiteSecurityState;
    #[inline]
    fn deref(&self) -> &nsISiteSecurityState {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISiteSecurityStateCoerce> nsISiteHSTSStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteHSTSState) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISiteHSTSStateVTable {
    pub __base: nsISiteSecurityStateVTable,

}


impl nsISiteHSTSState {
}


#[repr(C)]
pub struct nsISiteHPKPState {
    vtable: *const nsISiteHPKPStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISiteHPKPState {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae395078, 0xc7d0, 0x474d,
            [0xb1, 0x47, 0xf4, 0xaa, 0x20, 0x3a, 0x9b, 0x2c])
    }
}

unsafe impl RefCounted for nsISiteHPKPState {
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
pub trait nsISiteHPKPStateCoerce {
    fn coerce_from(v: &nsISiteHPKPState) -> &Self;
}

impl nsISiteHPKPStateCoerce for nsISiteHPKPState {
    #[inline]
    fn coerce_from(v: &nsISiteHPKPState) -> &Self {
        v
    }
}

impl nsISiteHPKPState {
    #[inline]
    pub fn coerce<T: nsISiteHPKPStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISiteHPKPState {
    type Target = nsISiteSecurityState;
    #[inline]
    fn deref(&self) -> &nsISiteSecurityState {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISiteSecurityStateCoerce> nsISiteHPKPStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteHPKPState) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISiteHPKPStateVTable {
    pub __base: nsISiteSecurityStateVTable,

    /* readonly attribute nsISimpleEnumerator sha256Keys; */
    pub get_sha256Keys: unsafe extern "C" fn (this: *const nsISiteHPKPState, aSha256Keys: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsISiteHPKPState {
    /* readonly attribute nsISimpleEnumerator sha256Keys; */
    #[inline]
    pub unsafe fn get_sha256Keys(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sha256Keys)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


pub mod nsISiteSecurityService_consts {
    pub const HEADER_HSTS: i64 = 0;
    pub const HEADER_HPKP: i64 = 1;
    pub const HEADER_OMS: i64 = 2;
    pub const Success: i64 = 0;
    pub const ERROR_UNKNOWN: i64 = 1;
    pub const ERROR_UNTRUSTWORTHY_CONNECTION: i64 = 2;
    pub const ERROR_COULD_NOT_PARSE_HEADER: i64 = 3;
    pub const ERROR_NO_MAX_AGE: i64 = 4;
    pub const ERROR_MULTIPLE_MAX_AGES: i64 = 5;
    pub const ERROR_INVALID_MAX_AGE: i64 = 6;
    pub const ERROR_MULTIPLE_INCLUDE_SUBDOMAINS: i64 = 7;
    pub const ERROR_INVALID_INCLUDE_SUBDOMAINS: i64 = 8;
    pub const ERROR_INVALID_PIN: i64 = 9;
    pub const ERROR_MULTIPLE_REPORT_URIS: i64 = 10;
    pub const ERROR_PINSET_DOES_NOT_MATCH_CHAIN: i64 = 11;
    pub const ERROR_NO_BACKUP_PIN: i64 = 12;
    pub const ERROR_COULD_NOT_SAVE_STATE: i64 = 13;
    pub const ERROR_ROOT_NOT_BUILT_IN: i64 = 14;
}


#[repr(C)]
pub struct nsISiteSecurityService {
    vtable: *const nsISiteSecurityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISiteSecurityService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x275127f8, 0xdbd7, 0x4681,
            [0xaf, 0xbf, 0x6d, 0xf0, 0xc6, 0x58, 0x7a, 0x01])
    }
}

unsafe impl RefCounted for nsISiteSecurityService {
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
pub trait nsISiteSecurityServiceCoerce {
    fn coerce_from(v: &nsISiteSecurityService) -> &Self;
}

impl nsISiteSecurityServiceCoerce for nsISiteSecurityService {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityService) -> &Self {
        v
    }
}

impl nsISiteSecurityService {
    #[inline]
    pub fn coerce<T: nsISiteSecurityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISiteSecurityService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISiteSecurityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISiteSecurityServiceVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(ProcessHeader),noscript] void processHeaderNative (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsISSLStatus aSSLStatus, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub ProcessHeader: *const ::libc::c_void,

    /* [binaryname(ProcessHeaderScriptable),implicit_jscontext,optional_argc] void processHeader (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsISSLStatus aSSLStatus, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub ProcessHeaderScriptable: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] void removeState (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub removeState: *const ::libc::c_void,

    /* [binaryname(IsSecureURI),noscript] boolean isSecureURINative (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out boolean aCached); */
    /// Unable to call function as its signature contains a non-rust type
    pub IsSecureURI: *const ::libc::c_void,

    /* [binaryname(IsSecureURIScriptable),implicit_jscontext,optional_argc] boolean isSecureURI (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out boolean aCached); */
    /// Unable to call function as its signature contains a non-rust type
    pub IsSecureURIScriptable: *const ::libc::c_void,

    /* void clearAll (); */
    pub clearAll: unsafe extern "C" fn (this: *const nsISiteSecurityService) -> nsresult,

    /* void clearPreloads (); */
    pub clearPreloads: unsafe extern "C" fn (this: *const nsISiteSecurityService) -> nsresult,

    /* [noscript] boolean getKeyPinsForHostname (in ACString aHostname, in mozillaPkixTime evalTime, in const_OriginAttributesRef aOriginAttributes, out nsCStringTArrayRef aPinArray, out boolean aIncludeSubdomains); */
    /// Unable to call function as its signature contains a non-rust type
    pub getKeyPinsForHostname: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] boolean setKeyPins (in ACString aHost, in boolean aIncludeSubdomains, in int64_t aExpires, in unsigned long aPinCount, [array, size_is (aPinCount)] in string aSha256Pins, [optional] in boolean aIsPreload, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub setKeyPins: *const ::libc::c_void,

    /* boolean setHSTSPreload (in ACString aHost, in boolean aIncludesSubdomains, in int64_t aExpires); */
    pub setHSTSPreload: unsafe extern "C" fn (this: *const nsISiteSecurityService, aHost: *const nsACString, aIncludesSubdomains: bool, aExpires: int64_t, _retval: *mut bool) -> nsresult,

    /* [noscript] void cacheNegativeHSTSResult (in nsIURI aURI, in unsigned long long aMaxAge, in const_OriginAttributesRef aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cacheNegativeHSTSResult: *const ::libc::c_void,

    /* nsISimpleEnumerator enumerate (in uint32_t aType); */
    pub enumerate: unsafe extern "C" fn (this: *const nsISiteSecurityService, aType: uint32_t, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

}


impl nsISiteSecurityService {
    /* [binaryname(ProcessHeader),noscript] void processHeaderNative (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsISSLStatus aSSLStatus, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */


    /* [binaryname(ProcessHeaderScriptable),implicit_jscontext,optional_argc] void processHeader (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsISSLStatus aSSLStatus, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */


    /* [implicit_jscontext,optional_argc] void removeState (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes); */


    /* [binaryname(IsSecureURI),noscript] boolean isSecureURINative (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out boolean aCached); */


    /* [binaryname(IsSecureURIScriptable),implicit_jscontext,optional_argc] boolean isSecureURI (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out boolean aCached); */


    /* void clearAll (); */
    #[inline]
    pub unsafe fn clearAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearPreloads (); */
    #[inline]
    pub unsafe fn clearPreloads(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearPreloads)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] boolean getKeyPinsForHostname (in ACString aHostname, in mozillaPkixTime evalTime, in const_OriginAttributesRef aOriginAttributes, out nsCStringTArrayRef aPinArray, out boolean aIncludeSubdomains); */


    /* [implicit_jscontext,optional_argc] boolean setKeyPins (in ACString aHost, in boolean aIncludeSubdomains, in int64_t aExpires, in unsigned long aPinCount, [array, size_is (aPinCount)] in string aSha256Pins, [optional] in boolean aIsPreload, [optional] in jsval aOriginAttributes); */


    /* boolean setHSTSPreload (in ACString aHost, in boolean aIncludesSubdomains, in int64_t aExpires); */
    #[inline]
    pub unsafe fn setHSTSPreload(&self, aHost: &[u8], aIncludesSubdomains: bool, aExpires: int64_t) -> Result<bool, nsresult> {
        let aHost = nsCString::from(aHost);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).setHSTSPreload)(self as *const _, &*aHost, aIncludesSubdomains, aExpires, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void cacheNegativeHSTSResult (in nsIURI aURI, in unsigned long long aMaxAge, in const_OriginAttributesRef aOriginAttributes); */


    /* nsISimpleEnumerator enumerate (in uint32_t aType); */
    #[inline]
    pub unsafe fn enumerate(&self, aType: uint32_t) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).enumerate)(self as *const _, aType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


