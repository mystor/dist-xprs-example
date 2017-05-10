//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLHtmlElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLHtmlElement {
    vtable: *const nsIDOMHTMLHtmlElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLHtmlElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6a5d2ce7, 0x2c45, 0x43c1,
            [0xbd, 0xab, 0x9d, 0xf7, 0xa0, 0x6c, 0xae, 0xd1])
    }
}

unsafe impl RefCounted for nsIDOMHTMLHtmlElement {
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
pub trait nsIDOMHTMLHtmlElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLHtmlElement) -> &Self;
}

impl nsIDOMHTMLHtmlElementCoerce for nsIDOMHTMLHtmlElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHtmlElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLHtmlElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLHtmlElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLHtmlElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLHtmlElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHtmlElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLHtmlElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIDOMHTMLHtmlElement, aVersion: *mut nsAString) -> nsresult,
    pub set_version: unsafe extern "C" fn (this: *const nsIDOMHTMLHtmlElement, aVersion: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLHtmlElement {
    /* attribute DOMString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_version(&self, aVersion: &[u16]) -> Result<(), nsresult> {
        let aVersion = nsString::from(aVersion);
        match ((*self.vtable).set_version)(self as *const _, &*aVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


