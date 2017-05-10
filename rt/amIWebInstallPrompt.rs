//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/amIWebInstallPrompt.idl
//


#[repr(C)]
pub struct amIWebInstallPrompt {
    vtable: *const amIWebInstallPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for amIWebInstallPrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x386906f1, 0x4d18, 0x45bf,
            [0xbc, 0x81, 0x5d, 0xcd, 0x68, 0xe4, 0x2c, 0x3b])
    }
}

unsafe impl RefCounted for amIWebInstallPrompt {
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
pub trait amIWebInstallPromptCoerce {
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self;
}

impl amIWebInstallPromptCoerce for amIWebInstallPrompt {
    #[inline]
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self {
        v
    }
}

impl amIWebInstallPrompt {
    #[inline]
    pub fn coerce<T: amIWebInstallPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for amIWebInstallPrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> amIWebInstallPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &amIWebInstallPrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct amIWebInstallPromptVTable {
    pub __base: nsISupportsVTable,

    /* void confirm (in nsIDOMElement aBrowser, in nsIURI aUri, [array, size_is (aCount)] in nsIVariant aInstalls, [optional] in uint32_t aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub confirm: *const ::libc::c_void,

}


impl amIWebInstallPrompt {
    /* void confirm (in nsIDOMElement aBrowser, in nsIURI aUri, [array, size_is (aCount)] in nsIVariant aInstalls, [optional] in uint32_t aCount); */


}


