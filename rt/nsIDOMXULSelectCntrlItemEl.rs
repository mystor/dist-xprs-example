//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULSelectCntrlItemEl.idl
//


#[repr(C)]
pub struct nsIDOMXULSelectControlItemElement {
    vtable: *const nsIDOMXULSelectControlItemElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULSelectControlItemElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5c6be58f, 0x17df, 0x4750,
            [0x88, 0xa5, 0x4a, 0x59, 0xac, 0x28, 0xad, 0xc9])
    }
}

unsafe impl RefCounted for nsIDOMXULSelectControlItemElement {
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
pub trait nsIDOMXULSelectControlItemElementCoerce {
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self;
}

impl nsIDOMXULSelectControlItemElementCoerce for nsIDOMXULSelectControlItemElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self {
        v
    }
}

impl nsIDOMXULSelectControlItemElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULSelectControlItemElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULSelectControlItemElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULSelectControlItemElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULSelectControlItemElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULSelectControlItemElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aDisabled: bool) -> nsresult,

    /* attribute DOMString crop; */
    pub get_crop: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aCrop: *mut nsAString) -> nsresult,
    pub set_crop: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aCrop: *const nsAString) -> nsresult,

    /* attribute DOMString image; */
    pub get_image: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aImage: *mut nsAString) -> nsresult,
    pub set_image: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aImage: *const nsAString) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aLabel: *const nsAString) -> nsresult,

    /* attribute DOMString accessKey; */
    pub get_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aAccessKey: *mut nsAString) -> nsresult,
    pub set_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aAccessKey: *const nsAString) -> nsresult,

    /* attribute DOMString command; */
    pub get_command: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aCommand: *mut nsAString) -> nsresult,
    pub set_command: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aCommand: *const nsAString) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aValue: *const nsAString) -> nsresult,

    /* readonly attribute boolean selected; */
    pub get_selected: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aSelected: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMXULSelectControlElement control; */
    pub get_control: unsafe extern "C" fn (this: *const nsIDOMXULSelectControlItemElement, aControl: *mut *const nsIDOMXULSelectControlElement) -> nsresult,

}


impl nsIDOMXULSelectControlItemElement {
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

    /* attribute DOMString crop; */
    #[inline]
    pub unsafe fn get_crop(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_crop)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_crop(&self, aCrop: &[u16]) -> Result<(), nsresult> {
        let aCrop = nsString::from(aCrop);
        match ((*self.vtable).set_crop)(self as *const _, &*aCrop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString image; */
    #[inline]
    pub unsafe fn get_image(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_image)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_image(&self, aImage: &[u16]) -> Result<(), nsresult> {
        let aImage = nsString::from(aImage);
        match ((*self.vtable).set_image)(self as *const _, &*aImage) {
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

    /* attribute DOMString accessKey; */
    #[inline]
    pub unsafe fn get_accessKey(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_accessKey(&self, aAccessKey: &[u16]) -> Result<(), nsresult> {
        let aAccessKey = nsString::from(aAccessKey);
        match ((*self.vtable).set_accessKey)(self as *const _, &*aAccessKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString command; */
    #[inline]
    pub unsafe fn get_command(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_command)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_command(&self, aCommand: &[u16]) -> Result<(), nsresult> {
        let aCommand = nsString::from(aCommand);
        match ((*self.vtable).set_command)(self as *const _, &*aCommand) {
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

    /* readonly attribute boolean selected; */
    #[inline]
    pub unsafe fn get_selected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMXULSelectControlElement control; */
    #[inline]
    pub unsafe fn get_control(&self, ) -> Result<Option<RefPtr<nsIDOMXULSelectControlElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_control)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


