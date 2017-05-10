//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIQuotaCallbacks.idl
//


#[repr(C)]
pub struct nsIQuotaUsageCallback {
    vtable: *const nsIQuotaUsageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaUsageCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc8a21a2a, 0x17b9, 0x4b63,
            [0xad, 0x95, 0xe0, 0xfb, 0xcf, 0xf5, 0xde, 0x18])
    }
}

unsafe impl RefCounted for nsIQuotaUsageCallback {
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
pub trait nsIQuotaUsageCallbackCoerce {
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self;
}

impl nsIQuotaUsageCallbackCoerce for nsIQuotaUsageCallback {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self {
        v
    }
}

impl nsIQuotaUsageCallback {
    #[inline]
    pub fn coerce<T: nsIQuotaUsageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaUsageCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaUsageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaUsageCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onUsageResult (in nsIQuotaUsageRequest aRequest); */
    pub onUsageResult: unsafe extern "C" fn (this: *const nsIQuotaUsageCallback, aRequest: *const nsIQuotaUsageRequest) -> nsresult,

}


impl nsIQuotaUsageCallback {
    /* void onUsageResult (in nsIQuotaUsageRequest aRequest); */
    #[inline]
    pub unsafe fn onUsageResult(&self, aRequest: Option<&nsIQuotaUsageRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).onUsageResult)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIQuotaCallback {
    vtable: *const nsIQuotaCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIQuotaCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa08a28e2, 0x5a74, 0x4c84,
            [0x80, 0x70, 0xed, 0x45, 0xa0, 0x7e, 0xb0, 0x13])
    }
}

unsafe impl RefCounted for nsIQuotaCallback {
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
pub trait nsIQuotaCallbackCoerce {
    fn coerce_from(v: &nsIQuotaCallback) -> &Self;
}

impl nsIQuotaCallbackCoerce for nsIQuotaCallback {
    #[inline]
    fn coerce_from(v: &nsIQuotaCallback) -> &Self {
        v
    }
}

impl nsIQuotaCallback {
    #[inline]
    pub fn coerce<T: nsIQuotaCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIQuotaCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIQuotaCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIQuotaCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsIQuotaRequest aRequest); */
    pub onComplete: unsafe extern "C" fn (this: *const nsIQuotaCallback, aRequest: *const nsIQuotaRequest) -> nsresult,

}


impl nsIQuotaCallback {
    /* void onComplete (in nsIQuotaRequest aRequest); */
    #[inline]
    pub unsafe fn onComplete(&self, aRequest: Option<&nsIQuotaRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).onComplete)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


