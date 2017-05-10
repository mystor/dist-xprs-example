//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLAppletElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLAppletElement {
    vtable: *const nsIDOMHTMLAppletElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLAppletElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0b7d12c9, 0x4cd3, 0x47db,
            [0x99, 0xc6, 0x0b, 0x5f, 0xf9, 0x10, 0x44, 0x6c])
    }
}

unsafe impl RefCounted for nsIDOMHTMLAppletElement {
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
pub trait nsIDOMHTMLAppletElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLAppletElement) -> &Self;
}

impl nsIDOMHTMLAppletElementCoerce for nsIDOMHTMLAppletElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAppletElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLAppletElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLAppletElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLAppletElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLAppletElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLAppletElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLAppletElementVTable {
    pub __base: nsISupportsVTable,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString alt; */
    pub get_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aAlt: *mut nsAString) -> nsresult,
    pub set_alt: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aAlt: *const nsAString) -> nsresult,

    /* attribute DOMString archive; */
    pub get_archive: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aArchive: *mut nsAString) -> nsresult,
    pub set_archive: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aArchive: *const nsAString) -> nsresult,

    /* attribute DOMString code; */
    pub get_code: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aCode: *mut nsAString) -> nsresult,
    pub set_code: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aCode: *const nsAString) -> nsresult,

    /* attribute DOMString codeBase; */
    pub get_codeBase: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aCodeBase: *mut nsAString) -> nsresult,
    pub set_codeBase: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aCodeBase: *const nsAString) -> nsresult,

    /* attribute DOMString height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aHeight: *mut nsAString) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aHeight: *const nsAString) -> nsresult,

    /* attribute long hspace; */
    pub get_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aHspace: *mut libc::int32_t) -> nsresult,
    pub set_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aHspace: libc::int32_t) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString object; */
    pub get_object: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aObject: *mut nsAString) -> nsresult,
    pub set_object: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aObject: *const nsAString) -> nsresult,

    /* attribute long vspace; */
    pub get_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aVspace: *mut libc::int32_t) -> nsresult,
    pub set_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aVspace: libc::int32_t) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLAppletElement, aWidth: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLAppletElement {
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

    /* attribute DOMString alt; */
    #[inline]
    pub unsafe fn get_alt(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alt)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alt(&self, aAlt: &[u16]) -> Result<(), nsresult> {
        let aAlt = nsString::from(aAlt);
        match ((*self.vtable).set_alt)(self as *const _, &*aAlt) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString archive; */
    #[inline]
    pub unsafe fn get_archive(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_archive)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_archive(&self, aArchive: &[u16]) -> Result<(), nsresult> {
        let aArchive = nsString::from(aArchive);
        match ((*self.vtable).set_archive)(self as *const _, &*aArchive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString code; */
    #[inline]
    pub unsafe fn get_code(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_code)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_code(&self, aCode: &[u16]) -> Result<(), nsresult> {
        let aCode = nsString::from(aCode);
        match ((*self.vtable).set_code)(self as *const _, &*aCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString codeBase; */
    #[inline]
    pub unsafe fn get_codeBase(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_codeBase)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_codeBase(&self, aCodeBase: &[u16]) -> Result<(), nsresult> {
        let aCodeBase = nsString::from(aCodeBase);
        match ((*self.vtable).set_codeBase)(self as *const _, &*aCodeBase) {
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

    /* attribute long hspace; */
    #[inline]
    pub unsafe fn get_hspace(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hspace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hspace(&self, aHspace: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_hspace)(self as *const _, aHspace) {
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

    /* attribute DOMString object; */
    #[inline]
    pub unsafe fn get_object(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_object)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_object(&self, aObject: &[u16]) -> Result<(), nsresult> {
        let aObject = nsString::from(aObject);
        match ((*self.vtable).set_object)(self as *const _, &*aObject) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long vspace; */
    #[inline]
    pub unsafe fn get_vspace(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_vspace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_vspace(&self, aVspace: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_vspace)(self as *const _, aVspace) {
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


