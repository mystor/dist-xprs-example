//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLQuoteElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLQuoteElement {
    vtable: *const nsIDOMHTMLQuoteElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLQuoteElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf02502b5, 0x32a6, 0x4df7,
            [0x8a, 0x57, 0x14, 0x16, 0x55, 0x3a, 0x31, 0x88])
    }
}

unsafe impl RefCounted for nsIDOMHTMLQuoteElement {
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
pub trait nsIDOMHTMLQuoteElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLQuoteElement) -> &Self;
}

impl nsIDOMHTMLQuoteElementCoerce for nsIDOMHTMLQuoteElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLQuoteElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLQuoteElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLQuoteElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLQuoteElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLQuoteElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLQuoteElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLQuoteElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString cite; */
    pub get_cite: unsafe extern "C" fn (this: *const nsIDOMHTMLQuoteElement, aCite: *mut nsAString) -> nsresult,
    pub set_cite: unsafe extern "C" fn (this: *const nsIDOMHTMLQuoteElement, aCite: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLQuoteElement {
    /* attribute DOMString cite; */
    #[inline]
    pub unsafe fn get_cite(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cite)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cite(&self, aCite: &[u16]) -> Result<(), nsresult> {
        let aCite = nsString::from(aCite);
        match ((*self.vtable).set_cite)(self as *const _, &*aCite) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


