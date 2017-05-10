//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFrameElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLFrameElement {
    vtable: *const nsIDOMHTMLFrameElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLFrameElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x012a8982, 0xc9d3, 0x4614,
            [0x91, 0xe2, 0x18, 0xee, 0x51, 0xc9, 0x7c, 0x06])
    }
}

unsafe impl RefCounted for nsIDOMHTMLFrameElement {
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
pub trait nsIDOMHTMLFrameElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLFrameElement) -> &Self;
}

impl nsIDOMHTMLFrameElementCoerce for nsIDOMHTMLFrameElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFrameElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLFrameElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLFrameElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLFrameElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLFrameElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFrameElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLFrameElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString frameBorder; */
    pub get_frameBorder: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aFrameBorder: *mut nsAString) -> nsresult,
    pub set_frameBorder: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aFrameBorder: *const nsAString) -> nsresult,

    /* attribute DOMString longDesc; */
    pub get_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aLongDesc: *mut nsAString) -> nsresult,
    pub set_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aLongDesc: *const nsAString) -> nsresult,

    /* attribute DOMString marginHeight; */
    pub get_marginHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aMarginHeight: *mut nsAString) -> nsresult,
    pub set_marginHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aMarginHeight: *const nsAString) -> nsresult,

    /* attribute DOMString marginWidth; */
    pub get_marginWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aMarginWidth: *mut nsAString) -> nsresult,
    pub set_marginWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aMarginWidth: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aName: *const nsAString) -> nsresult,

    /* attribute boolean noResize; */
    pub get_noResize: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aNoResize: *mut bool) -> nsresult,
    pub set_noResize: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aNoResize: bool) -> nsresult,

    /* attribute DOMString scrolling; */
    pub get_scrolling: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aScrolling: *mut nsAString) -> nsresult,
    pub set_scrolling: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aScrolling: *const nsAString) -> nsresult,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aSrc: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMDocument contentDocument; */
    pub get_contentDocument: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameElement, aContentDocument: *mut *const nsIDOMDocument) -> nsresult,

}


impl nsIDOMHTMLFrameElement {
    /* attribute DOMString frameBorder; */
    #[inline]
    pub unsafe fn get_frameBorder(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_frameBorder)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_frameBorder(&self, aFrameBorder: &[u16]) -> Result<(), nsresult> {
        let aFrameBorder = nsString::from(aFrameBorder);
        match ((*self.vtable).set_frameBorder)(self as *const _, &*aFrameBorder) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString longDesc; */
    #[inline]
    pub unsafe fn get_longDesc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_longDesc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_longDesc(&self, aLongDesc: &[u16]) -> Result<(), nsresult> {
        let aLongDesc = nsString::from(aLongDesc);
        match ((*self.vtable).set_longDesc)(self as *const _, &*aLongDesc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString marginHeight; */
    #[inline]
    pub unsafe fn get_marginHeight(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_marginHeight)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginHeight(&self, aMarginHeight: &[u16]) -> Result<(), nsresult> {
        let aMarginHeight = nsString::from(aMarginHeight);
        match ((*self.vtable).set_marginHeight)(self as *const _, &*aMarginHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString marginWidth; */
    #[inline]
    pub unsafe fn get_marginWidth(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_marginWidth)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginWidth(&self, aMarginWidth: &[u16]) -> Result<(), nsresult> {
        let aMarginWidth = nsString::from(aMarginWidth);
        match ((*self.vtable).set_marginWidth)(self as *const _, &*aMarginWidth) {
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

    /* attribute boolean noResize; */
    #[inline]
    pub unsafe fn get_noResize(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noResize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_noResize(&self, aNoResize: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_noResize)(self as *const _, aNoResize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString scrolling; */
    #[inline]
    pub unsafe fn get_scrolling(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_scrolling)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scrolling(&self, aScrolling: &[u16]) -> Result<(), nsresult> {
        let aScrolling = nsString::from(aScrolling);
        match ((*self.vtable).set_scrolling)(self as *const _, &*aScrolling) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* readonly attribute nsIDOMDocument contentDocument; */
    #[inline]
    pub unsafe fn get_contentDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


