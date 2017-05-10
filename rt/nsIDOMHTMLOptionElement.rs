//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptionElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLOptionElement {
    vtable: *const nsIDOMHTMLOptionElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLOptionElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2b3e9ff, 0x6b36, 0x4158,
            [0xac, 0xe3, 0x05, 0xa9, 0xc5, 0xb8, 0xe1, 0xc1])
    }
}

unsafe impl RefCounted for nsIDOMHTMLOptionElement {
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
pub trait nsIDOMHTMLOptionElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLOptionElement) -> &Self;
}

impl nsIDOMHTMLOptionElementCoerce for nsIDOMHTMLOptionElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptionElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLOptionElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLOptionElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLOptionElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLOptionElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptionElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLOptionElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aDisabled: bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aLabel: *const nsAString) -> nsresult,

    /* attribute boolean defaultSelected; */
    pub get_defaultSelected: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aDefaultSelected: *mut bool) -> nsresult,
    pub set_defaultSelected: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aDefaultSelected: bool) -> nsresult,

    /* attribute boolean selected; */
    pub get_selected: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aSelected: *mut bool) -> nsresult,
    pub set_selected: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aSelected: bool) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aValue: *const nsAString) -> nsresult,

    /* attribute DOMString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aText: *mut nsAString) -> nsresult,
    pub set_text: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aText: *const nsAString) -> nsresult,

    /* readonly attribute long index; */
    pub get_index: unsafe extern "C" fn (this: *const nsIDOMHTMLOptionElement, aIndex: *mut libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLOptionElement {
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

    /* attribute boolean defaultSelected; */
    #[inline]
    pub unsafe fn get_defaultSelected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultSelected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultSelected(&self, aDefaultSelected: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultSelected)(self as *const _, aDefaultSelected) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean selected; */
    #[inline]
    pub unsafe fn get_selected(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_selected)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selected(&self, aSelected: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_selected)(self as *const _, aSelected) {
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

    /* attribute DOMString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_text(&self, aText: &[u16]) -> Result<(), nsresult> {
        let aText = nsString::from(aText);
        match ((*self.vtable).set_text)(self as *const _, &*aText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long index; */
    #[inline]
    pub unsafe fn get_index(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_index)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


