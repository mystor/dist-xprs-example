//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUrlClassifierUtils.idl
//


#[repr(C)]
pub struct nsIUrlClassifierParseFindFullHashCallback {
    vtable: *const nsIUrlClassifierParseFindFullHashCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierParseFindFullHashCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfbb9684a, 0xa0aa, 0x11e6,
            [0x88, 0xb0, 0x08, 0x60, 0x6e, 0x45, 0x6b, 0x8a])
    }
}

unsafe impl RefCounted for nsIUrlClassifierParseFindFullHashCallback {
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
pub trait nsIUrlClassifierParseFindFullHashCallbackCoerce {
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self;
}

impl nsIUrlClassifierParseFindFullHashCallbackCoerce for nsIUrlClassifierParseFindFullHashCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierParseFindFullHashCallback {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierParseFindFullHashCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierParseFindFullHashCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierParseFindFullHashCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierParseFindFullHashCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierParseFindFullHashCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
    pub onCompleteHashFound: unsafe extern "C" fn (this: *const nsIUrlClassifierParseFindFullHashCallback, aCompleteHash: *const nsACString, aTableNames: *const nsACString, aPerHashCacheDuration: libc::uint32_t) -> nsresult,

    /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
    pub onResponseParsed: unsafe extern "C" fn (this: *const nsIUrlClassifierParseFindFullHashCallback, aMinWaitDuration: libc::uint32_t, aNegCacheDuration: libc::uint32_t) -> nsresult,

}


impl nsIUrlClassifierParseFindFullHashCallback {
    /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
    #[inline]
    pub unsafe fn onCompleteHashFound(&self, aCompleteHash: &[u8], aTableNames: &[u8], aPerHashCacheDuration: libc::uint32_t) -> Result<(), nsresult> {
        let aCompleteHash = nsCString::from(aCompleteHash);
        let aTableNames = nsCString::from(aTableNames);
        match ((*self.vtable).onCompleteHashFound)(self as *const _, &*aCompleteHash, &*aTableNames, aPerHashCacheDuration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
    #[inline]
    pub unsafe fn onResponseParsed(&self, aMinWaitDuration: libc::uint32_t, aNegCacheDuration: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onResponseParsed)(self as *const _, aMinWaitDuration, aNegCacheDuration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUrlClassifierUtils {
    vtable: *const nsIUrlClassifierUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUrlClassifierUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe4f0e59c, 0xb922, 0x48b0,
            [0xa7, 0xb6, 0x17, 0x35, 0xc1, 0xf9, 0x6f, 0xed])
    }
}

unsafe impl RefCounted for nsIUrlClassifierUtils {
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
pub trait nsIUrlClassifierUtilsCoerce {
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self;
}

impl nsIUrlClassifierUtilsCoerce for nsIUrlClassifierUtils {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self {
        v
    }
}

impl nsIUrlClassifierUtils {
    #[inline]
    pub fn coerce<T: nsIUrlClassifierUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUrlClassifierUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUrlClassifierUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUrlClassifierUtilsVTable {
    pub __base: nsISupportsVTable,

    /* ACString getKeyForURI (in nsIURI uri); */
    pub getKeyForURI: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, uri: *const nsIURI, _retval: *mut nsACString) -> nsresult,

    /* ACString getProvider (in ACString tableName); */
    pub getProvider: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, tableName: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString getTelemetryProvider (in ACString tableName); */
    pub getTelemetryProvider: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, tableName: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString getProtocolVersion (in ACString provider); */
    pub getProtocolVersion: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, provider: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString convertThreatTypeToListNames (in uint32_t threatType); */
    pub convertThreatTypeToListNames: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, threatType: uint32_t, _retval: *mut nsACString) -> nsresult,

    /* uint32_t convertListNameToThreatType (in ACString listName); */
    pub convertListNameToThreatType: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, listName: *const nsACString, _retval: *mut uint32_t) -> nsresult,

    /* ACString makeUpdateRequestV4 ([array, size_is (aCount)] in string aListNames, [array, size_is (aCount)] in string aStatesBase64, in uint32_t aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub makeUpdateRequestV4: *const ::libc::c_void,

    /* ACString makeFindFullHashRequestV4 ([array, size_is (aListCount)] in string aListNames, [array, size_is (aListCount)] in string aListStatesBase64, [array, size_is (aPrefixCount)] in string aPrefixes, in uint32_t aListCount, in uint32_t aPrefixCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub makeFindFullHashRequestV4: *const ::libc::c_void,

    /* void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback); */
    pub parseFindFullHashResponseV4: unsafe extern "C" fn (this: *const nsIUrlClassifierUtils, aResponse: *const nsACString, aCallback: *const nsIUrlClassifierParseFindFullHashCallback) -> nsresult,

}


impl nsIUrlClassifierUtils {
    /* ACString getKeyForURI (in nsIURI uri); */
    #[inline]
    pub unsafe fn getKeyForURI(&self, uri: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getKeyForURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getProvider (in ACString tableName); */
    #[inline]
    pub unsafe fn getProvider(&self, tableName: &[u8]) -> Result<nsCString, nsresult> {
        let tableName = nsCString::from(tableName);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getProvider)(self as *const _, &*tableName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getTelemetryProvider (in ACString tableName); */
    #[inline]
    pub unsafe fn getTelemetryProvider(&self, tableName: &[u8]) -> Result<nsCString, nsresult> {
        let tableName = nsCString::from(tableName);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getTelemetryProvider)(self as *const _, &*tableName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getProtocolVersion (in ACString provider); */
    #[inline]
    pub unsafe fn getProtocolVersion(&self, provider: &[u8]) -> Result<nsCString, nsresult> {
        let provider = nsCString::from(provider);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getProtocolVersion)(self as *const _, &*provider, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString convertThreatTypeToListNames (in uint32_t threatType); */
    #[inline]
    pub unsafe fn convertThreatTypeToListNames(&self, threatType: uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).convertThreatTypeToListNames)(self as *const _, threatType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t convertListNameToThreatType (in ACString listName); */
    #[inline]
    pub unsafe fn convertListNameToThreatType(&self, listName: &[u8]) -> Result<uint32_t, nsresult> {
        let listName = nsCString::from(listName);
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).convertListNameToThreatType)(self as *const _, &*listName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString makeUpdateRequestV4 ([array, size_is (aCount)] in string aListNames, [array, size_is (aCount)] in string aStatesBase64, in uint32_t aCount); */


    /* ACString makeFindFullHashRequestV4 ([array, size_is (aListCount)] in string aListNames, [array, size_is (aListCount)] in string aListStatesBase64, [array, size_is (aPrefixCount)] in string aPrefixes, in uint32_t aListCount, in uint32_t aPrefixCount); */


    /* void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback); */
    #[inline]
    pub unsafe fn parseFindFullHashResponseV4(&self, aResponse: &[u8], aCallback: Option<&nsIUrlClassifierParseFindFullHashCallback>) -> Result<(), nsresult> {
        let aResponse = nsCString::from(aResponse);
        match ((*self.vtable).parseFindFullHashResponseV4)(self as *const _, &*aResponse, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


