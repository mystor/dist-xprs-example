//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLSourceElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLSourceElement {
    vtable: *const nsIDOMHTMLSourceElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLSourceElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1deb68f8, 0x2ed6, 0x4a41,
            [0xb8, 0xc8, 0xe0, 0xf8, 0x65, 0x10, 0xf7, 0x99])
    }
}

unsafe impl RefCounted for nsIDOMHTMLSourceElement {
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
pub trait nsIDOMHTMLSourceElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLSourceElement) -> &Self;
}

impl nsIDOMHTMLSourceElementCoerce for nsIDOMHTMLSourceElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLSourceElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLSourceElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLSourceElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLSourceElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLSourceElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLSourceElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLSourceElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSrc: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString srcset; */
    pub get_srcset: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSrcset: *mut nsAString) -> nsresult,
    pub set_srcset: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSrcset: *const nsAString) -> nsresult,

    /* attribute DOMString sizes; */
    pub get_sizes: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSizes: *mut nsAString) -> nsresult,
    pub set_sizes: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aSizes: *const nsAString) -> nsresult,

    /* attribute DOMString media; */
    pub get_media: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aMedia: *mut nsAString) -> nsresult,
    pub set_media: unsafe extern "C" fn (this: *const nsIDOMHTMLSourceElement, aMedia: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLSourceElement {
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

    /* attribute DOMString srcset; */
    #[inline]
    pub unsafe fn get_srcset(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcset(&self, aSrcset: &[u16]) -> Result<(), nsresult> {
        let aSrcset = nsString::from(aSrcset);
        match ((*self.vtable).set_srcset)(self as *const _, &*aSrcset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString sizes; */
    #[inline]
    pub unsafe fn get_sizes(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_sizes)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sizes(&self, aSizes: &[u16]) -> Result<(), nsresult> {
        let aSizes = nsString::from(aSizes);
        match ((*self.vtable).set_sizes)(self as *const _, &*aSizes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString media; */
    #[inline]
    pub unsafe fn get_media(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_media)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_media(&self, aMedia: &[u16]) -> Result<(), nsresult> {
        let aMedia = nsString::from(aMedia);
        match ((*self.vtable).set_media)(self as *const _, &*aMedia) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


