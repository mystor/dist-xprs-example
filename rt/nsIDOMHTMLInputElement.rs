//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLInputElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLInputElement {
    vtable: *const nsIDOMHTMLInputElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLInputElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x64aeda0b, 0xe9b5, 0x4868,
            [0xa4, 0xf9, 0xe4, 0x77, 0x6e, 0x32, 0xe7, 0x33])
    }
}

unsafe impl RefCounted for nsIDOMHTMLInputElement {
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
pub trait nsIDOMHTMLInputElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLInputElement) -> &Self;
}

impl nsIDOMHTMLInputElementCoerce for nsIDOMHTMLInputElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLInputElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLInputElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLInputElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLInputElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLInputElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLInputElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLInputElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString accept; */
    pub get_accept: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAccept: *mut nsAString) -> nsresult,
    pub set_accept: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAccept: *const nsAString) -> nsresult,

    /* attribute DOMString alt; */
    pub get_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAlt: *mut nsAString) -> nsresult,
    pub set_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAlt: *const nsAString) -> nsresult,

    /* attribute DOMString autocomplete; */
    pub get_autocomplete: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAutocomplete: *mut nsAString) -> nsresult,
    pub set_autocomplete: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAutocomplete: *const nsAString) -> nsresult,

    /* attribute boolean autofocus; */
    pub get_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAutofocus: *mut bool) -> nsresult,
    pub set_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAutofocus: bool) -> nsresult,

    /* attribute boolean defaultChecked; */
    pub get_defaultChecked: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDefaultChecked: *mut bool) -> nsresult,
    pub set_defaultChecked: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDefaultChecked: bool) -> nsresult,

    /* attribute boolean checked; */
    pub get_checked: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aChecked: *mut bool) -> nsresult,
    pub set_checked: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aChecked: bool) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString formAction; */
    pub get_formAction: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormAction: *mut nsAString) -> nsresult,
    pub set_formAction: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormAction: *const nsAString) -> nsresult,

    /* attribute DOMString formEnctype; */
    pub get_formEnctype: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormEnctype: *mut nsAString) -> nsresult,
    pub set_formEnctype: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormEnctype: *const nsAString) -> nsresult,

    /* attribute DOMString formMethod; */
    pub get_formMethod: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormMethod: *mut nsAString) -> nsresult,
    pub set_formMethod: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormMethod: *const nsAString) -> nsresult,

    /* attribute boolean formNoValidate; */
    pub get_formNoValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormNoValidate: *mut bool) -> nsresult,
    pub set_formNoValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormNoValidate: bool) -> nsresult,

    /* attribute DOMString formTarget; */
    pub get_formTarget: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormTarget: *mut nsAString) -> nsresult,
    pub set_formTarget: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFormTarget: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMFileList files; */
    pub get_files: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aFiles: *mut *const nsIDOMFileList) -> nsresult,

    /* attribute unsigned long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aHeight: *mut libc::uint32_t) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aHeight: libc::uint32_t) -> nsresult,

    /* attribute boolean indeterminate; */
    pub get_indeterminate: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aIndeterminate: *mut bool) -> nsresult,
    pub set_indeterminate: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aIndeterminate: bool) -> nsresult,

    /* attribute DOMString inputMode; */
    pub get_inputMode: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aInputMode: *mut nsAString) -> nsresult,
    pub set_inputMode: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aInputMode: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLElement list; */
    pub get_list: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aList: *mut *const nsIDOMHTMLElement) -> nsresult,

    /* attribute DOMString max; */
    pub get_max: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMax: *mut nsAString) -> nsresult,
    pub set_max: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMax: *const nsAString) -> nsresult,

    /* attribute long maxLength; */
    pub get_maxLength: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMaxLength: *mut libc::int32_t) -> nsresult,
    pub set_maxLength: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMaxLength: libc::int32_t) -> nsresult,

    /* attribute DOMString min; */
    pub get_min: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMin: *mut nsAString) -> nsresult,
    pub set_min: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMin: *const nsAString) -> nsresult,

    /* attribute long minLength; */
    pub get_minLength: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMinLength: *mut libc::int32_t) -> nsresult,
    pub set_minLength: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMinLength: libc::int32_t) -> nsresult,

    /* attribute boolean multiple; */
    pub get_multiple: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMultiple: *mut bool) -> nsresult,
    pub set_multiple: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aMultiple: bool) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString pattern; */
    pub get_pattern: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aPattern: *mut nsAString) -> nsresult,
    pub set_pattern: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aPattern: *const nsAString) -> nsresult,

    /* attribute DOMString placeholder; */
    pub get_placeholder: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aPlaceholder: *mut nsAString) -> nsresult,
    pub set_placeholder: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aPlaceholder: *const nsAString) -> nsresult,

    /* attribute boolean readOnly; */
    pub get_readOnly: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aReadOnly: *mut bool) -> nsresult,
    pub set_readOnly: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aReadOnly: bool) -> nsresult,

    /* attribute boolean required; */
    pub get_required: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aRequired: *mut bool) -> nsresult,
    pub set_required: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aRequired: bool) -> nsresult,

    /* attribute DOMString step; */
    pub get_step: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aStep: *mut nsAString) -> nsresult,
    pub set_step: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aStep: *const nsAString) -> nsresult,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aAlign: *const nsAString) -> nsresult,

    /* attribute unsigned long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aSize: *mut libc::uint32_t) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aSize: libc::uint32_t) -> nsresult,

    /* attribute unsigned long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aWidth: *mut libc::uint32_t) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aWidth: libc::uint32_t) -> nsresult,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aSrc: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString defaultValue; */
    pub get_defaultValue: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDefaultValue: *mut nsAString) -> nsresult,
    pub set_defaultValue: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aDefaultValue: *const nsAString) -> nsresult,

    /* [optional_argc] void stepDown ([optional] in long n); */
    /// Unable to call function as its signature contains a non-rust type
    pub stepDown: *const ::libc::c_void,

    /* [optional_argc] void stepUp ([optional] in long n); */
    /// Unable to call function as its signature contains a non-rust type
    pub stepUp: *const ::libc::c_void,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, error: *const nsAString) -> nsresult,

    /* void select (); */
    pub select: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement) -> nsresult,

    /* attribute DOMString useMap; */
    pub get_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aUseMap: *mut nsAString) -> nsresult,
    pub set_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aUseMap: *const nsAString) -> nsresult,

    /* readonly attribute nsIControllers controllers; */
    pub get_controllers: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aControllers: *mut *const nsIControllers) -> nsresult,

    /* boolean mozIsTextField (in boolean aExcludePassword); */
    pub mozIsTextField: unsafe extern "C" fn (this: *const nsIDOMHTMLInputElement, aExcludePassword: bool, _retval: *mut bool) -> nsresult,

}


impl nsIDOMHTMLInputElement {
    /* attribute DOMString accept; */
    #[inline]
    pub unsafe fn get_accept(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accept)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_accept(&self, aAccept: &[u16]) -> Result<(), nsresult> {
        let aAccept = nsString::from(aAccept);
        match ((*self.vtable).set_accept)(self as *const _, &*aAccept) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString alt; */
    #[inline]
    pub unsafe fn get_alt(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alt)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alt(&self, aAlt: &[u16]) -> Result<(), nsresult> {
        let aAlt = nsString::from(aAlt);
        match ((*self.vtable).set_alt)(self as *const _, &*aAlt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString autocomplete; */
    #[inline]
    pub unsafe fn get_autocomplete(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_autocomplete)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_autocomplete(&self, aAutocomplete: &[u16]) -> Result<(), nsresult> {
        let aAutocomplete = nsString::from(aAutocomplete);
        match ((*self.vtable).set_autocomplete)(self as *const _, &*aAutocomplete) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* readonly attribute nsIDOMFileList files; */
    #[inline]
    pub unsafe fn get_files(&self, ) -> Result<Option<RefPtr<nsIDOMFileList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_files)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute unsigned long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_height)(self as *const _, aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean indeterminate; */
    #[inline]
    pub unsafe fn get_indeterminate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_indeterminate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_indeterminate(&self, aIndeterminate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_indeterminate)(self as *const _, aIndeterminate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString inputMode; */
    #[inline]
    pub unsafe fn get_inputMode(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_inputMode)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_inputMode(&self, aInputMode: &[u16]) -> Result<(), nsresult> {
        let aInputMode = nsString::from(aInputMode);
        match ((*self.vtable).set_inputMode)(self as *const _, &*aInputMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLElement list; */
    #[inline]
    pub unsafe fn get_list(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_list)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute DOMString max; */
    #[inline]
    pub unsafe fn get_max(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_max)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_max(&self, aMax: &[u16]) -> Result<(), nsresult> {
        let aMax = nsString::from(aMax);
        match ((*self.vtable).set_max)(self as *const _, &*aMax) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* attribute DOMString min; */
    #[inline]
    pub unsafe fn get_min(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_min)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_min(&self, aMin: &[u16]) -> Result<(), nsresult> {
        let aMin = nsString::from(aMin);
        match ((*self.vtable).set_min)(self as *const _, &*aMin) {
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

    /* attribute boolean multiple; */
    #[inline]
    pub unsafe fn get_multiple(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_multiple)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_multiple(&self, aMultiple: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_multiple)(self as *const _, aMultiple) {
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

    /* attribute DOMString pattern; */
    #[inline]
    pub unsafe fn get_pattern(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_pattern)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_pattern(&self, aPattern: &[u16]) -> Result<(), nsresult> {
        let aPattern = nsString::from(aPattern);
        match ((*self.vtable).set_pattern)(self as *const _, &*aPattern) {
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

    /* attribute DOMString step; */
    #[inline]
    pub unsafe fn get_step(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_step)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_step(&self, aStep: &[u16]) -> Result<(), nsresult> {
        let aStep = nsString::from(aStep);
        match ((*self.vtable).set_step)(self as *const _, &*aStep) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString align; */
    #[inline]
    pub unsafe fn get_align(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_align)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_align(&self, aAlign: &[u16]) -> Result<(), nsresult> {
        let aAlign = nsString::from(aAlign);
        match ((*self.vtable).set_align)(self as *const _, &*aAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_size(&self, aSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_size)(self as *const _, aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_width)(self as *const _, aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString src; */
    #[inline]
    pub unsafe fn get_src(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_src)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_src(&self, aSrc: &[u16]) -> Result<(), nsresult> {
        let aSrc = nsString::from(aSrc);
        match ((*self.vtable).set_src)(self as *const _, &*aSrc) {
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

    /* [optional_argc] void stepDown ([optional] in long n); */


    /* [optional_argc] void stepUp ([optional] in long n); */


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

    /* attribute DOMString useMap; */
    #[inline]
    pub unsafe fn get_useMap(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_useMap)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useMap(&self, aUseMap: &[u16]) -> Result<(), nsresult> {
        let aUseMap = nsString::from(aUseMap);
        match ((*self.vtable).set_useMap)(self as *const _, &*aUseMap) {
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

    /* boolean mozIsTextField (in boolean aExcludePassword); */
    #[inline]
    pub unsafe fn mozIsTextField(&self, aExcludePassword: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).mozIsTextField)(self as *const _, aExcludePassword, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


