//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLEmbedElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLEmbedElement {
    vtable: *const nsIDOMHTMLEmbedElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLEmbedElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xadae53da, 0x713d, 0x4570,
            [0x81, 0xad, 0xda, 0xbd, 0xd6, 0xd4, 0x62, 0x41])
    }
}

unsafe impl RefCounted for nsIDOMHTMLEmbedElement {
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
pub trait nsIDOMHTMLEmbedElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLEmbedElement) -> &Self;
}

impl nsIDOMHTMLEmbedElementCoerce for nsIDOMHTMLEmbedElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLEmbedElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLEmbedElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLEmbedElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLEmbedElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLEmbedElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLEmbedElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLEmbedElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aHeight: *mut nsAString) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aHeight: *const nsAString) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString src; */
    pub get_src: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aSrc: *mut nsAString) -> nsresult,
    pub set_src: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aSrc: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLEmbedElement, aWidth: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLEmbedElement {
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

    /* attribute DOMString height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_height)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_height(&self, aHeight: &[u16]) -> Result<(), nsresult> {
        let aHeight = nsString::from(aHeight);
        match ((*self.vtable).set_height)(self as *const _, &*aHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_name(&self, aName: &[u16]) -> Result<(), nsresult> {
        let aName = nsString::from(aName);
        match ((*self.vtable).set_name)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString src; */
    #[inline]
    pub unsafe fn get_src(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_src)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_src(&self, aSrc: &[u16]) -> Result<(), nsresult> {
        let aSrc = nsString::from(aSrc);
        match ((*self.vtable).set_src)(self as *const _, &*aSrc) {
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

}


