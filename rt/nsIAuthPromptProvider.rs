//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAuthPromptProvider.idl
//


pub mod nsIAuthPromptProvider_consts {
    pub const PROMPT_NORMAL: i64 = 0;
    pub const PROMPT_PROXY: i64 = 1;
}


#[repr(C)]
pub struct nsIAuthPromptProvider {
    vtable: *const nsIAuthPromptProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAuthPromptProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbd9dc0fa, 0x68ce, 0x47d0,
            [0x88, 0x59, 0x64, 0x18, 0xc2, 0xae, 0x85, 0x76])
    }
}

unsafe impl RefCounted for nsIAuthPromptProvider {
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
pub trait nsIAuthPromptProviderCoerce {
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self;
}

impl nsIAuthPromptProviderCoerce for nsIAuthPromptProvider {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self {
        v
    }
}

impl nsIAuthPromptProvider {
    #[inline]
    pub fn coerce<T: nsIAuthPromptProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAuthPromptProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAuthPromptProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPromptProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAuthPromptProviderVTable {
    pub __base: nsISupportsVTable,

    /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub getAuthPrompt: unsafe extern "C" fn (this: *const nsIAuthPromptProvider, aPromptReason: uint32_t, iid: *const nsIID, result: *mut *const libc::c_void) -> nsresult,

}


impl nsIAuthPromptProvider {
    /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    #[inline]
    pub unsafe fn getAuthPrompt<T: XpCom>(&self, aPromptReason: uint32_t) -> Result<Option<RefPtr<T>>, nsresult> {
        let mut result : GetterAddrefs<T> = GetterAddrefs::new();
        match ((*self.vtable).getAuthPrompt)(self as *const _, aPromptReason, &T::iid(), result.ptr() as *mut *const T as *mut *const ::libc::c_void) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(result.refptr())
    }

}


