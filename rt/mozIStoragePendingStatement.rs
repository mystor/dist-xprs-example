//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStoragePendingStatement.idl
//


#[repr(C)]
pub struct mozIStoragePendingStatement {
    vtable: *const mozIStoragePendingStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStoragePendingStatement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x00da7d20, 0x3768, 0x4398,
            [0xbe, 0xdc, 0xe3, 0x10, 0xc3, 0x24, 0xb3, 0xf0])
    }
}

unsafe impl RefCounted for mozIStoragePendingStatement {
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
pub trait mozIStoragePendingStatementCoerce {
    fn coerce_from(v: &mozIStoragePendingStatement) -> &Self;
}

impl mozIStoragePendingStatementCoerce for mozIStoragePendingStatement {
    #[inline]
    fn coerce_from(v: &mozIStoragePendingStatement) -> &Self {
        v
    }
}

impl mozIStoragePendingStatement {
    #[inline]
    pub fn coerce<T: mozIStoragePendingStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStoragePendingStatement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStoragePendingStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStoragePendingStatement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStoragePendingStatementVTable {
    pub __base: nsISupportsVTable,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const mozIStoragePendingStatement) -> nsresult,

}


impl mozIStoragePendingStatement {
    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


