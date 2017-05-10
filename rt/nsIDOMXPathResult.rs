//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXPathResult.idl
//


pub mod nsIDOMXPathResult_consts {
    pub const ANY_TYPE: i64 = 0;
    pub const NUMBER_TYPE: i64 = 1;
    pub const STRING_TYPE: i64 = 2;
    pub const BOOLEAN_TYPE: i64 = 3;
    pub const UNORDERED_NODE_ITERATOR_TYPE: i64 = 4;
    pub const ORDERED_NODE_ITERATOR_TYPE: i64 = 5;
    pub const UNORDERED_NODE_SNAPSHOT_TYPE: i64 = 6;
    pub const ORDERED_NODE_SNAPSHOT_TYPE: i64 = 7;
    pub const ANY_UNORDERED_NODE_TYPE: i64 = 8;
    pub const FIRST_ORDERED_NODE_TYPE: i64 = 9;
}


#[repr(C)]
pub struct nsIDOMXPathResult {
    vtable: *const nsIDOMXPathResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXPathResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x75506f84, 0xb504, 0x11d5,
            [0xa7, 0xf2, 0xca, 0x10, 0x8a, 0xb8, 0xb6, 0xfc])
    }
}

unsafe impl RefCounted for nsIDOMXPathResult {
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
pub trait nsIDOMXPathResultCoerce {
    fn coerce_from(v: &nsIDOMXPathResult) -> &Self;
}

impl nsIDOMXPathResultCoerce for nsIDOMXPathResult {
    #[inline]
    fn coerce_from(v: &nsIDOMXPathResult) -> &Self {
        v
    }
}

impl nsIDOMXPathResult {
    #[inline]
    pub fn coerce<T: nsIDOMXPathResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXPathResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXPathResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXPathResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXPathResultVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDOMXPathResult {
}


