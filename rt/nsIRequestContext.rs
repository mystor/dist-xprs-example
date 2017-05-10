//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequestContext.idl
//


#[repr(C)]
pub struct nsIRequestContext {
    vtable: *const nsIRequestContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRequestContext {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x658e3e6e, 0x8633, 0x4b1a,
            [0x8d, 0x66, 0xfa, 0x9f, 0x72, 0x29, 0x3e, 0x63])
    }
}

unsafe impl RefCounted for nsIRequestContext {
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
pub trait nsIRequestContextCoerce {
    fn coerce_from(v: &nsIRequestContext) -> &Self;
}

impl nsIRequestContextCoerce for nsIRequestContext {
    #[inline]
    fn coerce_from(v: &nsIRequestContext) -> &Self {
        v
    }
}

impl nsIRequestContext {
    #[inline]
    pub fn coerce<T: nsIRequestContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRequestContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRequestContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestContext) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRequestContextVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute unsigned long long ID; */
    pub get_ID: unsafe extern "C" fn (this: *const nsIRequestContext, aID: *mut libc::uint64_t) -> nsresult,

    /* readonly attribute unsigned long blockingTransactionCount; */
    pub get_blockingTransactionCount: unsafe extern "C" fn (this: *const nsIRequestContext, aBlockingTransactionCount: *mut libc::uint32_t) -> nsresult,

    /* void addBlockingTransaction (); */
    pub addBlockingTransaction: unsafe extern "C" fn (this: *const nsIRequestContext) -> nsresult,

    /* unsigned long removeBlockingTransaction (); */
    pub removeBlockingTransaction: unsafe extern "C" fn (this: *const nsIRequestContext, _retval: *mut libc::uint32_t) -> nsresult,

    /* [noscript] attribute SpdyPushCachePtr spdyPushCache; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_spdyPushCache: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_spdyPushCache: *const ::libc::c_void,

    /* [noscript] attribute ACString userAgentOverride; */
    pub get_userAgentOverride: unsafe extern "C" fn (this: *const nsIRequestContext, aUserAgentOverride: *mut nsACString) -> nsresult,
    pub set_userAgentOverride: unsafe extern "C" fn (this: *const nsIRequestContext, aUserAgentOverride: *const nsACString) -> nsresult,

}


impl nsIRequestContext {
    /* [noscript] readonly attribute unsigned long long ID; */
    #[inline]
    pub unsafe fn get_ID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_ID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long blockingTransactionCount; */
    #[inline]
    pub unsafe fn get_blockingTransactionCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_blockingTransactionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addBlockingTransaction (); */
    #[inline]
    pub unsafe fn addBlockingTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).addBlockingTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long removeBlockingTransaction (); */
    #[inline]
    pub unsafe fn removeBlockingTransaction(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).removeBlockingTransaction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] attribute SpdyPushCachePtr spdyPushCache; */



    /* [noscript] attribute ACString userAgentOverride; */
    #[inline]
    pub unsafe fn get_userAgentOverride(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_userAgentOverride)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_userAgentOverride(&self, aUserAgentOverride: &[u8]) -> Result<(), nsresult> {
        let aUserAgentOverride = nsCString::from(aUserAgentOverride);
        match ((*self.vtable).set_userAgentOverride)(self as *const _, &*aUserAgentOverride) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIRequestContextService {
    vtable: *const nsIRequestContextServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRequestContextService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7fcbf4da, 0xd828, 0x4acc,
            [0xb1, 0x44, 0xe5, 0x43, 0x51, 0x98, 0xf7, 0x27])
    }
}

unsafe impl RefCounted for nsIRequestContextService {
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
pub trait nsIRequestContextServiceCoerce {
    fn coerce_from(v: &nsIRequestContextService) -> &Self;
}

impl nsIRequestContextServiceCoerce for nsIRequestContextService {
    #[inline]
    fn coerce_from(v: &nsIRequestContextService) -> &Self {
        v
    }
}

impl nsIRequestContextService {
    #[inline]
    pub fn coerce<T: nsIRequestContextServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRequestContextService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRequestContextServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestContextService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRequestContextServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIRequestContext getRequestContext (in unsigned long long id); */
    pub getRequestContext: unsafe extern "C" fn (this: *const nsIRequestContextService, id: libc::uint64_t, _retval: *mut *const nsIRequestContext) -> nsresult,

    /* nsIRequestContext newRequestContext (); */
    pub newRequestContext: unsafe extern "C" fn (this: *const nsIRequestContextService, _retval: *mut *const nsIRequestContext) -> nsresult,

    /* void removeRequestContext (in unsigned long long id); */
    pub removeRequestContext: unsafe extern "C" fn (this: *const nsIRequestContextService, id: libc::uint64_t) -> nsresult,

}


impl nsIRequestContextService {
    /* nsIRequestContext getRequestContext (in unsigned long long id); */
    #[inline]
    pub unsafe fn getRequestContext(&self, id: libc::uint64_t) -> Result<Option<RefPtr<nsIRequestContext>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRequestContext)(self as *const _, id, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRequestContext newRequestContext (); */
    #[inline]
    pub unsafe fn newRequestContext(&self, ) -> Result<Option<RefPtr<nsIRequestContext>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newRequestContext)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeRequestContext (in unsigned long long id); */
    #[inline]
    pub unsafe fn removeRequestContext(&self, id: libc::uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeRequestContext)(self as *const _, id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


