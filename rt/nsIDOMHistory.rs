//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHistory.idl
//


#[repr(C)]
pub struct nsIDOMHistory {
    vtable: *const nsIDOMHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55226663, 0xfe68, 0x48ba,
            [0xad, 0xdf, 0x08, 0xe3, 0x2e, 0xaa, 0xb5, 0x69])
    }
}

unsafe impl RefCounted for nsIDOMHistory {
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
pub trait nsIDOMHistoryCoerce {
    fn coerce_from(v: &nsIDOMHistory) -> &Self;
}

impl nsIDOMHistoryCoerce for nsIDOMHistory {
    #[inline]
    fn coerce_from(v: &nsIDOMHistory) -> &Self {
        v
    }
}

impl nsIDOMHistory {
    #[inline]
    pub fn coerce<T: nsIDOMHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHistoryVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMHistory {
}


