//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMMouseScrollEvent.idl
//


pub mod nsIDOMMouseScrollEvent_consts {
    pub const HORIZONTAL_AXIS: i64 = 1;
    pub const VERTICAL_AXIS: i64 = 2;
}


#[repr(C)]
pub struct nsIDOMMouseScrollEvent {
    vtable: *const nsIDOMMouseScrollEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMMouseScrollEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x327bdd54, 0xf772, 0x4015,
            [0xb8, 0x56, 0x96, 0x92, 0x15, 0x4a, 0x06, 0x6c])
    }
}

unsafe impl RefCounted for nsIDOMMouseScrollEvent {
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
pub trait nsIDOMMouseScrollEventCoerce {
    fn coerce_from(v: &nsIDOMMouseScrollEvent) -> &Self;
}

impl nsIDOMMouseScrollEventCoerce for nsIDOMMouseScrollEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMMouseScrollEvent) -> &Self {
        v
    }
}

impl nsIDOMMouseScrollEvent {
    #[inline]
    pub fn coerce<T: nsIDOMMouseScrollEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMMouseScrollEvent {
    type Target = nsIDOMMouseEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMMouseEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMMouseEventCoerce> nsIDOMMouseScrollEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMouseScrollEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMMouseScrollEventVTable {
    pub __base: nsIDOMMouseEventVTable,

}


impl nsIDOMMouseScrollEvent {
}


