//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMenuItemElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLMenuItemElement {
    vtable: *const nsIDOMHTMLMenuItemElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLMenuItemElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x979d6e44, 0x5930, 0x4232,
            [0xb4, 0x05, 0x87, 0x39, 0x39, 0x65, 0x5c, 0x19])
    }
}

unsafe impl RefCounted for nsIDOMHTMLMenuItemElement {
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
pub trait nsIDOMHTMLMenuItemElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLMenuItemElement) -> &Self;
}

impl nsIDOMHTMLMenuItemElementCoerce for nsIDOMHTMLMenuItemElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMenuItemElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLMenuItemElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLMenuItemElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLMenuItemElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLMenuItemElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMenuItemElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLMenuItemElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aLabel: *const nsAString) -> nsresult,

    /* attribute DOMString icon; */
    pub get_icon: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aIcon: *mut nsAString) -> nsresult,
    pub set_icon: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aIcon: *const nsAString) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aDisabled: bool) -> nsresult,

    /* attribute boolean defaultChecked; */
    pub get_defaultChecked: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aDefaultChecked: *mut bool) -> nsresult,
    pub set_defaultChecked: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aDefaultChecked: bool) -> nsresult,

    /* attribute boolean checked; */
    pub get_checked: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aChecked: *mut bool) -> nsresult,
    pub set_checked: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aChecked: bool) -> nsresult,

    /* attribute DOMString radiogroup; */
    pub get_radiogroup: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aRadiogroup: *mut nsAString) -> nsresult,
    pub set_radiogroup: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuItemElement, aRadiogroup: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLMenuItemElement {
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

    /* attribute DOMString label; */
    #[inline]
    pub unsafe fn get_label(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_label)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_label(&self, aLabel: &[u16]) -> Result<(), nsresult> {
        let aLabel = nsString::from(aLabel);
        match ((*self.vtable).set_label)(self as *const _, &*aLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString icon; */
    #[inline]
    pub unsafe fn get_icon(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_icon)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_icon(&self, aIcon: &[u16]) -> Result<(), nsresult> {
        let aIcon = nsString::from(aIcon);
        match ((*self.vtable).set_icon)(self as *const _, &*aIcon) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* attribute boolean defaultChecked; */
    #[inline]
    pub unsafe fn get_defaultChecked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultChecked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultChecked(&self, aDefaultChecked: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultChecked)(self as *const _, aDefaultChecked) {
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

    /* attribute DOMString radiogroup; */
    #[inline]
    pub unsafe fn get_radiogroup(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_radiogroup)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_radiogroup(&self, aRadiogroup: &[u16]) -> Result<(), nsresult> {
        let aRadiogroup = nsString::from(aRadiogroup);
        match ((*self.vtable).set_radiogroup)(self as *const _, &*aRadiogroup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


