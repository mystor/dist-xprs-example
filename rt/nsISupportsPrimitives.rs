//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISupportsPrimitives.idl
//


pub mod nsISupportsPrimitive_consts {
    pub const TYPE_ID: i64 = 1;
    pub const TYPE_CSTRING: i64 = 2;
    pub const TYPE_STRING: i64 = 3;
    pub const TYPE_PRBOOL: i64 = 4;
    pub const TYPE_PRUINT8: i64 = 5;
    pub const TYPE_PRUINT16: i64 = 6;
    pub const TYPE_PRUINT32: i64 = 7;
    pub const TYPE_PRUINT64: i64 = 8;
    pub const TYPE_PRTIME: i64 = 9;
    pub const TYPE_CHAR: i64 = 10;
    pub const TYPE_PRINT16: i64 = 11;
    pub const TYPE_PRINT32: i64 = 12;
    pub const TYPE_PRINT64: i64 = 13;
    pub const TYPE_FLOAT: i64 = 14;
    pub const TYPE_DOUBLE: i64 = 15;
    pub const TYPE_INTERFACE_POINTER: i64 = 17;
}


#[repr(C)]
pub struct nsISupportsPrimitive {
    vtable: *const nsISupportsPrimitiveVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPrimitive {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd0d4b136, 0x1dd1, 0x11b2,
            [0x93, 0x71, 0xf0, 0x72, 0x7e, 0xf8, 0x27, 0xc0])
    }
}

unsafe impl RefCounted for nsISupportsPrimitive {
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
pub trait nsISupportsPrimitiveCoerce {
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self;
}

impl nsISupportsPrimitiveCoerce for nsISupportsPrimitive {
    #[inline]
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self {
        v
    }
}

impl nsISupportsPrimitive {
    #[inline]
    pub fn coerce<T: nsISupportsPrimitiveCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPrimitive {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISupportsPrimitiveCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPrimitive) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPrimitiveVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsISupportsPrimitive, aType: *mut libc::uint16_t) -> nsresult,

}


impl nsISupportsPrimitive {
    /* readonly attribute unsigned short type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsID {
    vtable: *const nsISupportsIDVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsID {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd18290a0, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsID {
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
pub trait nsISupportsIDCoerce {
    fn coerce_from(v: &nsISupportsID) -> &Self;
}

impl nsISupportsIDCoerce for nsISupportsID {
    #[inline]
    fn coerce_from(v: &nsISupportsID) -> &Self {
        v
    }
}

impl nsISupportsID {
    #[inline]
    pub fn coerce<T: nsISupportsIDCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsID {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsIDCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsID) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsIDVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute nsIDPtr data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsID, aData: *mut *const nsID) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsID, aData: *const nsID) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsID, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsID {
    /* attribute nsIDPtr data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<*const nsID, nsresult> {
        let mut _retval: *const nsID = ::std::ptr::null();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsCString {
    vtable: *const nsISupportsCStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsCString {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd65ff270, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsCString {
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
pub trait nsISupportsCStringCoerce {
    fn coerce_from(v: &nsISupportsCString) -> &Self;
}

impl nsISupportsCStringCoerce for nsISupportsCString {
    #[inline]
    fn coerce_from(v: &nsISupportsCString) -> &Self {
        v
    }
}

impl nsISupportsCString {
    #[inline]
    pub fn coerce<T: nsISupportsCStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsCString {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsCStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsCString) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsCStringVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute ACString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsCString, aData: *mut nsACString) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsCString, aData: *const nsACString) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsCString, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsCString {
    /* attribute ACString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: &[u8]) -> Result<(), nsresult> {
        let aData = nsCString::from(aData);
        match ((*self.vtable).set_data)(self as *const _, &*aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsString {
    vtable: *const nsISupportsStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsString {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd79dc970, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsString {
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
pub trait nsISupportsStringCoerce {
    fn coerce_from(v: &nsISupportsString) -> &Self;
}

impl nsISupportsStringCoerce for nsISupportsString {
    #[inline]
    fn coerce_from(v: &nsISupportsString) -> &Self {
        v
    }
}

impl nsISupportsString {
    #[inline]
    pub fn coerce<T: nsISupportsStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsString {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsString) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsStringVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute AString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsString, aData: *mut nsAString) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsString, aData: *const nsAString) -> nsresult,

    /* wstring toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsString, _retval: *mut *const libc::int16_t) -> nsresult,

}


impl nsISupportsString {
    /* attribute AString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: &[u16]) -> Result<(), nsresult> {
        let aData = nsString::from(aData);
        match ((*self.vtable).set_data)(self as *const _, &*aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRBool {
    vtable: *const nsISupportsPRBoolVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRBool {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xddc3b490, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRBool {
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
pub trait nsISupportsPRBoolCoerce {
    fn coerce_from(v: &nsISupportsPRBool) -> &Self;
}

impl nsISupportsPRBoolCoerce for nsISupportsPRBool {
    #[inline]
    fn coerce_from(v: &nsISupportsPRBool) -> &Self {
        v
    }
}

impl nsISupportsPRBool {
    #[inline]
    pub fn coerce<T: nsISupportsPRBoolCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRBool {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRBoolCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRBool) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRBoolVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute boolean data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRBool, aData: *mut bool) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRBool, aData: bool) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRBool, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRBool {
    /* attribute boolean data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRUint8 {
    vtable: *const nsISupportsPRUint8VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRUint8 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdec2e4e0, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRUint8 {
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
pub trait nsISupportsPRUint8Coerce {
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self;
}

impl nsISupportsPRUint8Coerce for nsISupportsPRUint8 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self {
        v
    }
}

impl nsISupportsPRUint8 {
    #[inline]
    pub fn coerce<T: nsISupportsPRUint8Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRUint8 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint8Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint8) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRUint8VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint8_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRUint8, aData: *mut uint8_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRUint8, aData: uint8_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRUint8, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRUint8 {
    /* attribute uint8_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRUint16 {
    vtable: *const nsISupportsPRUint16VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRUint16 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdfacb090, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRUint16 {
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
pub trait nsISupportsPRUint16Coerce {
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self;
}

impl nsISupportsPRUint16Coerce for nsISupportsPRUint16 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self {
        v
    }
}

impl nsISupportsPRUint16 {
    #[inline]
    pub fn coerce<T: nsISupportsPRUint16Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRUint16 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint16Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint16) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRUint16VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint16_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRUint16, aData: *mut uint16_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRUint16, aData: uint16_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRUint16, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRUint16 {
    /* attribute uint16_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRUint32 {
    vtable: *const nsISupportsPRUint32VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRUint32 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe01dc470, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRUint32 {
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
pub trait nsISupportsPRUint32Coerce {
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self;
}

impl nsISupportsPRUint32Coerce for nsISupportsPRUint32 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self {
        v
    }
}

impl nsISupportsPRUint32 {
    #[inline]
    pub fn coerce<T: nsISupportsPRUint32Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRUint32 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint32Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint32) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRUint32VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint32_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRUint32, aData: *mut uint32_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRUint32, aData: uint32_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRUint32, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRUint32 {
    /* attribute uint32_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRUint64 {
    vtable: *const nsISupportsPRUint64VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRUint64 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe13567c0, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRUint64 {
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
pub trait nsISupportsPRUint64Coerce {
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self;
}

impl nsISupportsPRUint64Coerce for nsISupportsPRUint64 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self {
        v
    }
}

impl nsISupportsPRUint64 {
    #[inline]
    pub fn coerce<T: nsISupportsPRUint64Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRUint64 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRUint64Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRUint64) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRUint64VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute uint64_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRUint64, aData: *mut uint64_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRUint64, aData: uint64_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRUint64, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRUint64 {
    /* attribute uint64_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: uint64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRTime {
    vtable: *const nsISupportsPRTimeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRTime {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe2563630, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRTime {
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
pub trait nsISupportsPRTimeCoerce {
    fn coerce_from(v: &nsISupportsPRTime) -> &Self;
}

impl nsISupportsPRTimeCoerce for nsISupportsPRTime {
    #[inline]
    fn coerce_from(v: &nsISupportsPRTime) -> &Self {
        v
    }
}

impl nsISupportsPRTime {
    #[inline]
    pub fn coerce<T: nsISupportsPRTimeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRTime {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRTimeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRTime) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRTimeVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute PRTime data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRTime, aData: *mut PRTime) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRTime, aData: PRTime) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRTime, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRTime {
    /* attribute PRTime data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsChar {
    vtable: *const nsISupportsCharVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsChar {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe2b05e40, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsChar {
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
pub trait nsISupportsCharCoerce {
    fn coerce_from(v: &nsISupportsChar) -> &Self;
}

impl nsISupportsCharCoerce for nsISupportsChar {
    #[inline]
    fn coerce_from(v: &nsISupportsChar) -> &Self {
        v
    }
}

impl nsISupportsChar {
    #[inline]
    pub fn coerce<T: nsISupportsCharCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsChar {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsCharCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsChar) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsCharVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute char data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsChar, aData: *mut libc::c_char) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsChar, aData: libc::c_char) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsChar, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsChar {
    /* attribute char data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<libc::c_char, nsresult> {
        let mut _retval: libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRInt16 {
    vtable: *const nsISupportsPRInt16VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRInt16 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe30d94b0, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRInt16 {
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
pub trait nsISupportsPRInt16Coerce {
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self;
}

impl nsISupportsPRInt16Coerce for nsISupportsPRInt16 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self {
        v
    }
}

impl nsISupportsPRInt16 {
    #[inline]
    pub fn coerce<T: nsISupportsPRInt16Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRInt16 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt16Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt16) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRInt16VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int16_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRInt16, aData: *mut int16_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRInt16, aData: int16_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRInt16, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRInt16 {
    /* attribute int16_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<int16_t, nsresult> {
        let mut _retval: int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRInt32 {
    vtable: *const nsISupportsPRInt32VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRInt32 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe36c5250, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRInt32 {
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
pub trait nsISupportsPRInt32Coerce {
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self;
}

impl nsISupportsPRInt32Coerce for nsISupportsPRInt32 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self {
        v
    }
}

impl nsISupportsPRInt32 {
    #[inline]
    pub fn coerce<T: nsISupportsPRInt32Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRInt32 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt32Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt32) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRInt32VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int32_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRInt32, aData: *mut int32_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRInt32, aData: int32_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRInt32, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRInt32 {
    /* attribute int32_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsPRInt64 {
    vtable: *const nsISupportsPRInt64VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsPRInt64 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe3cb0ff0, 0x4a1c, 0x11d3,
            [0x98, 0x90, 0x00, 0x60, 0x08, 0x96, 0x24, 0x22])
    }
}

unsafe impl RefCounted for nsISupportsPRInt64 {
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
pub trait nsISupportsPRInt64Coerce {
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self;
}

impl nsISupportsPRInt64Coerce for nsISupportsPRInt64 {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self {
        v
    }
}

impl nsISupportsPRInt64 {
    #[inline]
    pub fn coerce<T: nsISupportsPRInt64Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsPRInt64 {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsPRInt64Coerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPRInt64) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsPRInt64VTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute int64_t data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsPRInt64, aData: *mut int64_t) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsPRInt64, aData: int64_t) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsPRInt64, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsPRInt64 {
    /* attribute int64_t data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsFloat {
    vtable: *const nsISupportsFloatVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsFloat {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xabeaa390, 0x4ac0, 0x11d3,
            [0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7])
    }
}

unsafe impl RefCounted for nsISupportsFloat {
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
pub trait nsISupportsFloatCoerce {
    fn coerce_from(v: &nsISupportsFloat) -> &Self;
}

impl nsISupportsFloatCoerce for nsISupportsFloat {
    #[inline]
    fn coerce_from(v: &nsISupportsFloat) -> &Self {
        v
    }
}

impl nsISupportsFloat {
    #[inline]
    pub fn coerce<T: nsISupportsFloatCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsFloat {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsFloatCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsFloat) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsFloatVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute float data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsFloat, aData: *mut libc::c_float) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsFloat, aData: libc::c_float) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsFloat, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsFloat {
    /* attribute float data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsDouble {
    vtable: *const nsISupportsDoubleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsDouble {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb32523a0, 0x4ac0, 0x11d3,
            [0xba, 0xea, 0x00, 0x80, 0x5f, 0x8a, 0x5d, 0xd7])
    }
}

unsafe impl RefCounted for nsISupportsDouble {
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
pub trait nsISupportsDoubleCoerce {
    fn coerce_from(v: &nsISupportsDouble) -> &Self;
}

impl nsISupportsDoubleCoerce for nsISupportsDouble {
    #[inline]
    fn coerce_from(v: &nsISupportsDouble) -> &Self {
        v
    }
}

impl nsISupportsDouble {
    #[inline]
    pub fn coerce<T: nsISupportsDoubleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsDouble {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsDoubleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsDouble) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsDoubleVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute double data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsDouble, aData: *mut libc::c_double) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsDouble, aData: libc::c_double) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsDouble, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsDouble {
    /* attribute double data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_data)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISupportsInterfacePointer {
    vtable: *const nsISupportsInterfacePointerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISupportsInterfacePointer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x995ea724, 0x1dd1, 0x11b2,
            [0x92, 0x11, 0xc2, 0x1b, 0xdd, 0x3e, 0x7e, 0xd0])
    }
}

unsafe impl RefCounted for nsISupportsInterfacePointer {
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
pub trait nsISupportsInterfacePointerCoerce {
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self;
}

impl nsISupportsInterfacePointerCoerce for nsISupportsInterfacePointer {
    #[inline]
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self {
        v
    }
}

impl nsISupportsInterfacePointer {
    #[inline]
    pub fn coerce<T: nsISupportsInterfacePointerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISupportsInterfacePointer {
    type Target = nsISupportsPrimitive;
    #[inline]
    fn deref(&self) -> &nsISupportsPrimitive {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsPrimitiveCoerce> nsISupportsInterfacePointerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsInterfacePointer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISupportsInterfacePointerVTable {
    pub __base: nsISupportsPrimitiveVTable,

    /* attribute nsISupports data; */
    pub get_data: unsafe extern "C" fn (this: *const nsISupportsInterfacePointer, aData: *mut *const nsISupports) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsISupportsInterfacePointer, aData: *const nsISupports) -> nsresult,

    /* attribute nsIDPtr dataIID; */
    pub get_dataIID: unsafe extern "C" fn (this: *const nsISupportsInterfacePointer, aDataIID: *mut *const nsID) -> nsresult,
    pub set_dataIID: unsafe extern "C" fn (this: *const nsISupportsInterfacePointer, aDataIID: *const nsID) -> nsresult,

    /* string toString (); */
    pub toString: unsafe extern "C" fn (this: *const nsISupportsInterfacePointer, _retval: *mut *const libc::c_char) -> nsresult,

}


impl nsISupportsInterfacePointer {
    /* attribute nsISupports data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_data)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_data)(self as *const _, aData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDPtr dataIID; */
    #[inline]
    pub unsafe fn get_dataIID(&self, ) -> Result<*const nsID, nsresult> {
        let mut _retval: *const nsID = ::std::ptr::null();
        match ((*self.vtable).get_dataIID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dataIID(&self, aDataIID: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).set_dataIID)(self as *const _, aDataIID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* string toString (); */
    #[inline]
    pub unsafe fn toString(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).toString)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


