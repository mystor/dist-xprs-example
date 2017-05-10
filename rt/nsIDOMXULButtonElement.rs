//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULButtonElement.idl
//


pub mod nsIDOMXULButtonElement_consts {
    pub const CHECKSTATE_UNCHECKED: i64 = 0;
    pub const CHECKSTATE_CHECKED: i64 = 1;
    pub const CHECKSTATE_MIXED: i64 = 2;
}


#[repr(C)]
pub struct nsIDOMXULButtonElement {
    vtable: *const nsIDOMXULButtonElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULButtonElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd56f1f8f, 0xfc4e, 0x4650,
            [0x9a, 0x85, 0x25, 0x10, 0x8b, 0xbd, 0x19, 0x80])
    }
}

unsafe impl RefCounted for nsIDOMXULButtonElement {
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
pub trait nsIDOMXULButtonElementCoerce {
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self;
}

impl nsIDOMXULButtonElementCoerce for nsIDOMXULButtonElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self {
        v
    }
}

impl nsIDOMXULButtonElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULButtonElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULButtonElement {
    type Target = nsIDOMXULLabeledControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULLabeledControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULLabeledControlElementCoerce> nsIDOMXULButtonElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULButtonElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULButtonElementVTable {
    pub __base: nsIDOMXULLabeledControlElementVTable,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString dlgType; */
    pub get_dlgType: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aDlgType: *mut nsAString) -> nsresult,
    pub set_dlgType: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aDlgType: *const nsAString) -> nsresult,

    /* attribute boolean open; */
    pub get_open: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aOpen: *mut bool) -> nsresult,
    pub set_open: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aOpen: bool) -> nsresult,

    /* attribute boolean checked; */
    pub get_checked: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aChecked: *mut bool) -> nsresult,
    pub set_checked: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aChecked: bool) -> nsresult,

    /* attribute long checkState; */
    pub get_checkState: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aCheckState: *mut libc::int32_t) -> nsresult,
    pub set_checkState: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aCheckState: libc::int32_t) -> nsresult,

    /* attribute boolean autoCheck; */
    pub get_autoCheck: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aAutoCheck: *mut bool) -> nsresult,
    pub set_autoCheck: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aAutoCheck: bool) -> nsresult,

    /* attribute DOMString group; */
    pub get_group: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aGroup: *mut nsAString) -> nsresult,
    pub set_group: unsafe extern "C" fn (this: *const nsIDOMXULButtonElement, aGroup: *const nsAString) -> nsresult,

}


impl nsIDOMXULButtonElement {
    /* attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: &[u16]) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).set_type_)(self as *const _, &*aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString dlgType; */
    #[inline]
    pub unsafe fn get_dlgType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_dlgType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dlgType(&self, aDlgType: &[u16]) -> Result<(), nsresult> {
        let aDlgType = nsString::from(aDlgType);
        match ((*self.vtable).set_dlgType)(self as *const _, &*aDlgType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean open; */
    #[inline]
    pub unsafe fn get_open(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_open)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_open(&self, aOpen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_open)(self as *const _, aOpen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean checked; */
    #[inline]
    pub unsafe fn get_checked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_checked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_checked(&self, aChecked: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_checked)(self as *const _, aChecked) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long checkState; */
    #[inline]
    pub unsafe fn get_checkState(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_checkState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_checkState(&self, aCheckState: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_checkState)(self as *const _, aCheckState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean autoCheck; */
    #[inline]
    pub unsafe fn get_autoCheck(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_autoCheck)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_autoCheck(&self, aAutoCheck: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_autoCheck)(self as *const _, aAutoCheck) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString group; */
    #[inline]
    pub unsafe fn get_group(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_group)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_group(&self, aGroup: &[u16]) -> Result<(), nsresult> {
        let aGroup = nsString::from(aGroup);
        match ((*self.vtable).set_group)(self as *const _, &*aGroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


