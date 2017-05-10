//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdlePeriod.idl
//


#[repr(C)]
pub struct nsIIdlePeriod {
    vtable: *const nsIIdlePeriodVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdlePeriod {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x21dd35a2, 0xeae9, 0x4bd8,
            [0xb4, 0x70, 0x0d, 0xfa, 0x35, 0xa0, 0xe3, 0xb9])
    }
}

unsafe impl RefCounted for nsIIdlePeriod {
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
pub trait nsIIdlePeriodCoerce {
    fn coerce_from(v: &nsIIdlePeriod) -> &Self;
}

impl nsIIdlePeriodCoerce for nsIIdlePeriod {
    #[inline]
    fn coerce_from(v: &nsIIdlePeriod) -> &Self {
        v
    }
}

impl nsIIdlePeriod {
    #[inline]
    pub fn coerce<T: nsIIdlePeriodCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdlePeriod {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIIdlePeriodCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdlePeriod) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdlePeriodVTable {
    pub __base: nsISupportsVTable,

    /* TimeStamp getIdlePeriodHint (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIdlePeriodHint: *const ::libc::c_void,

}


impl nsIIdlePeriod {
    /* TimeStamp getIdlePeriodHint (); */


}


