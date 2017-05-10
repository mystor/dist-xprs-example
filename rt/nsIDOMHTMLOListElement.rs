//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLOListElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLOListElement {
    vtable: *const nsIDOMHTMLOListElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLOListElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd899642a, 0x53e2, 0x4eb4,
            [0x9d, 0x65, 0x4a, 0x66, 0x6a, 0x45, 0xee, 0x01])
    }
}

unsafe impl RefCounted for nsIDOMHTMLOListElement {
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
pub trait nsIDOMHTMLOListElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLOListElement) -> &Self;
}

impl nsIDOMHTMLOListElementCoerce for nsIDOMHTMLOListElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOListElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLOListElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLOListElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLOListElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLOListElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLOListElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLOListElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean compact; */
    pub get_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aCompact: *mut bool) -> nsresult,
    pub set_compact: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aCompact: bool) -> nsresult,

    /* attribute boolean reversed; */
    pub get_reversed: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aReversed: *mut bool) -> nsresult,
    pub set_reversed: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aReversed: bool) -> nsresult,

    /* attribute long start; */
    pub get_start: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aStart: *mut libc::int32_t) -> nsresult,
    pub set_start: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aStart: libc::int32_t) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLOListElement, aType: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLOListElement {
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

    /* attribute boolean reversed; */
    #[inline]
    pub unsafe fn get_reversed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_reversed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_reversed(&self, aReversed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_reversed)(self as *const _, aReversed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long start; */
    #[inline]
    pub unsafe fn get_start(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_start)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_start(&self, aStart: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_start)(self as *const _, aStart) {
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


