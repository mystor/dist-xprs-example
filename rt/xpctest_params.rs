//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_params.idl
//


#[repr(C)]
pub struct nsIXPCTestParams {
    vtable: *const nsIXPCTestParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x812145c7, 0x9fcc, 0x425e,
            [0xa8, 0x78, 0x36, 0xad, 0x1b, 0x77, 0x30, 0xb7])
    }
}

unsafe impl RefCounted for nsIXPCTestParams {
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
pub trait nsIXPCTestParamsCoerce {
    fn coerce_from(v: &nsIXPCTestParams) -> &Self;
}

impl nsIXPCTestParamsCoerce for nsIXPCTestParams {
    #[inline]
    fn coerce_from(v: &nsIXPCTestParams) -> &Self {
        v
    }
}

impl nsIXPCTestParams {
    #[inline]
    pub fn coerce<T: nsIXPCTestParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestParamsVTable {
    pub __base: nsISupportsVTable,

    /* boolean testBoolean (in boolean a, inout boolean b); */
    pub testBoolean: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: bool, b: *mut bool, _retval: *mut bool) -> nsresult,

    /* octet testOctet (in octet a, inout octet b); */
    pub testOctet: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::uint8_t, b: *mut libc::uint8_t, _retval: *mut libc::uint8_t) -> nsresult,

    /* short testShort (in short a, inout short b); */
    pub testShort: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::int16_t, b: *mut libc::int16_t, _retval: *mut libc::int16_t) -> nsresult,

    /* long testLong (in long a, inout long b); */
    pub testLong: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::int32_t, b: *mut libc::int32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* long long testLongLong (in long long a, inout long long b); */
    pub testLongLong: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::int64_t, b: *mut libc::int64_t, _retval: *mut libc::int64_t) -> nsresult,

    /* unsigned short testUnsignedShort (in unsigned short a, inout unsigned short b); */
    pub testUnsignedShort: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::uint16_t, b: *mut libc::uint16_t, _retval: *mut libc::uint16_t) -> nsresult,

    /* unsigned long testUnsignedLong (in unsigned long a, inout unsigned long b); */
    pub testUnsignedLong: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::uint32_t, b: *mut libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* unsigned long long testUnsignedLongLong (in unsigned long long a, inout unsigned long long b); */
    pub testUnsignedLongLong: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::uint64_t, b: *mut libc::uint64_t, _retval: *mut libc::uint64_t) -> nsresult,

    /* float testFloat (in float a, inout float b); */
    pub testFloat: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::c_float, b: *mut libc::c_float, _retval: *mut libc::c_float) -> nsresult,

    /* double testDouble (in double a, inout float b); */
    pub testDouble: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::c_double, b: *mut libc::c_float, _retval: *mut libc::c_double) -> nsresult,

    /* char testChar (in char a, inout char b); */
    pub testChar: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::c_char, b: *mut libc::c_char, _retval: *mut libc::c_char) -> nsresult,

    /* string testString (in string a, inout string b); */
    pub testString: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const libc::c_char, b: *mut *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* wchar testWchar (in wchar a, inout wchar b); */
    pub testWchar: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: libc::int16_t, b: *mut libc::int16_t, _retval: *mut libc::int16_t) -> nsresult,

    /* wstring testWstring (in wstring a, inout wstring b); */
    pub testWstring: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const libc::int16_t, b: *mut *const libc::int16_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* DOMString testDOMString (in DOMString a, inout DOMString b); */
    pub testDOMString: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const nsAString, b: *mut nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString testAString (in AString a, inout AString b); */
    pub testAString: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const nsAString, b: *mut nsAString, _retval: *mut nsAString) -> nsresult,

    /* AUTF8String testAUTF8String (in AUTF8String a, inout AUTF8String b); */
    pub testAUTF8String: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const nsACString, b: *mut nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString testACString (in ACString a, inout ACString b); */
    pub testACString: unsafe extern "C" fn (this: *const nsIXPCTestParams, a: *const nsACString, b: *mut nsACString, _retval: *mut nsACString) -> nsresult,

    /* jsval testJsval (in jsval a, inout jsval b); */
    /// Unable to call function as its signature contains a non-rust type
    pub testJsval: *const ::libc::c_void,

    /* void testShortArray (in unsigned long aLength, [array, size_is (aLength)] in short a, inout unsigned long bLength, [array, size_is (bLength)] inout short b, out unsigned long rvLength, [array, size_is (rvLength), retval] out short rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testShortArray: *const ::libc::c_void,

    /* void testDoubleArray (in unsigned long aLength, [array, size_is (aLength)] in double a, inout unsigned long bLength, [array, size_is (bLength)] inout double b, out unsigned long rvLength, [array, size_is (rvLength), retval] out double rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testDoubleArray: *const ::libc::c_void,

    /* void testStringArray (in unsigned long aLength, [array, size_is (aLength)] in string a, inout unsigned long bLength, [array, size_is (bLength)] inout string b, out unsigned long rvLength, [array, size_is (rvLength), retval] out string rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testStringArray: *const ::libc::c_void,

    /* void testWstringArray (in unsigned long aLength, [array, size_is (aLength)] in wstring a, inout unsigned long bLength, [array, size_is (bLength)] inout wstring b, out unsigned long rvLength, [array, size_is (rvLength), retval] out wstring rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testWstringArray: *const ::libc::c_void,

    /* void testInterfaceArray (in unsigned long aLength, [array, size_is (aLength)] in nsIXPCTestInterfaceA a, inout unsigned long bLength, [array, size_is (bLength)] inout nsIXPCTestInterfaceA b, out unsigned long rvLength, [array, size_is (rvLength), retval] out nsIXPCTestInterfaceA rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testInterfaceArray: *const ::libc::c_void,

    /* void testSizedString (in unsigned long aLength, [size_is (aLength)] in string a, inout unsigned long bLength, [size_is (bLength)] inout string b, out unsigned long rvLength, [size_is (rvLength), retval] out string rv); */
    pub testSizedString: unsafe extern "C" fn (this: *const nsIXPCTestParams, aLength: libc::uint32_t, a: *const libc::c_char, bLength: *mut libc::uint32_t, b: *mut *const libc::c_char, rvLength: *mut libc::uint32_t, rv: *mut *const libc::c_char) -> nsresult,

    /* void testSizedWstring (in unsigned long aLength, [size_is (aLength)] in wstring a, inout unsigned long bLength, [size_is (bLength)] inout wstring b, out unsigned long rvLength, [size_is (rvLength), retval] out wstring rv); */
    pub testSizedWstring: unsafe extern "C" fn (this: *const nsIXPCTestParams, aLength: libc::uint32_t, a: *const libc::int16_t, bLength: *mut libc::uint32_t, b: *mut *const libc::int16_t, rvLength: *mut libc::uint32_t, rv: *mut *const libc::int16_t) -> nsresult,

    /* void testInterfaceIs (in nsIIDPtr aIID, [iid_is (aIID)] in nsQIResult a, inout nsIIDPtr bIID, [iid_is (bIID)] inout nsQIResult b, out nsIIDPtr rvIID, [iid_is (rvIID), retval] out nsQIResult rv); */
    pub testInterfaceIs: unsafe extern "C" fn (this: *const nsIXPCTestParams, aIID: *const nsIID, a: *const libc::c_void, bIID: *mut *const nsIID, b: *mut *const libc::c_void, rvIID: *mut *const nsIID, rv: *mut *const libc::c_void) -> nsresult,

    /* void testInterfaceIsArray (in unsigned long aLength, in nsIIDPtr aIID, [array, size_is (aLength), iid_is (aIID)] in nsQIResult a, inout unsigned long bLength, inout nsIIDPtr bIID, [array, size_is (bLength), iid_is (bIID)] inout nsQIResult b, out unsigned long rvLength, out nsIIDPtr rvIID, [retval, array, size_is (rvLength), iid_is (rvIID)] out nsQIResult rv); */
    /// Unable to call function as its signature contains a non-rust type
    pub testInterfaceIsArray: *const ::libc::c_void,

    /* void testOutAString (out AString o); */
    pub testOutAString: unsafe extern "C" fn (this: *const nsIXPCTestParams, o: *mut nsAString) -> nsresult,

    /* ACString testStringArrayOptionalSize ([array, size_is (aLength)] in string a, [optional] in unsigned long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub testStringArrayOptionalSize: *const ::libc::c_void,

}


impl nsIXPCTestParams {
    /* boolean testBoolean (in boolean a, inout boolean b); */
    #[inline]
    pub unsafe fn testBoolean(&self, a: bool) -> Result<(bool, bool), nsresult> {
        let mut b: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).testBoolean)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* octet testOctet (in octet a, inout octet b); */
    #[inline]
    pub unsafe fn testOctet(&self, a: libc::uint8_t) -> Result<(libc::uint8_t, libc::uint8_t), nsresult> {
        let mut b: libc::uint8_t = ::std::mem::zeroed();
        let mut _retval: libc::uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).testOctet)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* short testShort (in short a, inout short b); */
    #[inline]
    pub unsafe fn testShort(&self, a: libc::int16_t) -> Result<(libc::int16_t, libc::int16_t), nsresult> {
        let mut b: libc::int16_t = ::std::mem::zeroed();
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).testShort)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* long testLong (in long a, inout long b); */
    #[inline]
    pub unsafe fn testLong(&self, a: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut b: libc::int32_t = ::std::mem::zeroed();
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).testLong)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* long long testLongLong (in long long a, inout long long b); */
    #[inline]
    pub unsafe fn testLongLong(&self, a: libc::int64_t) -> Result<(libc::int64_t, libc::int64_t), nsresult> {
        let mut b: libc::int64_t = ::std::mem::zeroed();
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).testLongLong)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* unsigned short testUnsignedShort (in unsigned short a, inout unsigned short b); */
    #[inline]
    pub unsafe fn testUnsignedShort(&self, a: libc::uint16_t) -> Result<(libc::uint16_t, libc::uint16_t), nsresult> {
        let mut b: libc::uint16_t = ::std::mem::zeroed();
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).testUnsignedShort)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* unsigned long testUnsignedLong (in unsigned long a, inout unsigned long b); */
    #[inline]
    pub unsafe fn testUnsignedLong(&self, a: libc::uint32_t) -> Result<(libc::uint32_t, libc::uint32_t), nsresult> {
        let mut b: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).testUnsignedLong)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* unsigned long long testUnsignedLongLong (in unsigned long long a, inout unsigned long long b); */
    #[inline]
    pub unsafe fn testUnsignedLongLong(&self, a: libc::uint64_t) -> Result<(libc::uint64_t, libc::uint64_t), nsresult> {
        let mut b: libc::uint64_t = ::std::mem::zeroed();
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).testUnsignedLongLong)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* float testFloat (in float a, inout float b); */
    #[inline]
    pub unsafe fn testFloat(&self, a: libc::c_float) -> Result<(libc::c_float, libc::c_float), nsresult> {
        let mut b: libc::c_float = ::std::mem::zeroed();
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).testFloat)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* double testDouble (in double a, inout float b); */
    #[inline]
    pub unsafe fn testDouble(&self, a: libc::c_double) -> Result<(libc::c_float, libc::c_double), nsresult> {
        let mut b: libc::c_float = ::std::mem::zeroed();
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).testDouble)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* char testChar (in char a, inout char b); */
    #[inline]
    pub unsafe fn testChar(&self, a: libc::c_char) -> Result<(libc::c_char, libc::c_char), nsresult> {
        let mut b: libc::c_char = ::std::mem::zeroed();
        let mut _retval: libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).testChar)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* string testString (in string a, inout string b); */
    #[inline]
    pub unsafe fn testString(&self, a: *const libc::c_char) -> Result<(*const libc::c_char, *const libc::c_char), nsresult> {
        let mut b: *const libc::c_char = ::std::mem::zeroed();
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).testString)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* wchar testWchar (in wchar a, inout wchar b); */
    #[inline]
    pub unsafe fn testWchar(&self, a: libc::int16_t) -> Result<(libc::int16_t, libc::int16_t), nsresult> {
        let mut b: libc::int16_t = ::std::mem::zeroed();
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).testWchar)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* wstring testWstring (in wstring a, inout wstring b); */
    #[inline]
    pub unsafe fn testWstring(&self, a: *const libc::int16_t) -> Result<(*const libc::int16_t, *const libc::int16_t), nsresult> {
        let mut b: *const libc::int16_t = ::std::mem::zeroed();
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).testWstring)(self as *const _, a, &mut b as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* DOMString testDOMString (in DOMString a, inout DOMString b); */
    #[inline]
    pub unsafe fn testDOMString(&self, a: &[u16]) -> Result<(nsString, nsString), nsresult> {
        let a = nsString::from(a);
        let mut b = nsString::new();
        let mut _retval = nsString::new();
        match ((*self.vtable).testDOMString)(self as *const _, &*a, &mut *b, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* AString testAString (in AString a, inout AString b); */
    #[inline]
    pub unsafe fn testAString(&self, a: &[u16]) -> Result<(nsString, nsString), nsresult> {
        let a = nsString::from(a);
        let mut b = nsString::new();
        let mut _retval = nsString::new();
        match ((*self.vtable).testAString)(self as *const _, &*a, &mut *b, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* AUTF8String testAUTF8String (in AUTF8String a, inout AUTF8String b); */
    #[inline]
    pub unsafe fn testAUTF8String(&self, a: &[u8]) -> Result<(nsCString, nsCString), nsresult> {
        let a = nsCString::from(a);
        let mut b = nsCString::new();
        let mut _retval = nsCString::new();
        match ((*self.vtable).testAUTF8String)(self as *const _, &*a, &mut *b, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* ACString testACString (in ACString a, inout ACString b); */
    #[inline]
    pub unsafe fn testACString(&self, a: &[u8]) -> Result<(nsCString, nsCString), nsresult> {
        let a = nsCString::from(a);
        let mut b = nsCString::new();
        let mut _retval = nsCString::new();
        match ((*self.vtable).testACString)(self as *const _, &*a, &mut *b, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((b, _retval))
    }

    /* jsval testJsval (in jsval a, inout jsval b); */


    /* void testShortArray (in unsigned long aLength, [array, size_is (aLength)] in short a, inout unsigned long bLength, [array, size_is (bLength)] inout short b, out unsigned long rvLength, [array, size_is (rvLength), retval] out short rv); */


    /* void testDoubleArray (in unsigned long aLength, [array, size_is (aLength)] in double a, inout unsigned long bLength, [array, size_is (bLength)] inout double b, out unsigned long rvLength, [array, size_is (rvLength), retval] out double rv); */


    /* void testStringArray (in unsigned long aLength, [array, size_is (aLength)] in string a, inout unsigned long bLength, [array, size_is (bLength)] inout string b, out unsigned long rvLength, [array, size_is (rvLength), retval] out string rv); */


    /* void testWstringArray (in unsigned long aLength, [array, size_is (aLength)] in wstring a, inout unsigned long bLength, [array, size_is (bLength)] inout wstring b, out unsigned long rvLength, [array, size_is (rvLength), retval] out wstring rv); */


    /* void testInterfaceArray (in unsigned long aLength, [array, size_is (aLength)] in nsIXPCTestInterfaceA a, inout unsigned long bLength, [array, size_is (bLength)] inout nsIXPCTestInterfaceA b, out unsigned long rvLength, [array, size_is (rvLength), retval] out nsIXPCTestInterfaceA rv); */


    /* void testSizedString (in unsigned long aLength, [size_is (aLength)] in string a, inout unsigned long bLength, [size_is (bLength)] inout string b, out unsigned long rvLength, [size_is (rvLength), retval] out string rv); */
    #[inline]
    pub unsafe fn testSizedString(&self, aLength: libc::uint32_t, a: *const libc::c_char) -> Result<(libc::uint32_t, *const libc::c_char, libc::uint32_t, *const libc::c_char), nsresult> {
        let mut bLength: libc::uint32_t = ::std::mem::zeroed();
        let mut b: *const libc::c_char = ::std::mem::zeroed();
        let mut rvLength: libc::uint32_t = ::std::mem::zeroed();
        let mut rv: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).testSizedString)(self as *const _, aLength, a, &mut bLength as *mut _, &mut b as *mut _, &mut rvLength as *mut _, &mut rv as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((bLength, b, rvLength, rv))
    }

    /* void testSizedWstring (in unsigned long aLength, [size_is (aLength)] in wstring a, inout unsigned long bLength, [size_is (bLength)] inout wstring b, out unsigned long rvLength, [size_is (rvLength), retval] out wstring rv); */
    #[inline]
    pub unsafe fn testSizedWstring(&self, aLength: libc::uint32_t, a: *const libc::int16_t) -> Result<(libc::uint32_t, *const libc::int16_t, libc::uint32_t, *const libc::int16_t), nsresult> {
        let mut bLength: libc::uint32_t = ::std::mem::zeroed();
        let mut b: *const libc::int16_t = ::std::mem::zeroed();
        let mut rvLength: libc::uint32_t = ::std::mem::zeroed();
        let mut rv: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).testSizedWstring)(self as *const _, aLength, a, &mut bLength as *mut _, &mut b as *mut _, &mut rvLength as *mut _, &mut rv as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((bLength, b, rvLength, rv))
    }

    /* void testInterfaceIs (in nsIIDPtr aIID, [iid_is (aIID)] in nsQIResult a, inout nsIIDPtr bIID, [iid_is (bIID)] inout nsQIResult b, out nsIIDPtr rvIID, [iid_is (rvIID), retval] out nsQIResult rv); */
    #[inline]
    pub unsafe fn testInterfaceIs(&self, aIID: *const nsIID, a: *const libc::c_void) -> Result<(*const nsIID, *const libc::c_void, *const nsIID, *const libc::c_void), nsresult> {
        let mut bIID: *const nsIID = ::std::ptr::null();
        let mut b: *const libc::c_void = ::std::ptr::null();
        let mut rvIID: *const nsIID = ::std::ptr::null();
        let mut rv: *const libc::c_void = ::std::ptr::null();
        match ((*self.vtable).testInterfaceIs)(self as *const _, aIID, a, &mut bIID as *mut _, &mut b as *mut _, &mut rvIID as *mut _, &mut rv as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((bIID, b, rvIID, rv))
    }

    /* void testInterfaceIsArray (in unsigned long aLength, in nsIIDPtr aIID, [array, size_is (aLength), iid_is (aIID)] in nsQIResult a, inout unsigned long bLength, inout nsIIDPtr bIID, [array, size_is (bLength), iid_is (bIID)] inout nsQIResult b, out unsigned long rvLength, out nsIIDPtr rvIID, [retval, array, size_is (rvLength), iid_is (rvIID)] out nsQIResult rv); */


    /* void testOutAString (out AString o); */
    #[inline]
    pub unsafe fn testOutAString(&self, ) -> Result<nsString, nsresult> {
        let mut o = nsString::new();
        match ((*self.vtable).testOutAString)(self as *const _, &mut *o) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(o)
    }

    /* ACString testStringArrayOptionalSize ([array, size_is (aLength)] in string a, [optional] in unsigned long aLength); */


}


