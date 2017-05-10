//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMComment.idl
//


#[repr(C)]
pub struct nsIDOMComment {
    vtable: *const nsIDOMCommentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMComment {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe7866ff8, 0xb7fc, 0x494f,
            [0x87, 0xc0, 0xfb, 0x01, 0x7d, 0x8a, 0x4d, 0x30])
    }
}

unsafe impl RefCounted for nsIDOMComment {
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
pub trait nsIDOMCommentCoerce {
    fn coerce_from(v: &nsIDOMComment) -> &Self;
}

impl nsIDOMCommentCoerce for nsIDOMComment {
    #[inline]
    fn coerce_from(v: &nsIDOMComment) -> &Self {
        v
    }
}

impl nsIDOMComment {
    #[inline]
    pub fn coerce<T: nsIDOMCommentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMComment {
    type Target = nsIDOMCharacterData;
    #[inline]
    fn deref(&self) -> &nsIDOMCharacterData {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCharacterDataCoerce> nsIDOMCommentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMComment) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCommentVTable {
    pub __base: nsIDOMCharacterDataVTable,

}


impl nsIDOMComment {
}


