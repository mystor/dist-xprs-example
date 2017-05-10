//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLHeadElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLHeadElement {
    vtable: *const nsIDOMHTMLHeadElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLHeadElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x59b80014, 0x00f5, 0x412d,
            [0x84, 0x6f, 0x72, 0x54, 0x94, 0x12, 0x2d, 0x42])
    }
}

unsafe impl RefCounted for nsIDOMHTMLHeadElement {
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
pub trait nsIDOMHTMLHeadElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLHeadElement) -> &Self;
}

impl nsIDOMHTMLHeadElementCoerce for nsIDOMHTMLHeadElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHeadElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLHeadElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLHeadElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLHeadElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLHeadElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHeadElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLHeadElementVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMHTMLHeadElement {
}


