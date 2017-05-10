//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLIFrameElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLIFrameElement {
    vtable: *const nsIDOMHTMLIFrameElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLIFrameElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fd7b656, 0x1055, 0x4cb2,
            [0xb8, 0xb1, 0xed, 0x13, 0xef, 0xe2, 0x44, 0x57])
    }
}

unsafe impl RefCounted for nsIDOMHTMLIFrameElement {
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
pub trait nsIDOMHTMLIFrameElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLIFrameElement) -> &Self;
}

impl nsIDOMHTMLIFrameElementCoerce for nsIDOMHTMLIFrameElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLIFrameElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLIFrameElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLIFrameElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLIFrameElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLIFrameElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLIFrameElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLIFrameElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString frameBorder; */
    pub get_frameBorder: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aFrameBorder: *mut nsAString) -> nsresult,
    pub set_frameBorder: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aFrameBorder: *const nsAString) -> nsresult,

    /* attribute DOMString height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aHeight: *mut nsAString) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aHeight: *const nsAString) -> nsresult,

    /* attribute DOMString longDesc; */
    pub get_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aLongDesc: *mut nsAString) -> nsresult,
    pub set_longDesc: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aLongDesc: *const nsAString) -> nsresult,

    /* attribute DOMString marginHeight; */
    pub get_marginHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aMarginHeight: *mut nsAString) -> nsresult,
    pub set_marginHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aMarginHeight: *const nsAString) -> nsresult,

    /* attribute DOMString marginWidth; */
    pub get_marginWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aMarginWidth: *mut nsAString) -> nsresult,
    pub set_marginWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aMarginWidth: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString scrolling; */
    pub get_scrolling: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aScrolling: *mut nsAString) -> nsresult,
    pub set_scrolling: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aScrolling: *const nsAString) -> nsresult,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aSrc: *const nsAString) -> nsresult,

    /* attribute DOMString srcdoc; */
    pub get_srcdoc: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aSrcdoc: *mut nsAString) -> nsresult,
    pub set_srcdoc: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aSrcdoc: *const nsAString) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aWidth: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMDocument contentDocument; */
    pub get_contentDocument: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aContentDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* attribute boolean allowFullscreen; */
    pub get_allowFullscreen: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aAllowFullscreen: *mut bool) -> nsresult,
    pub set_allowFullscreen: unsafe extern "C" fn (this: *const nsIDOMHTMLIFrameElement, aAllowFullscreen: bool) -> nsresult,

}


impl nsIDOMHTMLIFrameElement {
    /* attribute DOMString align; */
    #[inline]
    pub unsafe fn get_align(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_align)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_align(&self, aAlign: &[u16]) -> Result<(), nsresult> {
        let aAlign = nsString::from(aAlign);
        match ((*self.vtable).set_align)(self as *const _, &*aAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* attribute DOMString height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_height)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: &[u16]) -> Result<(), nsresult> {
        let aHeight = nsString::from(aHeight);
        match ((*self.vtable).set_height)(self as *const _, &*aHeight) {
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

    /* attribute DOMString srcdoc; */
    #[inline]
    pub unsafe fn get_srcdoc(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcdoc)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcdoc(&self, aSrcdoc: &[u16]) -> Result<(), nsresult> {
        let aSrcdoc = nsString::from(aSrcdoc);
        match ((*self.vtable).set_srcdoc)(self as *const _, &*aSrcdoc) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_width)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: &[u16]) -> Result<(), nsresult> {
        let aWidth = nsString::from(aWidth);
        match ((*self.vtable).set_width)(self as *const _, &*aWidth) {
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

    /* attribute boolean allowFullscreen; */
    #[inline]
    pub unsafe fn get_allowFullscreen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowFullscreen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowFullscreen(&self, aAllowFullscreen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowFullscreen)(self as *const _, aAllowFullscreen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


