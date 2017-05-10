//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNSEvent.idl
//


pub mod nsIDOMNSEvent_consts {
    pub const ALT_MASK: i64 = 1;
    pub const CONTROL_MASK: i64 = 2;
    pub const SHIFT_MASK: i64 = 4;
    pub const META_MASK: i64 = 8;
}


#[repr(C)]
pub struct nsIDOMNSEvent {
    vtable: *const nsIDOMNSEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNSEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2580b4a2, 0x6d85, 0x4ca6,
            [0x9b, 0xe2, 0x98, 0xf3, 0x40, 0x6a, 0xd2, 0x96])
    }
}

unsafe impl RefCounted for nsIDOMNSEvent {
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
pub trait nsIDOMNSEventCoerce {
    fn coerce_from(v: &nsIDOMNSEvent) -> &Self;
}

impl nsIDOMNSEventCoerce for nsIDOMNSEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMNSEvent) -> &Self {
        v
    }
}

impl nsIDOMNSEvent {
    #[inline]
    pub fn coerce<T: nsIDOMNSEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNSEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNSEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNSEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNSEventVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMNSEvent {
}


