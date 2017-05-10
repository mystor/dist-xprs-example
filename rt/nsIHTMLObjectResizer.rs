//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLObjectResizer.idl
//


pub mod nsIHTMLObjectResizer_consts {
    pub const eTopLeft: i64 = 0;
    pub const eTop: i64 = 1;
    pub const eTopRight: i64 = 2;
    pub const eLeft: i64 = 3;
    pub const eRight: i64 = 4;
    pub const eBottomLeft: i64 = 5;
    pub const eBottom: i64 = 6;
    pub const eBottomRight: i64 = 7;
}


#[repr(C)]
pub struct nsIHTMLObjectResizer {
    vtable: *const nsIHTMLObjectResizerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTMLObjectResizer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8b396020, 0x69d3, 0x451f,
            [0x80, 0xc1, 0x1a, 0x96, 0xa7, 0xda, 0x25, 0xa9])
    }
}

unsafe impl RefCounted for nsIHTMLObjectResizer {
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
pub trait nsIHTMLObjectResizerCoerce {
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self;
}

impl nsIHTMLObjectResizerCoerce for nsIHTMLObjectResizer {
    #[inline]
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self {
        v
    }
}

impl nsIHTMLObjectResizer {
    #[inline]
    pub fn coerce<T: nsIHTMLObjectResizerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTMLObjectResizer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTMLObjectResizerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTMLObjectResizerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMElement resizedObject; */
    pub get_resizedObject: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aResizedObject: *mut *const nsIDOMElement) -> nsresult,

    /* attribute boolean objectResizingEnabled; */
    pub get_objectResizingEnabled: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aObjectResizingEnabled: *mut bool) -> nsresult,
    pub set_objectResizingEnabled: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aObjectResizingEnabled: bool) -> nsresult,

    /* void showResizers (in nsIDOMElement aResizedElement); */
    pub showResizers: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aResizedElement: *const nsIDOMElement) -> nsresult,

    /* void hideResizers (); */
    pub hideResizers: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer) -> nsresult,

    /* void refreshResizers (); */
    pub refreshResizers: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer) -> nsresult,

    /* void mouseDown (in long aX, in long aY, in nsIDOMElement aTarget, in nsIDOMEvent aMouseEvent); */
    pub mouseDown: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aX: libc::int32_t, aY: libc::int32_t, aTarget: *const nsIDOMElement, aMouseEvent: *const nsIDOMEvent) -> nsresult,

    /* void mouseUp (in long aX, in long aY, in nsIDOMElement aTarget); */
    pub mouseUp: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aX: libc::int32_t, aY: libc::int32_t, aTarget: *const nsIDOMElement) -> nsresult,

    /* void mouseMove (in nsIDOMEvent aMouseEvent); */
    pub mouseMove: unsafe extern "C" fn (this: *const nsIHTMLObjectResizer, aMouseEvent: *const nsIDOMEvent) -> nsresult,

}


impl nsIHTMLObjectResizer {
    /* readonly attribute nsIDOMElement resizedObject; */
    #[inline]
    pub unsafe fn get_resizedObject(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_resizedObject)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean objectResizingEnabled; */
    #[inline]
    pub unsafe fn get_objectResizingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_objectResizingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_objectResizingEnabled(&self, aObjectResizingEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_objectResizingEnabled)(self as *const _, aObjectResizingEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void showResizers (in nsIDOMElement aResizedElement); */
    #[inline]
    pub unsafe fn showResizers(&self, aResizedElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).showResizers)(self as *const _, aResizedElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hideResizers (); */
    #[inline]
    pub unsafe fn hideResizers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hideResizers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void refreshResizers (); */
    #[inline]
    pub unsafe fn refreshResizers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refreshResizers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void mouseDown (in long aX, in long aY, in nsIDOMElement aTarget, in nsIDOMEvent aMouseEvent); */
    #[inline]
    pub unsafe fn mouseDown(&self, aX: libc::int32_t, aY: libc::int32_t, aTarget: Option<&nsIDOMElement>, aMouseEvent: Option<&nsIDOMEvent>) -> Result<(), nsresult> {

        match ((*self.vtable).mouseDown)(self as *const _, aX, aY, aTarget.map_or(::std::ptr::null(), |x| x as *const _), aMouseEvent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void mouseUp (in long aX, in long aY, in nsIDOMElement aTarget); */
    #[inline]
    pub unsafe fn mouseUp(&self, aX: libc::int32_t, aY: libc::int32_t, aTarget: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).mouseUp)(self as *const _, aX, aY, aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void mouseMove (in nsIDOMEvent aMouseEvent); */
    #[inline]
    pub unsafe fn mouseMove(&self, aMouseEvent: Option<&nsIDOMEvent>) -> Result<(), nsresult> {

        match ((*self.vtable).mouseMove)(self as *const _, aMouseEvent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


