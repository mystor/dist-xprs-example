//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICertOverrideService.idl
//


pub mod nsICertOverrideService_consts {
    pub const ERROR_UNTRUSTED: i64 = 1;
    pub const ERROR_MISMATCH: i64 = 2;
    pub const ERROR_TIME: i64 = 4;
}


#[repr(C)]
pub struct nsICertOverrideService {
    vtable: *const nsICertOverrideServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICertOverrideService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbe019e47, 0x22fc, 0x4355,
            [0x9f, 0x16, 0x9a, 0xb0, 0x47, 0xd6, 0x74, 0x2d])
    }
}

unsafe impl RefCounted for nsICertOverrideService {
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
pub trait nsICertOverrideServiceCoerce {
    fn coerce_from(v: &nsICertOverrideService) -> &Self;
}

impl nsICertOverrideServiceCoerce for nsICertOverrideService {
    #[inline]
    fn coerce_from(v: &nsICertOverrideService) -> &Self {
        v
    }
}

impl nsICertOverrideService {
    #[inline]
    pub fn coerce<T: nsICertOverrideServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICertOverrideService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICertOverrideServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICertOverrideService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICertOverrideServiceVTable {
    pub __base: nsISupportsVTable,

    /* void rememberValidityOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
    pub rememberValidityOverride: unsafe extern "C" fn (this: *const nsICertOverrideService, aHostName: *const nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: uint32_t, aTemporary: bool) -> nsresult,

    /* void rememberTemporaryValidityOverrideUsingFingerprint (in ACString aHostName, in int32_t aPort, in ACString aCertFingerprint, in uint32_t aOverrideBits); */
    pub rememberTemporaryValidityOverrideUsingFingerprint: unsafe extern "C" fn (this: *const nsICertOverrideService, aHostName: *const nsACString, aPort: int32_t, aCertFingerprint: *const nsACString, aOverrideBits: uint32_t) -> nsresult,

    /* boolean hasMatchingOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
    pub hasMatchingOverride: unsafe extern "C" fn (this: *const nsICertOverrideService, aHostName: *const nsACString, aPort: int32_t, aCert: *const nsIX509Cert, aOverrideBits: *mut uint32_t, aIsTemporary: *mut bool, _retval: *mut bool) -> nsresult,

    /* boolean getValidityOverride (in ACString aHostName, in int32_t aPort, out ACString aHashAlg, out ACString aFingerprint, out uint32_t aOverrideBits, out boolean aIsTemporary); */
    pub getValidityOverride: unsafe extern "C" fn (this: *const nsICertOverrideService, aHostName: *const nsACString, aPort: int32_t, aHashAlg: *mut nsACString, aFingerprint: *mut nsACString, aOverrideBits: *mut uint32_t, aIsTemporary: *mut bool, _retval: *mut bool) -> nsresult,

    /* void clearValidityOverride (in ACString aHostName, in int32_t aPort); */
    pub clearValidityOverride: unsafe extern "C" fn (this: *const nsICertOverrideService, aHostName: *const nsACString, aPort: int32_t) -> nsresult,

    /* uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
    pub isCertUsedForOverrides: unsafe extern "C" fn (this: *const nsICertOverrideService, aCert: *const nsIX509Cert, aCheckTemporaries: bool, aCheckPermanents: bool, _retval: *mut uint32_t) -> nsresult,

}


impl nsICertOverrideService {
    /* void rememberValidityOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
    #[inline]
    pub unsafe fn rememberValidityOverride(&self, aHostName: &[u8], aPort: int32_t, aCert: Option<&nsIX509Cert>, aOverrideBits: uint32_t, aTemporary: bool) -> Result<(), nsresult> {
        let aHostName = nsCString::from(aHostName);
        match ((*self.vtable).rememberValidityOverride)(self as *const _, &*aHostName, aPort, aCert.map_or(::std::ptr::null(), |x| x as *const _), aOverrideBits, aTemporary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rememberTemporaryValidityOverrideUsingFingerprint (in ACString aHostName, in int32_t aPort, in ACString aCertFingerprint, in uint32_t aOverrideBits); */
    #[inline]
    pub unsafe fn rememberTemporaryValidityOverrideUsingFingerprint(&self, aHostName: &[u8], aPort: int32_t, aCertFingerprint: &[u8], aOverrideBits: uint32_t) -> Result<(), nsresult> {
        let aHostName = nsCString::from(aHostName);
        let aCertFingerprint = nsCString::from(aCertFingerprint);
        match ((*self.vtable).rememberTemporaryValidityOverrideUsingFingerprint)(self as *const _, &*aHostName, aPort, &*aCertFingerprint, aOverrideBits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasMatchingOverride (in ACString aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
    #[inline]
    pub unsafe fn hasMatchingOverride(&self, aHostName: &[u8], aPort: int32_t, aCert: Option<&nsIX509Cert>) -> Result<(uint32_t, bool, bool), nsresult> {
        let aHostName = nsCString::from(aHostName);
        let mut aOverrideBits: uint32_t = ::std::mem::zeroed();
        let mut aIsTemporary: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasMatchingOverride)(self as *const _, &*aHostName, aPort, aCert.map_or(::std::ptr::null(), |x| x as *const _), &mut aOverrideBits as *mut _, &mut aIsTemporary as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aOverrideBits, aIsTemporary, _retval))
    }

    /* boolean getValidityOverride (in ACString aHostName, in int32_t aPort, out ACString aHashAlg, out ACString aFingerprint, out uint32_t aOverrideBits, out boolean aIsTemporary); */
    #[inline]
    pub unsafe fn getValidityOverride(&self, aHostName: &[u8], aPort: int32_t) -> Result<(nsCString, nsCString, uint32_t, bool, bool), nsresult> {
        let aHostName = nsCString::from(aHostName);
        let mut aHashAlg = nsCString::new();
        let mut aFingerprint = nsCString::new();
        let mut aOverrideBits: uint32_t = ::std::mem::zeroed();
        let mut aIsTemporary: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getValidityOverride)(self as *const _, &*aHostName, aPort, &mut *aHashAlg, &mut *aFingerprint, &mut aOverrideBits as *mut _, &mut aIsTemporary as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aHashAlg, aFingerprint, aOverrideBits, aIsTemporary, _retval))
    }

    /* void clearValidityOverride (in ACString aHostName, in int32_t aPort); */
    #[inline]
    pub unsafe fn clearValidityOverride(&self, aHostName: &[u8], aPort: int32_t) -> Result<(), nsresult> {
        let aHostName = nsCString::from(aHostName);
        match ((*self.vtable).clearValidityOverride)(self as *const _, &*aHostName, aPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
    #[inline]
    pub unsafe fn isCertUsedForOverrides(&self, aCert: Option<&nsIX509Cert>, aCheckTemporaries: bool, aCheckPermanents: bool) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).isCertUsedForOverrides)(self as *const _, aCert.map_or(::std::ptr::null(), |x| x as *const _), aCheckTemporaries, aCheckPermanents, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


