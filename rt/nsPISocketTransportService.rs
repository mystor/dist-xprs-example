//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPISocketTransportService.idl
//


#[repr(C)]
pub struct nsPISocketTransportService {
    vtable: *const nsPISocketTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPISocketTransportService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x18f73bf1, 0xb35b, 0x4b7b,
            [0xaa, 0x9a, 0x11, 0xbc, 0xbd, 0xbc, 0x38, 0x9c])
    }
}

unsafe impl RefCounted for nsPISocketTransportService {
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
pub trait nsPISocketTransportServiceCoerce {
    fn coerce_from(v: &nsPISocketTransportService) -> &Self;
}

impl nsPISocketTransportServiceCoerce for nsPISocketTransportService {
    #[inline]
    fn coerce_from(v: &nsPISocketTransportService) -> &Self {
        v
    }
}

impl nsPISocketTransportService {
    #[inline]
    pub fn coerce<T: nsPISocketTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPISocketTransportService {
    type Target = nsIRoutedSocketTransportService;
    #[inline]
    fn deref(&self) -> &nsIRoutedSocketTransportService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRoutedSocketTransportServiceCoerce> nsPISocketTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPISocketTransportService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPISocketTransportServiceVTable {
    pub __base: nsIRoutedSocketTransportServiceVTable,

    /* void init (); */
    pub init: unsafe extern "C" fn (this: *const nsPISocketTransportService) -> nsresult,

    /* void shutdown (in bool aXpcomShutdown); */
    pub shutdown: unsafe extern "C" fn (this: *const nsPISocketTransportService, aXpcomShutdown: bool) -> nsresult,

    /* readonly attribute long sendBufferSize; */
    pub get_sendBufferSize: unsafe extern "C" fn (this: *const nsPISocketTransportService, aSendBufferSize: *mut libc::int32_t) -> nsresult,

    /* attribute boolean offline; */
    pub get_offline: unsafe extern "C" fn (this: *const nsPISocketTransportService, aOffline: *mut bool) -> nsresult,
    pub set_offline: unsafe extern "C" fn (this: *const nsPISocketTransportService, aOffline: bool) -> nsresult,

    /* readonly attribute long keepaliveIdleTime; */
    pub get_keepaliveIdleTime: unsafe extern "C" fn (this: *const nsPISocketTransportService, aKeepaliveIdleTime: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long keepaliveRetryInterval; */
    pub get_keepaliveRetryInterval: unsafe extern "C" fn (this: *const nsPISocketTransportService, aKeepaliveRetryInterval: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long keepaliveProbeCount; */
    pub get_keepaliveProbeCount: unsafe extern "C" fn (this: *const nsPISocketTransportService, aKeepaliveProbeCount: *mut libc::int32_t) -> nsresult,

}


impl nsPISocketTransportService {
    /* void init (); */
    #[inline]
    pub unsafe fn init(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void shutdown (in bool aXpcomShutdown); */
    #[inline]
    pub unsafe fn shutdown(&self, aXpcomShutdown: bool) -> Result<(), nsresult> {

        match ((*self.vtable).shutdown)(self as *const _, aXpcomShutdown) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long sendBufferSize; */
    #[inline]
    pub unsafe fn get_sendBufferSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sendBufferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean offline; */
    #[inline]
    pub unsafe fn get_offline(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_offline)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_offline(&self, aOffline: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_offline)(self as *const _, aOffline) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long keepaliveIdleTime; */
    #[inline]
    pub unsafe fn get_keepaliveIdleTime(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keepaliveIdleTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long keepaliveRetryInterval; */
    #[inline]
    pub unsafe fn get_keepaliveRetryInterval(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keepaliveRetryInterval)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long keepaliveProbeCount; */
    #[inline]
    pub unsafe fn get_keepaliveProbeCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keepaliveProbeCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


