//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaResults.idl
//


#[repr(C)]
pub struct nsIQuotaUsageResult {
    vtable: *const nsIQuotaUsageResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaUsageResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd8c9328b, 0x9aa8, 0x4f5d,
            [0x90, 0xe6, 0x48, 0x2d, 0xe4, 0xa6, 0xd5, 0xb8])
    }
}

unsafe impl RefCounted for nsIQuotaUsageResult {
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
pub trait nsIQuotaUsageResultCoerce {
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self;
}

impl nsIQuotaUsageResultCoerce for nsIQuotaUsageResult {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self {
        v
    }
}

impl nsIQuotaUsageResult {
    #[inline]
    pub fn coerce<T: nsIQuotaUsageResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaUsageResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaUsageResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaUsageResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString origin; */
    pub get_origin: unsafe extern "C" fn (this: *const nsIQuotaUsageResult, aOrigin: *mut nsACString) -> nsresult,

    /* readonly attribute boolean persisted; */
    pub get_persisted: unsafe extern "C" fn (this: *const nsIQuotaUsageResult, aPersisted: *mut bool) -> nsresult,

    /* readonly attribute unsigned long long usage; */
    pub get_usage: unsafe extern "C" fn (this: *const nsIQuotaUsageResult, aUsage: *mut libc::uint64_t) -> nsresult,

}


impl nsIQuotaUsageResult {
    /* readonly attribute ACString origin; */
    #[inline]
    pub unsafe fn get_origin(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_origin)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean persisted; */
    #[inline]
    pub unsafe fn get_persisted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_persisted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long usage; */
    #[inline]
    pub unsafe fn get_usage(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_usage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIQuotaOriginUsageResult {
    vtable: *const nsIQuotaOriginUsageResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaOriginUsageResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x96df03d2, 0x116a, 0x493f,
            [0xbb, 0x0b, 0x11, 0x8c, 0x21, 0x2a, 0x6b, 0x32])
    }
}

unsafe impl RefCounted for nsIQuotaOriginUsageResult {
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
pub trait nsIQuotaOriginUsageResultCoerce {
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self;
}

impl nsIQuotaOriginUsageResultCoerce for nsIQuotaOriginUsageResult {
    #[inline]
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self {
        v
    }
}

impl nsIQuotaOriginUsageResult {
    #[inline]
    pub fn coerce<T: nsIQuotaOriginUsageResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaOriginUsageResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaOriginUsageResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaOriginUsageResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long long usage; */
    pub get_usage: unsafe extern "C" fn (this: *const nsIQuotaOriginUsageResult, aUsage: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long fileUsage; */
    pub get_fileUsage: unsafe extern "C" fn (this: *const nsIQuotaOriginUsageResult, aFileUsage: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long long limit; */
    pub get_limit: unsafe extern "C" fn (this: *const nsIQuotaOriginUsageResult, aLimit: *mut libc::uint64_t) -> nsresult,

}


impl nsIQuotaOriginUsageResult {
    /* readonly attribute unsigned long long usage; */
    #[inline]
    pub unsafe fn get_usage(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_usage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long fileUsage; */
    #[inline]
    pub unsafe fn get_fileUsage(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileUsage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long long limit; */
    #[inline]
    pub unsafe fn get_limit(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_limit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


