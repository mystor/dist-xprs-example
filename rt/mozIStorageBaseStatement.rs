//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageBaseStatement.idl
//


pub mod mozIStorageBaseStatement_consts {
    pub const MOZ_STORAGE_STATEMENT_INVALID: i64 = 0;
    pub const MOZ_STORAGE_STATEMENT_READY: i64 = 1;
    pub const MOZ_STORAGE_STATEMENT_EXECUTING: i64 = 2;
}


#[repr(C)]
pub struct mozIStorageBaseStatement {
    vtable: *const mozIStorageBaseStatementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageBaseStatement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x16ca67aa, 0x1325, 0x43e2,
            [0xaa, 0xc7, 0x85, 0x9a, 0xfd, 0x15, 0x90, 0xb2])
    }
}

unsafe impl RefCounted for mozIStorageBaseStatement {
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
pub trait mozIStorageBaseStatementCoerce {
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self;
}

impl mozIStorageBaseStatementCoerce for mozIStorageBaseStatement {
    #[inline]
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self {
        v
    }
}

impl mozIStorageBaseStatement {
    #[inline]
    pub fn coerce<T: mozIStorageBaseStatementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageBaseStatement {
    type Target = mozIStorageBindingParams;
    #[inline]
    fn deref(&self) -> &mozIStorageBindingParams {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: mozIStorageBindingParamsCoerce> mozIStorageBaseStatementCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBaseStatement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageBaseStatementVTable {
    pub __base: mozIStorageBindingParamsVTable,

    /* void finalize (); */
    pub finalize: unsafe extern "C" fn (this: *const mozIStorageBaseStatement) -> nsresult,

    /* [deprecated] void bindUTF8StringParameter (in unsigned long aParamIndex, in AUTF8String aValue); */
    pub bindUTF8StringParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: *const nsACString) -> nsresult,

    /* [deprecated] void bindStringParameter (in unsigned long aParamIndex, in AString aValue); */
    pub bindStringParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: *const nsAString) -> nsresult,

    /* [deprecated] void bindDoubleParameter (in unsigned long aParamIndex, in double aValue); */
    pub bindDoubleParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: libc::c_double) -> nsresult,

    /* [deprecated] void bindInt32Parameter (in unsigned long aParamIndex, in long aValue); */
    pub bindInt32Parameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: libc::int32_t) -> nsresult,

    /* [deprecated] void bindInt64Parameter (in unsigned long aParamIndex, in long long aValue); */
    pub bindInt64Parameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: libc::int64_t) -> nsresult,

    /* [deprecated] void bindNullParameter (in unsigned long aParamIndex); */
    pub bindNullParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t) -> nsresult,

    /* [deprecated] void bindBlobParameter (in unsigned long aParamIndex, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindBlobParameter: *const ::libc::c_void,

    /* [deprecated] void bindStringAsBlobParameter (in unsigned long aParamIndex, in AString aValue); */
    pub bindStringAsBlobParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: *const nsAString) -> nsresult,

    /* [deprecated] void bindUTF8StringAsBlobParameter (in unsigned long aParamIndex, in AUTF8String aValue); */
    pub bindUTF8StringAsBlobParameter: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParamIndex: libc::uint32_t, aValue: *const nsACString) -> nsresult,

    /* [deprecated] void bindAdoptedBlobParameter (in unsigned long aParamIndex, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindAdoptedBlobParameter: *const ::libc::c_void,

    /* void bindParameters (in mozIStorageBindingParamsArray aParameters); */
    pub bindParameters: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aParameters: *const mozIStorageBindingParamsArray) -> nsresult,

    /* mozIStorageBindingParamsArray newBindingParamsArray (); */
    pub newBindingParamsArray: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, _retval: *mut *const mozIStorageBindingParamsArray) -> nsresult,

    /* mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback); */
    pub executeAsync: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aCallback: *const mozIStorageStatementCallback, _retval: *mut *const mozIStoragePendingStatement) -> nsresult,

    /* readonly attribute long state; */
    pub get_state: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aState: *mut libc::int32_t) -> nsresult,

    /* AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar); */
    pub escapeStringForLIKE: unsafe extern "C" fn (this: *const mozIStorageBaseStatement, aValue: *const nsAString, aEscapeChar: libc::int16_t, _retval: *mut nsAString) -> nsresult,

}


impl mozIStorageBaseStatement {
    /* void finalize (); */
    #[inline]
    pub unsafe fn finalize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finalize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindUTF8StringParameter (in unsigned long aParamIndex, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringParameter(&self, aParamIndex: libc::uint32_t, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringParameter)(self as *const _, aParamIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindStringParameter (in unsigned long aParamIndex, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringParameter(&self, aParamIndex: libc::uint32_t, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringParameter)(self as *const _, aParamIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindDoubleParameter (in unsigned long aParamIndex, in double aValue); */
    #[inline]
    pub unsafe fn bindDoubleParameter(&self, aParamIndex: libc::uint32_t, aValue: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).bindDoubleParameter)(self as *const _, aParamIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindInt32Parameter (in unsigned long aParamIndex, in long aValue); */
    #[inline]
    pub unsafe fn bindInt32Parameter(&self, aParamIndex: libc::uint32_t, aValue: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindInt32Parameter)(self as *const _, aParamIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindInt64Parameter (in unsigned long aParamIndex, in long long aValue); */
    #[inline]
    pub unsafe fn bindInt64Parameter(&self, aParamIndex: libc::uint32_t, aValue: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindInt64Parameter)(self as *const _, aParamIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindNullParameter (in unsigned long aParamIndex); */
    #[inline]
    pub unsafe fn bindNullParameter(&self, aParamIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindNullParameter)(self as *const _, aParamIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindBlobParameter (in unsigned long aParamIndex, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */


    /* [deprecated] void bindStringAsBlobParameter (in unsigned long aParamIndex, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringAsBlobParameter(&self, aParamIndex: libc::uint32_t, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringAsBlobParameter)(self as *const _, aParamIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindUTF8StringAsBlobParameter (in unsigned long aParamIndex, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringAsBlobParameter(&self, aParamIndex: libc::uint32_t, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringAsBlobParameter)(self as *const _, aParamIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [deprecated] void bindAdoptedBlobParameter (in unsigned long aParamIndex, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */


    /* void bindParameters (in mozIStorageBindingParamsArray aParameters); */
    #[inline]
    pub unsafe fn bindParameters(&self, aParameters: Option<&mozIStorageBindingParamsArray>) -> Result<(), nsresult> {

        match ((*self.vtable).bindParameters)(self as *const _, aParameters.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIStorageBindingParamsArray newBindingParamsArray (); */
    #[inline]
    pub unsafe fn newBindingParamsArray(&self, ) -> Result<Option<RefPtr<mozIStorageBindingParamsArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newBindingParamsArray)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIStoragePendingStatement executeAsync ([optional] in mozIStorageStatementCallback aCallback); */
    #[inline]
    pub unsafe fn executeAsync(&self, aCallback: Option<&mozIStorageStatementCallback>) -> Result<Option<RefPtr<mozIStoragePendingStatement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).executeAsync)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long state; */
    #[inline]
    pub unsafe fn get_state(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_state)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString escapeStringForLIKE (in AString aValue, in wchar aEscapeChar); */
    #[inline]
    pub unsafe fn escapeStringForLIKE(&self, aValue: &[u16], aEscapeChar: libc::int16_t) -> Result<nsString, nsresult> {
        let aValue = nsString::from(aValue);
        let mut _retval = nsString::new();
        match ((*self.vtable).escapeStringForLIKE)(self as *const _, &*aValue, aEscapeChar, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


