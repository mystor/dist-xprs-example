//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLScriptElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLScriptElement {
    vtable: *const nsIDOMHTMLScriptElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLScriptElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfe96dc1c, 0x40e4, 0x4974,
            [0x93, 0x54, 0xe3, 0xfc, 0xe6, 0x63, 0xc3, 0xd5])
    }
}

unsafe impl RefCounted for nsIDOMHTMLScriptElement {
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
pub trait nsIDOMHTMLScriptElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLScriptElement) -> &Self;
}

impl nsIDOMHTMLScriptElementCoerce for nsIDOMHTMLScriptElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLScriptElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLScriptElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLScriptElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLScriptElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLScriptElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLScriptElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLScriptElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aSrc: *const nsAString) -> nsresult,

    /* attribute boolean async; */
    pub get_async: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aAsync: *mut bool) -> nsresult,
    pub set_async: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aAsync: bool) -> nsresult,

    /* attribute boolean defer; */
    pub get_defer: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aDefer: *mut bool) -> nsresult,
    pub set_defer: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aDefer: bool) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aCharset: *mut nsAString) -> nsresult,
    pub set_charset: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aCharset: *const nsAString) -> nsresult,

    /* attribute DOMString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aText: *mut nsAString) -> nsresult,
    pub set_text: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aText: *const nsAString) -> nsresult,

    /* attribute DOMString htmlFor; */
    pub get_htmlFor: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aHtmlFor: *mut nsAString) -> nsresult,
    pub set_htmlFor: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aHtmlFor: *const nsAString) -> nsresult,

    /* attribute DOMString event; */
    pub get_event: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aEvent: *mut nsAString) -> nsresult,
    pub set_event: unsafe extern "C" fn (this: *const nsIDOMHTMLScriptElement, aEvent: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLScriptElement {
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

    /* attribute boolean async; */
    #[inline]
    pub unsafe fn get_async(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_async)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_async(&self, aAsync: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_async)(self as *const _, aAsync) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean defer; */
    #[inline]
    pub unsafe fn get_defer(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_defer)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defer(&self, aDefer: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_defer)(self as *const _, aDefer) {
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

    /* attribute DOMString charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charset(&self, aCharset: &[u16]) -> Result<(), nsresult> {
        let aCharset = nsString::from(aCharset);
        match ((*self.vtable).set_charset)(self as *const _, &*aCharset) {
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

    /* attribute DOMString event; */
    #[inline]
    pub unsafe fn get_event(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_event)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_event(&self, aEvent: &[u16]) -> Result<(), nsresult> {
        let aEvent = nsString::from(aEvent);
        match ((*self.vtable).set_event)(self as *const _, &*aEvent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


