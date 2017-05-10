//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRaceCacheWithNetwork.idl
//


#[repr(C)]
pub struct nsIRaceCacheWithNetwork {
    vtable: *const nsIRaceCacheWithNetworkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRaceCacheWithNetwork {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d963475, 0x8b16, 0x4c58,
            [0xb8, 0x04, 0x8a, 0x23, 0xd4, 0x94, 0x36, 0xc5])
    }
}

unsafe impl RefCounted for nsIRaceCacheWithNetwork {
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
pub trait nsIRaceCacheWithNetworkCoerce {
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self;
}

impl nsIRaceCacheWithNetworkCoerce for nsIRaceCacheWithNetwork {
    #[inline]
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self {
        v
    }
}

impl nsIRaceCacheWithNetwork {
    #[inline]
    pub fn coerce<T: nsIRaceCacheWithNetworkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRaceCacheWithNetwork {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRaceCacheWithNetworkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRaceCacheWithNetwork) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRaceCacheWithNetworkVTable {
    pub __base: nsISupportsVTable,

    /* void test_triggerNetwork (in long timeout); */
    pub test_triggerNetwork: unsafe extern "C" fn (this: *const nsIRaceCacheWithNetwork, timeout: libc::int32_t) -> nsresult,

    /* void test_delayCacheEntryOpeningBy (in long timeout); */
    pub test_delayCacheEntryOpeningBy: unsafe extern "C" fn (this: *const nsIRaceCacheWithNetwork, timeout: libc::int32_t) -> nsresult,

    /* void test_triggerDelayedOpenCacheEntry (); */
    pub test_triggerDelayedOpenCacheEntry: unsafe extern "C" fn (this: *const nsIRaceCacheWithNetwork) -> nsresult,

}


impl nsIRaceCacheWithNetwork {
    /* void test_triggerNetwork (in long timeout); */
    #[inline]
    pub unsafe fn test_triggerNetwork(&self, timeout: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).test_triggerNetwork)(self as *const _, timeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void test_delayCacheEntryOpeningBy (in long timeout); */
    #[inline]
    pub unsafe fn test_delayCacheEntryOpeningBy(&self, timeout: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).test_delayCacheEntryOpeningBy)(self as *const _, timeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void test_triggerDelayedOpenCacheEntry (); */
    #[inline]
    pub unsafe fn test_triggerDelayedOpenCacheEntry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).test_triggerDelayedOpenCacheEntry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


