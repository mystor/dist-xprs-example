//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIDNSService.idl
//


#[repr(C)]
pub struct nsPIDNSService {
    vtable: *const nsPIDNSServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIDNSService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24e598fd, 0x7b1a, 0x436c,
            [0x91, 0x54, 0x14, 0xd8, 0xb3, 0x8d, 0xf8, 0xa5])
    }
}

unsafe impl RefCounted for nsPIDNSService {
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
pub trait nsPIDNSServiceCoerce {
    fn coerce_from(v: &nsPIDNSService) -> &Self;
}

impl nsPIDNSServiceCoerce for nsPIDNSService {
    #[inline]
    fn coerce_from(v: &nsPIDNSService) -> &Self {
        v
    }
}

impl nsPIDNSService {
    #[inline]
    pub fn coerce<T: nsPIDNSServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIDNSService {
    type Target = nsIDNSService;
    #[inline]
    fn deref(&self) -> &nsIDNSService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDNSServiceCoerce> nsPIDNSServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIDNSService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIDNSServiceVTable {
    pub __base: nsIDNSServiceVTable,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsPIDNSService) -> nsresult,

    /* void shutdown (); */
    pub shutdown: unsafe extern "C" fn (this: *const nsPIDNSService) -> nsresult,

    /* attribute boolean prefetchEnabled; */
    pub get_prefetchEnabled: unsafe extern "C" fn (this: *const nsPIDNSService, aPrefetchEnabled: *mut bool) -> nsresult,
    pub set_prefetchEnabled: unsafe extern "C" fn (this: *const nsPIDNSService, aPrefetchEnabled: bool) -> nsresult,

}


impl nsPIDNSService {
    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void shutdown (); */
    #[inline]
    pub unsafe fn shutdown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).shutdown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean prefetchEnabled; */
    #[inline]
    pub unsafe fn get_prefetchEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_prefetchEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_prefetchEnabled(&self, aPrefetchEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_prefetchEnabled)(self as *const _, aPrefetchEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


