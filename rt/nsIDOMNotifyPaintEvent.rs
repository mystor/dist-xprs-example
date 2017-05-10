//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNotifyPaintEvent.idl
//


#[repr(C)]
pub struct nsIDOMNotifyPaintEvent {
    vtable: *const nsIDOMNotifyPaintEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNotifyPaintEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x63f573a0, 0x3e4e, 0x474b,
            [0xa0, 0xc2, 0xbb, 0x4c, 0xa9, 0x3f, 0xeb, 0xaa])
    }
}

unsafe impl RefCounted for nsIDOMNotifyPaintEvent {
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
pub trait nsIDOMNotifyPaintEventCoerce {
    fn coerce_from(v: &nsIDOMNotifyPaintEvent) -> &Self;
}

impl nsIDOMNotifyPaintEventCoerce for nsIDOMNotifyPaintEvent {
    #[inline]
    fn coerce_from(v: &nsIDOMNotifyPaintEvent) -> &Self {
        v
    }
}

impl nsIDOMNotifyPaintEvent {
    #[inline]
    pub fn coerce<T: nsIDOMNotifyPaintEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNotifyPaintEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNotifyPaintEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNotifyPaintEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNotifyPaintEventVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMNotifyPaintEvent {
}


