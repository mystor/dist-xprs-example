//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpAuthenticatorCallback.idl
//


#[repr(C)]
pub struct nsIHttpAuthenticatorCallback {
    vtable: *const nsIHttpAuthenticatorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpAuthenticatorCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd989cb03, 0xe446, 0x4086,
            [0xb9, 0xe6, 0x46, 0x84, 0x2c, 0xb9, 0x7b, 0xd5])
    }
}

unsafe impl RefCounted for nsIHttpAuthenticatorCallback {
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
pub trait nsIHttpAuthenticatorCallbackCoerce {
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self;
}

impl nsIHttpAuthenticatorCallbackCoerce for nsIHttpAuthenticatorCallback {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self {
        v
    }
}

impl nsIHttpAuthenticatorCallback {
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticatorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpAuthenticatorCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpAuthenticatorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticatorCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpAuthenticatorCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState); */
    pub onCredsGenerated: unsafe extern "C" fn (this: *const nsIHttpAuthenticatorCallback, aCreds: *const libc::c_char, aFlags: libc::uint32_t, aResult: nsresult, aSessionsState: *const nsISupports, aContinuationState: *const nsISupports) -> nsresult,

}


impl nsIHttpAuthenticatorCallback {
    /* void onCredsGenerated (in string aCreds, in unsigned long aFlags, in nsresult aResult, in nsISupports aSessionsState, in nsISupports aContinuationState); */
    #[inline]
    pub unsafe fn onCredsGenerated(&self, aCreds: *const libc::c_char, aFlags: libc::uint32_t, aResult: nsresult, aSessionsState: Option<&nsISupports>, aContinuationState: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onCredsGenerated)(self as *const _, aCreds, aFlags, aResult, aSessionsState.map_or(::std::ptr::null(), |x| x as *const _), aContinuationState.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


