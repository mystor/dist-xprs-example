//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageAsyncConnection.idl
//


#[repr(C)]
pub struct mozIStorageAsyncConnection {
    vtable: *const mozIStorageAsyncConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageAsyncConnection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8bfd34d5, 0x4ddf, 0x4e4b,
            [0x89, 0xdd, 0x9b, 0x14, 0xf3, 0x35, 0x34, 0xc6])
    }
}

unsafe impl RefCounted for mozIStorageAsyncConnection {
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
pub trait mozIStorageAsyncConnectionCoerce {
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self;
}

impl mozIStorageAsyncConnectionCoerce for mozIStorageAsyncConnection {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self {
        v
    }
}

impl mozIStorageAsyncConnection {
    #[inline]
    pub fn coerce<T: mozIStorageAsyncConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageAsyncConnection {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageAsyncConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageAsyncConnectionVTable {
    pub __base: nsISupportsVTable,

    /* void asyncClose ([optional] in mozIStorageCompletionCallback aCallback); */
    pub asyncClose: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aCallback: *const mozIStorageCompletionCallback) -> nsresult,

    /* void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback); */
    pub asyncClone: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aReadOnly: bool, aCallback: *const mozIStorageCompletionCallback) -> nsresult,

    /* readonly attribute nsIFile databaseFile; */
    pub get_databaseFile: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aDatabaseFile: *mut *const nsIFile) -> nsresult,

    /* mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement); */
    pub createAsyncStatement: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aSQLStatement: *const nsACString, _retval: *mut *const mozIStorageAsyncStatement) -> nsresult,

    /* mozIStoragePendingStatement executeAsync ([array, size_is (aNumStatements)] in mozIStorageBaseStatement aStatements, in unsigned long aNumStatements, [optional] in mozIStorageStatementCallback aCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub executeAsync: *const ::libc::c_void,

    /* mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback); */
    pub executeSimpleSQLAsync: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aSQLStatement: *const nsACString, aCallback: *const mozIStorageStatementCallback, _retval: *mut *const mozIStoragePendingStatement) -> nsresult,

    /* void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction); */
    pub createFunction: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aFunctionName: *const nsACString, aNumArguments: libc::int32_t, aFunction: *const mozIStorageFunction) -> nsresult,

    /* void createAggregateFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageAggregateFunction aFunction); */
    pub createAggregateFunction: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aFunctionName: *const nsACString, aNumArguments: libc::int32_t, aFunction: *const mozIStorageAggregateFunction) -> nsresult,

    /* void removeFunction (in AUTF8String aFunctionName); */
    pub removeFunction: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aFunctionName: *const nsACString) -> nsresult,

    /* mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler); */
    pub setProgressHandler: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, aGranularity: int32_t, aHandler: *const mozIStorageProgressHandler, _retval: *mut *const mozIStorageProgressHandler) -> nsresult,

    /* mozIStorageProgressHandler removeProgressHandler (); */
    pub removeProgressHandler: unsafe extern "C" fn (this: *const mozIStorageAsyncConnection, _retval: *mut *const mozIStorageProgressHandler) -> nsresult,

}


impl mozIStorageAsyncConnection {
    /* void asyncClose ([optional] in mozIStorageCompletionCallback aCallback); */
    #[inline]
    pub unsafe fn asyncClose(&self, aCallback: Option<&mozIStorageCompletionCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncClose)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback); */
    #[inline]
    pub unsafe fn asyncClone(&self, aReadOnly: bool, aCallback: Option<&mozIStorageCompletionCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncClone)(self as *const _, aReadOnly, aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile databaseFile; */
    #[inline]
    pub unsafe fn get_databaseFile(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_databaseFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement); */
    #[inline]
    pub unsafe fn createAsyncStatement(&self, aSQLStatement: &[u8]) -> Result<Option<RefPtr<mozIStorageAsyncStatement>>, nsresult> {
        let aSQLStatement = nsCString::from(aSQLStatement);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAsyncStatement)(self as *const _, &*aSQLStatement, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStoragePendingStatement executeAsync ([array, size_is (aNumStatements)] in mozIStorageBaseStatement aStatements, in unsigned long aNumStatements, [optional] in mozIStorageStatementCallback aCallback); */


    /* mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback); */
    #[inline]
    pub unsafe fn executeSimpleSQLAsync(&self, aSQLStatement: &[u8], aCallback: Option<&mozIStorageStatementCallback>) -> Result<Option<RefPtr<mozIStoragePendingStatement>>, nsresult> {
        let aSQLStatement = nsCString::from(aSQLStatement);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).executeSimpleSQLAsync)(self as *const _, &*aSQLStatement, aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction); */
    #[inline]
    pub unsafe fn createFunction(&self, aFunctionName: &[u8], aNumArguments: libc::int32_t, aFunction: Option<&mozIStorageFunction>) -> Result<(), nsresult> {
        let aFunctionName = nsCString::from(aFunctionName);
        match ((*self.vtable).createFunction)(self as *const _, &*aFunctionName, aNumArguments, aFunction.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createAggregateFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageAggregateFunction aFunction); */
    #[inline]
    pub unsafe fn createAggregateFunction(&self, aFunctionName: &[u8], aNumArguments: libc::int32_t, aFunction: Option<&mozIStorageAggregateFunction>) -> Result<(), nsresult> {
        let aFunctionName = nsCString::from(aFunctionName);
        match ((*self.vtable).createAggregateFunction)(self as *const _, &*aFunctionName, aNumArguments, aFunction.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFunction (in AUTF8String aFunctionName); */
    #[inline]
    pub unsafe fn removeFunction(&self, aFunctionName: &[u8]) -> Result<(), nsresult> {
        let aFunctionName = nsCString::from(aFunctionName);
        match ((*self.vtable).removeFunction)(self as *const _, &*aFunctionName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler); */
    #[inline]
    pub unsafe fn setProgressHandler(&self, aGranularity: int32_t, aHandler: Option<&mozIStorageProgressHandler>) -> Result<Option<RefPtr<mozIStorageProgressHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setProgressHandler)(self as *const _, aGranularity, aHandler.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStorageProgressHandler removeProgressHandler (); */
    #[inline]
    pub unsafe fn removeProgressHandler(&self, ) -> Result<Option<RefPtr<mozIStorageProgressHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeProgressHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


