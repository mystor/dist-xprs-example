//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIDOMWindow.idl
//


#[repr(C)]
pub struct mozIDOMWindow {
    vtable: *const mozIDOMWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIDOMWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x75fbabd6, 0x7a2e, 0x4787,
            [0xaa, 0x33, 0x44, 0x9a, 0x33, 0x51, 0x21, 0x35])
    }
}

unsafe impl RefCounted for mozIDOMWindow {
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
pub trait mozIDOMWindowCoerce {
    fn coerce_from(v: &mozIDOMWindow) -> &Self;
}

impl mozIDOMWindowCoerce for mozIDOMWindow {
    #[inline]
    fn coerce_from(v: &mozIDOMWindow) -> &Self {
        v
    }
}

impl mozIDOMWindow {
    #[inline]
    pub fn coerce<T: mozIDOMWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIDOMWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIDOMWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDOMWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIDOMWindowVTable {
    pub __base: nsISupportsVTable,

}


impl mozIDOMWindow {
}


#[repr(C)]
pub struct mozIDOMWindowProxy {
    vtable: *const mozIDOMWindowProxyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIDOMWindowProxy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x53ca090c, 0xe739, 0x48b9,
            [0x89, 0x11, 0x20, 0x8c, 0x72, 0xf9, 0x19, 0x1e])
    }
}

unsafe impl RefCounted for mozIDOMWindowProxy {
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
pub trait mozIDOMWindowProxyCoerce {
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self;
}

impl mozIDOMWindowProxyCoerce for mozIDOMWindowProxy {
    #[inline]
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self {
        v
    }
}

impl mozIDOMWindowProxy {
    #[inline]
    pub fn coerce<T: mozIDOMWindowProxyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIDOMWindowProxy {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIDOMWindowProxyCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDOMWindowProxy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIDOMWindowProxyVTable {
    pub __base: nsISupportsVTable,

}


impl mozIDOMWindowProxy {
}


