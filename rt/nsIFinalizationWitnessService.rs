//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFinalizationWitnessService.idl
//


#[repr(C)]
pub struct nsIFinalizationWitnessService {
    vtable: *const nsIFinalizationWitnessServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFinalizationWitnessService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x15686f9d, 0x483e, 0x4361,
            [0x98, 0xcd, 0x37, 0xf1, 0xe8, 0xf1, 0xe6, 0x1d])
    }
}

unsafe impl RefCounted for nsIFinalizationWitnessService {
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
pub trait nsIFinalizationWitnessServiceCoerce {
    fn coerce_from(v: &nsIFinalizationWitnessService) -> &Self;
}

impl nsIFinalizationWitnessServiceCoerce for nsIFinalizationWitnessService {
    #[inline]
    fn coerce_from(v: &nsIFinalizationWitnessService) -> &Self {
        v
    }
}

impl nsIFinalizationWitnessService {
    #[inline]
    pub fn coerce<T: nsIFinalizationWitnessServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFinalizationWitnessService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFinalizationWitnessServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFinalizationWitnessService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFinalizationWitnessServiceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval make (in string aTopic, in wstring aString); */
    /// Unable to call function as its signature contains a non-rust type
    pub make: *const ::libc::c_void,

}


impl nsIFinalizationWitnessService {
    /* [implicit_jscontext] jsval make (in string aTopic, in wstring aString); */


}


