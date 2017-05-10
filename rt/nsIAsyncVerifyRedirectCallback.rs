//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncVerifyRedirectCallback.idl
//


#[repr(C)]
pub struct nsIAsyncVerifyRedirectCallback {
    vtable: *const nsIAsyncVerifyRedirectCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncVerifyRedirectCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8d171460, 0xa716, 0x41f1,
            [0x92, 0xbe, 0x8c, 0x65, 0x9d, 0xb3, 0x9b, 0x45])
    }
}

unsafe impl RefCounted for nsIAsyncVerifyRedirectCallback {
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
pub trait nsIAsyncVerifyRedirectCallbackCoerce {
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self;
}

impl nsIAsyncVerifyRedirectCallbackCoerce for nsIAsyncVerifyRedirectCallback {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self {
        v
    }
}

impl nsIAsyncVerifyRedirectCallback {
    #[inline]
    pub fn coerce<T: nsIAsyncVerifyRedirectCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncVerifyRedirectCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAsyncVerifyRedirectCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncVerifyRedirectCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncVerifyRedirectCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onRedirectVerifyCallback (in nsresult result); */
    pub onRedirectVerifyCallback: unsafe extern "C" fn (this: *const nsIAsyncVerifyRedirectCallback, result: nsresult) -> nsresult,

}


impl nsIAsyncVerifyRedirectCallback {
    /* void onRedirectVerifyCallback (in nsresult result); */
    #[inline]
    pub unsafe fn onRedirectVerifyCallback(&self, result: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onRedirectVerifyCallback)(self as *const _, result) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


