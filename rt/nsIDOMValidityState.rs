//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMValidityState.idl
//


#[repr(C)]
pub struct nsIDOMValidityState {
    vtable: *const nsIDOMValidityStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMValidityState {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x00bed276, 0xf1f7, 0x492f,
            [0xa0, 0x39, 0xdb, 0xd9, 0xb9, 0xef, 0xc1, 0x0b])
    }
}

unsafe impl RefCounted for nsIDOMValidityState {
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
pub trait nsIDOMValidityStateCoerce {
    fn coerce_from(v: &nsIDOMValidityState) -> &Self;
}

impl nsIDOMValidityStateCoerce for nsIDOMValidityState {
    #[inline]
    fn coerce_from(v: &nsIDOMValidityState) -> &Self {
        v
    }
}

impl nsIDOMValidityState {
    #[inline]
    pub fn coerce<T: nsIDOMValidityStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMValidityState {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMValidityStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMValidityState) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMValidityStateVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean valueMissing; */
    pub get_valueMissing: unsafe extern "C" fn (this: *const nsIDOMValidityState, aValueMissing: *mut bool) -> nsresult,

    /* readonly attribute boolean typeMismatch; */
    pub get_typeMismatch: unsafe extern "C" fn (this: *const nsIDOMValidityState, aTypeMismatch: *mut bool) -> nsresult,

    /* readonly attribute boolean patternMismatch; */
    pub get_patternMismatch: unsafe extern "C" fn (this: *const nsIDOMValidityState, aPatternMismatch: *mut bool) -> nsresult,

    /* readonly attribute boolean tooLong; */
    pub get_tooLong: unsafe extern "C" fn (this: *const nsIDOMValidityState, aTooLong: *mut bool) -> nsresult,

    /* readonly attribute boolean tooShort; */
    pub get_tooShort: unsafe extern "C" fn (this: *const nsIDOMValidityState, aTooShort: *mut bool) -> nsresult,

    /* readonly attribute boolean rangeUnderflow; */
    pub get_rangeUnderflow: unsafe extern "C" fn (this: *const nsIDOMValidityState, aRangeUnderflow: *mut bool) -> nsresult,

    /* readonly attribute boolean rangeOverflow; */
    pub get_rangeOverflow: unsafe extern "C" fn (this: *const nsIDOMValidityState, aRangeOverflow: *mut bool) -> nsresult,

    /* readonly attribute boolean stepMismatch; */
    pub get_stepMismatch: unsafe extern "C" fn (this: *const nsIDOMValidityState, aStepMismatch: *mut bool) -> nsresult,

    /* readonly attribute boolean badInput; */
    pub get_badInput: unsafe extern "C" fn (this: *const nsIDOMValidityState, aBadInput: *mut bool) -> nsresult,

    /* readonly attribute boolean customError; */
    pub get_customError: unsafe extern "C" fn (this: *const nsIDOMValidityState, aCustomError: *mut bool) -> nsresult,

    /* readonly attribute boolean valid; */
    pub get_valid: unsafe extern "C" fn (this: *const nsIDOMValidityState, aValid: *mut bool) -> nsresult,

}


impl nsIDOMValidityState {
    /* readonly attribute boolean valueMissing; */
    #[inline]
    pub unsafe fn get_valueMissing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_valueMissing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean typeMismatch; */
    #[inline]
    pub unsafe fn get_typeMismatch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_typeMismatch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean patternMismatch; */
    #[inline]
    pub unsafe fn get_patternMismatch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_patternMismatch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean tooLong; */
    #[inline]
    pub unsafe fn get_tooLong(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_tooLong)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean tooShort; */
    #[inline]
    pub unsafe fn get_tooShort(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_tooShort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean rangeUnderflow; */
    #[inline]
    pub unsafe fn get_rangeUnderflow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rangeUnderflow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean rangeOverflow; */
    #[inline]
    pub unsafe fn get_rangeOverflow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_rangeOverflow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean stepMismatch; */
    #[inline]
    pub unsafe fn get_stepMismatch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_stepMismatch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean badInput; */
    #[inline]
    pub unsafe fn get_badInput(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_badInput)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean customError; */
    #[inline]
    pub unsafe fn get_customError(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_customError)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean valid; */
    #[inline]
    pub unsafe fn get_valid(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_valid)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


