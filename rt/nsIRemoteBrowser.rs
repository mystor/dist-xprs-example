//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRemoteBrowser.idl
//


#[repr(C)]
pub struct nsIRemoteBrowser {
    vtable: *const nsIRemoteBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRemoteBrowser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc8379366, 0xf79f, 0x4d25,
            [0x89, 0xa6, 0x22, 0xbe, 0xc0, 0xa9, 0x3d, 0x16])
    }
}

unsafe impl RefCounted for nsIRemoteBrowser {
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
pub trait nsIRemoteBrowserCoerce {
    fn coerce_from(v: &nsIRemoteBrowser) -> &Self;
}

impl nsIRemoteBrowserCoerce for nsIRemoteBrowser {
    #[inline]
    fn coerce_from(v: &nsIRemoteBrowser) -> &Self {
        v
    }
}

impl nsIRemoteBrowser {
    #[inline]
    pub fn coerce<T: nsIRemoteBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRemoteBrowser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRemoteBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRemoteBrowser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRemoteBrowserVTable {
    pub __base: nsISupportsVTable,

    /* void enableDisableCommands (in AString action, in unsigned long enabledLength, [array, size_is (enabledLength)] in string enabledCommands, in unsigned long disabledLength, [array, size_is (disabledLength)] in string disabledCommands); */
    /// Unable to call function as its signature contains a non-rust type
    pub enableDisableCommands: *const ::libc::c_void,

}


impl nsIRemoteBrowser {
    /* void enableDisableCommands (in AString action, in unsigned long enabledLength, [array, size_is (enabledLength)] in string enabledCommands, in unsigned long disabledLength, [array, size_is (disabledLength)] in string disabledCommands); */


}


