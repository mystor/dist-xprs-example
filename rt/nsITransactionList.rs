//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransactionList.idl
//


#[repr(C)]
pub struct nsITransactionList {
    vtable: *const nsITransactionListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransactionList {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd007ceff, 0xc978, 0x486a,
            [0xb6, 0x97, 0x38, 0x4c, 0xa0, 0x19, 0x97, 0xbe])
    }
}

unsafe impl RefCounted for nsITransactionList {
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
pub trait nsITransactionListCoerce {
    fn coerce_from(v: &nsITransactionList) -> &Self;
}

impl nsITransactionListCoerce for nsITransactionList {
    #[inline]
    fn coerce_from(v: &nsITransactionList) -> &Self {
        v
    }
}

impl nsITransactionList {
    #[inline]
    pub fn coerce<T: nsITransactionListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransactionList {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransactionListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransactionList) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransactionListVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long numItems; */
    pub get_numItems: unsafe extern "C" fn (this: *const nsITransactionList, aNumItems: *mut libc::int32_t) -> nsresult,

    /* boolean itemIsBatch (in long aIndex); */
    pub itemIsBatch: unsafe extern "C" fn (this: *const nsITransactionList, aIndex: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* nsITransaction getItem (in long aIndex); */
    pub getItem: unsafe extern "C" fn (this: *const nsITransactionList, aIndex: libc::int32_t, _retval: *mut *const nsITransaction) -> nsresult,

    /* void getData (in long aIndex, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out nsISupports aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub getData: *const ::libc::c_void,

    /* long getNumChildrenForItem (in long aIndex); */
    pub getNumChildrenForItem: unsafe extern "C" fn (this: *const nsITransactionList, aIndex: libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* nsITransactionList getChildListForItem (in long aIndex); */
    pub getChildListForItem: unsafe extern "C" fn (this: *const nsITransactionList, aIndex: libc::int32_t, _retval: *mut *const nsITransactionList) -> nsresult,

}


impl nsITransactionList {
    /* readonly attribute long numItems; */
    #[inline]
    pub unsafe fn get_numItems(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean itemIsBatch (in long aIndex); */
    #[inline]
    pub unsafe fn itemIsBatch(&self, aIndex: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).itemIsBatch)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsITransaction getItem (in long aIndex); */
    #[inline]
    pub unsafe fn getItem(&self, aIndex: libc::int32_t) -> Result<Option<RefPtr<nsITransaction>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getItem)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getData (in long aIndex, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out nsISupports aData); */


    /* long getNumChildrenForItem (in long aIndex); */
    #[inline]
    pub unsafe fn getNumChildrenForItem(&self, aIndex: libc::int32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getNumChildrenForItem)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsITransactionList getChildListForItem (in long aIndex); */
    #[inline]
    pub unsafe fn getChildListForItem(&self, aIndex: libc::int32_t) -> Result<Option<RefPtr<nsITransactionList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildListForItem)(self as *const _, aIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


