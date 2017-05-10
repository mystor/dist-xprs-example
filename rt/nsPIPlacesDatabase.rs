//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIPlacesDatabase.idl
//


#[repr(C)]
pub struct nsPIPlacesDatabase {
    vtable: *const nsPIPlacesDatabaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIPlacesDatabase {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x366ee63e, 0xa413, 0x477d,
            [0x9a, 0xd6, 0x8d, 0x68, 0x63, 0xe8, 0x94, 0x01])
    }
}

unsafe impl RefCounted for nsPIPlacesDatabase {
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
pub trait nsPIPlacesDatabaseCoerce {
    fn coerce_from(v: &nsPIPlacesDatabase) -> &Self;
}

impl nsPIPlacesDatabaseCoerce for nsPIPlacesDatabase {
    #[inline]
    fn coerce_from(v: &nsPIPlacesDatabase) -> &Self {
        v
    }
}

impl nsPIPlacesDatabase {
    #[inline]
    pub fn coerce<T: nsPIPlacesDatabaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIPlacesDatabase {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIPlacesDatabaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIPlacesDatabase) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIPlacesDatabaseVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute mozIStorageConnection DBConnection; */
    pub get_DBConnection: unsafe extern "C" fn (this: *const nsPIPlacesDatabase, aDBConnection: *mut *const mozIStorageConnection) -> nsresult,

    /* mozIStoragePendingStatement asyncExecuteLegacyQueries ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub asyncExecuteLegacyQueries: *const ::libc::c_void,

    /* readonly attribute nsIAsyncShutdownClient shutdownClient; */
    pub get_shutdownClient: unsafe extern "C" fn (this: *const nsPIPlacesDatabase, aShutdownClient: *mut *const nsIAsyncShutdownClient) -> nsresult,

}


impl nsPIPlacesDatabase {
    /* readonly attribute mozIStorageConnection DBConnection; */
    #[inline]
    pub unsafe fn get_DBConnection(&self, ) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DBConnection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStoragePendingStatement asyncExecuteLegacyQueries ([array, size_is (aQueryCount)] in nsINavHistoryQuery aQueries, in unsigned long aQueryCount, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback); */


    /* readonly attribute nsIAsyncShutdownClient shutdownClient; */
    #[inline]
    pub unsafe fn get_shutdownClient(&self, ) -> Result<Option<RefPtr<nsIAsyncShutdownClient>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_shutdownClient)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


