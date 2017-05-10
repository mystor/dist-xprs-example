//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLBaseElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLBaseElement {
    vtable: *const nsIDOMHTMLBaseElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLBaseElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa348ac22, 0x7880, 0x4613,
            [0xaf, 0x4c, 0x98, 0x4e, 0xc2, 0xef, 0x5a, 0xdc])
    }
}

unsafe impl RefCounted for nsIDOMHTMLBaseElement {
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
pub trait nsIDOMHTMLBaseElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLBaseElement) -> &Self;
}

impl nsIDOMHTMLBaseElementCoerce for nsIDOMHTMLBaseElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLBaseElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLBaseElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLBaseElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLBaseElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLBaseElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLBaseElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLBaseElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString href; */
    pub get_href: unsafe extern "C" fn (this: *const nsIDOMHTMLBaseElement, aHref: *mut nsAString) -> nsresult,
    pub set_href: unsafe extern "C" fn (this: *const nsIDOMHTMLBaseElement, aHref: *const nsAString) -> nsresult,

    /* attribute DOMString target; */
    pub get_target: unsafe extern "C" fn (this: *const nsIDOMHTMLBaseElement, aTarget: *mut nsAString) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsIDOMHTMLBaseElement, aTarget: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLBaseElement {
    /* attribute DOMString href; */
    #[inline]
    pub unsafe fn get_href(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_href)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_href(&self, aHref: &[u16]) -> Result<(), nsresult> {
        let aHref = nsString::from(aHref);
        match ((*self.vtable).set_href)(self as *const _, &*aHref) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_target)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: &[u16]) -> Result<(), nsresult> {
        let aTarget = nsString::from(aTarget);
        match ((*self.vtable).set_target)(self as *const _, &*aTarget) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


