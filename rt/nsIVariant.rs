//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIVariant.idl
//


pub mod nsIDataType_consts {
    pub const VTYPE_INT8: i64 = 0;
    pub const VTYPE_INT16: i64 = 1;
    pub const VTYPE_INT32: i64 = 2;
    pub const VTYPE_INT64: i64 = 3;
    pub const VTYPE_UINT8: i64 = 4;
    pub const VTYPE_UINT16: i64 = 5;
    pub const VTYPE_UINT32: i64 = 6;
    pub const VTYPE_UINT64: i64 = 7;
    pub const VTYPE_FLOAT: i64 = 8;
    pub const VTYPE_DOUBLE: i64 = 9;
    pub const VTYPE_BOOL: i64 = 10;
    pub const VTYPE_CHAR: i64 = 11;
    pub const VTYPE_WCHAR: i64 = 12;
    pub const VTYPE_VOID: i64 = 13;
    pub const VTYPE_ID: i64 = 14;
    pub const VTYPE_DOMSTRING: i64 = 15;
    pub const VTYPE_CHAR_STR: i64 = 16;
    pub const VTYPE_WCHAR_STR: i64 = 17;
    pub const VTYPE_INTERFACE: i64 = 18;
    pub const VTYPE_INTERFACE_IS: i64 = 19;
    pub const VTYPE_ARRAY: i64 = 20;
    pub const VTYPE_STRING_SIZE_IS: i64 = 21;
    pub const VTYPE_WSTRING_SIZE_IS: i64 = 22;
    pub const VTYPE_UTF8STRING: i64 = 23;
    pub const VTYPE_CSTRING: i64 = 24;
    pub const VTYPE_ASTRING: i64 = 25;
    pub const VTYPE_EMPTY_ARRAY: i64 = 254;
    pub const VTYPE_EMPTY: i64 = 255;
}


#[repr(C)]
pub struct nsIDataType {
    vtable: *const nsIDataTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDataType {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4d12e540, 0x83d7, 0x11d5,
            [0x90, 0xed, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIDataType {
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
pub trait nsIDataTypeCoerce {
    fn coerce_from(v: &nsIDataType) -> &Self;
}

impl nsIDataTypeCoerce for nsIDataType {
    #[inline]
    fn coerce_from(v: &nsIDataType) -> &Self {
        v
    }
}

impl nsIDataType {
    #[inline]
    pub fn coerce<T: nsIDataTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDataType {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDataTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDataType) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDataTypeVTable {
    pub __base: nsISupportsVTable,

}


impl nsIDataType {
}


#[repr(C)]
pub struct nsIVariant {
    vtable: *const nsIVariantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIVariant {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x81e4c2de, 0xacac, 0x4ad6,
            [0x90, 0x1a, 0xb5, 0xfb, 0x1b, 0x85, 0x1a, 0x0d])
    }
}

unsafe impl RefCounted for nsIVariant {
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
pub trait nsIVariantCoerce {
    fn coerce_from(v: &nsIVariant) -> &Self;
}

impl nsIVariantCoerce for nsIVariant {
    #[inline]
    fn coerce_from(v: &nsIVariant) -> &Self {
        v
    }
}

impl nsIVariant {
    #[inline]
    pub fn coerce<T: nsIVariantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIVariant {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIVariantCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIVariant) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIVariantVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute uint16_t dataType; */
    pub get_dataType: unsafe extern "C" fn (this: *const nsIVariant, aDataType: *mut uint16_t) -> nsresult,

    /* [noscript] uint8_t getAsInt8 (); */
    pub getAsInt8: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut uint8_t) -> nsresult,

    /* [noscript] int16_t getAsInt16 (); */
    pub getAsInt16: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut int16_t) -> nsresult,

    /* [noscript] int32_t getAsInt32 (); */
    pub getAsInt32: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut int32_t) -> nsresult,

    /* [noscript] int64_t getAsInt64 (); */
    pub getAsInt64: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut int64_t) -> nsresult,

    /* [noscript] uint8_t getAsUint8 (); */
    pub getAsUint8: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut uint8_t) -> nsresult,

    /* [noscript] uint16_t getAsUint16 (); */
    pub getAsUint16: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut uint16_t) -> nsresult,

    /* [noscript] uint32_t getAsUint32 (); */
    pub getAsUint32: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut uint32_t) -> nsresult,

    /* [noscript] uint64_t getAsUint64 (); */
    pub getAsUint64: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut uint64_t) -> nsresult,

    /* [noscript] float getAsFloat (); */
    pub getAsFloat: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut libc::c_float) -> nsresult,

    /* [noscript] double getAsDouble (); */
    pub getAsDouble: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut libc::c_double) -> nsresult,

    /* [noscript] boolean getAsBool (); */
    pub getAsBool: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut bool) -> nsresult,

    /* [noscript] char getAsChar (); */
    pub getAsChar: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut libc::c_char) -> nsresult,

    /* [noscript] wchar getAsWChar (); */
    pub getAsWChar: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut libc::int16_t) -> nsresult,

    /* [notxpcom] nsresult getAsID (out nsID retval); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAsID: *const ::libc::c_void,

    /* [noscript] AString getAsAString (); */
    pub getAsAString: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut nsAString) -> nsresult,

    /* [noscript] DOMString getAsDOMString (); */
    pub getAsDOMString: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut nsAString) -> nsresult,

    /* [noscript] ACString getAsACString (); */
    pub getAsACString: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut nsACString) -> nsresult,

    /* [noscript] AUTF8String getAsAUTF8String (); */
    pub getAsAUTF8String: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut nsACString) -> nsresult,

    /* [noscript] string getAsString (); */
    pub getAsString: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut *const libc::c_char) -> nsresult,

    /* [noscript] wstring getAsWString (); */
    pub getAsWString: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut *const libc::int16_t) -> nsresult,

    /* [noscript] nsISupports getAsISupports (); */
    pub getAsISupports: unsafe extern "C" fn (this: *const nsIVariant, _retval: *mut *const nsISupports) -> nsresult,

    /* [noscript] jsval getAsJSVal (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAsJSVal: *const ::libc::c_void,

    /* [noscript] void getAsInterface (out nsIIDPtr iid, [iid_is (iid), retval] out nsQIResult iface); */
    pub getAsInterface: unsafe extern "C" fn (this: *const nsIVariant, iid: *mut *const nsIID, iface: *mut *const libc::c_void) -> nsresult,

    /* [notxpcom] nsresult getAsArray (out uint16_t type, out nsIID iid, out uint32_t count, out voidPtr ptr); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAsArray: *const ::libc::c_void,

    /* [noscript] void getAsStringWithSize (out uint32_t size, [size_is (size), retval] out string str); */
    pub getAsStringWithSize: unsafe extern "C" fn (this: *const nsIVariant, size: *mut uint32_t, str: *mut *const libc::c_char) -> nsresult,

    /* [noscript] void getAsWStringWithSize (out uint32_t size, [size_is (size), retval] out wstring str); */
    pub getAsWStringWithSize: unsafe extern "C" fn (this: *const nsIVariant, size: *mut uint32_t, str: *mut *const libc::int16_t) -> nsresult,

}


impl nsIVariant {
    /* [noscript] readonly attribute uint16_t dataType; */
    #[inline]
    pub unsafe fn get_dataType(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dataType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] uint8_t getAsInt8 (); */
    #[inline]
    pub unsafe fn getAsInt8(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsInt8)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] int16_t getAsInt16 (); */
    #[inline]
    pub unsafe fn getAsInt16(&self, ) -> Result<int16_t, nsresult> {
        let mut _retval: int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsInt16)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] int32_t getAsInt32 (); */
    #[inline]
    pub unsafe fn getAsInt32(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsInt32)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] int64_t getAsInt64 (); */
    #[inline]
    pub unsafe fn getAsInt64(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsInt64)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] uint8_t getAsUint8 (); */
    #[inline]
    pub unsafe fn getAsUint8(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsUint8)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] uint16_t getAsUint16 (); */
    #[inline]
    pub unsafe fn getAsUint16(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsUint16)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] uint32_t getAsUint32 (); */
    #[inline]
    pub unsafe fn getAsUint32(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsUint32)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] uint64_t getAsUint64 (); */
    #[inline]
    pub unsafe fn getAsUint64(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsUint64)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] float getAsFloat (); */
    #[inline]
    pub unsafe fn getAsFloat(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getAsFloat)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] double getAsDouble (); */
    #[inline]
    pub unsafe fn getAsDouble(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getAsDouble)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] boolean getAsBool (); */
    #[inline]
    pub unsafe fn getAsBool(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getAsBool)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] char getAsChar (); */
    #[inline]
    pub unsafe fn getAsChar(&self, ) -> Result<libc::c_char, nsresult> {
        let mut _retval: libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getAsChar)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] wchar getAsWChar (); */
    #[inline]
    pub unsafe fn getAsWChar(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsWChar)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [notxpcom] nsresult getAsID (out nsID retval); */


    /* [noscript] AString getAsAString (); */
    #[inline]
    pub unsafe fn getAsAString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getAsAString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] DOMString getAsDOMString (); */
    #[inline]
    pub unsafe fn getAsDOMString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getAsDOMString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] ACString getAsACString (); */
    #[inline]
    pub unsafe fn getAsACString(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAsACString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] AUTF8String getAsAUTF8String (); */
    #[inline]
    pub unsafe fn getAsAUTF8String(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAsAUTF8String)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] string getAsString (); */
    #[inline]
    pub unsafe fn getAsString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getAsString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] wstring getAsWString (); */
    #[inline]
    pub unsafe fn getAsWString(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsWString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] nsISupports getAsISupports (); */
    #[inline]
    pub unsafe fn getAsISupports(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAsISupports)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] jsval getAsJSVal (); */


    /* [noscript] void getAsInterface (out nsIIDPtr iid, [iid_is (iid), retval] out nsQIResult iface); */
    #[inline]
    pub unsafe fn getAsInterface(&self, ) -> Result<(*const nsIID, *const libc::c_void), nsresult> {
        let mut iid: *const nsIID = ::std::ptr::null();
        let mut iface: *const libc::c_void = ::std::ptr::null();
        match ((*self.vtable).getAsInterface)(self as *const _, &mut iid as *mut _, &mut iface as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((iid, iface))
    }

    /* [notxpcom] nsresult getAsArray (out uint16_t type, out nsIID iid, out uint32_t count, out voidPtr ptr); */


    /* [noscript] void getAsStringWithSize (out uint32_t size, [size_is (size), retval] out string str); */
    #[inline]
    pub unsafe fn getAsStringWithSize(&self, ) -> Result<(uint32_t, *const libc::c_char), nsresult> {
        let mut size: uint32_t = ::std::mem::zeroed();
        let mut str: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getAsStringWithSize)(self as *const _, &mut size as *mut _, &mut str as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((size, str))
    }

    /* [noscript] void getAsWStringWithSize (out uint32_t size, [size_is (size), retval] out wstring str); */
    #[inline]
    pub unsafe fn getAsWStringWithSize(&self, ) -> Result<(uint32_t, *const libc::int16_t), nsresult> {
        let mut size: uint32_t = ::std::mem::zeroed();
        let mut str: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAsWStringWithSize)(self as *const _, &mut size as *mut _, &mut str as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((size, str))
    }

}


#[repr(C)]
pub struct nsIWritableVariant {
    vtable: *const nsIWritableVariantVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWritableVariant {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5586a590, 0x8c82, 0x11d5,
            [0x90, 0xf3, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a])
    }
}

unsafe impl RefCounted for nsIWritableVariant {
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
pub trait nsIWritableVariantCoerce {
    fn coerce_from(v: &nsIWritableVariant) -> &Self;
}

impl nsIWritableVariantCoerce for nsIWritableVariant {
    #[inline]
    fn coerce_from(v: &nsIWritableVariant) -> &Self {
        v
    }
}

impl nsIWritableVariant {
    #[inline]
    pub fn coerce<T: nsIWritableVariantCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWritableVariant {
    type Target = nsIVariant;
    #[inline]
    fn deref(&self) -> &nsIVariant {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIVariantCoerce> nsIWritableVariantCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWritableVariant) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWritableVariantVTable {
    pub __base: nsIVariantVTable,

    /* attribute boolean writable; */
    pub get_writable: unsafe extern "C" fn (this: *const nsIWritableVariant, aWritable: *mut bool) -> nsresult,
    pub set_writable: unsafe extern "C" fn (this: *const nsIWritableVariant, aWritable: bool) -> nsresult,

    /* void setAsInt8 (in uint8_t aValue); */
    pub setAsInt8: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: uint8_t) -> nsresult,

    /* void setAsInt16 (in int16_t aValue); */
    pub setAsInt16: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: int16_t) -> nsresult,

    /* void setAsInt32 (in int32_t aValue); */
    pub setAsInt32: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: int32_t) -> nsresult,

    /* void setAsInt64 (in int64_t aValue); */
    pub setAsInt64: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: int64_t) -> nsresult,

    /* void setAsUint8 (in uint8_t aValue); */
    pub setAsUint8: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: uint8_t) -> nsresult,

    /* void setAsUint16 (in uint16_t aValue); */
    pub setAsUint16: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: uint16_t) -> nsresult,

    /* void setAsUint32 (in uint32_t aValue); */
    pub setAsUint32: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: uint32_t) -> nsresult,

    /* void setAsUint64 (in uint64_t aValue); */
    pub setAsUint64: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: uint64_t) -> nsresult,

    /* void setAsFloat (in float aValue); */
    pub setAsFloat: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: libc::c_float) -> nsresult,

    /* void setAsDouble (in double aValue); */
    pub setAsDouble: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: libc::c_double) -> nsresult,

    /* void setAsBool (in boolean aValue); */
    pub setAsBool: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: bool) -> nsresult,

    /* void setAsChar (in char aValue); */
    pub setAsChar: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: libc::c_char) -> nsresult,

    /* void setAsWChar (in wchar aValue); */
    pub setAsWChar: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: libc::int16_t) -> nsresult,

    /* void setAsID (in nsIDRef aValue); */
    pub setAsID: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsID) -> nsresult,

    /* void setAsAString (in AString aValue); */
    pub setAsAString: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsAString) -> nsresult,

    /* void setAsDOMString (in DOMString aValue); */
    pub setAsDOMString: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsAString) -> nsresult,

    /* void setAsACString (in ACString aValue); */
    pub setAsACString: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsACString) -> nsresult,

    /* void setAsAUTF8String (in AUTF8String aValue); */
    pub setAsAUTF8String: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsACString) -> nsresult,

    /* void setAsString (in string aValue); */
    pub setAsString: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const libc::c_char) -> nsresult,

    /* void setAsWString (in wstring aValue); */
    pub setAsWString: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const libc::int16_t) -> nsresult,

    /* void setAsISupports (in nsISupports aValue); */
    pub setAsISupports: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsISupports) -> nsresult,

    /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
    pub setAsInterface: unsafe extern "C" fn (this: *const nsIWritableVariant, iid: *const nsIID, iface: *const libc::c_void) -> nsresult,

    /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
    pub setAsArray: unsafe extern "C" fn (this: *const nsIWritableVariant, type_: uint16_t, iid: *const nsIID, count: uint32_t, ptr: *const libc::c_void) -> nsresult,

    /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
    pub setAsStringWithSize: unsafe extern "C" fn (this: *const nsIWritableVariant, size: uint32_t, str: *const libc::c_char) -> nsresult,

    /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
    pub setAsWStringWithSize: unsafe extern "C" fn (this: *const nsIWritableVariant, size: uint32_t, str: *const libc::int16_t) -> nsresult,

    /* void setAsVoid (); */
    pub setAsVoid: unsafe extern "C" fn (this: *const nsIWritableVariant) -> nsresult,

    /* void setAsEmpty (); */
    pub setAsEmpty: unsafe extern "C" fn (this: *const nsIWritableVariant) -> nsresult,

    /* void setAsEmptyArray (); */
    pub setAsEmptyArray: unsafe extern "C" fn (this: *const nsIWritableVariant) -> nsresult,

    /* void setFromVariant (in nsIVariant aValue); */
    pub setFromVariant: unsafe extern "C" fn (this: *const nsIWritableVariant, aValue: *const nsIVariant) -> nsresult,

}


impl nsIWritableVariant {
    /* attribute boolean writable; */
    #[inline]
    pub unsafe fn get_writable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_writable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_writable(&self, aWritable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_writable)(self as *const _, aWritable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsInt8 (in uint8_t aValue); */
    #[inline]
    pub unsafe fn setAsInt8(&self, aValue: uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsInt8)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsInt16 (in int16_t aValue); */
    #[inline]
    pub unsafe fn setAsInt16(&self, aValue: int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsInt16)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsInt32 (in int32_t aValue); */
    #[inline]
    pub unsafe fn setAsInt32(&self, aValue: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsInt32)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsInt64 (in int64_t aValue); */
    #[inline]
    pub unsafe fn setAsInt64(&self, aValue: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsInt64)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsUint8 (in uint8_t aValue); */
    #[inline]
    pub unsafe fn setAsUint8(&self, aValue: uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsUint8)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsUint16 (in uint16_t aValue); */
    #[inline]
    pub unsafe fn setAsUint16(&self, aValue: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsUint16)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsUint32 (in uint32_t aValue); */
    #[inline]
    pub unsafe fn setAsUint32(&self, aValue: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsUint32)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsUint64 (in uint64_t aValue); */
    #[inline]
    pub unsafe fn setAsUint64(&self, aValue: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsUint64)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsFloat (in float aValue); */
    #[inline]
    pub unsafe fn setAsFloat(&self, aValue: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setAsFloat)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsDouble (in double aValue); */
    #[inline]
    pub unsafe fn setAsDouble(&self, aValue: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).setAsDouble)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsBool (in boolean aValue); */
    #[inline]
    pub unsafe fn setAsBool(&self, aValue: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setAsBool)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsChar (in char aValue); */
    #[inline]
    pub unsafe fn setAsChar(&self, aValue: libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setAsChar)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsWChar (in wchar aValue); */
    #[inline]
    pub unsafe fn setAsWChar(&self, aValue: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsWChar)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsID (in nsIDRef aValue); */
    #[inline]
    pub unsafe fn setAsID(&self, aValue: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).setAsID)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsAString (in AString aValue); */
    #[inline]
    pub unsafe fn setAsAString(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setAsAString)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsDOMString (in DOMString aValue); */
    #[inline]
    pub unsafe fn setAsDOMString(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setAsDOMString)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsACString (in ACString aValue); */
    #[inline]
    pub unsafe fn setAsACString(&self, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).setAsACString)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsAUTF8String (in AUTF8String aValue); */
    #[inline]
    pub unsafe fn setAsAUTF8String(&self, aValue: &[u8]) -> Result<(), nsresult> {
        let aValue = nsCString::from(aValue);
        match ((*self.vtable).setAsAUTF8String)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsString (in string aValue); */
    #[inline]
    pub unsafe fn setAsString(&self, aValue: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setAsString)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsWString (in wstring aValue); */
    #[inline]
    pub unsafe fn setAsWString(&self, aValue: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsWString)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsISupports (in nsISupports aValue); */
    #[inline]
    pub unsafe fn setAsISupports(&self, aValue: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setAsISupports)(self as *const _, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsInterface (in nsIIDRef iid, [iid_is (iid)] in nsQIResult iface); */
    #[inline]
    pub unsafe fn setAsInterface(&self, iid: *const nsIID, iface: *const libc::c_void) -> Result<(), nsresult> {

        match ((*self.vtable).setAsInterface)(self as *const _, iid, iface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setAsArray (in uint16_t type, in nsIIDPtr iid, in uint32_t count, in voidPtr ptr); */
    #[inline]
    pub unsafe fn setAsArray(&self, type_: uint16_t, iid: *const nsIID, count: uint32_t, ptr: *const libc::c_void) -> Result<(), nsresult> {

        match ((*self.vtable).setAsArray)(self as *const _, type_, iid, count, ptr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsStringWithSize (in uint32_t size, [size_is (size)] in string str); */
    #[inline]
    pub unsafe fn setAsStringWithSize(&self, size: uint32_t, str: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setAsStringWithSize)(self as *const _, size, str) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsWStringWithSize (in uint32_t size, [size_is (size)] in wstring str); */
    #[inline]
    pub unsafe fn setAsWStringWithSize(&self, size: uint32_t, str: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setAsWStringWithSize)(self as *const _, size, str) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsVoid (); */
    #[inline]
    pub unsafe fn setAsVoid(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setAsVoid)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsEmpty (); */
    #[inline]
    pub unsafe fn setAsEmpty(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setAsEmpty)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsEmptyArray (); */
    #[inline]
    pub unsafe fn setAsEmptyArray(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setAsEmptyArray)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFromVariant (in nsIVariant aValue); */
    #[inline]
    pub unsafe fn setFromVariant(&self, aValue: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).setFromVariant)(self as *const _, aValue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


