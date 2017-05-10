//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMCSSPrimitiveValue.idl
//


pub mod nsIDOMCSSPrimitiveValue_consts {
    pub const CSS_UNKNOWN: i64 = 0;
    pub const CSS_NUMBER: i64 = 1;
    pub const CSS_PERCENTAGE: i64 = 2;
    pub const CSS_EMS: i64 = 3;
    pub const CSS_EXS: i64 = 4;
    pub const CSS_PX: i64 = 5;
    pub const CSS_CM: i64 = 6;
    pub const CSS_MM: i64 = 7;
    pub const CSS_IN: i64 = 8;
    pub const CSS_PT: i64 = 9;
    pub const CSS_PC: i64 = 10;
    pub const CSS_DEG: i64 = 11;
    pub const CSS_RAD: i64 = 12;
    pub const CSS_GRAD: i64 = 13;
    pub const CSS_MS: i64 = 14;
    pub const CSS_S: i64 = 15;
    pub const CSS_HZ: i64 = 16;
    pub const CSS_KHZ: i64 = 17;
    pub const CSS_DIMENSION: i64 = 18;
    pub const CSS_STRING: i64 = 19;
    pub const CSS_URI: i64 = 20;
    pub const CSS_IDENT: i64 = 21;
    pub const CSS_ATTR: i64 = 22;
    pub const CSS_COUNTER: i64 = 23;
    pub const CSS_RECT: i64 = 24;
    pub const CSS_RGBCOLOR: i64 = 25;
}


#[repr(C)]
pub struct nsIDOMCSSPrimitiveValue {
    vtable: *const nsIDOMCSSPrimitiveValueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMCSSPrimitiveValue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf6df7293, 0x2dc9, 0x4cb9,
            [0x95, 0x31, 0x77, 0x8c, 0xaf, 0x43, 0x70, 0xe0])
    }
}

unsafe impl RefCounted for nsIDOMCSSPrimitiveValue {
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
pub trait nsIDOMCSSPrimitiveValueCoerce {
    fn coerce_from(v: &nsIDOMCSSPrimitiveValue) -> &Self;
}

impl nsIDOMCSSPrimitiveValueCoerce for nsIDOMCSSPrimitiveValue {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSPrimitiveValue) -> &Self {
        v
    }
}

impl nsIDOMCSSPrimitiveValue {
    #[inline]
    pub fn coerce<T: nsIDOMCSSPrimitiveValueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMCSSPrimitiveValue {
    type Target = nsIDOMCSSValue;
    #[inline]
    fn deref(&self) -> &nsIDOMCSSValue {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMCSSValueCoerce> nsIDOMCSSPrimitiveValueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMCSSPrimitiveValue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMCSSPrimitiveValueVTable {
    pub __base: nsIDOMCSSValueVTable,

    /* readonly attribute unsigned short primitiveType; */
    pub get_primitiveType: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, aPrimitiveType: *mut libc::uint16_t) -> nsresult,

    /* void setFloatValue (in unsigned short unitType, in float floatValue) raises (DOMException); */
    pub setFloatValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, unitType: libc::uint16_t, floatValue: libc::c_float) -> nsresult,

    /* float getFloatValue (in unsigned short unitType) raises (DOMException); */
    pub getFloatValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, unitType: libc::uint16_t, _retval: *mut libc::c_float) -> nsresult,

    /* void setStringValue (in unsigned short stringType, in DOMString stringValue) raises (DOMException); */
    pub setStringValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, stringType: libc::uint16_t, stringValue: *const nsAString) -> nsresult,

    /* DOMString getStringValue () raises (DOMException); */
    pub getStringValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMCounter getCounterValue () raises (DOMException); */
    pub getCounterValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, _retval: *mut *const nsIDOMCounter) -> nsresult,

    /* nsIDOMRect getRectValue () raises (DOMException); */
    pub getRectValue: unsafe extern "C" fn (this: *const nsIDOMCSSPrimitiveValue, _retval: *mut *const nsIDOMRect) -> nsresult,

}


impl nsIDOMCSSPrimitiveValue {
    /* readonly attribute unsigned short primitiveType; */
    #[inline]
    pub unsafe fn get_primitiveType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_primitiveType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setFloatValue (in unsigned short unitType, in float floatValue) raises (DOMException); */
    #[inline]
    pub unsafe fn setFloatValue(&self, unitType: libc::uint16_t, floatValue: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).setFloatValue)(self as *const _, unitType, floatValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* float getFloatValue (in unsigned short unitType) raises (DOMException); */
    #[inline]
    pub unsafe fn getFloatValue(&self, unitType: libc::uint16_t) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).getFloatValue)(self as *const _, unitType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setStringValue (in unsigned short stringType, in DOMString stringValue) raises (DOMException); */
    #[inline]
    pub unsafe fn setStringValue(&self, stringType: libc::uint16_t, stringValue: &[u16]) -> Result<(), nsresult> {
        let stringValue = nsString::from(stringValue);
        match ((*self.vtable).setStringValue)(self as *const _, stringType, &*stringValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString getStringValue () raises (DOMException); */
    #[inline]
    pub unsafe fn getStringValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStringValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMCounter getCounterValue () raises (DOMException); */
    #[inline]
    pub unsafe fn getCounterValue(&self, ) -> Result<Option<RefPtr<nsIDOMCounter>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCounterValue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMRect getRectValue () raises (DOMException); */
    #[inline]
    pub unsafe fn getRectValue(&self, ) -> Result<Option<RefPtr<nsIDOMRect>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRectValue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


