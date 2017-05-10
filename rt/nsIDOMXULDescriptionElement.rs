//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULDescriptionElement.idl
//


#[repr(C)]
pub struct nsIDOMXULDescriptionElement {
    vtable: *const nsIDOMXULDescriptionElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULDescriptionElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x64c3500e, 0xe258, 0x4d49,
            [0xb7, 0xca, 0xc9, 0x3a, 0xb0, 0x93, 0x1c, 0xe4])
    }
}

unsafe impl RefCounted for nsIDOMXULDescriptionElement {
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
pub trait nsIDOMXULDescriptionElementCoerce {
    fn coerce_from(v: &nsIDOMXULDescriptionElement) -> &Self;
}

impl nsIDOMXULDescriptionElementCoerce for nsIDOMXULDescriptionElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULDescriptionElement) -> &Self {
        v
    }
}

impl nsIDOMXULDescriptionElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULDescriptionElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULDescriptionElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULDescriptionElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULDescriptionElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULDescriptionElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aDisabled: bool) -> nsresult,

    /* attribute boolean crop; */
    pub get_crop: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aCrop: *mut bool) -> nsresult,
    pub set_crop: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aCrop: bool) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMXULDescriptionElement, aValue: *const nsAString) -> nsresult,

}


impl nsIDOMXULDescriptionElement {
    /* attribute boolean disabled; */
    #[inline]
    pub unsafe fn get_disabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_disabled(&self, aDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_disabled)(self as *const _, aDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean crop; */
    #[inline]
    pub unsafe fn get_crop(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_crop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_crop(&self, aCrop: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_crop)(self as *const _, aCrop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_value)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).set_value)(self as *const _, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


