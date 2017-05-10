//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRecoveryService.idl
//


pub mod nsIRecoveryService_consts {
    pub const FOTA_UPDATE_UNKNOWN: i64 = 0;
    pub const FOTA_UPDATE_FAIL: i64 = 1;
    pub const FOTA_UPDATE_SUCCESS: i64 = 2;
}


#[repr(C)]
pub struct nsIRecoveryService {
    vtable: *const nsIRecoveryServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRecoveryService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbc24fb33, 0xa0c1, 0x49ca,
            [0xaa, 0x43, 0x05, 0xf1, 0x67, 0xe0, 0x2f, 0xb6])
    }
}

unsafe impl RefCounted for nsIRecoveryService {
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
pub trait nsIRecoveryServiceCoerce {
    fn coerce_from(v: &nsIRecoveryService) -> &Self;
}

impl nsIRecoveryServiceCoerce for nsIRecoveryService {
    #[inline]
    fn coerce_from(v: &nsIRecoveryService) -> &Self {
        v
    }
}

impl nsIRecoveryService {
    #[inline]
    pub fn coerce<T: nsIRecoveryServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRecoveryService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRecoveryServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRecoveryService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRecoveryServiceVTable {
    pub __base: nsISupportsVTable,

    /* void factoryReset (in string reason); */
    pub factoryReset: unsafe extern "C" fn (this: *const nsIRecoveryService, reason: *const libc::c_char) -> nsresult,

    /* void installFotaUpdate (in string updatePath); */
    pub installFotaUpdate: unsafe extern "C" fn (this: *const nsIRecoveryService, updatePath: *const libc::c_char) -> nsresult,

    /* long getFotaUpdateStatus (); */
    pub getFotaUpdateStatus: unsafe extern "C" fn (this: *const nsIRecoveryService, _retval: *mut libc::int32_t) -> nsresult,

}


impl nsIRecoveryService {
    /* void factoryReset (in string reason); */
    #[inline]
    pub unsafe fn factoryReset(&self, reason: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).factoryReset)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void installFotaUpdate (in string updatePath); */
    #[inline]
    pub unsafe fn installFotaUpdate(&self, updatePath: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).installFotaUpdate)(self as *const _, updatePath) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getFotaUpdateStatus (); */
    #[inline]
    pub unsafe fn getFotaUpdateStatus(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getFotaUpdateStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


