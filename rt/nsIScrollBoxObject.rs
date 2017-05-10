//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScrollBoxObject.idl
//


#[repr(C)]
pub struct nsIScrollBoxObject {
    vtable: *const nsIScrollBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScrollBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x56e2ada8, 0x4631, 0x11d4,
            [0xba, 0x11, 0x00, 0x10, 0x83, 0x02, 0x3c, 0x1e])
    }
}

unsafe impl RefCounted for nsIScrollBoxObject {
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
pub trait nsIScrollBoxObjectCoerce {
    fn coerce_from(v: &nsIScrollBoxObject) -> &Self;
}

impl nsIScrollBoxObjectCoerce for nsIScrollBoxObject {
    #[inline]
    fn coerce_from(v: &nsIScrollBoxObject) -> &Self {
        v
    }
}

impl nsIScrollBoxObject {
    #[inline]
    pub fn coerce<T: nsIScrollBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScrollBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScrollBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScrollBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScrollBoxObjectVTable {
    pub __base: nsISupportsVTable,

}


impl nsIScrollBoxObject {
}


