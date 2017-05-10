//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLUListElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLUListElement {
    vtable: *const nsIDOMHTMLUListElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLUListElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8ba1ee8d, 0x36a4, 0x43fc,
            [0x91, 0x48, 0x51, 0x44, 0xc2, 0xa2, 0x9c, 0x96])
    }
}

unsafe impl RefCounted for nsIDOMHTMLUListElement {
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
pub trait nsIDOMHTMLUListElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLUListElement) -> &Self;
}

impl nsIDOMHTMLUListElementCoerce for nsIDOMHTMLUListElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLUListElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLUListElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLUListElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLUListElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLUListElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLUListElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLUListElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean compact; */
    pub get_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLUListElement, aCompact: *mut bool) -> nsresult,
    pub set_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLUListElement, aCompact: bool) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLUListElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLUListElement, aType: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLUListElement {
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

}


