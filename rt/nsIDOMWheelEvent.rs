//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMWheelEvent.idl
//


pub mod nsIDOMWheelEvent_consts {
    pub const DOM_DELTA_PIXEL: i64 = 0;
    pub const DOM_DELTA_LINE: i64 = 1;
    pub const DOM_DELTA_PAGE: i64 = 2;
}


#[repr(C)]
pub struct nsIDOMWheelEvent {
    vtable: *const nsIDOMWheelEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMWheelEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd320d075, 0xb29d, 0x4edd,
            [0xb3, 0xa3, 0x3f, 0xfd, 0x46, 0x06, 0x40, 0xde])
    }
}

unsafe impl RefCounted for nsIDOMWheelEvent {
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
pub trait nsIDOMWheelEventCoerce {
    fn coerce_from(v: &nsIDOMWheelEvent) -> &Self;
}

impl nsIDOMWheelEventCoerce for nsIDOMWheelEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMWheelEvent) -> &Self {
        v
    }
}

impl nsIDOMWheelEvent {
    #[inline]
    pub fn coerce<T: nsIDOMWheelEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMWheelEvent {
    type Target = nsIDOMMouseEvent;
    #[inline]
    fn deref(&self) -> &nsIDOMMouseEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMMouseEventCoerce> nsIDOMWheelEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWheelEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMWheelEventVTable {
    pub __base: nsIDOMMouseEventVTable,

}


impl nsIDOMWheelEvent {
}


