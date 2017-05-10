//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageResultSet.idl
//


#[repr(C)]
pub struct mozIStorageResultSet {
    vtable: *const mozIStorageResultSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageResultSet {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x18dd7953, 0x076d, 0x4598,
            [0x81, 0x05, 0x3e, 0x32, 0xad, 0x26, 0xab, 0x24])
    }
}

unsafe impl RefCounted for mozIStorageResultSet {
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
pub trait mozIStorageResultSetCoerce {
    fn coerce_from(v: &mozIStorageResultSet) -> &Self;
}

impl mozIStorageResultSetCoerce for mozIStorageResultSet {
    #[inline]
    fn coerce_from(v: &mozIStorageResultSet) -> &Self {
        v
    }
}

impl mozIStorageResultSet {
    #[inline]
    pub fn coerce<T: mozIStorageResultSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageResultSet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageResultSetCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageResultSet) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageResultSetVTable {
    pub __base: nsISupportsVTable,

    /* mozIStorageRow getNextRow (); */
    pub getNextRow: unsafe extern "C" fn (this: *const mozIStorageResultSet, _retval: *mut *const mozIStorageRow) -> nsresult,

}


impl mozIStorageResultSet {
    /* mozIStorageRow getNextRow (); */
    #[inline]
    pub unsafe fn getNextRow(&self, ) -> Result<Option<RefPtr<mozIStorageRow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNextRow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


