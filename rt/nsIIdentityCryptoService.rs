//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdentityCryptoService.idl
//


#[repr(C)]
pub struct nsIIdentityCryptoService {
    vtable: *const nsIIdentityCryptoServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdentityCryptoService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf087e6bc, 0xdd33, 0x4f6c,
            [0xa1, 0x06, 0xdd, 0x78, 0x6e, 0x05, 0x2e, 0xe9])
    }
}

unsafe impl RefCounted for nsIIdentityCryptoService {
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
pub trait nsIIdentityCryptoServiceCoerce {
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self;
}

impl nsIIdentityCryptoServiceCoerce for nsIIdentityCryptoService {
    #[inline]
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self {
        v
    }
}

impl nsIIdentityCryptoService {
    #[inline]
    pub fn coerce<T: nsIIdentityCryptoServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdentityCryptoService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdentityCryptoServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityCryptoService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdentityCryptoServiceVTable {
    pub __base: nsISupportsVTable,

    /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
    pub generateKeyPair: unsafe extern "C" fn (this: *const nsIIdentityCryptoService, algorithm: *const nsACString, callback: *const nsIIdentityKeyGenCallback) -> nsresult,

    /* ACString base64UrlEncode (in AUTF8String toEncode); */
    pub base64UrlEncode: unsafe extern "C" fn (this: *const nsIIdentityCryptoService, toEncode: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIIdentityCryptoService {
    /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
    #[inline]
    pub unsafe fn generateKeyPair(&self, algorithm: &[u8], callback: Option<&nsIIdentityKeyGenCallback>) -> Result<(), nsresult> {
        let algorithm = nsCString::from(algorithm);
        match ((*self.vtable).generateKeyPair)(self as *const _, &*algorithm, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString base64UrlEncode (in AUTF8String toEncode); */
    #[inline]
    pub unsafe fn base64UrlEncode(&self, toEncode: &[u8]) -> Result<nsCString, nsresult> {
        let toEncode = nsCString::from(toEncode);
        let mut _retval = nsCString::new();
        match ((*self.vtable).base64UrlEncode)(self as *const _, &*toEncode, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIIdentityKeyPair {
    vtable: *const nsIIdentityKeyPairVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdentityKeyPair {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x73962dc7, 0x8ee7, 0x4346,
            [0xa1, 0x2b, 0xb0, 0x39, 0xe1, 0xd9, 0xb5, 0x4d])
    }
}

unsafe impl RefCounted for nsIIdentityKeyPair {
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
pub trait nsIIdentityKeyPairCoerce {
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self;
}

impl nsIIdentityKeyPairCoerce for nsIIdentityKeyPair {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self {
        v
    }
}

impl nsIIdentityKeyPair {
    #[inline]
    pub fn coerce<T: nsIIdentityKeyPairCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdentityKeyPair {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdentityKeyPairCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyPair) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdentityKeyPairVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String keyType; */
    pub get_keyType: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aKeyType: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
    pub get_hexRSAPublicKeyExponent: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexRSAPublicKeyExponent: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
    pub get_hexRSAPublicKeyModulus: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexRSAPublicKeyModulus: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexDSAPrime; */
    pub get_hexDSAPrime: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexDSAPrime: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexDSASubPrime; */
    pub get_hexDSASubPrime: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexDSASubPrime: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexDSAGenerator; */
    pub get_hexDSAGenerator: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexDSAGenerator: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String hexDSAPublicValue; */
    pub get_hexDSAPublicValue: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aHexDSAPublicValue: *mut nsACString) -> nsresult,

    /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
    pub sign: unsafe extern "C" fn (this: *const nsIIdentityKeyPair, aText: *const nsACString, callback: *const nsIIdentitySignCallback) -> nsresult,

}


impl nsIIdentityKeyPair {
    /* readonly attribute AUTF8String keyType; */
    #[inline]
    pub unsafe fn get_keyType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_keyType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
    #[inline]
    pub unsafe fn get_hexRSAPublicKeyExponent(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexRSAPublicKeyExponent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
    #[inline]
    pub unsafe fn get_hexRSAPublicKeyModulus(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexRSAPublicKeyModulus)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexDSAPrime; */
    #[inline]
    pub unsafe fn get_hexDSAPrime(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexDSAPrime)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexDSASubPrime; */
    #[inline]
    pub unsafe fn get_hexDSASubPrime(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexDSASubPrime)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexDSAGenerator; */
    #[inline]
    pub unsafe fn get_hexDSAGenerator(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexDSAGenerator)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String hexDSAPublicValue; */
    #[inline]
    pub unsafe fn get_hexDSAPublicValue(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hexDSAPublicValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
    #[inline]
    pub unsafe fn sign(&self, aText: &[u8], callback: Option<&nsIIdentitySignCallback>) -> Result<(), nsresult> {
        let aText = nsCString::from(aText);
        match ((*self.vtable).sign)(self as *const _, &*aText, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIIdentityKeyGenCallback {
    vtable: *const nsIIdentityKeyGenCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdentityKeyGenCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x90f24ca2, 0x2b05, 0x4ca9,
            [0x8a, 0xec, 0x89, 0xd3, 0x8e, 0x2f, 0x90, 0x5a])
    }
}

unsafe impl RefCounted for nsIIdentityKeyGenCallback {
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
pub trait nsIIdentityKeyGenCallbackCoerce {
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self;
}

impl nsIIdentityKeyGenCallbackCoerce for nsIIdentityKeyGenCallback {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self {
        v
    }
}

impl nsIIdentityKeyGenCallback {
    #[inline]
    pub fn coerce<T: nsIIdentityKeyGenCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdentityKeyGenCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdentityKeyGenCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentityKeyGenCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdentityKeyGenCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
    pub generateKeyPairFinished: unsafe extern "C" fn (this: *const nsIIdentityKeyGenCallback, rv: nsresult, keyPair: *const nsIIdentityKeyPair) -> nsresult,

}


impl nsIIdentityKeyGenCallback {
    /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
    #[inline]
    pub unsafe fn generateKeyPairFinished(&self, rv: nsresult, keyPair: Option<&nsIIdentityKeyPair>) -> Result<(), nsresult> {

        match ((*self.vtable).generateKeyPairFinished)(self as *const _, rv, keyPair.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIIdentitySignCallback {
    vtable: *const nsIIdentitySignCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdentitySignCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2d3e5036, 0x374b, 0x4b47,
            [0xa4, 0x30, 0x11, 0x96, 0xb6, 0x7b, 0x89, 0x0f])
    }
}

unsafe impl RefCounted for nsIIdentitySignCallback {
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
pub trait nsIIdentitySignCallbackCoerce {
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self;
}

impl nsIIdentitySignCallbackCoerce for nsIIdentitySignCallback {
    #[inline]
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self {
        v
    }
}

impl nsIIdentitySignCallback {
    #[inline]
    pub fn coerce<T: nsIIdentitySignCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdentitySignCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdentitySignCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdentitySignCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdentitySignCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
    pub signFinished: unsafe extern "C" fn (this: *const nsIIdentitySignCallback, rv: nsresult, base64urlSignature: *const nsACString) -> nsresult,

}


impl nsIIdentitySignCallback {
    /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
    #[inline]
    pub unsafe fn signFinished(&self, rv: nsresult, base64urlSignature: &[u8]) -> Result<(), nsresult> {
        let base64urlSignature = nsCString::from(base64urlSignature);
        match ((*self.vtable).signFinished)(self as *const _, rv, &*base64urlSignature) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


