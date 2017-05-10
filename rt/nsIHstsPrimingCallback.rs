//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHstsPrimingCallback.idl
//


#[repr(C)]
pub struct nsIHstsPrimingCallback {
    vtable: *const nsIHstsPrimingCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHstsPrimingCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeca6daca, 0x3f2a, 0x4a2a,
            [0xb3, 0xbf, 0x9f, 0x24, 0xf7, 0x9b, 0xc9, 0x99])
    }
}

unsafe impl RefCounted for nsIHstsPrimingCallback {
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
pub trait nsIHstsPrimingCallbackCoerce {
    fn coerce_from(v: &nsIHstsPrimingCallback) -> &Self;
}

impl nsIHstsPrimingCallbackCoerce for nsIHstsPrimingCallback {
    #[inline]
    fn coerce_from(v: &nsIHstsPrimingCallback) -> &Self {
        v
    }
}

impl nsIHstsPrimingCallback {
    #[inline]
    pub fn coerce<T: nsIHstsPrimingCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHstsPrimingCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHstsPrimingCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHstsPrimingCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHstsPrimingCallbackVTable {
    pub __base: nsISupportsVTable,

    /* [must_use,noscript,nostdcall] void onHSTSPrimingSucceeded (in bool aCached); */
    pub onHSTSPrimingSucceeded: unsafe extern "C" fn (this: *const nsIHstsPrimingCallback, aCached: bool) -> nsresult,

    /* [must_use,noscript,nostdcall] void onHSTSPrimingFailed (in nsresult aError, in bool aCached); */
    pub onHSTSPrimingFailed: unsafe extern "C" fn (this: *const nsIHstsPrimingCallback, aError: nsresult, aCached: bool) -> nsresult,

}


impl nsIHstsPrimingCallback {
    /* [must_use,noscript,nostdcall] void onHSTSPrimingSucceeded (in bool aCached); */
    #[inline]
    pub unsafe fn onHSTSPrimingSucceeded(&self, aCached: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onHSTSPrimingSucceeded)(self as *const _, aCached) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use,noscript,nostdcall] void onHSTSPrimingFailed (in nsresult aError, in bool aCached); */
    #[inline]
    pub unsafe fn onHSTSPrimingFailed(&self, aError: nsresult, aCached: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onHSTSPrimingFailed)(self as *const _, aError, aCached) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


