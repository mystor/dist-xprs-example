//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIASN1Object.idl
//


pub mod nsIASN1Object_consts {
    pub const ASN1_END_CONTENTS: i64 = 0;
    pub const ASN1_BOOLEAN: i64 = 1;
    pub const ASN1_INTEGER: i64 = 2;
    pub const ASN1_BIT_STRING: i64 = 3;
    pub const ASN1_OCTET_STRING: i64 = 4;
    pub const ASN1_NULL: i64 = 5;
    pub const ASN1_OBJECT_ID: i64 = 6;
    pub const ASN1_ENUMERATED: i64 = 10;
    pub const ASN1_UTF8_STRING: i64 = 12;
    pub const ASN1_SEQUENCE: i64 = 16;
    pub const ASN1_SET: i64 = 17;
    pub const ASN1_PRINTABLE_STRING: i64 = 19;
    pub const ASN1_T61_STRING: i64 = 20;
    pub const ASN1_IA5_STRING: i64 = 22;
    pub const ASN1_UTC_TIME: i64 = 23;
    pub const ASN1_GEN_TIME: i64 = 24;
    pub const ASN1_VISIBLE_STRING: i64 = 26;
    pub const ASN1_UNIVERSAL_STRING: i64 = 28;
    pub const ASN1_BMP_STRING: i64 = 30;
    pub const ASN1_HIGH_TAG_NUMBER: i64 = 31;
    pub const ASN1_CONTEXT_SPECIFIC: i64 = 32;
    pub const ASN1_APPLICATION: i64 = 33;
    pub const ASN1_PRIVATE: i64 = 34;
}


#[repr(C)]
pub struct nsIASN1Object {
    vtable: *const nsIASN1ObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIASN1Object {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xba8bf582, 0x1dd1, 0x11b2,
            [0x89, 0x8c, 0xf4, 0x02, 0x46, 0xbc, 0x9a, 0x63])
    }
}

unsafe impl RefCounted for nsIASN1Object {
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
pub trait nsIASN1ObjectCoerce {
    fn coerce_from(v: &nsIASN1Object) -> &Self;
}

impl nsIASN1ObjectCoerce for nsIASN1Object {
    #[inline]
    fn coerce_from(v: &nsIASN1Object) -> &Self {
        v
    }
}

impl nsIASN1Object {
    #[inline]
    pub fn coerce<T: nsIASN1ObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIASN1Object {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIASN1ObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIASN1Object) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIASN1ObjectVTable {
    pub __base: nsISupportsVTable,

    /* attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIASN1Object, aType: *mut libc::uint32_t) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIASN1Object, aType: libc::uint32_t) -> nsresult,

    /* attribute unsigned long tag; */
    pub get_tag: unsafe extern "C" fn (this: *const nsIASN1Object, aTag: *mut libc::uint32_t) -> nsresult,
    pub set_tag: unsafe extern "C" fn (this: *const nsIASN1Object, aTag: libc::uint32_t) -> nsresult,

    /* attribute AString displayName; */
    pub get_displayName: unsafe extern "C" fn (this: *const nsIASN1Object, aDisplayName: *mut nsAString) -> nsresult,
    pub set_displayName: unsafe extern "C" fn (this: *const nsIASN1Object, aDisplayName: *const nsAString) -> nsresult,

    /* attribute AString displayValue; */
    pub get_displayValue: unsafe extern "C" fn (this: *const nsIASN1Object, aDisplayValue: *mut nsAString) -> nsresult,
    pub set_displayValue: unsafe extern "C" fn (this: *const nsIASN1Object, aDisplayValue: *const nsAString) -> nsresult,

}


impl nsIASN1Object {
    /* attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_type_)(self as *const _, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long tag; */
    #[inline]
    pub unsafe fn get_tag(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tag)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_tag(&self, aTag: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_tag)(self as *const _, aTag) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString displayName; */
    #[inline]
    pub unsafe fn get_displayName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displayName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_displayName(&self, aDisplayName: &[u16]) -> Result<(), nsresult> {
        let aDisplayName = nsString::from(aDisplayName);
        match ((*self.vtable).set_displayName)(self as *const _, &*aDisplayName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString displayValue; */
    #[inline]
    pub unsafe fn get_displayValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_displayValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_displayValue(&self, aDisplayValue: &[u16]) -> Result<(), nsresult> {
        let aDisplayValue = nsString::from(aDisplayValue);
        match ((*self.vtable).set_displayValue)(self as *const _, &*aDisplayValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


