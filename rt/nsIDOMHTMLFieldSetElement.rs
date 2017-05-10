//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFieldSetElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLFieldSetElement {
    vtable: *const nsIDOMHTMLFieldSetElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLFieldSetElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe3d91535, 0x9da3, 0x4c4b,
            [0xa8, 0x09, 0xf1, 0x7d, 0x85, 0xa4, 0xfb, 0x9f])
    }
}

unsafe impl RefCounted for nsIDOMHTMLFieldSetElement {
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
pub trait nsIDOMHTMLFieldSetElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLFieldSetElement) -> &Self;
}

impl nsIDOMHTMLFieldSetElementCoerce for nsIDOMHTMLFieldSetElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFieldSetElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLFieldSetElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLFieldSetElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLFieldSetElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLFieldSetElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFieldSetElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLFieldSetElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aName: *const nsAString) -> nsresult,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aType: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection elements; */
    pub get_elements: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aElements: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLFieldSetElement, error: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLFieldSetElement {
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

    /* readonly attribute DOMString type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_type_)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMHTMLCollection elements; */
    #[inline]
    pub unsafe fn get_elements(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_elements)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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


