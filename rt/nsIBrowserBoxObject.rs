//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserBoxObject.idl
//


#[repr(C)]
pub struct nsIBrowserBoxObject {
    vtable: *const nsIBrowserBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdb436f2f, 0xc656, 0x4754,
            [0xb0, 0xfa, 0x99, 0xbc, 0x35, 0x3b, 0xd6, 0x3f])
    }
}

unsafe impl RefCounted for nsIBrowserBoxObject {
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
pub trait nsIBrowserBoxObjectCoerce {
    fn coerce_from(v: &nsIBrowserBoxObject) -> &Self;
}

impl nsIBrowserBoxObjectCoerce for nsIBrowserBoxObject {
    #[inline]
    fn coerce_from(v: &nsIBrowserBoxObject) -> &Self {
        v
    }
}

impl nsIBrowserBoxObject {
    #[inline]
    pub fn coerce<T: nsIBrowserBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserBoxObject {
    type Target = nsIContainerBoxObject;
    #[inline]
    fn deref(&self) -> &nsIContainerBoxObject {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIContainerBoxObjectCoerce> nsIBrowserBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserBoxObjectVTable {
    pub __base: nsIContainerBoxObjectVTable,

}


impl nsIBrowserBoxObject {
}


