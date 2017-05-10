//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPrompt2.idl
//


pub mod nsIAuthPrompt2_consts {
    pub const LEVEL_NONE: i64 = 0;
    pub const LEVEL_PW_ENCRYPTED: i64 = 1;
    pub const LEVEL_SECURE: i64 = 2;
}


#[repr(C)]
pub struct nsIAuthPrompt2 {
    vtable: *const nsIAuthPrompt2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthPrompt2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x651395eb, 0x8612, 0x4876,
            [0x8a, 0xc0, 0xa8, 0x8d, 0x4d, 0xce, 0x9e, 0x1e])
    }
}

unsafe impl RefCounted for nsIAuthPrompt2 {
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
pub trait nsIAuthPrompt2Coerce {
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self;
}

impl nsIAuthPrompt2Coerce for nsIAuthPrompt2 {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self {
        v
    }
}

impl nsIAuthPrompt2 {
    #[inline]
    pub fn coerce<T: nsIAuthPrompt2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthPrompt2 {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthPrompt2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthPrompt2VTable {
    pub __base: nsISupportsVTable,

    /* boolean promptAuth (in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo); */
    pub promptAuth: unsafe extern "C" fn (this: *const nsIAuthPrompt2, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut bool) -> nsresult,

    /* nsICancelable asyncPromptAuth (in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo); */
    pub asyncPromptAuth: unsafe extern "C" fn (this: *const nsIAuthPrompt2, aChannel: *const nsIChannel, aCallback: *const nsIAuthPromptCallback, aContext: *const nsISupports, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut *const nsICancelable) -> nsresult,

}


impl nsIAuthPrompt2 {
    /* boolean promptAuth (in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo); */
    #[inline]
    pub unsafe fn promptAuth(&self, aChannel: Option<&nsIChannel>, level: uint32_t, authInfo: Option<&nsIAuthInformation>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).promptAuth)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), level, authInfo.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsICancelable asyncPromptAuth (in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo); */
    #[inline]
    pub unsafe fn asyncPromptAuth(&self, aChannel: Option<&nsIChannel>, aCallback: Option<&nsIAuthPromptCallback>, aContext: Option<&nsISupports>, level: uint32_t, authInfo: Option<&nsIAuthInformation>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).asyncPromptAuth)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), level, authInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


