//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXSLTProcessorPrivate.idl
//


pub mod nsIXSLTProcessorPrivate_consts {
    pub const DISABLE_ALL_LOADS: i64 = 1;
}


#[repr(C)]
pub struct nsIXSLTProcessorPrivate {
    vtable: *const nsIXSLTProcessorPrivateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXSLTProcessorPrivate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8d727f7, 0x67f4, 0x4dc1,
            [0xa3, 0x18, 0xec, 0x0c, 0x87, 0x28, 0x08, 0x16])
    }
}

unsafe impl RefCounted for nsIXSLTProcessorPrivate {
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
pub trait nsIXSLTProcessorPrivateCoerce {
    fn coerce_from(v: &nsIXSLTProcessorPrivate) -> &Self;
}

impl nsIXSLTProcessorPrivateCoerce for nsIXSLTProcessorPrivate {
    #[inline]
    fn coerce_from(v: &nsIXSLTProcessorPrivate) -> &Self {
        v
    }
}

impl nsIXSLTProcessorPrivate {
    #[inline]
    pub fn coerce<T: nsIXSLTProcessorPrivateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXSLTProcessorPrivate {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXSLTProcessorPrivateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXSLTProcessorPrivate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXSLTProcessorPrivateVTable {
    pub __base: nsISupportsVTable,

}


impl nsIXSLTProcessorPrivate {
}


