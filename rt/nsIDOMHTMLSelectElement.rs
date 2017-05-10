//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLSelectElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLSelectElement {
    vtable: *const nsIDOMHTMLSelectElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLSelectElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd8914a2d, 0x3556, 0x4b66,
            [0x91, 0x1c, 0xa8, 0x4c, 0x43, 0x94, 0xe7, 0xfa])
    }
}

unsafe impl RefCounted for nsIDOMHTMLSelectElement {
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
pub trait nsIDOMHTMLSelectElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLSelectElement) -> &Self;
}

impl nsIDOMHTMLSelectElementCoerce for nsIDOMHTMLSelectElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLSelectElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLSelectElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLSelectElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLSelectElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLSelectElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLSelectElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLSelectElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean autofocus; */
    pub get_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aAutofocus: *mut bool) -> nsresult,
    pub set_autofocus: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aAutofocus: bool) -> nsresult,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute boolean multiple; */
    pub get_multiple: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aMultiple: *mut bool) -> nsresult,
    pub set_multiple: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aMultiple: bool) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aName: *const nsAString) -> nsresult,

    /* attribute unsigned long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aSize: *mut libc::uint32_t) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aSize: libc::uint32_t) -> nsresult,

    /* readonly attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aType: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLOptionsCollection options; */
    pub get_options: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aOptions: *mut *const nsIDOMHTMLOptionsCollection) -> nsresult,

    /* attribute unsigned long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aLength: *mut libc::uint32_t) -> nsresult,
    pub set_length: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aLength: libc::uint32_t) -> nsresult,

    /* nsIDOMNode item (in unsigned long index); */
    pub item: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, index: libc::uint32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode namedItem (in DOMString name); */
    pub namedItem: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, name: *const nsAString, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void add (in nsIDOMHTMLElement element, [optional] in nsIVariant before) raises (DOMException); */
    pub add: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, element: *const nsIDOMHTMLElement, before: *const nsIVariant) -> nsresult,

    /* void remove (in long index); */
    pub remove: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, index: libc::int32_t) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection selectedOptions; */
    pub get_selectedOptions: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aSelectedOptions: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* attribute long selectedIndex; */
    pub get_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aSelectedIndex: *mut libc::int32_t) -> nsresult,
    pub set_selectedIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aSelectedIndex: libc::int32_t) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aValue: *const nsAString) -> nsresult,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, error: *const nsAString) -> nsresult,

    /* attribute boolean required; */
    pub get_required: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aRequired: *mut bool) -> nsresult,
    pub set_required: unsafe extern "C" fn (this: *const nsIDOMHTMLSelectElement, aRequired: bool) -> nsresult,

}


impl nsIDOMHTMLSelectElement {
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

    /* readonly attribute nsIDOMHTMLOptionsCollection options; */
    #[inline]
    pub unsafe fn get_options(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLOptionsCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_options)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute unsigned long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_length(&self, aLength: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_length)(self as *const _, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode item (in unsigned long index); */
    #[inline]
    pub unsafe fn item(&self, index: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).item)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode namedItem (in DOMString name); */
    #[inline]
    pub unsafe fn namedItem(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).namedItem)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void add (in nsIDOMHTMLElement element, [optional] in nsIVariant before) raises (DOMException); */
    #[inline]
    pub unsafe fn add(&self, element: Option<&nsIDOMHTMLElement>, before: Option<&nsIVariant>) -> Result<(), nsresult> {

        match ((*self.vtable).add)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), before.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void remove (in long index); */
    #[inline]
    pub unsafe fn remove(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).remove)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLCollection selectedOptions; */
    #[inline]
    pub unsafe fn get_selectedOptions(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedOptions)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute long selectedIndex; */
    #[inline]
    pub unsafe fn get_selectedIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectedIndex(&self, aSelectedIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedIndex)(self as *const _, aSelectedIndex) {
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

}


