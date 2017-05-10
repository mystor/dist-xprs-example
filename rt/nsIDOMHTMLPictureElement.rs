//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLPictureElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLPictureElement {
    vtable: *const nsIDOMHTMLPictureElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLPictureElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0e5ac7f, 0xb969, 0x494c,
            [0xa6, 0x1e, 0x9d, 0x74, 0x0e, 0x38, 0xab, 0xba])
    }
}

unsafe impl RefCounted for nsIDOMHTMLPictureElement {
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
pub trait nsIDOMHTMLPictureElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLPictureElement) -> &Self;
}

impl nsIDOMHTMLPictureElementCoerce for nsIDOMHTMLPictureElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLPictureElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLPictureElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLPictureElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLPictureElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLPictureElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLPictureElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLPictureElementVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMHTMLPictureElement {
}


