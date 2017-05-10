//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMStorage.idl
//


#[repr(C)]
pub struct nsIDOMStorage {
    vtable: *const nsIDOMStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMStorage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x425a33f0, 0xe0e9, 0x45e7,
            [0xa9, 0x5f, 0x99, 0x08, 0xbd, 0x6a, 0xe9, 0x88])
    }
}

unsafe impl RefCounted for nsIDOMStorage {
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
pub trait nsIDOMStorageCoerce {
    fn coerce_from(v: &nsIDOMStorage) -> &Self;
}

impl nsIDOMStorageCoerce for nsIDOMStorage {
    #[inline]
    fn coerce_from(v: &nsIDOMStorage) -> &Self {
        v
    }
}

impl nsIDOMStorage {
    #[inline]
    pub fn coerce<T: nsIDOMStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMStorage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMStorage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMStorageVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMStorage {
}


