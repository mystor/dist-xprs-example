//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLParagraphElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLParagraphElement {
    vtable: *const nsIDOMHTMLParagraphElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLParagraphElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb494e517, 0x2388, 0x4a63,
            [0x80, 0xe7, 0x2f, 0x73, 0xbe, 0x3c, 0x38, 0xa3])
    }
}

unsafe impl RefCounted for nsIDOMHTMLParagraphElement {
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
pub trait nsIDOMHTMLParagraphElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLParagraphElement) -> &Self;
}

impl nsIDOMHTMLParagraphElementCoerce for nsIDOMHTMLParagraphElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLParagraphElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLParagraphElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLParagraphElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLParagraphElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLParagraphElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLParagraphElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLParagraphElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLParagraphElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLParagraphElement, aAlign: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLParagraphElement {
    /* attribute DOMString align; */
    #[inline]
    pub unsafe fn get_align(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_align)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_align(&self, aAlign: &[u16]) -> Result<(), nsresult> {
        let aAlign = nsString::from(aAlign);
        match ((*self.vtable).set_align)(self as *const _, &*aAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


