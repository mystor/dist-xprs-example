//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWindow.idl
//


#[repr(C)]
pub struct nsIDOMWindow {
    vtable: *const nsIDOMWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8343993, 0x0383, 0x4add,
            [0x99, 0x30, 0xad, 0x17, 0x6b, 0x18, 0x92, 0x40])
    }
}

unsafe impl RefCounted for nsIDOMWindow {
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
pub trait nsIDOMWindowCoerce {
    fn coerce_from(v: &nsIDOMWindow) -> &Self;
}

impl nsIDOMWindowCoerce for nsIDOMWindow {
    #[inline]
    fn coerce_from(v: &nsIDOMWindow) -> &Self {
        v
    }
}

impl nsIDOMWindow {
    #[inline]
    pub fn coerce<T: nsIDOMWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWindowVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMWindow {
}


#[repr(C)]
pub struct nsIDOMWindowInternal {
    vtable: *const nsIDOMWindowInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWindowInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8c589e65, 0x3237, 0x4cd1,
            [0x8b, 0xad, 0xc5, 0xc4, 0x71, 0x35, 0xe7, 0x9b])
    }
}

unsafe impl RefCounted for nsIDOMWindowInternal {
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
pub trait nsIDOMWindowInternalCoerce {
    fn coerce_from(v: &nsIDOMWindowInternal) -> &Self;
}

impl nsIDOMWindowInternalCoerce for nsIDOMWindowInternal {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowInternal) -> &Self {
        v
    }
}

impl nsIDOMWindowInternal {
    #[inline]
    pub fn coerce<T: nsIDOMWindowInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWindowInternal {
    type Target = nsIDOMWindow;
    #[inline]
    fn deref(&self) -> &nsIDOMWindow {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMWindowCoerce> nsIDOMWindowInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWindowInternalVTable {
    pub __base: nsIDOMWindowVTable,

}


impl nsIDOMWindowInternal {
}


