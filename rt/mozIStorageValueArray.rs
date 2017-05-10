//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageValueArray.idl
//


pub mod mozIStorageValueArray_consts {
    pub const VALUE_TYPE_NULL: i64 = 0;
    pub const VALUE_TYPE_INTEGER: i64 = 1;
    pub const VALUE_TYPE_FLOAT: i64 = 2;
    pub const VALUE_TYPE_TEXT: i64 = 3;
    pub const VALUE_TYPE_BLOB: i64 = 4;
}


#[repr(C)]
pub struct mozIStorageValueArray {
    vtable: *const mozIStorageValueArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageValueArray {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6e6306f4, 0xffa7, 0x40f5,
            [0x96, 0xca, 0x36, 0x15, 0x9c, 0xe8, 0xf4, 0x31])
    }
}

unsafe impl RefCounted for mozIStorageValueArray {
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
pub trait mozIStorageValueArrayCoerce {
    fn coerce_from(v: &mozIStorageValueArray) -> &Self;
}

impl mozIStorageValueArrayCoerce for mozIStorageValueArray {
    #[inline]
    fn coerce_from(v: &mozIStorageValueArray) -> &Self {
        v
    }
}

impl mozIStorageValueArray {
    #[inline]
    pub fn coerce<T: mozIStorageValueArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageValueArray {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageValueArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageValueArray) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageValueArrayVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long numEntries; */
    pub get_numEntries: unsafe extern "C" fn (this: *const mozIStorageValueArray, aNumEntries: *mut libc::uint32_t) -> nsresult,

    /* long getTypeOfIndex (in unsigned long aIndex); */
    pub getTypeOfIndex: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long getInt32 (in unsigned long aIndex); */
    pub getInt32: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long long getInt64 (in unsigned long aIndex); */
    pub getInt64: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut libc::int64_t) -> nsresult,

    /* double getDouble (in unsigned long aIndex); */
    pub getDouble: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut libc::c_double) -> nsresult,

    /* AUTF8String getUTF8String (in unsigned long aIndex); */
    pub getUTF8String: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* AString getString (in unsigned long aIndex); */
    pub getString: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void getBlob (in unsigned long aIndex, out unsigned long aDataSize, [array, size_is (aDataSize)] out octet aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub getBlob: *const ::libc::c_void,

    /* AString getBlobAsString (in unsigned long aIndex); */
    pub getBlobAsString: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* AUTF8String getBlobAsUTF8String (in unsigned long aIndex); */
    pub getBlobAsUTF8String: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* boolean getIsNull (in unsigned long aIndex); */
    pub getIsNull: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* [noscript] void getSharedUTF8String (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out string aResult); */
    pub getSharedUTF8String: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, aLength: *mut libc::uint32_t, aResult: *mut *const libc::c_char) -> nsresult,

    /* [noscript] void getSharedString (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out wstring aResult); */
    pub getSharedString: unsafe extern "C" fn (this: *const mozIStorageValueArray, aIndex: libc::uint32_t, aLength: *mut libc::uint32_t, aResult: *mut *const libc::int16_t) -> nsresult,

    /* [noscript] void getSharedBlob (in unsigned long aIndex, out unsigned long aLength, [shared, retval] out octetPtr aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSharedBlob: *const ::libc::c_void,

}


impl mozIStorageValueArray {
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


