//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLBodyElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLBodyElement {
    vtable: *const nsIDOMHTMLBodyElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLBodyElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x068630db, 0x2c00, 0x43dd,
            [0xb1, 0x67, 0x49, 0x57, 0x57, 0xa8, 0x82, 0x36])
    }
}

unsafe impl RefCounted for nsIDOMHTMLBodyElement {
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
pub trait nsIDOMHTMLBodyElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLBodyElement) -> &Self;
}

impl nsIDOMHTMLBodyElementCoerce for nsIDOMHTMLBodyElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLBodyElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLBodyElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLBodyElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLBodyElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLBodyElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLBodyElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLBodyElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString aLink; */
    pub get_aLink: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aALink: *mut nsAString) -> nsresult,
    pub set_aLink: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aALink: *const nsAString) -> nsresult,

    /* attribute DOMString background; */
    pub get_background: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aBackground: *mut nsAString) -> nsresult,
    pub set_background: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aBackground: *const nsAString) -> nsresult,

    /* attribute DOMString bgColor; */
    pub get_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aBgColor: *mut nsAString) -> nsresult,
    pub set_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aBgColor: *const nsAString) -> nsresult,

    /* attribute DOMString link; */
    pub get_link: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aLink: *mut nsAString) -> nsresult,
    pub set_link: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aLink: *const nsAString) -> nsresult,

    /* attribute DOMString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aText: *mut nsAString) -> nsresult,
    pub set_text: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aText: *const nsAString) -> nsresult,

    /* attribute DOMString vLink; */
    pub get_vLink: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aVLink: *mut nsAString) -> nsresult,
    pub set_vLink: unsafe extern "C" fn (this: *const nsIDOMHTMLBodyElement, aVLink: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLBodyElement {
    /* attribute DOMString aLink; */
    #[inline]
    pub unsafe fn get_aLink(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_aLink)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_aLink(&self, aALink: &[u16]) -> Result<(), nsresult> {
        let aALink = nsString::from(aALink);
        match ((*self.vtable).set_aLink)(self as *const _, &*aALink) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString background; */
    #[inline]
    pub unsafe fn get_background(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_background)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_background(&self, aBackground: &[u16]) -> Result<(), nsresult> {
        let aBackground = nsString::from(aBackground);
        match ((*self.vtable).set_background)(self as *const _, &*aBackground) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString bgColor; */
    #[inline]
    pub unsafe fn get_bgColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_bgColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_bgColor(&self, aBgColor: &[u16]) -> Result<(), nsresult> {
        let aBgColor = nsString::from(aBgColor);
        match ((*self.vtable).set_bgColor)(self as *const _, &*aBgColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString link; */
    #[inline]
    pub unsafe fn get_link(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_link)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_link(&self, aLink: &[u16]) -> Result<(), nsresult> {
        let aLink = nsString::from(aLink);
        match ((*self.vtable).set_link)(self as *const _, &*aLink) {
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

    /* attribute DOMString vLink; */
    #[inline]
    pub unsafe fn get_vLink(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_vLink)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_vLink(&self, aVLink: &[u16]) -> Result<(), nsresult> {
        let aVLink = nsString::from(aVLink);
        match ((*self.vtable).set_vLink)(self as *const _, &*aVLink) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


