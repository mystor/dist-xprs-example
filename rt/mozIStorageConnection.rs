//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageConnection.idl
//


pub mod mozIStorageConnection_consts {
    pub const TRANSACTION_DEFERRED: i64 = 0;
    pub const TRANSACTION_IMMEDIATE: i64 = 1;
    pub const TRANSACTION_EXCLUSIVE: i64 = 2;
}


#[repr(C)]
pub struct mozIStorageConnection {
    vtable: *const mozIStorageConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageConnection {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4aa2ac47, 0x8d24, 0x4004,
            [0x9b, 0x31, 0xec, 0x0b, 0xd8, 0x5f, 0x0c, 0xc3])
    }
}

unsafe impl RefCounted for mozIStorageConnection {
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
pub trait mozIStorageConnectionCoerce {
    fn coerce_from(v: &mozIStorageConnection) -> &Self;
}

impl mozIStorageConnectionCoerce for mozIStorageConnection {
    #[inline]
    fn coerce_from(v: &mozIStorageConnection) -> &Self {
        v
    }
}

impl mozIStorageConnection {
    #[inline]
    pub fn coerce<T: mozIStorageConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageConnection {
    type Target = mozIStorageAsyncConnection;
    #[inline]
    fn deref(&self) -> &mozIStorageAsyncConnection {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozIStorageAsyncConnectionCoerce> mozIStorageConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageConnection) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageConnectionVTable {
    pub __base: mozIStorageAsyncConnectionVTable,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const mozIStorageConnection) -> nsresult,

    /* mozIStorageConnection clone ([optional] in boolean aReadOnly); */
    pub clone: unsafe extern "C" fn (this: *const mozIStorageConnection, aReadOnly: bool, _retval: *mut *const mozIStorageConnection) -> nsresult,

    /* readonly attribute long defaultPageSize; */
    pub get_defaultPageSize: unsafe extern "C" fn (this: *const mozIStorageConnection, aDefaultPageSize: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean connectionReady; */
    pub get_connectionReady: unsafe extern "C" fn (this: *const mozIStorageConnection, aConnectionReady: *mut bool) -> nsresult,

    /* readonly attribute long long lastInsertRowID; */
    pub get_lastInsertRowID: unsafe extern "C" fn (this: *const mozIStorageConnection, aLastInsertRowID: *mut libc::int64_t) -> nsresult,

    /* readonly attribute long affectedRows; */
    pub get_affectedRows: unsafe extern "C" fn (this: *const mozIStorageConnection, aAffectedRows: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long lastError; */
    pub get_lastError: unsafe extern "C" fn (this: *const mozIStorageConnection, aLastError: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AUTF8String lastErrorString; */
    pub get_lastErrorString: unsafe extern "C" fn (this: *const mozIStorageConnection, aLastErrorString: *mut nsACString) -> nsresult,

    /* attribute long schemaVersion; */
    pub get_schemaVersion: unsafe extern "C" fn (this: *const mozIStorageConnection, aSchemaVersion: *mut libc::int32_t) -> nsresult,
    pub set_schemaVersion: unsafe extern "C" fn (this: *const mozIStorageConnection, aSchemaVersion: libc::int32_t) -> nsresult,

    /* mozIStorageStatement createStatement (in AUTF8String aSQLStatement); */
    pub createStatement: unsafe extern "C" fn (this: *const mozIStorageConnection, aSQLStatement: *const nsACString, _retval: *mut *const mozIStorageStatement) -> nsresult,

    /* void executeSimpleSQL (in AUTF8String aSQLStatement); */
    pub executeSimpleSQL: unsafe extern "C" fn (this: *const mozIStorageConnection, aSQLStatement: *const nsACString) -> nsresult,

    /* boolean tableExists (in AUTF8String aTableName); */
    pub tableExists: unsafe extern "C" fn (this: *const mozIStorageConnection, aTableName: *const nsACString, _retval: *mut bool) -> nsresult,

    /* boolean indexExists (in AUTF8String aIndexName); */
    pub indexExists: unsafe extern "C" fn (this: *const mozIStorageConnection, aIndexName: *const nsACString, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean transactionInProgress; */
    pub get_transactionInProgress: unsafe extern "C" fn (this: *const mozIStorageConnection, aTransactionInProgress: *mut bool) -> nsresult,

    /* void beginTransaction (); */
    pub beginTransaction: unsafe extern "C" fn (this: *const mozIStorageConnection) -> nsresult,

    /* void beginTransactionAs (in int32_t transactionType); */
    pub beginTransactionAs: unsafe extern "C" fn (this: *const mozIStorageConnection, transactionType: int32_t) -> nsresult,

    /* void commitTransaction (); */
    pub commitTransaction: unsafe extern "C" fn (this: *const mozIStorageConnection) -> nsresult,

    /* void rollbackTransaction (); */
    pub rollbackTransaction: unsafe extern "C" fn (this: *const mozIStorageConnection) -> nsresult,

    /* void createTable (in string aTableName, in string aTableSchema); */
    pub createTable: unsafe extern "C" fn (this: *const mozIStorageConnection, aTableName: *const libc::c_char, aTableSchema: *const libc::c_char) -> nsresult,

    /* void setGrowthIncrement (in int32_t aIncrement, in AUTF8String aDatabaseName); */
    pub setGrowthIncrement: unsafe extern "C" fn (this: *const mozIStorageConnection, aIncrement: int32_t, aDatabaseName: *const nsACString) -> nsresult,

    /* [noscript] void enableModule (in ACString aModuleName); */
    pub enableModule: unsafe extern "C" fn (this: *const mozIStorageConnection, aModuleName: *const nsACString) -> nsresult,

    /* [noscript] void getQuotaObjects (out QuotaObject aDatabaseQuotaObject, out QuotaObject aJournalQuotaObject); */
    /// Unable to call function as its signature contains a non-rust type
    pub getQuotaObjects: *const ::libc::c_void,

}


impl mozIStorageConnection {
    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIStorageConnection clone ([optional] in boolean aReadOnly); */
    #[inline]
    pub unsafe fn clone(&self, aReadOnly: bool) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, aReadOnly, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long defaultPageSize; */
    #[inline]
    pub unsafe fn get_defaultPageSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultPageSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean connectionReady; */
    #[inline]
    pub unsafe fn get_connectionReady(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_connectionReady)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long lastInsertRowID; */
    #[inline]
    pub unsafe fn get_lastInsertRowID(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastInsertRowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long affectedRows; */
    #[inline]
    pub unsafe fn get_affectedRows(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_affectedRows)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long lastError; */
    #[inline]
    pub unsafe fn get_lastError(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastError)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String lastErrorString; */
    #[inline]
    pub unsafe fn get_lastErrorString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_lastErrorString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long schemaVersion; */
    #[inline]
    pub unsafe fn get_schemaVersion(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_schemaVersion)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_schemaVersion(&self, aSchemaVersion: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_schemaVersion)(self as *const _, aSchemaVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIStorageStatement createStatement (in AUTF8String aSQLStatement); */
    #[inline]
    pub unsafe fn createStatement(&self, aSQLStatement: &[u8]) -> Result<Option<RefPtr<mozIStorageStatement>>, nsresult> {
        let aSQLStatement = nsCString::from(aSQLStatement);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createStatement)(self as *const _, &*aSQLStatement, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void executeSimpleSQL (in AUTF8String aSQLStatement); */
    #[inline]
    pub unsafe fn executeSimpleSQL(&self, aSQLStatement: &[u8]) -> Result<(), nsresult> {
        let aSQLStatement = nsCString::from(aSQLStatement);
        match ((*self.vtable).executeSimpleSQL)(self as *const _, &*aSQLStatement) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean tableExists (in AUTF8String aTableName); */
    #[inline]
    pub unsafe fn tableExists(&self, aTableName: &[u8]) -> Result<bool, nsresult> {
        let aTableName = nsCString::from(aTableName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).tableExists)(self as *const _, &*aTableName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean indexExists (in AUTF8String aIndexName); */
    #[inline]
    pub unsafe fn indexExists(&self, aIndexName: &[u8]) -> Result<bool, nsresult> {
        let aIndexName = nsCString::from(aIndexName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).indexExists)(self as *const _, &*aIndexName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean transactionInProgress; */
    #[inline]
    pub unsafe fn get_transactionInProgress(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_transactionInProgress)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void beginTransaction (); */
    #[inline]
    pub unsafe fn beginTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginTransactionAs (in int32_t transactionType); */
    #[inline]
    pub unsafe fn beginTransactionAs(&self, transactionType: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).beginTransactionAs)(self as *const _, transactionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void commitTransaction (); */
    #[inline]
    pub unsafe fn commitTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).commitTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rollbackTransaction (); */
    #[inline]
    pub unsafe fn rollbackTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).rollbackTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createTable (in string aTableName, in string aTableSchema); */
    #[inline]
    pub unsafe fn createTable(&self, aTableName: *const libc::c_char, aTableSchema: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).createTable)(self as *const _, aTableName, aTableSchema) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setGrowthIncrement (in int32_t aIncrement, in AUTF8String aDatabaseName); */
    #[inline]
    pub unsafe fn setGrowthIncrement(&self, aIncrement: int32_t, aDatabaseName: &[u8]) -> Result<(), nsresult> {
        let aDatabaseName = nsCString::from(aDatabaseName);
        match ((*self.vtable).setGrowthIncrement)(self as *const _, aIncrement, &*aDatabaseName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void enableModule (in ACString aModuleName); */
    #[inline]
    pub unsafe fn enableModule(&self, aModuleName: &[u8]) -> Result<(), nsresult> {
        let aModuleName = nsCString::from(aModuleName);
        match ((*self.vtable).enableModule)(self as *const _, &*aModuleName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getQuotaObjects (out QuotaObject aDatabaseQuotaObject, out QuotaObject aJournalQuotaObject); */


}


