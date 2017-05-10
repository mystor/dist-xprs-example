//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBoxObject.idl
//


#[repr(C)]
pub struct nsIBoxObject {
    vtable: *const nsIBoxObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBoxObject {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xce572460, 0xb0f2, 0x4650,
            [0xa9, 0xe7, 0xc5, 0x3a, 0x99, 0xd3, 0xb6, 0xad])
    }
}

unsafe impl RefCounted for nsIBoxObject {
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
pub trait nsIBoxObjectCoerce {
    fn coerce_from(v: &nsIBoxObject) -> &Self;
}

impl nsIBoxObjectCoerce for nsIBoxObject {
    #[inline]
    fn coerce_from(v: &nsIBoxObject) -> &Self {
        v
    }
}

impl nsIBoxObject {
    #[inline]
    pub fn coerce<T: nsIBoxObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBoxObject {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBoxObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBoxObject) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBoxObjectVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMElement element; */
    pub get_element: unsafe extern "C" fn (this: *const nsIBoxObject, aElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute long x; */
    pub get_x: unsafe extern "C" fn (this: *const nsIBoxObject, aX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long y; */
    pub get_y: unsafe extern "C" fn (this: *const nsIBoxObject, aY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long screenX; */
    pub get_screenX: unsafe extern "C" fn (this: *const nsIBoxObject, aScreenX: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long screenY; */
    pub get_screenY: unsafe extern "C" fn (this: *const nsIBoxObject, aScreenY: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIBoxObject, aWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIBoxObject, aHeight: *mut libc::int32_t) -> nsresult,

    /* nsISupports getPropertyAsSupports (in wstring propertyName); */
    pub getPropertyAsSupports: unsafe extern "C" fn (this: *const nsIBoxObject, propertyName: *const libc::int16_t, _retval: *mut *const nsISupports) -> nsresult,

    /* void setPropertyAsSupports (in wstring propertyName, in nsISupports value); */
    pub setPropertyAsSupports: unsafe extern "C" fn (this: *const nsIBoxObject, propertyName: *const libc::int16_t, value: *const nsISupports) -> nsresult,

    /* wstring getProperty (in wstring propertyName); */
    pub getProperty: unsafe extern "C" fn (this: *const nsIBoxObject, propertyName: *const libc::int16_t, _retval: *mut *const libc::int16_t) -> nsresult,

    /* void setProperty (in wstring propertyName, in wstring propertyValue); */
    pub setProperty: unsafe extern "C" fn (this: *const nsIBoxObject, propertyName: *const libc::int16_t, propertyValue: *const libc::int16_t) -> nsresult,

    /* void removeProperty (in wstring propertyName); */
    pub removeProperty: unsafe extern "C" fn (this: *const nsIBoxObject, propertyName: *const libc::int16_t) -> nsresult,

    /* readonly attribute nsIDOMElement parentBox; */
    pub get_parentBox: unsafe extern "C" fn (this: *const nsIBoxObject, aParentBox: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIDOMElement firstChild; */
    pub get_firstChild: unsafe extern "C" fn (this: *const nsIBoxObject, aFirstChild: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIDOMElement lastChild; */
    pub get_lastChild: unsafe extern "C" fn (this: *const nsIBoxObject, aLastChild: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIDOMElement nextSibling; */
    pub get_nextSibling: unsafe extern "C" fn (this: *const nsIBoxObject, aNextSibling: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIDOMElement previousSibling; */
    pub get_previousSibling: unsafe extern "C" fn (this: *const nsIBoxObject, aPreviousSibling: *mut *const nsIDOMElement) -> nsresult,

}


impl nsIBoxObject {
    /* readonly attribute nsIDOMElement element; */
    #[inline]
    pub unsafe fn get_element(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_element)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long x; */
    #[inline]
    pub unsafe fn get_x(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_x)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long y; */
    #[inline]
    pub unsafe fn get_y(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_y)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long screenX; */
    #[inline]
    pub unsafe fn get_screenX(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_screenX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long screenY; */
    #[inline]
    pub unsafe fn get_screenY(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_screenY)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISupports getPropertyAsSupports (in wstring propertyName); */
    #[inline]
    pub unsafe fn getPropertyAsSupports(&self, propertyName: *const libc::int16_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPropertyAsSupports)(self as *const _, propertyName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setPropertyAsSupports (in wstring propertyName, in nsISupports value); */
    #[inline]
    pub unsafe fn setPropertyAsSupports(&self, propertyName: *const libc::int16_t, value: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setPropertyAsSupports)(self as *const _, propertyName, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring getProperty (in wstring propertyName); */
    #[inline]
    pub unsafe fn getProperty(&self, propertyName: *const libc::int16_t) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getProperty)(self as *const _, propertyName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setProperty (in wstring propertyName, in wstring propertyValue); */
    #[inline]
    pub unsafe fn setProperty(&self, propertyName: *const libc::int16_t, propertyValue: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setProperty)(self as *const _, propertyName, propertyValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeProperty (in wstring propertyName); */
    #[inline]
    pub unsafe fn removeProperty(&self, propertyName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeProperty)(self as *const _, propertyName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement parentBox; */
    #[inline]
    pub unsafe fn get_parentBox(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentBox)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement firstChild; */
    #[inline]
    pub unsafe fn get_firstChild(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_firstChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement lastChild; */
    #[inline]
    pub unsafe fn get_lastChild(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_lastChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement nextSibling; */
    #[inline]
    pub unsafe fn get_nextSibling(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_nextSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement previousSibling; */
    #[inline]
    pub unsafe fn get_previousSibling(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_previousSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


