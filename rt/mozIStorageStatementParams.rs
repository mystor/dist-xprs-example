//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageStatementParams.idl
//


#[repr(C)]
pub struct mozIStorageStatementParams {
    vtable: *const mozIStorageStatementParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageStatementParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe65fe6e2, 0x2643, 0x463c,
            [0x97, 0xe2, 0x27, 0x66, 0x5e, 0xfe, 0x23, 0x86])
    }
}

unsafe impl RefCounted for mozIStorageStatementParams {
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
pub trait mozIStorageStatementParamsCoerce {
    fn coerce_from(v: &mozIStorageStatementParams) -> &Self;
}

impl mozIStorageStatementParamsCoerce for mozIStorageStatementParams {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementParams) -> &Self {
        v
    }
}

impl mozIStorageStatementParams {
    #[inline]
    pub fn coerce<T: mozIStorageStatementParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageStatementParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageStatementParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageStatementParamsVTable {
    pub __base: nsISupportsVTable,

}


impl mozIStorageStatementParams {
}


