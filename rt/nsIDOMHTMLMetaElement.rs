//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMetaElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLMetaElement {
    vtable: *const nsIDOMHTMLMetaElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLMetaElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2a3f789e, 0x0667, 0x464f,
            [0xa8, 0xd7, 0x6f, 0x58, 0x51, 0x34, 0x43, 0xd9])
    }
}

unsafe impl RefCounted for nsIDOMHTMLMetaElement {
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
pub trait nsIDOMHTMLMetaElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLMetaElement) -> &Self;
}

impl nsIDOMHTMLMetaElementCoerce for nsIDOMHTMLMetaElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMetaElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLMetaElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLMetaElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLMetaElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLMetaElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMetaElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLMetaElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString content; */
    pub get_content: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aContent: *mut nsAString) -> nsresult,
    pub set_content: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aContent: *const nsAString) -> nsresult,

    /* attribute DOMString httpEquiv; */
    pub get_httpEquiv: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aHttpEquiv: *mut nsAString) -> nsresult,
    pub set_httpEquiv: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aHttpEquiv: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString scheme; */
    pub get_scheme: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aScheme: *mut nsAString) -> nsresult,
    pub set_scheme: unsafe extern "C" fn (this: *const nsIDOMHTMLMetaElement, aScheme: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLMetaElement {
    /* attribute DOMString content; */
    #[inline]
    pub unsafe fn get_content(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_content)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_content(&self, aContent: &[u16]) -> Result<(), nsresult> {
        let aContent = nsString::from(aContent);
        match ((*self.vtable).set_content)(self as *const _, &*aContent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString httpEquiv; */
    #[inline]
    pub unsafe fn get_httpEquiv(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_httpEquiv)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_httpEquiv(&self, aHttpEquiv: &[u16]) -> Result<(), nsresult> {
        let aHttpEquiv = nsString::from(aHttpEquiv);
        match ((*self.vtable).set_httpEquiv)(self as *const _, &*aHttpEquiv) {
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

    /* attribute DOMString scheme; */
    #[inline]
    pub unsafe fn get_scheme(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scheme)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scheme(&self, aScheme: &[u16]) -> Result<(), nsresult> {
        let aScheme = nsString::from(aScheme);
        match ((*self.vtable).set_scheme)(self as *const _, &*aScheme) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


