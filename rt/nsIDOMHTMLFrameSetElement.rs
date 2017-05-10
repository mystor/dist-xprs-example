//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLFrameSetElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLFrameSetElement {
    vtable: *const nsIDOMHTMLFrameSetElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLFrameSetElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x14b29269, 0xc387, 0x4cff,
            [0x84, 0x63, 0xb0, 0x87, 0x1c, 0xa0, 0xbe, 0x3a])
    }
}

unsafe impl RefCounted for nsIDOMHTMLFrameSetElement {
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
pub trait nsIDOMHTMLFrameSetElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLFrameSetElement) -> &Self;
}

impl nsIDOMHTMLFrameSetElementCoerce for nsIDOMHTMLFrameSetElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFrameSetElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLFrameSetElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLFrameSetElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLFrameSetElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLFrameSetElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLFrameSetElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLFrameSetElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString cols; */
    pub get_cols: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameSetElement, aCols: *mut nsAString) -> nsresult,
    pub set_cols: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameSetElement, aCols: *const nsAString) -> nsresult,

    /* attribute DOMString rows; */
    pub get_rows: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameSetElement, aRows: *mut nsAString) -> nsresult,
    pub set_rows: unsafe extern "C" fn (this: *const nsIDOMHTMLFrameSetElement, aRows: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLFrameSetElement {
    /* attribute DOMString cols; */
    #[inline]
    pub unsafe fn get_cols(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cols)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cols(&self, aCols: &[u16]) -> Result<(), nsresult> {
        let aCols = nsString::from(aCols);
        match ((*self.vtable).set_cols)(self as *const _, &*aCols) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString rows; */
    #[inline]
    pub unsafe fn get_rows(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_rows)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_rows(&self, aRows: &[u16]) -> Result<(), nsresult> {
        let aRows = nsString::from(aRows);
        match ((*self.vtable).set_rows)(self as *const _, &*aRows) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


