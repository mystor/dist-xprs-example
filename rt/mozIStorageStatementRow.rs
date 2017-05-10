//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageStatementRow.idl
//


#[repr(C)]
pub struct mozIStorageStatementRow {
    vtable: *const mozIStorageStatementRowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageStatementRow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x02eeaf95, 0xc3db, 0x4182,
            [0x93, 0x40, 0x22, 0x2c, 0x29, 0xf6, 0x8f, 0x02])
    }
}

unsafe impl RefCounted for mozIStorageStatementRow {
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
pub trait mozIStorageStatementRowCoerce {
    fn coerce_from(v: &mozIStorageStatementRow) -> &Self;
}

impl mozIStorageStatementRowCoerce for mozIStorageStatementRow {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementRow) -> &Self {
        v
    }
}

impl mozIStorageStatementRow {
    #[inline]
    pub fn coerce<T: mozIStorageStatementRowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageStatementRow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageStatementRowCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatementRow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageStatementRowVTable {
    pub __base: nsISupportsVTable,

}


impl mozIStorageStatementRow {
}


