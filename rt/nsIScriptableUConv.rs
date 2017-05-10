//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptableUConv.idl
//


#[repr(C)]
pub struct nsIScriptableUnicodeConverter {
    vtable: *const nsIScriptableUnicodeConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptableUnicodeConverter {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf36ee324, 0x5c1c, 0x437f,
            [0xba, 0x10, 0x2b, 0x4d, 0xb7, 0xa1, 0x80, 0x31])
    }
}

unsafe impl RefCounted for nsIScriptableUnicodeConverter {
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
pub trait nsIScriptableUnicodeConverterCoerce {
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self;
}

impl nsIScriptableUnicodeConverterCoerce for nsIScriptableUnicodeConverter {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self {
        v
    }
}

impl nsIScriptableUnicodeConverter {
    #[inline]
    pub fn coerce<T: nsIScriptableUnicodeConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptableUnicodeConverter {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptableUnicodeConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableUnicodeConverter) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptableUnicodeConverterVTable {
    pub __base: nsISupportsVTable,

    /* ACString ConvertFromUnicode (in AString aSrc); */
    pub ConvertFromUnicode: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aSrc: *const nsAString, _retval: *mut nsACString) -> nsresult,

    /* ACString Finish (); */
    pub Finish: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, _retval: *mut nsACString) -> nsresult,

    /* AString ConvertToUnicode (in ACString aSrc); */
    pub ConvertToUnicode: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aSrc: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* AString convertFromByteArray ([array, size_is (aCount), const] in octet aData, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub convertFromByteArray: *const ::libc::c_void,

    /* void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub convertToByteArray: *const ::libc::c_void,

    /* nsIInputStream convertToInputStream (in AString aString); */
    pub convertToInputStream: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aString: *const nsAString, _retval: *mut *const nsIInputStream) -> nsresult,

    /* attribute string charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aCharset: *mut *const libc::c_char) -> nsresult,
    pub set_charset: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aCharset: *const libc::c_char) -> nsresult,

    /* attribute boolean isInternal; */
    pub get_isInternal: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aIsInternal: *mut bool) -> nsresult,
    pub set_isInternal: unsafe extern "C" fn (this: *const nsIScriptableUnicodeConverter, aIsInternal: bool) -> nsresult,

}


impl nsIScriptableUnicodeConverter {
    /* ACString ConvertFromUnicode (in AString aSrc); */
    #[inline]
    pub unsafe fn ConvertFromUnicode(&self, aSrc: &[u16]) -> Result<nsCString, nsresult> {
        let aSrc = nsString::from(aSrc);
        let mut _retval = nsCString::new();
        match ((*self.vtable).ConvertFromUnicode)(self as *const _, &*aSrc, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString Finish (); */
    #[inline]
    pub unsafe fn Finish(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).Finish)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString ConvertToUnicode (in ACString aSrc); */
    #[inline]
    pub unsafe fn ConvertToUnicode(&self, aSrc: &[u8]) -> Result<nsString, nsresult> {
        let aSrc = nsCString::from(aSrc);
        let mut _retval = nsString::new();
        match ((*self.vtable).ConvertToUnicode)(self as *const _, &*aSrc, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString convertFromByteArray ([array, size_is (aCount), const] in octet aData, in unsigned long aCount); */


    /* void convertToByteArray (in AString aString, [optional] out unsigned long aLen, [array, size_is (aLen), retval] out octet aData); */


    /* nsIInputStream convertToInputStream (in AString aString); */
    #[inline]
    pub unsafe fn convertToInputStream(&self, aString: &[u16]) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let aString = nsString::from(aString);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).convertToInputStream)(self as *const _, &*aString, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute string charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_charset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charset(&self, aCharset: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_charset)(self as *const _, aCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean isInternal; */
    #[inline]
    pub unsafe fn get_isInternal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInternal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isInternal(&self, aIsInternal: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isInternal)(self as *const _, aIsInternal) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


