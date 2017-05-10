//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRandomGenerator.idl
//


#[repr(C)]
pub struct nsIRandomGenerator {
    vtable: *const nsIRandomGeneratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRandomGenerator {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2362d97a, 0x747a, 0x4576,
            [0x88, 0x63, 0x69, 0x76, 0x67, 0x30, 0x92, 0x09])
    }
}

unsafe impl RefCounted for nsIRandomGenerator {
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
pub trait nsIRandomGeneratorCoerce {
    fn coerce_from(v: &nsIRandomGenerator) -> &Self;
}

impl nsIRandomGeneratorCoerce for nsIRandomGenerator {
    #[inline]
    fn coerce_from(v: &nsIRandomGenerator) -> &Self {
        v
    }
}

impl nsIRandomGenerator {
    #[inline]
    pub fn coerce<T: nsIRandomGeneratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRandomGenerator {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRandomGeneratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRandomGenerator) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRandomGeneratorVTable {
    pub __base: nsISupportsVTable,

    /* void generateRandomBytes (in unsigned long aLength, [array, size_is (aLength), retval] out octet aBuffer); */
    /// Unable to call function as its signature contains a non-rust type
    pub generateRandomBytes: *const ::libc::c_void,

}


impl nsIRandomGenerator {
    /* void generateRandomBytes (in unsigned long aLength, [array, size_is (aLength), retval] out octet aBuffer); */


}


