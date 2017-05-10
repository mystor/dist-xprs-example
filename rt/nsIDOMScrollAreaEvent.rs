//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMScrollAreaEvent.idl
//


#[repr(C)]
pub struct nsIDOMScrollAreaEvent {
    vtable: *const nsIDOMScrollAreaEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMScrollAreaEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5883e564, 0xe676, 0x4652,
            [0x94, 0x21, 0x7d, 0xf6, 0x13, 0x20, 0x16, 0xb2])
    }
}

unsafe impl RefCounted for nsIDOMScrollAreaEvent {
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
pub trait nsIDOMScrollAreaEventCoerce {
    fn coerce_from(v: &nsIDOMScrollAreaEvent) -> &Self;
}

impl nsIDOMScrollAreaEventCoerce for nsIDOMScrollAreaEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMScrollAreaEvent) -> &Self {
        v
    }
}

impl nsIDOMScrollAreaEvent {
    #[inline]
    pub fn coerce<T: nsIDOMScrollAreaEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMScrollAreaEvent {
    type Target = nsIDOMUIEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMUIEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMUIEventCoerce> nsIDOMScrollAreaEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMScrollAreaEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMScrollAreaEventVTable {
    pub __base: nsIDOMUIEventVTable,

}


impl nsIDOMScrollAreaEvent {
}


