//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFormElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLFormElement {
    vtable: *const nsIDOMHTMLFormElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLFormElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xad9b2ad0, 0x9d29, 0x43f6,
            [0xb1, 0xa2, 0xa1, 0xfd, 0x24, 0x62, 0x7e, 0x6b])
    }
}

unsafe impl RefCounted for nsIDOMHTMLFormElement {
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
pub trait nsIDOMHTMLFormElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLFormElement) -> &Self;
}

impl nsIDOMHTMLFormElementCoerce for nsIDOMHTMLFormElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFormElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLFormElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLFormElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLFormElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLFormElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFormElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLFormElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString acceptCharset; */
    pub get_acceptCharset: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAcceptCharset: *mut nsAString) -> nsresult,
    pub set_acceptCharset: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAcceptCharset: *const nsAString) -> nsresult,

    /* attribute DOMString action; */
    pub get_action: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAction: *mut nsAString) -> nsresult,
    pub set_action: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAction: *const nsAString) -> nsresult,

    /* attribute DOMString autocomplete; */
    pub get_autocomplete: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAutocomplete: *mut nsAString) -> nsresult,
    pub set_autocomplete: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aAutocomplete: *const nsAString) -> nsresult,

    /* attribute DOMString enctype; */
    pub get_enctype: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aEnctype: *mut nsAString) -> nsresult,
    pub set_enctype: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aEnctype: *const nsAString) -> nsresult,

    /* attribute DOMString encoding; */
    pub get_encoding: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aEncoding: *mut nsAString) -> nsresult,
    pub set_encoding: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aEncoding: *const nsAString) -> nsresult,

    /* attribute DOMString method; */
    pub get_method: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aMethod: *mut nsAString) -> nsresult,
    pub set_method: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aMethod: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aName: *const nsAString) -> nsresult,

    /* attribute boolean noValidate; */
    pub get_noValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aNoValidate: *mut bool) -> nsresult,
    pub set_noValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aNoValidate: bool) -> nsresult,

    /* attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aTarget: *mut nsAString) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aTarget: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection elements; */
    pub get_elements: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aElements: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute long length; */
    pub get_length: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, aLength: *mut libc::int32_t) -> nsresult,

    /* void submit (); */
    pub submit: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLFormElement, _retval: *mut bool) -> nsresult,

}


impl nsIDOMHTMLFormElement {
    /* attribute DOMString acceptCharset; */
    #[inline]
    pub unsafe fn get_acceptCharset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_acceptCharset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_acceptCharset(&self, aAcceptCharset: &[u16]) -> Result<(), nsresult> {
        let aAcceptCharset = nsString::from(aAcceptCharset);
        match ((*self.vtable).set_acceptCharset)(self as *const _, &*aAcceptCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString action; */
    #[inline]
    pub unsafe fn get_action(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_action)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_action(&self, aAction: &[u16]) -> Result<(), nsresult> {
        let aAction = nsString::from(aAction);
        match ((*self.vtable).set_action)(self as *const _, &*aAction) {
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

    /* attribute DOMString enctype; */
    #[inline]
    pub unsafe fn get_enctype(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_enctype)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enctype(&self, aEnctype: &[u16]) -> Result<(), nsresult> {
        let aEnctype = nsString::from(aEnctype);
        match ((*self.vtable).set_enctype)(self as *const _, &*aEnctype) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString encoding; */
    #[inline]
    pub unsafe fn get_encoding(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_encoding)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_encoding(&self, aEncoding: &[u16]) -> Result<(), nsresult> {
        let aEncoding = nsString::from(aEncoding);
        match ((*self.vtable).set_encoding)(self as *const _, &*aEncoding) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString method; */
    #[inline]
    pub unsafe fn get_method(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_method)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_method(&self, aMethod: &[u16]) -> Result<(), nsresult> {
        let aMethod = nsString::from(aMethod);
        match ((*self.vtable).set_method)(self as *const _, &*aMethod) {
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

    /* attribute boolean noValidate; */
    #[inline]
    pub unsafe fn get_noValidate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noValidate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_noValidate(&self, aNoValidate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_noValidate)(self as *const _, aNoValidate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: &[u16]) -> Result<(), nsresult> {
        let aTarget = nsString::from(aTarget);
        match ((*self.vtable).set_target)(self as *const _, &*aTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* readonly attribute long length; */
    #[inline]
    pub unsafe fn get_length(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_length)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void submit (); */
    #[inline]
    pub unsafe fn submit(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).submit)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

}


