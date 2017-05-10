//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMenuBoxObject.idl
//


#[repr(C)]
pub struct nsIMenuBoxObject {
    vtable: *const nsIMenuBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMenuBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x689ebf3d, 0x0184, 0x450a,
            [0x9b, 0xfa, 0x5a, 0x26, 0xbe, 0x0e, 0x7a, 0x8c])
    }
}

unsafe impl RefCounted for nsIMenuBoxObject {
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
pub trait nsIMenuBoxObjectCoerce {
    fn coerce_from(v: &nsIMenuBoxObject) -> &Self;
}

impl nsIMenuBoxObjectCoerce for nsIMenuBoxObject {
    #[inline]
    fn coerce_from(v: &nsIMenuBoxObject) -> &Self {
        v
    }
}

impl nsIMenuBoxObject {
    #[inline]
    pub fn coerce<T: nsIMenuBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMenuBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMenuBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMenuBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMenuBoxObjectVTable {
    pub __base: nsISupportsVTable,

}


impl nsIMenuBoxObject {
}


