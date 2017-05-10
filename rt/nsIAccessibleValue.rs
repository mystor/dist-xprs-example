//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleValue.idl
//


#[repr(C)]
pub struct nsIAccessibleValue {
    vtable: *const nsIAccessibleValueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleValue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x42a1e1dc, 0x58cf, 0x419d,
            [0xbf, 0xf0, 0xed, 0x33, 0x14, 0xc7, 0x00, 0x16])
    }
}

unsafe impl RefCounted for nsIAccessibleValue {
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
pub trait nsIAccessibleValueCoerce {
    fn coerce_from(v: &nsIAccessibleValue) -> &Self;
}

impl nsIAccessibleValueCoerce for nsIAccessibleValue {
    #[inline]
    fn coerce_from(v: &nsIAccessibleValue) -> &Self {
        v
    }
}

impl nsIAccessibleValue {
    #[inline]
    pub fn coerce<T: nsIAccessibleValueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleValue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleValueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleValue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleValueVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute double maximumValue; */
    pub get_maximumValue: unsafe extern "C" fn (this: *const nsIAccessibleValue, aMaximumValue: *mut libc::c_double) -> nsresult,

    /* readonly attribute double minimumValue; */
    pub get_minimumValue: unsafe extern "C" fn (this: *const nsIAccessibleValue, aMinimumValue: *mut libc::c_double) -> nsresult,

    /* attribute double currentValue; */
    pub get_currentValue: unsafe extern "C" fn (this: *const nsIAccessibleValue, aCurrentValue: *mut libc::c_double) -> nsresult,
    pub set_currentValue: unsafe extern "C" fn (this: *const nsIAccessibleValue, aCurrentValue: libc::c_double) -> nsresult,

    /* readonly attribute double minimumIncrement; */
    pub get_minimumIncrement: unsafe extern "C" fn (this: *const nsIAccessibleValue, aMinimumIncrement: *mut libc::c_double) -> nsresult,

}


impl nsIAccessibleValue {
    /* readonly attribute double maximumValue; */
    #[inline]
    pub unsafe fn get_maximumValue(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_maximumValue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute double minimumValue; */
    #[inline]
    pub unsafe fn get_minimumValue(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_minimumValue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute double currentValue; */
    #[inline]
    pub unsafe fn get_currentValue(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_currentValue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_currentValue(&self, aCurrentValue: libc::c_double) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentValue)(self as *const _, aCurrentValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute double minimumIncrement; */
    #[inline]
    pub unsafe fn get_minimumIncrement(&self, ) -> Result<libc::c_double, nsresult> {
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).get_minimumIncrement)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


