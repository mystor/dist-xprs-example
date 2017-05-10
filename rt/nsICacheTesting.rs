//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICacheTesting.idl
//


#[repr(C)]
pub struct nsICacheTesting {
    vtable: *const nsICacheTestingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICacheTesting {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4e8ba935, 0x92e1, 0x4a74,
            [0x94, 0x4b, 0xb1, 0xa2, 0xf0, 0x2a, 0x74, 0x80])
    }
}

unsafe impl RefCounted for nsICacheTesting {
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
pub trait nsICacheTestingCoerce {
    fn coerce_from(v: &nsICacheTesting) -> &Self;
}

impl nsICacheTestingCoerce for nsICacheTesting {
    #[inline]
    fn coerce_from(v: &nsICacheTesting) -> &Self {
        v
    }
}

impl nsICacheTesting {
    #[inline]
    pub fn coerce<T: nsICacheTestingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICacheTesting {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICacheTestingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheTesting) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICacheTestingVTable {
    pub __base: nsISupportsVTable,

    /* void suspendCacheIOThread (in uint32_t aLevel); */
    pub suspendCacheIOThread: unsafe extern "C" fn (this: *const nsICacheTesting, aLevel: uint32_t) -> nsresult,

    /* void resumeCacheIOThread (); */
    pub resumeCacheIOThread: unsafe extern "C" fn (this: *const nsICacheTesting) -> nsresult,

    /* void flush (in nsIObserver aObserver); */
    pub flush: unsafe extern "C" fn (this: *const nsICacheTesting, aObserver: *const nsIObserver) -> nsresult,

}


impl nsICacheTesting {
    /* void suspendCacheIOThread (in uint32_t aLevel); */
    #[inline]
    pub unsafe fn suspendCacheIOThread(&self, aLevel: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).suspendCacheIOThread)(self as *const _, aLevel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumeCacheIOThread (); */
    #[inline]
    pub unsafe fn resumeCacheIOThread(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resumeCacheIOThread)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void flush (in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn flush(&self, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


