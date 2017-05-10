//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLMenuElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLMenuElement {
    vtable: *const nsIDOMHTMLMenuElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLMenuElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa1ca9af6, 0xf865, 0x4fdf,
            [0x90, 0x1d, 0x58, 0x58, 0xbb, 0x0a, 0xd5, 0xea])
    }
}

unsafe impl RefCounted for nsIDOMHTMLMenuElement {
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
pub trait nsIDOMHTMLMenuElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLMenuElement) -> &Self;
}

impl nsIDOMHTMLMenuElementCoerce for nsIDOMHTMLMenuElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMenuElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLMenuElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLMenuElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLMenuElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLMenuElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLMenuElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLMenuElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean compact; */
    pub get_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aCompact: *mut bool) -> nsresult,
    pub set_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aCompact: bool) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString label; */
    pub get_label: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aLabel: *mut nsAString) -> nsresult,
    pub set_label: unsafe extern "C" fn (this: *const nsIDOMHTMLMenuElement, aLabel: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLMenuElement {
    /* attribute boolean compact; */
    #[inline]
    pub unsafe fn get_compact(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_compact)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_compact(&self, aCompact: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_compact)(self as *const _, aCompact) {
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

    /* attribute DOMString label; */
    #[inline]
    pub unsafe fn get_label(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_label)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_label(&self, aLabel: &[u16]) -> Result<(), nsresult> {
        let aLabel = nsString::from(aLabel);
        match ((*self.vtable).set_label)(self as *const _, &*aLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


