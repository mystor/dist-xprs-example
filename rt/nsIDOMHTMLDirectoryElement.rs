//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLDirectoryElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLDirectoryElement {
    vtable: *const nsIDOMHTMLDirectoryElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLDirectoryElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8cfff7a4, 0x8b14, 0x4ce0,
            [0x97, 0xb0, 0xba, 0xbe, 0x78, 0xda, 0x16, 0xf8])
    }
}

unsafe impl RefCounted for nsIDOMHTMLDirectoryElement {
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
pub trait nsIDOMHTMLDirectoryElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLDirectoryElement) -> &Self;
}

impl nsIDOMHTMLDirectoryElementCoerce for nsIDOMHTMLDirectoryElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLDirectoryElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLDirectoryElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLDirectoryElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLDirectoryElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLDirectoryElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLDirectoryElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLDirectoryElementVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMHTMLDirectoryElement {
}


