//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLabelElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLLabelElement {
    vtable: *const nsIDOMHTMLLabelElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLLabelElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xefc0eaf2, 0x5756, 0x4388,
            [0xa2, 0x29, 0xfb, 0xec, 0x20, 0x33, 0x52, 0x9d])
    }
}

unsafe impl RefCounted for nsIDOMHTMLLabelElement {
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
pub trait nsIDOMHTMLLabelElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLLabelElement) -> &Self;
}

impl nsIDOMHTMLLabelElementCoerce for nsIDOMHTMLLabelElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLabelElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLLabelElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLLabelElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLLabelElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLLabelElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLabelElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLLabelElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLLabelElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString htmlFor; */
    pub get_htmlFor: unsafe extern "C" fn (this: *const nsIDOMHTMLLabelElement, aHtmlFor: *mut nsAString) -> nsresult,
    pub set_htmlFor: unsafe extern "C" fn (this: *const nsIDOMHTMLLabelElement, aHtmlFor: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLElement control; */
    pub get_control: unsafe extern "C" fn (this: *const nsIDOMHTMLLabelElement, aControl: *mut *const nsIDOMHTMLElement) -> nsresult,

}


impl nsIDOMHTMLLabelElement {
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

    /* attribute DOMString htmlFor; */
    #[inline]
    pub unsafe fn get_htmlFor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_htmlFor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_htmlFor(&self, aHtmlFor: &[u16]) -> Result<(), nsresult> {
        let aHtmlFor = nsString::from(aHtmlFor);
        match ((*self.vtable).set_htmlFor)(self as *const _, &*aHtmlFor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLElement control; */
    #[inline]
    pub unsafe fn get_control(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_control)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


