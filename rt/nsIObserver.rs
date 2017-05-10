//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObserver.idl
//


#[repr(C)]
pub struct nsIObserver {
    vtable: *const nsIObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdb242e01, 0xe4d9, 0x11d2,
            [0x9d, 0xde, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74])
    }
}

unsafe impl RefCounted for nsIObserver {
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
pub trait nsIObserverCoerce {
    fn coerce_from(v: &nsIObserver) -> &Self;
}

impl nsIObserverCoerce for nsIObserver {
    #[inline]
    fn coerce_from(v: &nsIObserver) -> &Self {
        v
    }
}

impl nsIObserver {
    #[inline]
    pub fn coerce<T: nsIObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIObserverVTable {
    pub __base: nsISupportsVTable,

    /* void observe (in nsISupports aSubject, in string aTopic, in wstring aData); */
    pub observe: unsafe extern "C" fn (this: *const nsIObserver, aSubject: *const nsISupports, aTopic: *const libc::c_char, aData: *const libc::int16_t) -> nsresult,

}


impl nsIObserver {
    /* void observe (in nsISupports aSubject, in string aTopic, in wstring aData); */
    #[inline]
    pub unsafe fn observe(&self, aSubject: Option<&nsISupports>, aTopic: *const libc::c_char, aData: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).observe)(self as *const _, aSubject.map_or(::std::ptr::null(), |x| x as *const _), aTopic, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


