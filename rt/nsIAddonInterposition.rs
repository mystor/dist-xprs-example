//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAddonInterposition.idl
//


#[repr(C)]
pub struct nsIAddonInterposition {
    vtable: *const nsIAddonInterpositionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAddonInterposition {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd05cc5fd, 0xad88, 0x41a6,
            [0x85, 0x4c, 0x36, 0xfd, 0x94, 0xd6, 0x9d, 0xdb])
    }
}

unsafe impl RefCounted for nsIAddonInterposition {
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
pub trait nsIAddonInterpositionCoerce {
    fn coerce_from(v: &nsIAddonInterposition) -> &Self;
}

impl nsIAddonInterpositionCoerce for nsIAddonInterposition {
    #[inline]
    fn coerce_from(v: &nsIAddonInterposition) -> &Self {
        v
    }
}

impl nsIAddonInterposition {
    #[inline]
    pub fn coerce<T: nsIAddonInterpositionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAddonInterposition {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAddonInterpositionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAddonInterposition) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAddonInterpositionVTable {
    pub __base: nsISupportsVTable,

    /* jsval interposeProperty (in jsval addonId, in jsval target, in nsIIDPtr iface, in jsval prop); */
    /// Unable to call function as its signature contains a non-rust type
    pub interposeProperty: *const ::libc::c_void,

    /* jsval interposeCall (in jsval addonId, in jsval originalFunc, in jsval originalThis, in jsval args); */
    /// Unable to call function as its signature contains a non-rust type
    pub interposeCall: *const ::libc::c_void,

    /* jsval getWhitelist (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getWhitelist: *const ::libc::c_void,

}


impl nsIAddonInterposition {
    /* jsval interposeProperty (in jsval addonId, in jsval target, in nsIIDPtr iface, in jsval prop); */


    /* jsval interposeCall (in jsval addonId, in jsval originalFunc, in jsval originalThis, in jsval args); */


    /* jsval getWhitelist (); */


}


