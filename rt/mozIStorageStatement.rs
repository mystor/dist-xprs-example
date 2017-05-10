//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageStatement.idl
//


pub mod mozIStorageStatement_consts {
    pub const VALUE_TYPE_NULL: i64 = 0;
    pub const VALUE_TYPE_INTEGER: i64 = 1;
    pub const VALUE_TYPE_FLOAT: i64 = 2;
    pub const VALUE_TYPE_TEXT: i64 = 3;
    pub const VALUE_TYPE_BLOB: i64 = 4;
}


#[repr(C)]
pub struct mozIStorageStatement {
    vtable: *const mozIStorageStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageStatement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5f567c35, 0x6c32, 0x4140,
            [0x82, 0x8c, 0x68, 0x3e, 0xa4, 0x9c, 0xfd, 0x3a])
    }
}

unsafe impl RefCounted for mozIStorageStatement {
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
pub trait mozIStorageStatementCoerce {
    fn coerce_from(v: &mozIStorageStatement) -> &Self;
}

impl mozIStorageStatementCoerce for mozIStorageStatement {
    #[inline]
    fn coerce_from(v: &mozIStorageStatement) -> &Self {
        v
    }
}

impl mozIStorageStatement {
    #[inline]
    pub fn coerce<T: mozIStorageStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageStatement {
    type Target = mozIStorageBaseStatement;
    #[inline]
    fn deref(&self) -> &mozIStorageBaseStatement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozIStorageBaseStatementCoerce> mozIStorageStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageStatement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageStatementVTable {
    pub __base: mozIStorageBaseStatementVTable,

    /* mozIStorageStatement clone (); */
    pub clone: unsafe extern "C" fn (this: *const mozIStorageStatement, _retval: *mut *const mozIStorageStatement) -> nsresult,

    /* readonly attribute unsigned long parameterCount; */
    pub get_parameterCount: unsafe extern "C" fn (this: *const mozIStorageStatement, aParameterCount: *mut libc::uint32_t) -> nsresult,

    /* AUTF8String getParameterName (in unsigned long aParamIndex); */
    pub getParameterName: unsafe extern "C" fn (this: *const mozIStorageStatement, aParamIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* unsigned long getParameterIndex (in AUTF8String aName); */
    pub getParameterIndex: unsafe extern "C" fn (this: *const mozIStorageStatement, aName: *const nsACString, _retval: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long columnCount; */
    pub get_columnCount: unsafe extern "C" fn (this: *const mozIStorageStatement, aColumnCount: *mut libc::uint32_t) -> nsresult,

    /* AUTF8String getColumnName (in unsigned long aColumnIndex); */
    pub getColumnName: unsafe extern "C" fn (this: *const mozIStorageStatement, aColumnIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* unsigned long getColumnIndex (in AUTF8String aName); */
    pub getColumnIndex: unsafe extern "C" fn (this: *const mozIStorageStatement, aName: *const nsACString, _retval: *mut libc::uint32_t) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const mozIStorageStatement) -> nsresult,

    /* void execute (); */
    pub execute: unsafe extern "C" fn (this: *const mozIStorageStatement) -> nsresult,

    /* boolean executeStep (); */
    pub executeStep: unsafe extern "C" fn (this: *const mozIStorageStatement, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long numEntries; */
    pub get_numEntries: unsafe extern "C" fn (this: *const mozIStorageStatement, aNumEntries: *mut libc::uint32_t) -> nsresult,

    /* long getTypeOfIndex (in unsigned long aIndex); */
    pub getTypeOfIndex: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long getInt32 (in unsigned long aIndex); */
    pub getInt32: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long long getInt64 (in unsigned long aIndex); */
    pub getInt64: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut libc::int64_t) -> nsresult,

    /* double getDouble (in unsigned long aIndex); */
    pub getDouble: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut libc::c_double) -> nsresult,

    /* AUTF8String getUTF8String (in unsigned long aIndex); */
    pub getUTF8String: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* AString getString (in unsigned long aIndex); */
    pub getString: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub getBlob: *const ::libc::c_void,

    /* AString getBlobAsString (in unsigned long aIndex); */
    pub getBlobAsString: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
    pub getBlobAsUTF8String: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* boolean getIsNull (in unsigned long aIndex); */
    pub getIsNull: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out string aResult); */
    pub getSharedUTF8String: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, aLength: *mut libc::uint32_t, aResult: *mut *const libc::c_char) -> nsresult,

    /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out wstring aResult); */
    pub getSharedString: unsafe extern "C" fn (this: *const mozIStorageStatement, aIndex: libc::uint32_t, aLength: *mut libc::uint32_t, aResult: *mut *const libc::int16_t) -> nsresult,

    /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out octetPtr aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSharedBlob: *const ::libc::c_void,

}


impl mozIStorageStatement {
    /* mozIStorageStatement clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<mozIStorageStatement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long parameterCount; */
    #[inline]
    pub unsafe fn get_parameterCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parameterCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getParameterName (in unsigned long aParamIndex); */
    #[inline]
    pub unsafe fn getParameterName(&self, aParamIndex: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getParameterName)(self as *const _, aParamIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getParameterIndex (in AUTF8String aName); */
    #[inline]
    pub unsafe fn getParameterIndex(&self, aName: &[u8]) -> Result<libc::uint32_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getParameterIndex)(self as *const _, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long columnCount; */
    #[inline]
    pub unsafe fn get_columnCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_columnCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getColumnName (in unsigned long aColumnIndex); */
    #[inline]
    pub unsafe fn getColumnName(&self, aColumnIndex: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getColumnName)(self as *const _, aColumnIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getColumnIndex (in AUTF8String aName); */
    #[inline]
    pub unsafe fn getColumnIndex(&self, aName: &[u8]) -> Result<libc::uint32_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getColumnIndex)(self as *const _, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void execute (); */
    #[inline]
    pub unsafe fn execute(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).execute)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean executeStep (); */
    #[inline]
    pub unsafe fn executeStep(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).executeStep)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long numEntries; */
    #[inline]
    pub unsafe fn get_numEntries(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numEntries)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getTypeOfIndex (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getTypeOfIndex(&self, aIndex: libc::uint32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTypeOfIndex)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getInt32 (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getInt32(&self, aIndex: libc::uint32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getInt32)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long long getInt64 (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getInt64(&self, aIndex: libc::uint32_t) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getInt64)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double getDouble (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getDouble(&self, aIndex: libc::uint32_t) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getDouble)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getUTF8String (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getUTF8String(&self, aIndex: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getUTF8String)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getString (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getString(&self, aIndex: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getString)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */


    /* AString getBlobAsString (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getBlobAsString(&self, aIndex: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getBlobAsString)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getBlobAsUTF8String(&self, aIndex: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getBlobAsUTF8String)(self as *const _, aIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean getIsNull (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn getIsNull(&self, aIndex: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getIsNull)(self as *const _, aIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out string aResult); */
    #[inline]
    pub unsafe fn getSharedUTF8String(&self, aIndex: libc::uint32_t) -> Result<(libc::uint32_t, *const libc::c_char), nsresult> {
        let mut aLength: libc::uint32_t = ::std::mem::zeroed();
        let mut aResult: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getSharedUTF8String)(self as *const _, aIndex, &mut aLength as *mut _, &mut aResult as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLength, aResult))
    }

    /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out wstring aResult); */
    #[inline]
    pub unsafe fn getSharedString(&self, aIndex: libc::uint32_t) -> Result<(libc::uint32_t, *const libc::int16_t), nsresult> {
        let mut aLength: libc::uint32_t = ::std::mem::zeroed();
        let mut aResult: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getSharedString)(self as *const _, aIndex, &mut aLength as *mut _, &mut aResult as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aLength, aResult))
    }

    /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out octetPtr aResult); */


}


