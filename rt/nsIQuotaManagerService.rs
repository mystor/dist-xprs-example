//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaManagerService.idl
//


#[repr(C)]
pub struct nsIQuotaManagerService {
    vtable: *const nsIQuotaManagerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaManagerService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1b3d0a38, 0x8151, 0x4cf9,
            [0x89, 0xfa, 0x4f, 0x92, 0xc2, 0xef, 0x0e, 0x7e])
    }
}

unsafe impl RefCounted for nsIQuotaManagerService {
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
pub trait nsIQuotaManagerServiceCoerce {
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self;
}

impl nsIQuotaManagerServiceCoerce for nsIQuotaManagerService {
    #[inline]
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self {
        v
    }
}

impl nsIQuotaManagerService {
    #[inline]
    pub fn coerce<T: nsIQuotaManagerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaManagerService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaManagerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaManagerService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaManagerServiceVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] nsIQuotaRequest init (); */
    pub init: unsafe extern "C" fn (this: *const nsIQuotaManagerService, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest initStoragesForPrincipal (in nsIPrincipal aPrincipal, in ACString aPersistenceType); */
    pub initStoragesForPrincipal: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aPersistenceType: *const nsACString, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
    pub getUsage: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aCallback: *const nsIQuotaUsageCallback, aGetAll: bool, _retval: *mut *const nsIQuotaUsageRequest) -> nsresult,

    /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetGroupUsage); */
    pub getUsageForPrincipal: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aCallback: *const nsIQuotaUsageCallback, aGetGroupUsage: bool, _retval: *mut *const nsIQuotaUsageRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsIQuotaManagerService, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in boolean aClearAll); */
    pub clearStoragesForPrincipal: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, aPersistenceType: *const nsACString, aClearAll: bool, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIQuotaManagerService, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
    pub persisted: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut *const nsIQuotaRequest) -> nsresult,

    /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
    pub persist: unsafe extern "C" fn (this: *const nsIQuotaManagerService, aPrincipal: *const nsIPrincipal, _retval: *mut *const nsIQuotaRequest) -> nsresult,

}


impl nsIQuotaManagerService {
    /* [must_use] nsIQuotaRequest init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).init)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest initStoragesForPrincipal (in nsIPrincipal aPrincipal, in ACString aPersistenceType); */
    #[inline]
    pub unsafe fn initStoragesForPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, aPersistenceType: &[u8]) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let aPersistenceType = nsCString::from(aPersistenceType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).initStoragesForPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aPersistenceType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaUsageRequest getUsage (in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetAll); */
    #[inline]
    pub unsafe fn getUsage(&self, aCallback: Option<&nsIQuotaUsageCallback>, aGetAll: bool) -> Result<Option<RefPtr<nsIQuotaUsageRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUsage)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aGetAll, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaUsageRequest getUsageForPrincipal (in nsIPrincipal aPrincipal, in nsIQuotaUsageCallback aCallback, [optional] in boolean aGetGroupUsage); */
    #[inline]
    pub unsafe fn getUsageForPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, aCallback: Option<&nsIQuotaUsageCallback>, aGetGroupUsage: bool) -> Result<Option<RefPtr<nsIQuotaUsageRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUsageForPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aGetGroupUsage, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clear)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest clearStoragesForPrincipal (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType, [optional] in boolean aClearAll); */
    #[inline]
    pub unsafe fn clearStoragesForPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, aPersistenceType: &[u8], aClearAll: bool) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let aPersistenceType = nsCString::from(aPersistenceType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clearStoragesForPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*aPersistenceType, aClearAll, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).reset)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest persisted (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn persisted(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).persisted)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [must_use] nsIQuotaRequest persist (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn persist(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<Option<RefPtr<nsIQuotaRequest>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).persist)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


