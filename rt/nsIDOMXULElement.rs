//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULElement.idl
//


#[repr(C)]
pub struct nsIDOMXULElement {
    vtable: *const nsIDOMXULElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x75435ab3, 0x6863, 0x42a1,
            [0xad, 0xe3, 0x02, 0x53, 0x93, 0xd9, 0xe8, 0x0e])
    }
}

unsafe impl RefCounted for nsIDOMXULElement {
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
pub trait nsIDOMXULElementCoerce {
    fn coerce_from(v: &nsIDOMXULElement) -> &Self;
}

impl nsIDOMXULElementCoerce for nsIDOMXULElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULElement) -> &Self {
        v
    }
}

impl nsIDOMXULElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULElement {
    type Target = nsIDOMElement;
    #[inline]
    fn deref(&self) -> &nsIDOMElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMElementCoerce> nsIDOMXULElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULElementVTable {
    pub __base: nsIDOMElementVTable,

}


impl nsIDOMXULElement {
}


