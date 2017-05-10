//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULTextboxElement.idl
//


#[repr(C)]
pub struct nsIDOMXULTextBoxElement {
    vtable: *const nsIDOMXULTextBoxElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULTextBoxElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7edd8215, 0x5155, 0x4845,
            [0xa0, 0x2f, 0xdc, 0x2c, 0x08, 0x64, 0x5c, 0xb9])
    }
}

unsafe impl RefCounted for nsIDOMXULTextBoxElement {
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
pub trait nsIDOMXULTextBoxElementCoerce {
    fn coerce_from(v: &nsIDOMXULTextBoxElement) -> &Self;
}

impl nsIDOMXULTextBoxElementCoerce for nsIDOMXULTextBoxElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULTextBoxElement) -> &Self {
        v
    }
}

impl nsIDOMXULTextBoxElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULTextBoxElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULTextBoxElement {
    type Target = nsIDOMXULControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULControlElementCoerce> nsIDOMXULTextBoxElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULTextBoxElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULTextBoxElementVTable {
    pub __base: nsIDOMXULControlElementVTable,

    /* readonly attribute nsIDOMNode inputField; */
    pub get_inputField: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aInputField: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long textLength; */
    pub get_textLength: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aTextLength: *mut libc::int32_t) -> nsresult,

    /* attribute long maxLength; */
    pub get_maxLength: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aMaxLength: *mut libc::int32_t) -> nsresult,
    pub set_maxLength: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aMaxLength: libc::int32_t) -> nsresult,

    /* attribute long size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSize: *mut libc::int32_t) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSize: libc::int32_t) -> nsresult,

    /* attribute long selectionStart; */
    pub get_selectionStart: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSelectionStart: *mut libc::int32_t) -> nsresult,
    pub set_selectionStart: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSelectionStart: libc::int32_t) -> nsresult,

    /* attribute long selectionEnd; */
    pub get_selectionEnd: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSelectionEnd: *mut libc::int32_t) -> nsresult,
    pub set_selectionEnd: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aSelectionEnd: libc::int32_t) -> nsresult,

    /* attribute DOMString value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aValue: *mut nsAString) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aValue: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, aType: *const nsAString) -> nsresult,

    /* void select (); */
    pub select: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement) -> nsresult,

    /* void setSelectionRange (in long selectionStart, in long selectionEnd); */
    pub setSelectionRange: unsafe extern "C" fn (this: *const nsIDOMXULTextBoxElement, selectionStart: libc::int32_t, selectionEnd: libc::int32_t) -> nsresult,

}


impl nsIDOMXULTextBoxElement {
    /* readonly attribute nsIDOMNode inputField; */
    #[inline]
    pub unsafe fn get_inputField(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_inputField)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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

    /* attribute long size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_size)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_size(&self, aSize: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_size)(self as *const _, aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long selectionStart; */
    #[inline]
    pub unsafe fn get_selectionStart(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionStart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectionStart(&self, aSelectionStart: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectionStart)(self as *const _, aSelectionStart) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long selectionEnd; */
    #[inline]
    pub unsafe fn get_selectionEnd(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionEnd)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectionEnd(&self, aSelectionEnd: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectionEnd)(self as *const _, aSelectionEnd) {
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

    /* void select (); */
    #[inline]
    pub unsafe fn select(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).select)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSelectionRange (in long selectionStart, in long selectionEnd); */
    #[inline]
    pub unsafe fn setSelectionRange(&self, selectionStart: libc::int32_t, selectionEnd: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setSelectionRange)(self as *const _, selectionStart, selectionEnd) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


