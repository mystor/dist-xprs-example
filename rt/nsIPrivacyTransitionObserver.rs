//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrivacyTransitionObserver.idl
//


#[repr(C)]
pub struct nsIPrivacyTransitionObserver {
    vtable: *const nsIPrivacyTransitionObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrivacyTransitionObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb4b1449d, 0x0ef0, 0x47f5,
            [0xb6, 0x2e, 0xad, 0xc5, 0x7f, 0xd4, 0x97, 0x02])
    }
}

unsafe impl RefCounted for nsIPrivacyTransitionObserver {
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
pub trait nsIPrivacyTransitionObserverCoerce {
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self;
}

impl nsIPrivacyTransitionObserverCoerce for nsIPrivacyTransitionObserver {
    #[inline]
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self {
        v
    }
}

impl nsIPrivacyTransitionObserver {
    #[inline]
    pub fn coerce<T: nsIPrivacyTransitionObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrivacyTransitionObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPrivacyTransitionObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivacyTransitionObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrivacyTransitionObserverVTable {
    pub __base: nsISupportsVTable,

    /* void privateModeChanged (in bool enabled); */
    pub privateModeChanged: unsafe extern "C" fn (this: *const nsIPrivacyTransitionObserver, enabled: bool) -> nsresult,

}


impl nsIPrivacyTransitionObserver {
    /* void privateModeChanged (in bool enabled); */
    #[inline]
    pub unsafe fn privateModeChanged(&self, enabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).privateModeChanged)(self as *const _, enabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


