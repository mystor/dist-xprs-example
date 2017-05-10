//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEffectiveTLDService.idl
//


#[repr(C)]
pub struct nsIEffectiveTLDService {
    vtable: *const nsIEffectiveTLDServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEffectiveTLDService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x68067eb5, 0xad8d, 0x43cb,
            [0xa0, 0x43, 0x1c, 0xc8, 0x5e, 0xbe, 0x06, 0xe7])
    }
}

unsafe impl RefCounted for nsIEffectiveTLDService {
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
pub trait nsIEffectiveTLDServiceCoerce {
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self;
}

impl nsIEffectiveTLDServiceCoerce for nsIEffectiveTLDService {
    #[inline]
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self {
        v
    }
}

impl nsIEffectiveTLDService {
    #[inline]
    pub fn coerce<T: nsIEffectiveTLDServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEffectiveTLDService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEffectiveTLDServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEffectiveTLDService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEffectiveTLDServiceVTable {
    pub __base: nsISupportsVTable,

    /* ACString getPublicSuffix (in nsIURI aURI); */
    pub getPublicSuffix: unsafe extern "C" fn (this: *const nsIEffectiveTLDService, aURI: *const nsIURI, _retval: *mut nsACString) -> nsresult,

    /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
    pub getBaseDomain: unsafe extern "C" fn (this: *const nsIEffectiveTLDService, aURI: *const nsIURI, aAdditionalParts: uint32_t, _retval: *mut nsACString) -> nsresult,

    /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
    pub getPublicSuffixFromHost: unsafe extern "C" fn (this: *const nsIEffectiveTLDService, aHost: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
    pub getBaseDomainFromHost: unsafe extern "C" fn (this: *const nsIEffectiveTLDService, aHost: *const nsACString, aAdditionalParts: uint32_t, _retval: *mut nsACString) -> nsresult,

    /* ACString getNextSubDomain (in AUTF8String aHost); */
    pub getNextSubDomain: unsafe extern "C" fn (this: *const nsIEffectiveTLDService, aHost: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIEffectiveTLDService {
    /* ACString getPublicSuffix (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getPublicSuffix(&self, aURI: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPublicSuffix)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
    #[inline]
    pub unsafe fn getBaseDomain(&self, aURI: Option<&nsIURI>, aAdditionalParts: uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getBaseDomain)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aAdditionalParts, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
    #[inline]
    pub unsafe fn getPublicSuffixFromHost(&self, aHost: &[u8]) -> Result<nsCString, nsresult> {
        let aHost = nsCString::from(aHost);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPublicSuffixFromHost)(self as *const _, &*aHost, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
    #[inline]
    pub unsafe fn getBaseDomainFromHost(&self, aHost: &[u8], aAdditionalParts: uint32_t) -> Result<nsCString, nsresult> {
        let aHost = nsCString::from(aHost);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getBaseDomainFromHost)(self as *const _, &*aHost, aAdditionalParts, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getNextSubDomain (in AUTF8String aHost); */
    #[inline]
    pub unsafe fn getNextSubDomain(&self, aHost: &[u8]) -> Result<nsCString, nsresult> {
        let aHost = nsCString::from(aHost);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getNextSubDomain)(self as *const _, &*aHost, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


