//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMBlob.idl
//


#[repr(C)]
pub struct nsIDOMBlob {
    vtable: *const nsIDOMBlobVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMBlob {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf344146a, 0xee1f, 0x417e,
            [0x8a, 0x68, 0x69, 0x84, 0xca, 0x56, 0xf0, 0xae])
    }
}

unsafe impl RefCounted for nsIDOMBlob {
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
pub trait nsIDOMBlobCoerce {
    fn coerce_from(v: &nsIDOMBlob) -> &Self;
}

impl nsIDOMBlobCoerce for nsIDOMBlob {
    #[inline]
    fn coerce_from(v: &nsIDOMBlob) -> &Self {
        v
    }
}

impl nsIDOMBlob {
    #[inline]
    pub fn coerce<T: nsIDOMBlobCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMBlob {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMBlobCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMBlob) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMBlobVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMBlob {
}


