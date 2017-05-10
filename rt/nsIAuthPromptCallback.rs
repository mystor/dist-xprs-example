//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPromptCallback.idl
//


#[repr(C)]
pub struct nsIAuthPromptCallback {
    vtable: *const nsIAuthPromptCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthPromptCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbdc387d7, 0x2d29, 0x4cac,
            [0x92, 0xf1, 0xdd, 0x75, 0xd7, 0x86, 0x63, 0x1d])
    }
}

unsafe impl RefCounted for nsIAuthPromptCallback {
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
pub trait nsIAuthPromptCallbackCoerce {
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self;
}

impl nsIAuthPromptCallbackCoerce for nsIAuthPromptCallback {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self {
        v
    }
}

impl nsIAuthPromptCallback {
    #[inline]
    pub fn coerce<T: nsIAuthPromptCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthPromptCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthPromptCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthPromptCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
    pub onAuthAvailable: unsafe extern "C" fn (this: *const nsIAuthPromptCallback, aContext: *const nsISupports, aAuthInfo: *const nsIAuthInformation) -> nsresult,

    /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
    pub onAuthCancelled: unsafe extern "C" fn (this: *const nsIAuthPromptCallback, aContext: *const nsISupports, userCancel: bool) -> nsresult,

}


impl nsIAuthPromptCallback {
    /* void onAuthAvailable (in nsISupports aContext, in nsIAuthInformation aAuthInfo); */
    #[inline]
    pub unsafe fn onAuthAvailable(&self, aContext: Option<&nsISupports>, aAuthInfo: Option<&nsIAuthInformation>) -> Result<(), nsresult> {

        match ((*self.vtable).onAuthAvailable)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), aAuthInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onAuthCancelled (in nsISupports aContext, in boolean userCancel); */
    #[inline]
    pub unsafe fn onAuthCancelled(&self, aContext: Option<&nsISupports>, userCancel: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onAuthCancelled)(self as *const _, aContext.map_or(::std::ptr::null(), |x| x as *const _), userCancel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


