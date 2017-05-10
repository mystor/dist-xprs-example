//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefBranchInternal.idl
//


#[repr(C)]
pub struct nsIPrefBranchInternal {
    vtable: *const nsIPrefBranchInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefBranchInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x355bd1e9, 0x248a, 0x438b,
            [0x80, 0x9d, 0xe0, 0xdb, 0x1b, 0x28, 0x78, 0x82])
    }
}

unsafe impl RefCounted for nsIPrefBranchInternal {
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
pub trait nsIPrefBranchInternalCoerce {
    fn coerce_from(v: &nsIPrefBranchInternal) -> &Self;
}

impl nsIPrefBranchInternalCoerce for nsIPrefBranchInternal {
    #[inline]
    fn coerce_from(v: &nsIPrefBranchInternal) -> &Self {
        v
    }
}

impl nsIPrefBranchInternal {
    #[inline]
    pub fn coerce<T: nsIPrefBranchInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefBranchInternal {
    type Target = nsIPrefBranch2;
    #[inline]
    fn deref(&self) -> &nsIPrefBranch2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPrefBranch2Coerce> nsIPrefBranchInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefBranchInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefBranchInternalVTable {
    pub __base: nsIPrefBranch2VTable,

}


impl nsIPrefBranchInternal {
}


