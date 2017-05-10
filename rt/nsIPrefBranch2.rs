//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrefBranch2.idl
//


#[repr(C)]
pub struct nsIPrefBranch2 {
    vtable: *const nsIPrefBranch2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrefBranch2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8892016d, 0x07f7, 0x4530,
            [0xb5, 0xc1, 0xd7, 0x3d, 0xfc, 0xde, 0x4a, 0x1c])
    }
}

unsafe impl RefCounted for nsIPrefBranch2 {
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
pub trait nsIPrefBranch2Coerce {
    fn coerce_from(v: &nsIPrefBranch2) -> &Self;
}

impl nsIPrefBranch2Coerce for nsIPrefBranch2 {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch2) -> &Self {
        v
    }
}

impl nsIPrefBranch2 {
    #[inline]
    pub fn coerce<T: nsIPrefBranch2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrefBranch2 {
    type Target = nsIPrefBranch;
    #[inline]
    fn deref(&self) -> &nsIPrefBranch {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPrefBranchCoerce> nsIPrefBranch2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefBranch2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrefBranch2VTable {
    pub __base: nsIPrefBranchVTable,

}


impl nsIPrefBranch2 {
}


