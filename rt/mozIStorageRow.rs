//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageRow.idl
//


#[repr(C)]
pub struct mozIStorageRow {
    vtable: *const mozIStorageRowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageRow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62d1b6bd, 0xcbfe, 0x4f9b,
            [0xae, 0xe1, 0x0e, 0xad, 0x4a, 0xf4, 0xe6, 0xdc])
    }
}

unsafe impl RefCounted for mozIStorageRow {
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
pub trait mozIStorageRowCoerce {
    fn coerce_from(v: &mozIStorageRow) -> &Self;
}

impl mozIStorageRowCoerce for mozIStorageRow {
    #[inline]
    fn coerce_from(v: &mozIStorageRow) -> &Self {
        v
    }
}

impl mozIStorageRow {
    #[inline]
    pub fn coerce<T: mozIStorageRowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageRow {
    type Target = mozIStorageValueArray;
    #[inline]
    fn deref(&self) -> &mozIStorageValueArray {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozIStorageValueArrayCoerce> mozIStorageRowCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageRow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageRowVTable {
    pub __base: mozIStorageValueArrayVTable,

    /* nsIVariant getResultByIndex (in unsigned long aIndex); */
    pub getResultByIndex: unsafe extern "C" fn (this: *const mozIStorageRow, aIndex: libc::uint32_t, _retval: *mut *const nsIVariant) -> nsresult,

    /* nsIVariant getResultByName (in AUTF8String aName); */
    pub getResultByName: unsafe extern "C" fn (this: *const mozIStorageRow, aName: *const nsACString, _retval: *mut *const nsIVariant) -> nsresult,

}


impl mozIStorageRow {
    /* nsIVariant getResultByIndex (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getResultByIndex(&self, aIndex: libc::uint32_t) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getResultByIndex)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIVariant getResultByName (in AUTF8String aName); */
    #[inline]
    pub unsafe fn getResultByName(&self, aName: &[u8]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getResultByName)(self as *const _, &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


