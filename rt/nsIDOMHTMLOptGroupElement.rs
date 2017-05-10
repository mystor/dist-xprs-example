//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOptGroupElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLOptGroupElement {
    vtable: *const nsIDOMHTMLOptGroupElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLOptGroupElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6fa79f99, 0x4ce4, 0x4634,
            [0x84, 0x0a, 0x86, 0x7f, 0xcf, 0xb3, 0x2d, 0xba])
    }
}

unsafe impl RefCounted for nsIDOMHTMLOptGroupElement {
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
pub trait nsIDOMHTMLOptGroupElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLOptGroupElement) -> &Self;
}

impl nsIDOMHTMLOptGroupElementCoerce for nsIDOMHTMLOptGroupElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptGroupElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLOptGroupElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLOptGroupElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLOptGroupElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLOptGroupElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOptGroupElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLOptGroupElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean disabled; */
    pub get_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLOptGroupElement, aDisabled: *mut bool) -> nsresult,
    pub set_disabled: unsafe extern "C" fn (this: *const nsIDOMHTMLOptGroupElement, aDisabled: bool) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMHTMLOptGroupElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMHTMLOptGroupElement, aLabel: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLOptGroupElement {
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

}


