//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLTextAreaElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLTextAreaElement {
    vtable: *const nsIDOMHTMLTextAreaElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLTextAreaElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a4aeb2e, 0xfcf3, 0x443e,
            [0xb0, 0x02, 0xca, 0x1c, 0x8e, 0xa3, 0x22, 0xe9])
    }
}

unsafe impl RefCounted for nsIDOMHTMLTextAreaElement {
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
pub trait nsIDOMHTMLTextAreaElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLTextAreaElement) -> &Self;
}

impl nsIDOMHTMLTextAreaElementCoerce for nsIDOMHTMLTextAreaElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLTextAreaElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLTextAreaElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLTextAreaElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLTextAreaElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLTextAreaElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLTextAreaElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLTextAreaElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean autofocus; */
    pub get_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aAutofocus: *mut bool) -> nsresult,
    pub set_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aAutofocus: bool) -> nsresult,

    /* attribute unsigned long cols; */
    pub get_cols: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aCols: *mut libc::uint32_t) -> nsresult,
    pub set_cols: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aCols: libc::uint32_t) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute long maxLength; */
    pub get_maxLength: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aMaxLength: *mut libc::int32_t) -> nsresult,
    pub set_maxLength: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aMaxLength: libc::int32_t) -> nsresult,

    /* attribute long minLength; */
    pub get_minLength: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aMinLength: *mut libc::int32_t) -> nsresult,
    pub set_minLength: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aMinLength: libc::int32_t) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString placeholder; */
    pub get_placeholder: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aPlaceholder: *mut nsAString) -> nsresult,
    pub set_placeholder: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aPlaceholder: *const nsAString) -> nsresult,

    /* attribute boolean readOnly; */
    pub get_readOnly: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aReadOnly: *mut bool) -> nsresult,
    pub set_readOnly: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aReadOnly: bool) -> nsresult,

    /* attribute boolean required; */
    pub get_required: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aRequired: *mut bool) -> nsresult,
    pub set_required: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aRequired: bool) -> nsresult,

    /* attribute unsigned long rows; */
    pub get_rows: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aRows: *mut libc::uint32_t) -> nsresult,
    pub set_rows: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aRows: libc::uint32_t) -> nsresult,

    /* [Null(Stringify)] attribute DOMString wrap; */
    pub get_wrap: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aWrap: *mut nsAString) -> nsresult,
    pub set_wrap: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aWrap: *const nsAString) -> nsresult,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aType: *mut nsAString) -> nsresult,

    /* attribute DOMString defaultValue; */
    pub get_defaultValue: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aDefaultValue: *mut nsAString) -> nsresult,
    pub set_defaultValue: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aDefaultValue: *const nsAString) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aValue: *const nsAString) -> nsresult,

    /* readonly attribute long textLength; */
    pub get_textLength: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aTextLength: *mut libc::int32_t) -> nsresult,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, error: *const nsAString) -> nsresult,

    /* void select (); */
    pub select: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement) -> nsresult,

    /* readonly attribute nsIControllers controllers; */
    pub get_controllers: unsafe extern "C" fn (this: *const nsIDOMHTMLTextAreaElement, aControllers: *mut *const nsIControllers) -> nsresult,

}


impl nsIDOMHTMLTextAreaElement {
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

    /* attribute unsigned long cols; */
    #[inline]
    pub unsafe fn get_cols(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cols)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cols(&self, aCols: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_cols)(self as *const _, aCols) {
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

    /* attribute long maxLength; */
    #[inline]
    pub unsafe fn get_maxLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxLength(&self, aMaxLength: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxLength)(self as *const _, aMaxLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long minLength; */
    #[inline]
    pub unsafe fn get_minLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_minLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_minLength(&self, aMinLength: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_minLength)(self as *const _, aMinLength) {
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

    /* attribute DOMString placeholder; */
    #[inline]
    pub unsafe fn get_placeholder(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_placeholder)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_placeholder(&self, aPlaceholder: &[u16]) -> Result<(), nsresult> {
        let aPlaceholder = nsString::from(aPlaceholder);
        match ((*self.vtable).set_placeholder)(self as *const _, &*aPlaceholder) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean readOnly; */
    #[inline]
    pub unsafe fn get_readOnly(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_readOnly)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_readOnly(&self, aReadOnly: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_readOnly)(self as *const _, aReadOnly) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean required; */
    #[inline]
    pub unsafe fn get_required(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_required)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_required(&self, aRequired: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_required)(self as *const _, aRequired) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long rows; */
    #[inline]
    pub unsafe fn get_rows(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_rows)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rows(&self, aRows: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_rows)(self as *const _, aRows) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [Null(Stringify)] attribute DOMString wrap; */
    #[inline]
    pub unsafe fn get_wrap(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_wrap)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_wrap(&self, aWrap: &[u16]) -> Result<(), nsresult> {
        let aWrap = nsString::from(aWrap);
        match ((*self.vtable).set_wrap)(self as *const _, &*aWrap) {
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

    /* attribute DOMString defaultValue; */
    #[inline]
    pub unsafe fn get_defaultValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_defaultValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultValue(&self, aDefaultValue: &[u16]) -> Result<(), nsresult> {
        let aDefaultValue = nsString::from(aDefaultValue);
        match ((*self.vtable).set_defaultValue)(self as *const _, &*aDefaultValue) {
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

    /* readonly attribute long textLength; */
    #[inline]
    pub unsafe fn get_textLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_textLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* void select (); */
    #[inline]
    pub unsafe fn select(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).select)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIControllers controllers; */
    #[inline]
    pub unsafe fn get_controllers(&self, ) -> Result<Option<RefPtr<nsIControllers>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_controllers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


