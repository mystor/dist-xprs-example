//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageService.idl
//


#[repr(C)]
pub struct mozIStorageService {
    vtable: *const mozIStorageServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x07b6b2f5, 0x6d97, 0x47b4,
            [0x95, 0x84, 0xe6, 0x5b, 0xc4, 0x67, 0xfe, 0x9e])
    }
}

unsafe impl RefCounted for mozIStorageService {
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
pub trait mozIStorageServiceCoerce {
    fn coerce_from(v: &mozIStorageService) -> &Self;
}

impl mozIStorageServiceCoerce for mozIStorageService {
    #[inline]
    fn coerce_from(v: &mozIStorageService) -> &Self {
        v
    }
}

impl mozIStorageService {
    #[inline]
    pub fn coerce<T: mozIStorageServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageServiceVTable {
    pub __base: nsISupportsVTable,

    /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
    pub openAsyncDatabase: unsafe extern "C" fn (this: *const mozIStorageService, aDatabaseStore: *const nsIVariant, aOptions: *const nsIPropertyBag2, aCallback: *const mozIStorageCompletionCallback) -> nsresult,

    /* mozIStorageConnection openSpecialDatabase (in string aStorageKey); */
    pub openSpecialDatabase: unsafe extern "C" fn (this: *const mozIStorageService, aStorageKey: *const libc::c_char, _retval: *mut *const mozIStorageConnection) -> nsresult,

    /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
    pub openDatabase: unsafe extern "C" fn (this: *const mozIStorageService, aDatabaseFile: *const nsIFile, _retval: *mut *const mozIStorageConnection) -> nsresult,

    /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
    pub openUnsharedDatabase: unsafe extern "C" fn (this: *const mozIStorageService, aDatabaseFile: *const nsIFile, _retval: *mut *const mozIStorageConnection) -> nsresult,

    /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL); */
    pub openDatabaseWithFileURL: unsafe extern "C" fn (this: *const mozIStorageService, aFileURL: *const nsIFileURL, _retval: *mut *const mozIStorageConnection) -> nsresult,

    /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
    pub backupDatabaseFile: unsafe extern "C" fn (this: *const mozIStorageService, aDBFile: *const nsIFile, aBackupFileName: *const nsAString, aBackupParentDirectory: *const nsIFile, _retval: *mut *const nsIFile) -> nsresult,

}


impl mozIStorageService {
    /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
    #[inline]
    pub unsafe fn openAsyncDatabase(&self, aDatabaseStore: Option<&nsIVariant>, aOptions: Option<&nsIPropertyBag2>, aCallback: Option<&mozIStorageCompletionCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).openAsyncDatabase)(self as *const _, aDatabaseStore.map_or(::std::ptr::null(), |x| x as *const _), aOptions.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIStorageConnection openSpecialDatabase (in string aStorageKey); */
    #[inline]
    pub unsafe fn openSpecialDatabase(&self, aStorageKey: *const libc::c_char) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openSpecialDatabase)(self as *const _, aStorageKey, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
    #[inline]
    pub unsafe fn openDatabase(&self, aDatabaseFile: Option<&nsIFile>) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openDatabase)(self as *const _, aDatabaseFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
    #[inline]
    pub unsafe fn openUnsharedDatabase(&self, aDatabaseFile: Option<&nsIFile>) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openUnsharedDatabase)(self as *const _, aDatabaseFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL); */
    #[inline]
    pub unsafe fn openDatabaseWithFileURL(&self, aFileURL: Option<&nsIFileURL>) -> Result<Option<RefPtr<mozIStorageConnection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openDatabaseWithFileURL)(self as *const _, aFileURL.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
    #[inline]
    pub unsafe fn backupDatabaseFile(&self, aDBFile: Option<&nsIFile>, aBackupFileName: &[u16], aBackupParentDirectory: Option<&nsIFile>) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let aBackupFileName = nsString::from(aBackupFileName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).backupDatabaseFile)(self as *const _, aDBFile.map_or(::std::ptr::null(), |x| x as *const _), &*aBackupFileName, aBackupParentDirectory.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


