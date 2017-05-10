//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLHRElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLHRElement {
    vtable: *const nsIDOMHTMLHRElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLHRElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x30771953, 0xb9f4, 0x44de,
            [0xb0, 0xfe, 0xe4, 0x90, 0x94, 0x9a, 0xf9, 0x8b])
    }
}

unsafe impl RefCounted for nsIDOMHTMLHRElement {
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
pub trait nsIDOMHTMLHRElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLHRElement) -> &Self;
}

impl nsIDOMHTMLHRElementCoerce for nsIDOMHTMLHRElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHRElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLHRElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLHRElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLHRElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLHRElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLHRElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLHRElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aAlign: *const nsAString) -> nsresult,

    /* attribute boolean noShade; */
    pub get_noShade: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aNoShade: *mut bool) -> nsresult,
    pub set_noShade: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aNoShade: bool) -> nsresult,

    /* attribute DOMString size; */
    pub get_size: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aSize: *mut nsAString) -> nsresult,
    pub set_size: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aSize: *const nsAString) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aWidth: *const nsAString) -> nsresult,

    /* attribute DOMString color; */
    pub get_color: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aColor: *mut nsAString) -> nsresult,
    pub set_color: unsafe extern "C" fn (this: *const nsIDOMHTMLHRElement, aColor: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLHRElement {
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

    /* attribute boolean noShade; */
    #[inline]
    pub unsafe fn get_noShade(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noShade)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_noShade(&self, aNoShade: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_noShade)(self as *const _, aNoShade) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString size; */
    #[inline]
    pub unsafe fn get_size(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_size)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_size(&self, aSize: &[u16]) -> Result<(), nsresult> {
        let aSize = nsString::from(aSize);
        match ((*self.vtable).set_size)(self as *const _, &*aSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_width)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_width(&self, aWidth: &[u16]) -> Result<(), nsresult> {
        let aWidth = nsString::from(aWidth);
        match ((*self.vtable).set_width)(self as *const _, &*aWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString color; */
    #[inline]
    pub unsafe fn get_color(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_color)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_color(&self, aColor: &[u16]) -> Result<(), nsresult> {
        let aColor = nsString::from(aColor);
        match ((*self.vtable).set_color)(self as *const _, &*aColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


