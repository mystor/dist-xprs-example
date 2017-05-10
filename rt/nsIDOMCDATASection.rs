//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCDATASection.idl
//


#[repr(C)]
pub struct nsIDOMCDATASection {
    vtable: *const nsIDOMCDATASectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCDATASection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe14ef131, 0x34cc, 0x40c8,
            [0x9c, 0x99, 0xa4, 0x03, 0xc0, 0x01, 0x18, 0x4a])
    }
}

unsafe impl RefCounted for nsIDOMCDATASection {
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
pub trait nsIDOMCDATASectionCoerce {
    fn coerce_from(v: &nsIDOMCDATASection) -> &Self;
}

impl nsIDOMCDATASectionCoerce for nsIDOMCDATASection {
    #[inline]
    fn coerce_from(v: &nsIDOMCDATASection) -> &Self {
        v
    }
}

impl nsIDOMCDATASection {
    #[inline]
    pub fn coerce<T: nsIDOMCDATASectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCDATASection {
    type Target = nsIDOMText;
    #[inline]
    fn deref(&self) -> &nsIDOMText {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMTextCoerce> nsIDOMCDATASectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCDATASection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCDATASectionVTable {
    pub __base: nsIDOMTextVTable,

}


impl nsIDOMCDATASection {
}


