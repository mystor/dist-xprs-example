//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINSSErrorsService.idl
//


pub mod nsINSSErrorsService_consts {
    pub const ERROR_CLASS_SSL_PROTOCOL: i64 = 1;
    pub const ERROR_CLASS_BAD_CERT: i64 = 2;
    pub const NSS_SEC_ERROR_BASE: i64 = -8192;
    pub const NSS_SEC_ERROR_LIMIT: i64 = -7192;
    pub const NSS_SSL_ERROR_BASE: i64 = -12288;
    pub const NSS_SSL_ERROR_LIMIT: i64 = -11288;
    pub const MOZILLA_PKIX_ERROR_BASE: i64 = -16384;
    pub const MOZILLA_PKIX_ERROR_LIMIT: i64 = -15384;
}


#[repr(C)]
pub struct nsINSSErrorsService {
    vtable: *const nsINSSErrorsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINSSErrorsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x12f60021, 0xe14b, 0x4020,
            [0x99, 0xd1, 0xed, 0x2c, 0x79, 0x5b, 0xe6, 0x6a])
    }
}

unsafe impl RefCounted for nsINSSErrorsService {
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
pub trait nsINSSErrorsServiceCoerce {
    fn coerce_from(v: &nsINSSErrorsService) -> &Self;
}

impl nsINSSErrorsServiceCoerce for nsINSSErrorsService {
    #[inline]
    fn coerce_from(v: &nsINSSErrorsService) -> &Self {
        v
    }
}

impl nsINSSErrorsService {
    #[inline]
    pub fn coerce<T: nsINSSErrorsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINSSErrorsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINSSErrorsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSErrorsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINSSErrorsServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean isNSSErrorCode (in int32_t aNSPRCode); */
    pub isNSSErrorCode: unsafe extern "C" fn (this: *const nsINSSErrorsService, aNSPRCode: int32_t, _retval: *mut bool) -> nsresult,

    /* nsresult getXPCOMFromNSSError (in int32_t aNSPRCode); */
    pub getXPCOMFromNSSError: unsafe extern "C" fn (this: *const nsINSSErrorsService, aNSPRCode: int32_t, _retval: *mut nsresult) -> nsresult,

    /* AString getErrorMessage (in nsresult aXPCOMErrorCode); */
    pub getErrorMessage: unsafe extern "C" fn (this: *const nsINSSErrorsService, aXPCOMErrorCode: nsresult, _retval: *mut nsAString) -> nsresult,

    /* uint32_t getErrorClass (in nsresult aXPCOMErrorCode); */
    pub getErrorClass: unsafe extern "C" fn (this: *const nsINSSErrorsService, aXPCOMErrorCode: nsresult, _retval: *mut uint32_t) -> nsresult,

}


impl nsINSSErrorsService {
    /* boolean isNSSErrorCode (in int32_t aNSPRCode); */
    #[inline]
    pub unsafe fn isNSSErrorCode(&self, aNSPRCode: int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isNSSErrorCode)(self as *const _, aNSPRCode, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsresult getXPCOMFromNSSError (in int32_t aNSPRCode); */
    #[inline]
    pub unsafe fn getXPCOMFromNSSError(&self, aNSPRCode: int32_t) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).getXPCOMFromNSSError)(self as *const _, aNSPRCode, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getErrorMessage (in nsresult aXPCOMErrorCode); */
    #[inline]
    pub unsafe fn getErrorMessage(&self, aXPCOMErrorCode: nsresult) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getErrorMessage)(self as *const _, aXPCOMErrorCode, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t getErrorClass (in nsresult aXPCOMErrorCode); */
    #[inline]
    pub unsafe fn getErrorClass(&self, aXPCOMErrorCode: nsresult) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getErrorClass)(self as *const _, aXPCOMErrorCode, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


