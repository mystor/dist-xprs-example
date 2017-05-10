//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULMenuListElement.idl
//


#[repr(C)]
pub struct nsIDOMXULMenuListElement {
    vtable: *const nsIDOMXULMenuListElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULMenuListElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x36c16a17, 0xc0e9, 0x4b35,
            [0x95, 0x1b, 0x81, 0xa1, 0x47, 0x31, 0x4e, 0xf1])
    }
}

unsafe impl RefCounted for nsIDOMXULMenuListElement {
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
pub trait nsIDOMXULMenuListElementCoerce {
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self;
}

impl nsIDOMXULMenuListElementCoerce for nsIDOMXULMenuListElement {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self {
        v
    }
}

impl nsIDOMXULMenuListElement {
    #[inline]
    pub fn coerce<T: nsIDOMXULMenuListElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULMenuListElement {
    type Target = nsIDOMXULSelectControlElement;
    #[inline]
    fn deref(&self) -> &nsIDOMXULSelectControlElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMXULSelectControlElementCoerce> nsIDOMXULMenuListElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULMenuListElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULMenuListElementVTable {
    pub __base: nsIDOMXULSelectControlElementVTable,

    /* attribute boolean editable; */
    pub get_editable: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aEditable: *mut bool) -> nsresult,
    pub set_editable: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aEditable: bool) -> nsresult,

    /* attribute boolean open; */
    pub get_open: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aOpen: *mut bool) -> nsresult,
    pub set_open: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aOpen: bool) -> nsresult,

    /* readonly attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aLabel: *mut nsAString) -> nsresult,

    /* attribute DOMString crop; */
    pub get_crop: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aCrop: *mut nsAString) -> nsresult,
    pub set_crop: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aCrop: *const nsAString) -> nsresult,

    /* attribute DOMString image; */
    pub get_image: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aImage: *mut nsAString) -> nsresult,
    pub set_image: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aImage: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMNode inputField; */
    pub get_inputField: unsafe extern "C" fn (this: *const nsIDOMXULMenuListElement, aInputField: *mut *const nsIDOMNode) -> nsresult,

}


impl nsIDOMXULMenuListElement {
    /* attribute boolean editable; */
    #[inline]
    pub unsafe fn get_editable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_editable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_editable(&self, aEditable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_editable)(self as *const _, aEditable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean open; */
    #[inline]
    pub unsafe fn get_open(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_open)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_open(&self, aOpen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_open)(self as *const _, aOpen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString label; */
    #[inline]
    pub unsafe fn get_label(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_label)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString crop; */
    #[inline]
    pub unsafe fn get_crop(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_crop)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_crop(&self, aCrop: &[u16]) -> Result<(), nsresult> {
        let aCrop = nsString::from(aCrop);
        match ((*self.vtable).set_crop)(self as *const _, &*aCrop) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString image; */
    #[inline]
    pub unsafe fn get_image(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_image)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_image(&self, aImage: &[u16]) -> Result<(), nsresult> {
        let aImage = nsString::from(aImage);
        match ((*self.vtable).set_image)(self as *const _, &*aImage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMNode inputField; */
    #[inline]
    pub unsafe fn get_inputField(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_inputField)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


