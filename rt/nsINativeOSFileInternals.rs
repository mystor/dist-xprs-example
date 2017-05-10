//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINativeOSFileInternals.idl
//


#[repr(C)]
pub struct nsINativeOSFileResult {
    vtable: *const nsINativeOSFileResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeOSFileResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x08b4cf29, 0x3d65, 0x4e79,
            [0xb5, 0x22, 0xa6, 0x94, 0xc3, 0x22, 0xed, 0x07])
    }
}

unsafe impl RefCounted for nsINativeOSFileResult {
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
pub trait nsINativeOSFileResultCoerce {
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self;
}

impl nsINativeOSFileResultCoerce for nsINativeOSFileResult {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self {
        v
    }
}

impl nsINativeOSFileResult {
    #[inline]
    pub fn coerce<T: nsINativeOSFileResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeOSFileResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeOSFileResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeOSFileResultVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] readonly attribute jsval result; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_result: *const ::libc::c_void,

    /* readonly attribute double dispatchDurationMS; */
    pub get_dispatchDurationMS: unsafe extern "C" fn (this: *const nsINativeOSFileResult, aDispatchDurationMS: *mut libc::c_double) -> nsresult,

    /* readonly attribute double executionDurationMS; */
    pub get_executionDurationMS: unsafe extern "C" fn (this: *const nsINativeOSFileResult, aExecutionDurationMS: *mut libc::c_double) -> nsresult,

}


impl nsINativeOSFileResult {
    /* [implicit_jscontext] readonly attribute jsval result; */


    /* readonly attribute double dispatchDurationMS; */
    #[inline]
    pub unsafe fn get_dispatchDurationMS(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_dispatchDurationMS)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double executionDurationMS; */
    #[inline]
    pub unsafe fn get_executionDurationMS(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_executionDurationMS)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsINativeOSFileSuccessCallback {
    vtable: *const nsINativeOSFileSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeOSFileSuccessCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2c1922ca, 0xca1b, 0x4099,
            [0x8b, 0x61, 0xec, 0x23, 0xcf, 0xf4, 0x94, 0x12])
    }
}

unsafe impl RefCounted for nsINativeOSFileSuccessCallback {
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
pub trait nsINativeOSFileSuccessCallbackCoerce {
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self;
}

impl nsINativeOSFileSuccessCallbackCoerce for nsINativeOSFileSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self {
        v
    }
}

impl nsINativeOSFileSuccessCallback {
    #[inline]
    pub fn coerce<T: nsINativeOSFileSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeOSFileSuccessCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeOSFileSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeOSFileSuccessCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in nsINativeOSFileResult result); */
    pub complete: unsafe extern "C" fn (this: *const nsINativeOSFileSuccessCallback, result: *const nsINativeOSFileResult) -> nsresult,

}


impl nsINativeOSFileSuccessCallback {
    /* void complete (in nsINativeOSFileResult result); */
    #[inline]
    pub unsafe fn complete(&self, result: Option<&nsINativeOSFileResult>) -> Result<(), nsresult> {

        match ((*self.vtable).complete)(self as *const _, result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeOSFileErrorCallback {
    vtable: *const nsINativeOSFileErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeOSFileErrorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf612e0fc, 0x6736, 0x4d24,
            [0xaa, 0x50, 0xfd, 0x66, 0x1b, 0x3b, 0x40, 0xb6])
    }
}

unsafe impl RefCounted for nsINativeOSFileErrorCallback {
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
pub trait nsINativeOSFileErrorCallbackCoerce {
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self;
}

impl nsINativeOSFileErrorCallbackCoerce for nsINativeOSFileErrorCallback {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self {
        v
    }
}

impl nsINativeOSFileErrorCallback {
    #[inline]
    pub fn coerce<T: nsINativeOSFileErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeOSFileErrorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeOSFileErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeOSFileErrorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void complete (in ACString operation, in long OSstatus); */
    pub complete: unsafe extern "C" fn (this: *const nsINativeOSFileErrorCallback, operation: *const nsACString, OSstatus: libc::int32_t) -> nsresult,

}


impl nsINativeOSFileErrorCallback {
    /* void complete (in ACString operation, in long OSstatus); */
    #[inline]
    pub unsafe fn complete(&self, operation: &[u8], OSstatus: libc::int32_t) -> Result<(), nsresult> {
        let operation = nsCString::from(operation);
        match ((*self.vtable).complete)(self as *const _, &*operation, OSstatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINativeOSFileInternalsService {
    vtable: *const nsINativeOSFileInternalsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINativeOSFileInternalsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x913362ad, 0x1526, 0x4623,
            [0x9e, 0x6b, 0xa2, 0xeb, 0x08, 0xaf, 0xbb, 0xb9])
    }
}

unsafe impl RefCounted for nsINativeOSFileInternalsService {
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
pub trait nsINativeOSFileInternalsServiceCoerce {
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self;
}

impl nsINativeOSFileInternalsServiceCoerce for nsINativeOSFileInternalsService {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self {
        v
    }
}

impl nsINativeOSFileInternalsService {
    #[inline]
    pub fn coerce<T: nsINativeOSFileInternalsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINativeOSFileInternalsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINativeOSFileInternalsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINativeOSFileInternalsServiceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void read (in AString path, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */
    /// Unable to call function as its signature contains a non-rust type
    pub read: *const ::libc::c_void,

}


impl nsINativeOSFileInternalsService {
    /* [implicit_jscontext] void read (in AString path, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */


}


