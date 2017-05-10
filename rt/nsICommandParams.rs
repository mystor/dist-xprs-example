//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandParams.idl
//


pub mod nsICommandParams_consts {
    pub const eNoType: i64 = 0;
    pub const eBooleanType: i64 = 1;
    pub const eLongType: i64 = 2;
    pub const eDoubleType: i64 = 3;
    pub const eWStringType: i64 = 4;
    pub const eISupportsType: i64 = 5;
    pub const eStringType: i64 = 6;
}


#[repr(C)]
pub struct nsICommandParams {
    vtable: *const nsICommandParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandParams {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb1fdf3c4, 0x74e3, 0x4f7d,
            [0xa1, 0x4d, 0x2b, 0x76, 0xbc, 0xf5, 0x34, 0x82])
    }
}

unsafe impl RefCounted for nsICommandParams {
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
pub trait nsICommandParamsCoerce {
    fn coerce_from(v: &nsICommandParams) -> &Self;
}

impl nsICommandParamsCoerce for nsICommandParams {
    #[inline]
    fn coerce_from(v: &nsICommandParams) -> &Self {
        v
    }
}

impl nsICommandParams {
    #[inline]
    pub fn coerce<T: nsICommandParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandParams {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandParams) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandParamsVTable {
    pub __base: nsISupportsVTable,

    /* short getValueType (in string name); */
    pub getValueType: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut libc::int16_t) -> nsresult,

    /* boolean getBooleanValue (in string name); */
    pub getBooleanValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* long getLongValue (in string name); */
    pub getLongValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut libc::int32_t) -> nsresult,

    /* double getDoubleValue (in string name); */
    pub getDoubleValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut libc::c_double) -> nsresult,

    /* AString getStringValue (in string name); */
    pub getStringValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut nsAString) -> nsresult,

    /* string getCStringValue (in string name); */
    pub getCStringValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut *const libc::c_char) -> nsresult,

    /* nsISupports getISupportsValue (in string name); */
    pub getISupportsValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, _retval: *mut *const nsISupports) -> nsresult,

    /* void setBooleanValue (in string name, in boolean value); */
    pub setBooleanValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: bool) -> nsresult,

    /* void setLongValue (in string name, in long value); */
    pub setLongValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: libc::int32_t) -> nsresult,

    /* void setDoubleValue (in string name, in double value); */
    pub setDoubleValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: libc::c_double) -> nsresult,

    /* void setStringValue (in string name, in AString value); */
    pub setStringValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const nsAString) -> nsresult,

    /* void setCStringValue (in string name, in string value); */
    pub setCStringValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const libc::c_char) -> nsresult,

    /* void setISupportsValue (in string name, in nsISupports value); */
    pub setISupportsValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char, value: *const nsISupports) -> nsresult,

    /* void removeValue (in string name); */
    pub removeValue: unsafe extern "C" fn (this: *const nsICommandParams, name: *const libc::c_char) -> nsresult,

}


impl nsICommandParams {
    /* short getValueType (in string name); */
    #[inline]
    pub unsafe fn getValueType(&self, name: *const libc::c_char) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getValueType)(self as *const _, name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean getBooleanValue (in string name); */
    #[inline]
    pub unsafe fn getBooleanValue(&self, name: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getBooleanValue)(self as *const _, name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long getLongValue (in string name); */
    #[inline]
    pub unsafe fn getLongValue(&self, name: *const libc::c_char) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLongValue)(self as *const _, name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* double getDoubleValue (in string name); */
    #[inline]
    pub unsafe fn getDoubleValue(&self, name: *const libc::c_char) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getDoubleValue)(self as *const _, name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getStringValue (in string name); */
    #[inline]
    pub unsafe fn getStringValue(&self, name: *const libc::c_char) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringValue)(self as *const _, name, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* string getCStringValue (in string name); */
    #[inline]
    pub unsafe fn getCStringValue(&self, name: *const libc::c_char) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).getCStringValue)(self as *const _, name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports getISupportsValue (in string name); */
    #[inline]
    pub unsafe fn getISupportsValue(&self, name: *const libc::c_char) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getISupportsValue)(self as *const _, name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setBooleanValue (in string name, in boolean value); */
    #[inline]
    pub unsafe fn setBooleanValue(&self, name: *const libc::c_char, value: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setBooleanValue)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setLongValue (in string name, in long value); */
    #[inline]
    pub unsafe fn setLongValue(&self, name: *const libc::c_char, value: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setLongValue)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDoubleValue (in string name, in double value); */
    #[inline]
    pub unsafe fn setDoubleValue(&self, name: *const libc::c_char, value: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).setDoubleValue)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setStringValue (in string name, in AString value); */
    #[inline]
    pub unsafe fn setStringValue(&self, name: *const libc::c_char, value: &[u16]) -> Result<(), nsresult> {
        let value = nsString::from(value);
        match ((*self.vtable).setStringValue)(self as *const _, name, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCStringValue (in string name, in string value); */
    #[inline]
    pub unsafe fn setCStringValue(&self, name: *const libc::c_char, value: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).setCStringValue)(self as *const _, name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setISupportsValue (in string name, in nsISupports value); */
    #[inline]
    pub unsafe fn setISupportsValue(&self, name: *const libc::c_char, value: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setISupportsValue)(self as *const _, name, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeValue (in string name); */
    #[inline]
    pub unsafe fn removeValue(&self, name: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeValue)(self as *const _, name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


