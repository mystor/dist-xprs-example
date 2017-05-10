//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLObjectElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLObjectElement {
    vtable: *const nsIDOMHTMLObjectElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLObjectElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbaf443d2, 0xda5d, 0x40c9,
            [0xbe, 0x3c, 0xc6, 0x5a, 0x69, 0xa2, 0x52, 0x50])
    }
}

unsafe impl RefCounted for nsIDOMHTMLObjectElement {
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
pub trait nsIDOMHTMLObjectElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLObjectElement) -> &Self;
}

impl nsIDOMHTMLObjectElementCoerce for nsIDOMHTMLObjectElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLObjectElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLObjectElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLObjectElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLObjectElement {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMHTMLObjectElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLObjectElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLObjectElementVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDOMHTMLFormElement form; */
    pub get_form: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aForm: *mut *const nsIDOMHTMLFormElement) -> nsresult,

    /* attribute DOMString code; */
    pub get_code: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCode: *mut nsAString) -> nsresult,
    pub set_code: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCode: *const nsAString) -> nsresult,

    /* attribute DOMString align; */
    pub get_align: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aAlign: *mut nsAString) -> nsresult,
    pub set_align: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aAlign: *const nsAString) -> nsresult,

    /* attribute DOMString archive; */
    pub get_archive: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aArchive: *mut nsAString) -> nsresult,
    pub set_archive: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aArchive: *const nsAString) -> nsresult,

    /* attribute DOMString border; */
    pub get_border: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aBorder: *mut nsAString) -> nsresult,
    pub set_border: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aBorder: *const nsAString) -> nsresult,

    /* attribute DOMString codeBase; */
    pub get_codeBase: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCodeBase: *mut nsAString) -> nsresult,
    pub set_codeBase: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCodeBase: *const nsAString) -> nsresult,

    /* attribute DOMString codeType; */
    pub get_codeType: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCodeType: *mut nsAString) -> nsresult,
    pub set_codeType: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aCodeType: *const nsAString) -> nsresult,

    /* attribute DOMString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aData: *mut nsAString) -> nsresult,
    pub set_data: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aData: *const nsAString) -> nsresult,

    /* attribute boolean declare; */
    pub get_declare: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aDeclare: *mut bool) -> nsresult,
    pub set_declare: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aDeclare: bool) -> nsresult,

    /* attribute DOMString height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aHeight: *mut nsAString) -> nsresult,
    pub set_height: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aHeight: *const nsAString) -> nsresult,

    /* attribute long hspace; */
    pub get_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aHspace: *mut libc::int32_t) -> nsresult,
    pub set_hspace: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aHspace: libc::int32_t) -> nsresult,

    /* attribute DOMString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aName: *mut nsAString) -> nsresult,
    pub set_name: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aName: *const nsAString) -> nsresult,

    /* attribute DOMString standby; */
    pub get_standby: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aStandby: *mut nsAString) -> nsresult,
    pub set_standby: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aStandby: *const nsAString) -> nsresult,

    /* attribute DOMString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aType: *const nsAString) -> nsresult,

    /* attribute DOMString useMap; */
    pub get_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aUseMap: *mut nsAString) -> nsresult,
    pub set_useMap: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aUseMap: *const nsAString) -> nsresult,

    /* attribute long vspace; */
    pub get_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aVspace: *mut libc::int32_t) -> nsresult,
    pub set_vspace: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aVspace: libc::int32_t) -> nsresult,

    /* attribute DOMString width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aWidth: *mut nsAString) -> nsresult,
    pub set_width: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aWidth: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMDocument contentDocument; */
    pub get_contentDocument: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aContentDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute boolean willValidate; */
    pub get_willValidate: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aWillValidate: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMValidityState validity; */
    pub get_validity: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aValidity: *mut *const nsIDOMValidityState) -> nsresult,

    /* readonly attribute DOMString validationMessage; */
    pub get_validationMessage: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, aValidationMessage: *mut nsAString) -> nsresult,

    /* boolean checkValidity (); */
    pub checkValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, _retval: *mut bool) -> nsresult,

    /* void setCustomValidity (in DOMString error); */
    pub setCustomValidity: unsafe extern "C" fn (this: *const nsIDOMHTMLObjectElement, error: *const nsAString) -> nsresult,

}


impl nsIDOMHTMLObjectElement {
    /* readonly attribute nsIDOMHTMLFormElement form; */
    #[inline]
    pub unsafe fn get_form(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLFormElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_form)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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

    /* attribute DOMString border; */
    #[inline]
    pub unsafe fn get_border(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_border)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_border(&self, aBorder: &[u16]) -> Result<(), nsresult> {
        let aBorder = nsString::from(aBorder);
        match ((*self.vtable).set_border)(self as *const _, &*aBorder) {
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

    /* attribute DOMString codeType; */
    #[inline]
    pub unsafe fn get_codeType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_codeType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_codeType(&self, aCodeType: &[u16]) -> Result<(), nsresult> {
        let aCodeType = nsString::from(aCodeType);
        match ((*self.vtable).set_codeType)(self as *const _, &*aCodeType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_data(&self, aData: &[u16]) -> Result<(), nsresult> {
        let aData = nsString::from(aData);
        match ((*self.vtable).set_data)(self as *const _, &*aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean declare; */
    #[inline]
    pub unsafe fn get_declare(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_declare)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_declare(&self, aDeclare: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_declare)(self as *const _, aDeclare) {
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

    /* attribute DOMString standby; */
    #[inline]
    pub unsafe fn get_standby(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_standby)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_standby(&self, aStandby: &[u16]) -> Result<(), nsresult> {
        let aStandby = nsString::from(aStandby);
        match ((*self.vtable).set_standby)(self as *const _, &*aStandby) {
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

    /* attribute DOMString useMap; */
    #[inline]
    pub unsafe fn get_useMap(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_useMap)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useMap(&self, aUseMap: &[u16]) -> Result<(), nsresult> {
        let aUseMap = nsString::from(aUseMap);
        match ((*self.vtable).set_useMap)(self as *const _, &*aUseMap) {
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

    /* readonly attribute nsIDOMDocument contentDocument; */
    #[inline]
    pub unsafe fn get_contentDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean willValidate; */
    #[inline]
    pub unsafe fn get_willValidate(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_willValidate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMValidityState validity; */
    #[inline]
    pub unsafe fn get_validity(&self, ) -> Result<Option<RefPtr<nsIDOMValidityState>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_validity)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString validationMessage; */
    #[inline]
    pub unsafe fn get_validationMessage(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_validationMessage)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean checkValidity (); */
    #[inline]
    pub unsafe fn checkValidity(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkValidity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCustomValidity (in DOMString error); */
    #[inline]
    pub unsafe fn setCustomValidity(&self, error: &[u16]) -> Result<(), nsresult> {
        let error = nsString::from(error);
        match ((*self.vtable).setCustomValidity)(self as *const _, &*error) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


