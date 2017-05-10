//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContainerBoxObject.idl
//


#[repr(C)]
pub struct nsIContainerBoxObject {
    vtable: *const nsIContainerBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContainerBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x35d4c04b, 0x3bd3, 0x4375,
            [0x92, 0xe2, 0xa8, 0x18, 0xb4, 0xb4, 0xac, 0xb6])
    }
}

unsafe impl RefCounted for nsIContainerBoxObject {
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
pub trait nsIContainerBoxObjectCoerce {
    fn coerce_from(v: &nsIContainerBoxObject) -> &Self;
}

impl nsIContainerBoxObjectCoerce for nsIContainerBoxObject {
    #[inline]
    fn coerce_from(v: &nsIContainerBoxObject) -> &Self {
        v
    }
}

impl nsIContainerBoxObject {
    #[inline]
    pub fn coerce<T: nsIContainerBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContainerBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContainerBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContainerBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContainerBoxObjectVTable {
    pub __base: nsISupportsVTable,

}


impl nsIContainerBoxObject {
}


