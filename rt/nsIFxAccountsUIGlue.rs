//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFxAccountsUIGlue.idl
//


#[repr(C)]
pub struct nsIFxAccountsUIGlue {
    vtable: *const nsIFxAccountsUIGlueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFxAccountsUIGlue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xab8d0700, 0x9577, 0x11e3,
            [0xa5, 0xe2, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIFxAccountsUIGlue {
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
pub trait nsIFxAccountsUIGlueCoerce {
    fn coerce_from(v: &nsIFxAccountsUIGlue) -> &Self;
}

impl nsIFxAccountsUIGlueCoerce for nsIFxAccountsUIGlue {
    #[inline]
    fn coerce_from(v: &nsIFxAccountsUIGlue) -> &Self {
        v
    }
}

impl nsIFxAccountsUIGlue {
    #[inline]
    pub fn coerce<T: nsIFxAccountsUIGlueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFxAccountsUIGlue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFxAccountsUIGlueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFxAccountsUIGlue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFxAccountsUIGlueVTable {
    pub __base: nsISupportsVTable,

    /* jsval signInFlow (); */
    /// Unable to call function as its signature contains a non-rust type
    pub signInFlow: *const ::libc::c_void,

    /* jsval refreshAuthentication (in DOMString email); */
    /// Unable to call function as its signature contains a non-rust type
    pub refreshAuthentication: *const ::libc::c_void,

}


impl nsIFxAccountsUIGlue {
    /* jsval signInFlow (); */


    /* jsval refreshAuthentication (in DOMString email); */


}


