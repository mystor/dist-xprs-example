//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMSVGLength.idl
//


pub mod nsIDOMSVGLength_consts {
    pub const SVG_LENGTHTYPE_UNKNOWN: i64 = 0;
    pub const SVG_LENGTHTYPE_NUMBER: i64 = 1;
    pub const SVG_LENGTHTYPE_PERCENTAGE: i64 = 2;
    pub const SVG_LENGTHTYPE_EMS: i64 = 3;
    pub const SVG_LENGTHTYPE_EXS: i64 = 4;
    pub const SVG_LENGTHTYPE_PX: i64 = 5;
    pub const SVG_LENGTHTYPE_CM: i64 = 6;
    pub const SVG_LENGTHTYPE_MM: i64 = 7;
    pub const SVG_LENGTHTYPE_IN: i64 = 8;
    pub const SVG_LENGTHTYPE_PT: i64 = 9;
    pub const SVG_LENGTHTYPE_PC: i64 = 10;
}


#[repr(C)]
pub struct nsIDOMSVGLength {
    vtable: *const nsIDOMSVGLengthVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMSVGLength {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2596325c, 0xaed0, 0x487e,
            [0x96, 0xa1, 0x0a, 0x6d, 0x58, 0x9b, 0x9c, 0x6b])
    }
}

unsafe impl RefCounted for nsIDOMSVGLength {
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
pub trait nsIDOMSVGLengthCoerce {
    fn coerce_from(v: &nsIDOMSVGLength) -> &Self;
}

impl nsIDOMSVGLengthCoerce for nsIDOMSVGLength {
    #[inline]
    fn coerce_from(v: &nsIDOMSVGLength) -> &Self {
        v
    }
}

impl nsIDOMSVGLength {
    #[inline]
    pub fn coerce<T: nsIDOMSVGLengthCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMSVGLength {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMSVGLengthCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMSVGLength) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMSVGLengthVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short unitType; */
    pub get_unitType: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aUnitType: *mut libc::uint16_t) -> nsresult,

    /* attribute float value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValue: *mut libc::c_float) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValue: libc::c_float) -> nsresult,

    /* attribute float valueInSpecifiedUnits; */
    pub get_valueInSpecifiedUnits: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValueInSpecifiedUnits: *mut libc::c_float) -> nsresult,
    pub set_valueInSpecifiedUnits: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValueInSpecifiedUnits: libc::c_float) -> nsresult,

    /* attribute DOMString valueAsString; */
    pub get_valueAsString: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValueAsString: *mut nsAString) -> nsresult,
    pub set_valueAsString: unsafe extern "C" fn (this: *const nsIDOMSVGLength, aValueAsString: *const nsAString) -> nsresult,

    /* void newValueSpecifiedUnits (in unsigned short unitType, in float valueInSpecifiedUnits); */
    pub newValueSpecifiedUnits: unsafe extern "C" fn (this: *const nsIDOMSVGLength, unitType: libc::uint16_t, valueInSpecifiedUnits: libc::c_float) -> nsresult,

    /* void convertToSpecifiedUnits (in unsigned short unitType); */
    pub convertToSpecifiedUnits: unsafe extern "C" fn (this: *const nsIDOMSVGLength, unitType: libc::uint16_t) -> nsresult,

}


impl nsIDOMSVGLength {
    /* readonly attribute unsigned short unitType; */
    #[inline]
    pub unsafe fn get_unitType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_unitType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute float value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_value)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float valueInSpecifiedUnits; */
    #[inline]
    pub unsafe fn get_valueInSpecifiedUnits(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_valueInSpecifiedUnits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_valueInSpecifiedUnits(&self, aValueInSpecifiedUnits: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_valueInSpecifiedUnits)(self as *const _, aValueInSpecifiedUnits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString valueAsString; */
    #[inline]
    pub unsafe fn get_valueAsString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_valueAsString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_valueAsString(&self, aValueAsString: &[u16]) -> Result<(), nsresult> {
        let aValueAsString = nsString::from(aValueAsString);
        match ((*self.vtable).set_valueAsString)(self as *const _, &*aValueAsString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void newValueSpecifiedUnits (in unsigned short unitType, in float valueInSpecifiedUnits); */
    #[inline]
    pub unsafe fn newValueSpecifiedUnits(&self, unitType: libc::uint16_t, valueInSpecifiedUnits: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).newValueSpecifiedUnits)(self as *const _, unitType, valueInSpecifiedUnits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void convertToSpecifiedUnits (in unsigned short unitType); */
    #[inline]
    pub unsafe fn convertToSpecifiedUnits(&self, unitType: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).convertToSpecifiedUnits)(self as *const _, unitType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


