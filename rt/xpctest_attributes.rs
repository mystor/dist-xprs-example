//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/xpctest_attributes.idl
//


#[repr(C)]
pub struct nsIXPCTestObjectReadOnly {
    vtable: *const nsIXPCTestObjectReadOnlyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestObjectReadOnly {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x42fbd9f6, 0xb12d, 0x47ef,
            [0xb7, 0xa1, 0x02, 0xd7, 0x3c, 0x11, 0xfe, 0x53])
    }
}

unsafe impl RefCounted for nsIXPCTestObjectReadOnly {
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
pub trait nsIXPCTestObjectReadOnlyCoerce {
    fn coerce_from(v: &nsIXPCTestObjectReadOnly) -> &Self;
}

impl nsIXPCTestObjectReadOnlyCoerce for nsIXPCTestObjectReadOnly {
    #[inline]
    fn coerce_from(v: &nsIXPCTestObjectReadOnly) -> &Self {
        v
    }
}

impl nsIXPCTestObjectReadOnly {
    #[inline]
    pub fn coerce<T: nsIXPCTestObjectReadOnlyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestObjectReadOnly {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestObjectReadOnlyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestObjectReadOnly) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestObjectReadOnlyVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute string strReadOnly; */
    pub get_strReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aStrReadOnly: *mut *const libc::c_char) -> nsresult,

    /* readonly attribute boolean boolReadOnly; */
    pub get_boolReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aBoolReadOnly: *mut bool) -> nsresult,

    /* readonly attribute short shortReadOnly; */
    pub get_shortReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aShortReadOnly: *mut libc::int16_t) -> nsresult,

    /* readonly attribute long longReadOnly; */
    pub get_longReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aLongReadOnly: *mut libc::int32_t) -> nsresult,

    /* readonly attribute float floatReadOnly; */
    pub get_floatReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aFloatReadOnly: *mut libc::c_float) -> nsresult,

    /* readonly attribute char charReadOnly; */
    pub get_charReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aCharReadOnly: *mut libc::c_char) -> nsresult,

    /* readonly attribute PRTime timeReadOnly; */
    pub get_timeReadOnly: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadOnly, aTimeReadOnly: *mut PRTime) -> nsresult,

}


impl nsIXPCTestObjectReadOnly {
    /* readonly attribute string strReadOnly; */
    #[inline]
    pub unsafe fn get_strReadOnly(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_strReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean boolReadOnly; */
    #[inline]
    pub unsafe fn get_boolReadOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_boolReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute short shortReadOnly; */
    #[inline]
    pub unsafe fn get_shortReadOnly(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_shortReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long longReadOnly; */
    #[inline]
    pub unsafe fn get_longReadOnly(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_longReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute float floatReadOnly; */
    #[inline]
    pub unsafe fn get_floatReadOnly(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_floatReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute char charReadOnly; */
    #[inline]
    pub unsafe fn get_charReadOnly(&self, ) -> Result<libc::c_char, nsresult> {
        let mut _retval: libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_charReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime timeReadOnly; */
    #[inline]
    pub unsafe fn get_timeReadOnly(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_timeReadOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIXPCTestObjectReadWrite {
    vtable: *const nsIXPCTestObjectReadWriteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXPCTestObjectReadWrite {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf07529b0, 0xa479, 0x4954,
            [0xab, 0xa5, 0xab, 0x31, 0x42, 0xc6, 0xb1, 0xcb])
    }
}

unsafe impl RefCounted for nsIXPCTestObjectReadWrite {
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
pub trait nsIXPCTestObjectReadWriteCoerce {
    fn coerce_from(v: &nsIXPCTestObjectReadWrite) -> &Self;
}

impl nsIXPCTestObjectReadWriteCoerce for nsIXPCTestObjectReadWrite {
    #[inline]
    fn coerce_from(v: &nsIXPCTestObjectReadWrite) -> &Self {
        v
    }
}

impl nsIXPCTestObjectReadWrite {
    #[inline]
    pub fn coerce<T: nsIXPCTestObjectReadWriteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXPCTestObjectReadWrite {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXPCTestObjectReadWriteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCTestObjectReadWrite) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXPCTestObjectReadWriteVTable {
    pub __base: nsISupportsVTable,

    /* attribute string stringProperty; */
    pub get_stringProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aStringProperty: *mut *const libc::c_char) -> nsresult,
    pub set_stringProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aStringProperty: *const libc::c_char) -> nsresult,

    /* attribute boolean booleanProperty; */
    pub get_booleanProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aBooleanProperty: *mut bool) -> nsresult,
    pub set_booleanProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aBooleanProperty: bool) -> nsresult,

    /* attribute short shortProperty; */
    pub get_shortProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aShortProperty: *mut libc::int16_t) -> nsresult,
    pub set_shortProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aShortProperty: libc::int16_t) -> nsresult,

    /* attribute long longProperty; */
    pub get_longProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aLongProperty: *mut libc::int32_t) -> nsresult,
    pub set_longProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aLongProperty: libc::int32_t) -> nsresult,

    /* attribute float floatProperty; */
    pub get_floatProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aFloatProperty: *mut libc::c_float) -> nsresult,
    pub set_floatProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aFloatProperty: libc::c_float) -> nsresult,

    /* attribute char charProperty; */
    pub get_charProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aCharProperty: *mut libc::c_char) -> nsresult,
    pub set_charProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aCharProperty: libc::c_char) -> nsresult,

    /* attribute PRTime timeProperty; */
    pub get_timeProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aTimeProperty: *mut PRTime) -> nsresult,
    pub set_timeProperty: unsafe extern "C" fn (this: *const nsIXPCTestObjectReadWrite, aTimeProperty: PRTime) -> nsresult,

}


impl nsIXPCTestObjectReadWrite {
    /* attribute string stringProperty; */
    #[inline]
    pub unsafe fn get_stringProperty(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_stringProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_stringProperty(&self, aStringProperty: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_stringProperty)(self as *const _, aStringProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean booleanProperty; */
    #[inline]
    pub unsafe fn get_booleanProperty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_booleanProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_booleanProperty(&self, aBooleanProperty: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_booleanProperty)(self as *const _, aBooleanProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute short shortProperty; */
    #[inline]
    pub unsafe fn get_shortProperty(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_shortProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_shortProperty(&self, aShortProperty: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_shortProperty)(self as *const _, aShortProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long longProperty; */
    #[inline]
    pub unsafe fn get_longProperty(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_longProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_longProperty(&self, aLongProperty: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_longProperty)(self as *const _, aLongProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float floatProperty; */
    #[inline]
    pub unsafe fn get_floatProperty(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_floatProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_floatProperty(&self, aFloatProperty: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_floatProperty)(self as *const _, aFloatProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute char charProperty; */
    #[inline]
    pub unsafe fn get_charProperty(&self, ) -> Result<libc::c_char, nsresult> {
        let mut _retval: libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_charProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charProperty(&self, aCharProperty: libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_charProperty)(self as *const _, aCharProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute PRTime timeProperty; */
    #[inline]
    pub unsafe fn get_timeProperty(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_timeProperty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeProperty(&self, aTimeProperty: PRTime) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeProperty)(self as *const _, aTimeProperty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


