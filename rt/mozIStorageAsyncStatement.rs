//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageAsyncStatement.idl
//


#[repr(C)]
pub struct mozIStorageAsyncStatement {
    vtable: *const mozIStorageAsyncStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageAsyncStatement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x52e49370, 0x3b2e, 0x4a27,
            [0xa3, 0xfc, 0x79, 0xe2, 0x0a, 0xd4, 0x05, 0x6b])
    }
}

unsafe impl RefCounted for mozIStorageAsyncStatement {
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
pub trait mozIStorageAsyncStatementCoerce {
    fn coerce_from(v: &mozIStorageAsyncStatement) -> &Self;
}

impl mozIStorageAsyncStatementCoerce for mozIStorageAsyncStatement {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncStatement) -> &Self {
        v
    }
}

impl mozIStorageAsyncStatement {
    #[inline]
    pub fn coerce<T: mozIStorageAsyncStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageAsyncStatement {
    type Target = mozIStorageBaseStatement;
    #[inline]
    fn deref(&self) -> &mozIStorageBaseStatement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozIStorageBaseStatementCoerce> mozIStorageAsyncStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncStatement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageAsyncStatementVTable {
    pub __base: mozIStorageBaseStatementVTable,

}


impl mozIStorageAsyncStatement {
}


