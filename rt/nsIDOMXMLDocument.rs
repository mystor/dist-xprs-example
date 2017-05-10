//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXMLDocument.idl
//


#[repr(C)]
pub struct nsIDOMXMLDocument {
    vtable: *const nsIDOMXMLDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXMLDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x89ab39cb, 0xc568, 0x4d85,
            [0xbd, 0x34, 0x30, 0x6d, 0x5c, 0xd5, 0x16, 0x4d])
    }
}

unsafe impl RefCounted for nsIDOMXMLDocument {
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
pub trait nsIDOMXMLDocumentCoerce {
    fn coerce_from(v: &nsIDOMXMLDocument) -> &Self;
}

impl nsIDOMXMLDocumentCoerce for nsIDOMXMLDocument {
    #[inline]
    fn coerce_from(v: &nsIDOMXMLDocument) -> &Self {
        v
    }
}

impl nsIDOMXMLDocument {
    #[inline]
    pub fn coerce<T: nsIDOMXMLDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXMLDocument {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXMLDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXMLDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXMLDocumentVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMXMLDocument {
}


