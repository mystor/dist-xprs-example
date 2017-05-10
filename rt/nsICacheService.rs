//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheService.idl
//


#[repr(C)]
pub struct nsICacheService {
    vtable: *const nsICacheServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x14dbe1e9, 0xf3bc, 0x45af,
            [0x92, 0xf4, 0x2c, 0x57, 0x4f, 0xcd, 0x4e, 0x39])
    }
}

unsafe impl RefCounted for nsICacheService {
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
pub trait nsICacheServiceCoerce {
    fn coerce_from(v: &nsICacheService) -> &Self;
}

impl nsICacheServiceCoerce for nsICacheService {
    #[inline]
    fn coerce_from(v: &nsICacheService) -> &Self {
        v
    }
}

impl nsICacheService {
    #[inline]
    pub fn coerce<T: nsICacheServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
    pub createSession: unsafe extern "C" fn (this: *const nsICacheService, clientID: *const libc::c_char, storagePolicy: nsCacheStoragePolicy, streamBased: bool, _retval: *mut *const nsICacheSession) -> nsresult,

    /* void visitEntries (in nsICacheVisitor visitor); */
    pub visitEntries: unsafe extern "C" fn (this: *const nsICacheService, visitor: *const nsICacheVisitor) -> nsresult,

    /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
    pub evictEntries: unsafe extern "C" fn (this: *const nsICacheService, storagePolicy: nsCacheStoragePolicy) -> nsresult,

    /* readonly attribute nsIEventTarget cacheIOTarget; */
    pub get_cacheIOTarget: unsafe extern "C" fn (this: *const nsICacheService, aCacheIOTarget: *mut *const nsIEventTarget) -> nsresult,

}


impl nsICacheService {
    /* nsICacheSession createSession (in string clientID, in nsCacheStoragePolicy storagePolicy, in boolean streamBased); */
    #[inline]
    pub unsafe fn createSession(&self, clientID: *const libc::c_char, storagePolicy: nsCacheStoragePolicy, streamBased: bool) -> Result<Option<RefPtr<nsICacheSession>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createSession)(self as *const _, clientID, storagePolicy, streamBased, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void visitEntries (in nsICacheVisitor visitor); */
    #[inline]
    pub unsafe fn visitEntries(&self, visitor: Option<&nsICacheVisitor>) -> Result<(), nsresult> {

        match ((*self.vtable).visitEntries)(self as *const _, visitor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void evictEntries (in nsCacheStoragePolicy storagePolicy); */
    #[inline]
    pub unsafe fn evictEntries(&self, storagePolicy: nsCacheStoragePolicy) -> Result<(), nsresult> {

        match ((*self.vtable).evictEntries)(self as *const _, storagePolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIEventTarget cacheIOTarget; */
    #[inline]
    pub unsafe fn get_cacheIOTarget(&self, ) -> Result<Option<RefPtr<nsIEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cacheIOTarget)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsICacheServiceInternal {
    vtable: *const nsICacheServiceInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheServiceInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd0fc8d38, 0xdb80, 0x4928,
            [0xbf, 0x1c, 0xb0, 0x08, 0x5d, 0xdf, 0xa9, 0xdc])
    }
}

unsafe impl RefCounted for nsICacheServiceInternal {
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
pub trait nsICacheServiceInternalCoerce {
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self;
}

impl nsICacheServiceInternalCoerce for nsICacheServiceInternal {
    #[inline]
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self {
        v
    }
}

impl nsICacheServiceInternal {
    #[inline]
    pub fn coerce<T: nsICacheServiceInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheServiceInternal {
    type Target = nsICacheService;
    #[inline]
    fn deref(&self) -> &nsICacheService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICacheServiceCoerce> nsICacheServiceInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheServiceInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheServiceInternalVTable {
    pub __base: nsICacheServiceVTable,

    /* readonly attribute double lockHeldTime; */
    pub get_lockHeldTime: unsafe extern "C" fn (this: *const nsICacheServiceInternal, aLockHeldTime: *mut libc::c_double) -> nsresult,

}


impl nsICacheServiceInternal {
    /* readonly attribute double lockHeldTime; */
    #[inline]
    pub unsafe fn get_lockHeldTime(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_lockHeldTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


