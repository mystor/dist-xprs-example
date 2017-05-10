//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedTextConstruct.idl
//


#[repr(C)]
pub struct nsIFeedTextConstruct {
    vtable: *const nsIFeedTextConstructVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedTextConstruct {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfc97a2a9, 0xd649, 0x4494,
            [0x93, 0x1e, 0xdb, 0x81, 0xa1, 0x56, 0xc8, 0x73])
    }
}

unsafe impl RefCounted for nsIFeedTextConstruct {
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
pub trait nsIFeedTextConstructCoerce {
    fn coerce_from(v: &nsIFeedTextConstruct) -> &Self;
}

impl nsIFeedTextConstructCoerce for nsIFeedTextConstruct {
    #[inline]
    fn coerce_from(v: &nsIFeedTextConstruct) -> &Self {
        v
    }
}

impl nsIFeedTextConstruct {
    #[inline]
    pub fn coerce<T: nsIFeedTextConstructCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedTextConstruct {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFeedTextConstructCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedTextConstruct) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedTextConstructVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIURI base; */
    pub get_base: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aBase: *mut *const nsIURI) -> nsresult,
    pub set_base: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aBase: *const nsIURI) -> nsresult,

    /* attribute AString lang; */
    pub get_lang: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aLang: *mut nsAString) -> nsresult,
    pub set_lang: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aLang: *const nsAString) -> nsresult,

    /* attribute AString type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aType: *mut nsAString) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aType: *const nsAString) -> nsresult,

    /* attribute AString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aText: *mut nsAString) -> nsresult,
    pub set_text: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, aText: *const nsAString) -> nsresult,

    /* AString plainText (); */
    pub plainText: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, _retval: *mut nsAString) -> nsresult,

    /* nsIDOMDocumentFragment createDocumentFragment (in nsIDOMElement element); */
    pub createDocumentFragment: unsafe extern "C" fn (this: *const nsIFeedTextConstruct, element: *const nsIDOMElement, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

}


impl nsIFeedTextConstruct {
    /* attribute nsIURI base; */
    #[inline]
    pub unsafe fn get_base(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_base)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_base(&self, aBase: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_base)(self as *const _, aBase.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString lang; */
    #[inline]
    pub unsafe fn get_lang(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lang)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lang(&self, aLang: &[u16]) -> Result<(), nsresult> {
        let aLang = nsString::from(aLang);
        match ((*self.vtable).set_lang)(self as *const _, &*aLang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString type; */
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

    /* attribute AString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_text(&self, aText: &[u16]) -> Result<(), nsresult> {
        let aText = nsString::from(aText);
        match ((*self.vtable).set_text)(self as *const _, &*aText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString plainText (); */
    #[inline]
    pub unsafe fn plainText(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).plainText)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMDocumentFragment createDocumentFragment (in nsIDOMElement element); */
    #[inline]
    pub unsafe fn createDocumentFragment(&self, element: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createDocumentFragment)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


