//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIStorageBindingParams.idl
//


#[repr(C)]
pub struct mozIStorageBindingParams {
    vtable: *const mozIStorageBindingParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIStorageBindingParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2d09f42f, 0x966e, 0x4663,
            [0xb4, 0xb3, 0xb0, 0xc8, 0x67, 0x6b, 0xf2, 0xbf])
    }
}

unsafe impl RefCounted for mozIStorageBindingParams {
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
pub trait mozIStorageBindingParamsCoerce {
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self;
}

impl mozIStorageBindingParamsCoerce for mozIStorageBindingParams {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self {
        v
    }
}

impl mozIStorageBindingParams {
    #[inline]
    pub fn coerce<T: mozIStorageBindingParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIStorageBindingParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIStorageBindingParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIStorageBindingParamsVTable {
    pub __base: nsISupportsVTable,

    /* void bindByName (in AUTF8String aName, in nsIVariant aValue); */
    pub bindByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: *const nsIVariant) -> nsresult,

    /* [noscript] void bindUTF8StringByName (in AUTF8String aName, in AUTF8String aValue); */
    pub bindUTF8StringByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: *const nsACString) -> nsresult,

    /* [noscript] void bindStringByName (in AUTF8String aName, in AString aValue); */
    pub bindStringByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: *const nsAString) -> nsresult,

    /* [noscript] void bindDoubleByName (in AUTF8String aName, in double aValue); */
    pub bindDoubleByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: libc::c_double) -> nsresult,

    /* [noscript] void bindInt32ByName (in AUTF8String aName, in long aValue); */
    pub bindInt32ByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: libc::int32_t) -> nsresult,

    /* [noscript] void bindInt64ByName (in AUTF8String aName, in long long aValue); */
    pub bindInt64ByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: libc::int64_t) -> nsresult,

    /* [noscript] void bindNullByName (in AUTF8String aName); */
    pub bindNullByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString) -> nsresult,

    /* void bindBlobByName (in AUTF8String aName, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindBlobByName: *const ::libc::c_void,

    /* void bindStringAsBlobByName (in AUTF8String aName, in AString aValue); */
    pub bindStringAsBlobByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: *const nsAString) -> nsresult,

    /* void bindUTF8StringAsBlobByName (in AUTF8String aName, in AUTF8String aValue); */
    pub bindUTF8StringAsBlobByName: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aName: *const nsACString, aValue: *const nsACString) -> nsresult,

    /* [noscript] void bindAdoptedBlobByName (in AUTF8String aName, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindAdoptedBlobByName: *const ::libc::c_void,

    /* void bindByIndex (in unsigned long aIndex, in nsIVariant aValue); */
    pub bindByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: *const nsIVariant) -> nsresult,

    /* [noscript] void bindUTF8StringByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    pub bindUTF8StringByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: *const nsACString) -> nsresult,

    /* [noscript] void bindStringByIndex (in unsigned long aIndex, in AString aValue); */
    pub bindStringByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: *const nsAString) -> nsresult,

    /* [noscript] void bindDoubleByIndex (in unsigned long aIndex, in double aValue); */
    pub bindDoubleByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: libc::c_double) -> nsresult,

    /* [noscript] void bindInt32ByIndex (in unsigned long aIndex, in long aValue); */
    pub bindInt32ByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: libc::int32_t) -> nsresult,

    /* [noscript] void bindInt64ByIndex (in unsigned long aIndex, in long long aValue); */
    pub bindInt64ByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: libc::int64_t) -> nsresult,

    /* [noscript] void bindNullByIndex (in unsigned long aIndex); */
    pub bindNullByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t) -> nsresult,

    /* void bindBlobByIndex (in unsigned long aIndex, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindBlobByIndex: *const ::libc::c_void,

    /* void bindStringAsBlobByIndex (in unsigned long aIndex, in AString aValue); */
    pub bindStringAsBlobByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: *const nsAString) -> nsresult,

    /* void bindUTF8StringAsBlobByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    pub bindUTF8StringAsBlobByIndex: unsafe extern "C" fn (this: *const mozIStorageBindingParams, aIndex: libc::uint32_t, aValue: *const nsACString) -> nsresult,

    /* [noscript] void bindAdoptedBlobByIndex (in unsigned long aIndex, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */
    /// Unable to call function as its signature contains a non-rust type
    pub bindAdoptedBlobByIndex: *const ::libc::c_void,

}


impl mozIStorageBindingParams {
    /* void bindByName (in AUTF8String aName, in nsIVariant aValue); */
    #[inline]
    pub unsafe fn bindByName(&self, aName: &[u8], aValue: Option<&nsIVariant>) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).bindByName)(self as *const _, &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindUTF8StringByName (in AUTF8String aName, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringByName(&self, aName: &[u8], aValue: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringByName)(self as *const _, &*aName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindStringByName (in AUTF8String aName, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringByName(&self, aName: &[u8], aValue: &[u16]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringByName)(self as *const _, &*aName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindDoubleByName (in AUTF8String aName, in double aValue); */
    #[inline]
    pub unsafe fn bindDoubleByName(&self, aName: &[u8], aValue: libc::c_double) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).bindDoubleByName)(self as *const _, &*aName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindInt32ByName (in AUTF8String aName, in long aValue); */
    #[inline]
    pub unsafe fn bindInt32ByName(&self, aName: &[u8], aValue: libc::int32_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).bindInt32ByName)(self as *const _, &*aName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindInt64ByName (in AUTF8String aName, in long long aValue); */
    #[inline]
    pub unsafe fn bindInt64ByName(&self, aName: &[u8], aValue: libc::int64_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).bindInt64ByName)(self as *const _, &*aName, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindNullByName (in AUTF8String aName); */
    #[inline]
    pub unsafe fn bindNullByName(&self, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).bindNullByName)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void bindBlobByName (in AUTF8String aName, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */


    /* void bindStringAsBlobByName (in AUTF8String aName, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringAsBlobByName(&self, aName: &[u8], aValue: &[u16]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringAsBlobByName)(self as *const _, &*aName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void bindUTF8StringAsBlobByName (in AUTF8String aName, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringAsBlobByName(&self, aName: &[u8], aValue: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringAsBlobByName)(self as *const _, &*aName, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindAdoptedBlobByName (in AUTF8String aName, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */


    /* void bindByIndex (in unsigned long aIndex, in nsIVariant aValue); */
    #[inline]
    pub unsafe fn bindByIndex(&self, aIndex: libc::uint32_t, aValue: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).bindByIndex)(self as *const _, aIndex, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindUTF8StringByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringByIndex(&self, aIndex: libc::uint32_t, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringByIndex)(self as *const _, aIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindStringByIndex (in unsigned long aIndex, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringByIndex(&self, aIndex: libc::uint32_t, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringByIndex)(self as *const _, aIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindDoubleByIndex (in unsigned long aIndex, in double aValue); */
    #[inline]
    pub unsafe fn bindDoubleByIndex(&self, aIndex: libc::uint32_t, aValue: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).bindDoubleByIndex)(self as *const _, aIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindInt32ByIndex (in unsigned long aIndex, in long aValue); */
    #[inline]
    pub unsafe fn bindInt32ByIndex(&self, aIndex: libc::uint32_t, aValue: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindInt32ByIndex)(self as *const _, aIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindInt64ByIndex (in unsigned long aIndex, in long long aValue); */
    #[inline]
    pub unsafe fn bindInt64ByIndex(&self, aIndex: libc::uint32_t, aValue: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindInt64ByIndex)(self as *const _, aIndex, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindNullByIndex (in unsigned long aIndex); */
    #[inline]
    pub unsafe fn bindNullByIndex(&self, aIndex: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).bindNullByIndex)(self as *const _, aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void bindBlobByIndex (in unsigned long aIndex, [array, size_is (aValueSize), const] in octet aValue, in unsigned long aValueSize); */


    /* void bindStringAsBlobByIndex (in unsigned long aIndex, in AString aValue); */
    #[inline]
    pub unsafe fn bindStringAsBlobByIndex(&self, aIndex: libc::uint32_t, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).bindStringAsBlobByIndex)(self as *const _, aIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void bindUTF8StringAsBlobByIndex (in unsigned long aIndex, in AUTF8String aValue); */
    #[inline]
    pub unsafe fn bindUTF8StringAsBlobByIndex(&self, aIndex: libc::uint32_t, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).bindUTF8StringAsBlobByIndex)(self as *const _, aIndex, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void bindAdoptedBlobByIndex (in unsigned long aIndex, [array, size_is (aValueSize)] in octet aValue, in unsigned long aValueSize); */


}


