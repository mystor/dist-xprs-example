//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULLabeledControlEl.idl
//


#[repr(C)]
pub struct nsIDOMXULLabeledControlElement {
    vtable: *const nsIDOMXULLabeledControlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULLabeledControlElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc58a159f, 0xe27d, 0x40c8,
            [0x86, 0x5a, 0xd4, 0xdc, 0xfd, 0x92, 0x8f, 0x62])
    }
}

unsafe impl RefCounted for nsIDOMXULLabeledControlElement {
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
pub trait nsIDOMXULLabeledControlElementCoerce {
    fn coerce_from(v: &nsIDOMXULLabeledControlElement) -> &Self;
}

impl nsIDOMXULLabeledControlElementCoerce for nsIDOMXULLabeledControlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULLabeledControlElement) -> &Self {
        v
    }
}

impl nsIDOMXULLabeledControlElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULLabeledControlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULLabeledControlElement {
    type Target = nsIDOMXULControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULControlElementCoerce> nsIDOMXULLabeledControlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULLabeledControlElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULLabeledControlElementVTable {
    pub __base: nsIDOMXULControlElementVTable,

    /* attribute DOMString crop; */
    pub get_crop: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aCrop: *mut nsAString) -> nsresult,
    pub set_crop: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aCrop: *const nsAString) -> nsresult,

    /* attribute DOMString image; */
    pub get_image: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aImage: *mut nsAString) -> nsresult,
    pub set_image: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aImage: *const nsAString) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aLabel: *const nsAString) -> nsresult,

    /* attribute DOMString accessKey; */
    pub get_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aAccessKey: *mut nsAString) -> nsresult,
    pub set_accessKey: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aAccessKey: *const nsAString) -> nsresult,

    /* attribute DOMString command; */
    pub get_command: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aCommand: *mut nsAString) -> nsresult,
    pub set_command: unsafe extern "C" fn (this: *const nsIDOMXULLabeledControlElement, aCommand: *const nsAString) -> nsresult,

}


impl nsIDOMXULLabeledControlElement {
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

}


