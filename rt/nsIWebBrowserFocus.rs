//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserFocus.idl
//


#[repr(C)]
pub struct nsIWebBrowserFocus {
    vtable: *const nsIWebBrowserFocusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserFocus {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7f8c754e, 0x5b36, 0x44be,
            [0xbc, 0x96, 0x19, 0x1b, 0x49, 0xf0, 0x8e, 0xa6])
    }
}

unsafe impl RefCounted for nsIWebBrowserFocus {
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
pub trait nsIWebBrowserFocusCoerce {
    fn coerce_from(v: &nsIWebBrowserFocus) -> &Self;
}

impl nsIWebBrowserFocusCoerce for nsIWebBrowserFocus {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFocus) -> &Self {
        v
    }
}

impl nsIWebBrowserFocus {
    #[inline]
    pub fn coerce<T: nsIWebBrowserFocusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserFocus {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserFocusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserFocus) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserFocusVTable {
    pub __base: nsISupportsVTable,

    /* void activate (); */
    pub activate: unsafe extern "C" fn (this: *const nsIWebBrowserFocus) -> nsresult,

    /* void deactivate (); */
    pub deactivate: unsafe extern "C" fn (this: *const nsIWebBrowserFocus) -> nsresult,

    /* void setFocusAtFirstElement (); */
    pub setFocusAtFirstElement: unsafe extern "C" fn (this: *const nsIWebBrowserFocus) -> nsresult,

    /* void setFocusAtLastElement (); */
    pub setFocusAtLastElement: unsafe extern "C" fn (this: *const nsIWebBrowserFocus) -> nsresult,

    /* attribute mozIDOMWindowProxy focusedWindow; */
    pub get_focusedWindow: unsafe extern "C" fn (this: *const nsIWebBrowserFocus, aFocusedWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_focusedWindow: unsafe extern "C" fn (this: *const nsIWebBrowserFocus, aFocusedWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* attribute nsIDOMElement focusedElement; */
    pub get_focusedElement: unsafe extern "C" fn (this: *const nsIWebBrowserFocus, aFocusedElement: *mut *const nsIDOMElement) -> nsresult,
    pub set_focusedElement: unsafe extern "C" fn (this: *const nsIWebBrowserFocus, aFocusedElement: *const nsIDOMElement) -> nsresult,

}


impl nsIWebBrowserFocus {
    /* void activate (); */
    #[inline]
    pub unsafe fn activate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).activate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deactivate (); */
    #[inline]
    pub unsafe fn deactivate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deactivate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFocusAtFirstElement (); */
    #[inline]
    pub unsafe fn setFocusAtFirstElement(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setFocusAtFirstElement)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFocusAtLastElement (); */
    #[inline]
    pub unsafe fn setFocusAtLastElement(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setFocusAtLastElement)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute mozIDOMWindowProxy focusedWindow; */
    #[inline]
    pub unsafe fn get_focusedWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_focusedWindow(&self, aFocusedWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_focusedWindow)(self as *const _, aFocusedWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDOMElement focusedElement; */
    #[inline]
    pub unsafe fn get_focusedElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_focusedElement(&self, aFocusedElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_focusedElement)(self as *const _, aFocusedElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


