//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLButtonElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLButtonElement {
    vtable: *const nsIDOMHTMLButtonElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLButtonElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x44b7a468, 0x7dba, 0x4f0c,
            [0x9b, 0x4e, 0xee, 0x46, 0xdc, 0x0f, 0x26, 0xc7])
    }
}

unsafe impl RefCounted for nsIDOMHTMLButtonElement {
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
pub trait nsIDOMHTMLButtonElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLButtonElement) -> &Self;
}

impl nsIDOMHTMLButtonElementCoerce for nsIDOMHTMLButtonElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLButtonElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLButtonElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLButtonElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLButtonElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLButtonElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLButtonElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLButtonElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean autofocus; */
    pub get_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aAutofocus: *mut bool) -> nsresult,
    pub set_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aAutofocus: bool) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString formAction; */
    pub get_formAction: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormAction: *mut nsAString) -> nsresult,
    pub set_formAction: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormAction: *const nsAString) -> nsresult,

    /* attribute DOMString formEnctype; */
    pub get_formEnctype: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormEnctype: *mut nsAString) -> nsresult,
    pub set_formEnctype: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormEnctype: *const nsAString) -> nsresult,

    /* attribute DOMString formMethod; */
    pub get_formMethod: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormMethod: *mut nsAString) -> nsresult,
    pub set_formMethod: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormMethod: *const nsAString) -> nsresult,

    /* attribute boolean formNoValidate; */
    pub get_formNoValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormNoValidate: *mut bool) -> nsresult,
    pub set_formNoValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormNoValidate: bool) -> nsresult,

    /* attribute DOMString formTarget; */
    pub get_formTarget: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormTarget: *mut nsAString) -> nsresult,
    pub set_formTarget: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aFormTarget: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aValue: *const nsAString) -> nsresult,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLButtonElement, error: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLButtonElement {
    /* attribute boolean autofocus; */
    #[inline]
    pub unsafe fn get_autofocus(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_autofocus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_autofocus(&self, aAutofocus: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_autofocus)(self as *const _, aAutofocus) {
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

    /* readonly attribute nsIDOMHTMLFormElement form; */
    #[inline]
    pub unsafe fn get_form(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLFormElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_form)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute DOMString formAction; */
    #[inline]
    pub unsafe fn get_formAction(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_formAction)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formAction(&self, aFormAction: &[u16]) -> Result<(), nsresult> {
        let aFormAction = nsString::from(aFormAction);
        match ((*self.vtable).set_formAction)(self as *const _, &*aFormAction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString formEnctype; */
    #[inline]
    pub unsafe fn get_formEnctype(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_formEnctype)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formEnctype(&self, aFormEnctype: &[u16]) -> Result<(), nsresult> {
        let aFormEnctype = nsString::from(aFormEnctype);
        match ((*self.vtable).set_formEnctype)(self as *const _, &*aFormEnctype) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString formMethod; */
    #[inline]
    pub unsafe fn get_formMethod(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_formMethod)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formMethod(&self, aFormMethod: &[u16]) -> Result<(), nsresult> {
        let aFormMethod = nsString::from(aFormMethod);
        match ((*self.vtable).set_formMethod)(self as *const _, &*aFormMethod) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean formNoValidate; */
    #[inline]
    pub unsafe fn get_formNoValidate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_formNoValidate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formNoValidate(&self, aFormNoValidate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_formNoValidate)(self as *const _, aFormNoValidate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString formTarget; */
    #[inline]
    pub unsafe fn get_formTarget(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_formTarget)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_formTarget(&self, aFormTarget: &[u16]) -> Result<(), nsresult> {
        let aFormTarget = nsString::from(aFormTarget);
        match ((*self.vtable).set_formTarget)(self as *const _, &*aFormTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* readonly attribute boolean willValidate; */
    #[inline]
    pub unsafe fn get_willValidate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_willValidate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMValidityState validity; */
    #[inline]
    pub unsafe fn get_validity(&self, ) -> Result<Option<RefPtr<nsIDOMValidityState>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_validity)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString validationMessage; */
    #[inline]
    pub unsafe fn get_validationMessage(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_validationMessage)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean checkValidity (); */
    #[inline]
    pub unsafe fn checkValidity(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkValidity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCustomValidity (in DOMString error); */
    #[inline]
    pub unsafe fn setCustomValidity(&self, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).setCustomValidity)(self as *const _, &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


