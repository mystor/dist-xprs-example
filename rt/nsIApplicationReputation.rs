//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIApplicationReputation.idl
//


pub mod nsIApplicationReputationService_consts {
    pub const VERDICT_SAFE: i64 = 0;
    pub const VERDICT_DANGEROUS: i64 = 1;
    pub const VERDICT_UNCOMMON: i64 = 2;
    pub const VERDICT_POTENTIALLY_UNWANTED: i64 = 3;
    pub const VERDICT_DANGEROUS_HOST: i64 = 4;
}


#[repr(C)]
pub struct nsIApplicationReputationService {
    vtable: *const nsIApplicationReputationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationReputationService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc9f03479, 0xfd68, 0x4393,
            [0xac, 0xb2, 0xc8, 0x8d, 0x4f, 0x56, 0x31, 0x74])
    }
}

unsafe impl RefCounted for nsIApplicationReputationService {
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
pub trait nsIApplicationReputationServiceCoerce {
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self;
}

impl nsIApplicationReputationServiceCoerce for nsIApplicationReputationService {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self {
        v
    }
}

impl nsIApplicationReputationService {
    #[inline]
    pub fn coerce<T: nsIApplicationReputationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationReputationService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationReputationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationReputationServiceVTable {
    pub __base: nsISupportsVTable,

    /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
    pub queryReputation: unsafe extern "C" fn (this: *const nsIApplicationReputationService, aQuery: *const nsIApplicationReputationQuery, aCallback: *const nsIApplicationReputationCallback) -> nsresult,

}


impl nsIApplicationReputationService {
    /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
    #[inline]
    pub unsafe fn queryReputation(&self, aQuery: Option<&nsIApplicationReputationQuery>, aCallback: Option<&nsIApplicationReputationCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).queryReputation)(self as *const _, aQuery.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIApplicationReputationQuery {
    vtable: *const nsIApplicationReputationQueryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationReputationQuery {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x812d7509, 0xa9a3, 0x446e,
            [0xa6, 0x6f, 0x3e, 0xd8, 0xcc, 0x91, 0xeb, 0xd0])
    }
}

unsafe impl RefCounted for nsIApplicationReputationQuery {
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
pub trait nsIApplicationReputationQueryCoerce {
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self;
}

impl nsIApplicationReputationQueryCoerce for nsIApplicationReputationQuery {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self {
        v
    }
}

impl nsIApplicationReputationQuery {
    #[inline]
    pub fn coerce<T: nsIApplicationReputationQueryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationReputationQuery {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationReputationQueryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationQuery) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationReputationQueryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI sourceURI; */
    pub get_sourceURI: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aSourceURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI referrerURI; */
    pub get_referrerURI: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aReferrerURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AString suggestedFileName; */
    pub get_suggestedFileName: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aSuggestedFileName: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long fileSize; */
    pub get_fileSize: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aFileSize: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute ACString sha256Hash; */
    pub get_sha256Hash: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aSha256Hash: *mut nsACString) -> nsresult,

    /* readonly attribute nsIArray signatureInfo; */
    pub get_signatureInfo: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aSignatureInfo: *mut *const nsIArray) -> nsresult,

    /* readonly attribute nsIArray redirects; */
    pub get_redirects: unsafe extern "C" fn (this: *const nsIApplicationReputationQuery, aRedirects: *mut *const nsIArray) -> nsresult,

}


impl nsIApplicationReputationQuery {
    /* readonly attribute nsIURI sourceURI; */
    #[inline]
    pub unsafe fn get_sourceURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sourceURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI referrerURI; */
    #[inline]
    pub unsafe fn get_referrerURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrerURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString suggestedFileName; */
    #[inline]
    pub unsafe fn get_suggestedFileName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_suggestedFileName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long fileSize; */
    #[inline]
    pub unsafe fn get_fileSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString sha256Hash; */
    #[inline]
    pub unsafe fn get_sha256Hash(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_sha256Hash)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray signatureInfo; */
    #[inline]
    pub unsafe fn get_signatureInfo(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_signatureInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray redirects; */
    #[inline]
    pub unsafe fn get_redirects(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_redirects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIApplicationReputationCallback {
    vtable: *const nsIApplicationReputationCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIApplicationReputationCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9a228470, 0xcfe5, 0x11e2,
            [0x8b, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIApplicationReputationCallback {
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
pub trait nsIApplicationReputationCallbackCoerce {
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self;
}

impl nsIApplicationReputationCallbackCoerce for nsIApplicationReputationCallback {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self {
        v
    }
}

impl nsIApplicationReputationCallback {
    #[inline]
    pub fn coerce<T: nsIApplicationReputationCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIApplicationReputationCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIApplicationReputationCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationReputationCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIApplicationReputationCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
    pub onComplete: unsafe extern "C" fn (this: *const nsIApplicationReputationCallback, aShouldBlock: bool, aStatus: nsresult, aVerdict: libc::uint32_t) -> nsresult,

}


impl nsIApplicationReputationCallback {
    /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
    #[inline]
    pub unsafe fn onComplete(&self, aShouldBlock: bool, aStatus: nsresult, aVerdict: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onComplete)(self as *const _, aShouldBlock, aStatus, aVerdict) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


