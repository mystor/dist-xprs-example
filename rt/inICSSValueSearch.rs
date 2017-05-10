//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inICSSValueSearch.idl
//


#[repr(C)]
pub struct inICSSValueSearch {
    vtable: *const inICSSValueSearchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inICSSValueSearch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe0d39e48, 0x1dd1, 0x11b2,
            [0x81, 0xbd, 0x9a, 0x0c, 0x11, 0x7f, 0x07, 0x36])
    }
}

unsafe impl RefCounted for inICSSValueSearch {
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
pub trait inICSSValueSearchCoerce {
    fn coerce_from(v: &inICSSValueSearch) -> &Self;
}

impl inICSSValueSearchCoerce for inICSSValueSearch {
    #[inline]
    fn coerce_from(v: &inICSSValueSearch) -> &Self {
        v
    }
}

impl inICSSValueSearch {
    #[inline]
    pub fn coerce<T: inICSSValueSearchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inICSSValueSearch {
    type Target = inISearchProcess;
    #[inline]
    fn deref(&self) -> &inISearchProcess {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: inISearchProcessCoerce> inICSSValueSearchCoerce for T {
    #[inline]
    fn coerce_from(v: &inICSSValueSearch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inICSSValueSearchVTable {
    pub __base: inISearchProcessVTable,

    /* attribute nsIDOMDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const inICSSValueSearch, aDocument: *mut *const nsIDOMDocument) -> nsresult,
    pub set_document: unsafe extern "C" fn (this: *const inICSSValueSearch, aDocument: *const nsIDOMDocument) -> nsresult,

    /* attribute wstring baseURL; */
    pub get_baseURL: unsafe extern "C" fn (this: *const inICSSValueSearch, aBaseURL: *mut *const libc::int16_t) -> nsresult,
    pub set_baseURL: unsafe extern "C" fn (this: *const inICSSValueSearch, aBaseURL: *const libc::int16_t) -> nsresult,

    /* attribute boolean returnRelativeURLs; */
    pub get_returnRelativeURLs: unsafe extern "C" fn (this: *const inICSSValueSearch, aReturnRelativeURLs: *mut bool) -> nsresult,
    pub set_returnRelativeURLs: unsafe extern "C" fn (this: *const inICSSValueSearch, aReturnRelativeURLs: bool) -> nsresult,

    /* attribute boolean normalizeChromeURLs; */
    pub get_normalizeChromeURLs: unsafe extern "C" fn (this: *const inICSSValueSearch, aNormalizeChromeURLs: *mut bool) -> nsresult,
    pub set_normalizeChromeURLs: unsafe extern "C" fn (this: *const inICSSValueSearch, aNormalizeChromeURLs: bool) -> nsresult,

    /* void addPropertyCriteria (in wstring aPropName); */
    pub addPropertyCriteria: unsafe extern "C" fn (this: *const inICSSValueSearch, aPropName: *const libc::int16_t) -> nsresult,

    /* attribute wstring textCriteria; */
    pub get_textCriteria: unsafe extern "C" fn (this: *const inICSSValueSearch, aTextCriteria: *mut *const libc::int16_t) -> nsresult,
    pub set_textCriteria: unsafe extern "C" fn (this: *const inICSSValueSearch, aTextCriteria: *const libc::int16_t) -> nsresult,

}


impl inICSSValueSearch {
    /* attribute nsIDOMDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_document(&self, aDocument: Option<&nsIDOMDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).set_document)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring baseURL; */
    #[inline]
    pub unsafe fn get_baseURL(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_baseURL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_baseURL(&self, aBaseURL: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURL)(self as *const _, aBaseURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean returnRelativeURLs; */
    #[inline]
    pub unsafe fn get_returnRelativeURLs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_returnRelativeURLs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_returnRelativeURLs(&self, aReturnRelativeURLs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_returnRelativeURLs)(self as *const _, aReturnRelativeURLs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean normalizeChromeURLs; */
    #[inline]
    pub unsafe fn get_normalizeChromeURLs(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_normalizeChromeURLs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_normalizeChromeURLs(&self, aNormalizeChromeURLs: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_normalizeChromeURLs)(self as *const _, aNormalizeChromeURLs) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addPropertyCriteria (in wstring aPropName); */
    #[inline]
    pub unsafe fn addPropertyCriteria(&self, aPropName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).addPropertyCriteria)(self as *const _, aPropName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute wstring textCriteria; */
    #[inline]
    pub unsafe fn get_textCriteria(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_textCriteria)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_textCriteria(&self, aTextCriteria: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_textCriteria)(self as *const _, aTextCriteria) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


