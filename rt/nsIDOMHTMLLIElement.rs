//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLLIElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLLIElement {
    vtable: *const nsIDOMHTMLLIElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLLIElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x17bd5c1c, 0x3746, 0x4268,
            [0xa9, 0xf6, 0x45, 0x01, 0x80, 0x25, 0xf0, 0x9c])
    }
}

unsafe impl RefCounted for nsIDOMHTMLLIElement {
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
pub trait nsIDOMHTMLLIElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLLIElement) -> &Self;
}

impl nsIDOMHTMLLIElementCoerce for nsIDOMHTMLLIElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLIElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLLIElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLLIElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLLIElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLLIElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLLIElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLLIElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLLIElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLLIElement, aType: *const nsAString) -> nsresult,

    /* attribute long value; */
    pub get_value: unsafe extern "C" fn (this: *const nsIDOMHTMLLIElement, aValue: *mut libc::int32_t) -> nsresult,
    pub set_value: unsafe extern "C" fn (this: *const nsIDOMHTMLLIElement, aValue: libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLLIElement {
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

    /* attribute long value; */
    #[inline]
    pub unsafe fn get_value(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_value)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_value(&self, aValue: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_value)(self as *const _, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


