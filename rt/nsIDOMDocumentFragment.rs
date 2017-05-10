//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocumentFragment.idl
//


#[repr(C)]
pub struct nsIDOMDocumentFragment {
    vtable: *const nsIDOMDocumentFragmentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDocumentFragment {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x48eb8d72, 0x95bb, 0x402e,
            [0xa8, 0xfc, 0xf2, 0xb1, 0x87, 0xab, 0xcb, 0xdb])
    }
}

unsafe impl RefCounted for nsIDOMDocumentFragment {
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
pub trait nsIDOMDocumentFragmentCoerce {
    fn coerce_from(v: &nsIDOMDocumentFragment) -> &Self;
}

impl nsIDOMDocumentFragmentCoerce for nsIDOMDocumentFragment {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentFragment) -> &Self {
        v
    }
}

impl nsIDOMDocumentFragment {
    #[inline]
    pub fn coerce<T: nsIDOMDocumentFragmentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDocumentFragment {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMDocumentFragmentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentFragment) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDocumentFragmentVTable {
    pub __base: nsIDOMNodeVTable,

}


impl nsIDOMDocumentFragment {
}


