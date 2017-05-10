//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIURL.idl
//


#[repr(C)]
pub struct nsIURL {
    vtable: *const nsIURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIURL {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x86adcd89, 0x0b70, 0x47a2,
            [0xb0, 0xfe, 0x5b, 0xb2, 0xc5, 0xf3, 0x7e, 0x31])
    }
}

unsafe impl RefCounted for nsIURL {
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
pub trait nsIURLCoerce {
    fn coerce_from(v: &nsIURL) -> &Self;
}

impl nsIURLCoerce for nsIURL {
    #[inline]
    fn coerce_from(v: &nsIURL) -> &Self {
        v
    }
}

impl nsIURL {
    #[inline]
    pub fn coerce<T: nsIURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIURL {
    type Target = nsIURI;
    #[inline]
    fn deref(&self) -> &nsIURI {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIURICoerce> nsIURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURL) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIURLVTable {
    pub __base: nsIURIVTable,

    /* attribute AUTF8String directory; */
    pub get_directory: unsafe extern "C" fn (this: *const nsIURL, aDirectory: *mut nsACString) -> nsresult,
    pub set_directory: unsafe extern "C" fn (this: *const nsIURL, aDirectory: *const nsACString) -> nsresult,

    /* attribute AUTF8String fileName; */
    pub get_fileName: unsafe extern "C" fn (this: *const nsIURL, aFileName: *mut nsACString) -> nsresult,
    pub set_fileName: unsafe extern "C" fn (this: *const nsIURL, aFileName: *const nsACString) -> nsresult,

    /* attribute AUTF8String fileBaseName; */
    pub get_fileBaseName: unsafe extern "C" fn (this: *const nsIURL, aFileBaseName: *mut nsACString) -> nsresult,
    pub set_fileBaseName: unsafe extern "C" fn (this: *const nsIURL, aFileBaseName: *const nsACString) -> nsresult,

    /* attribute AUTF8String fileExtension; */
    pub get_fileExtension: unsafe extern "C" fn (this: *const nsIURL, aFileExtension: *mut nsACString) -> nsresult,
    pub set_fileExtension: unsafe extern "C" fn (this: *const nsIURL, aFileExtension: *const nsACString) -> nsresult,

    /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
    pub getCommonBaseSpec: unsafe extern "C" fn (this: *const nsIURL, aURIToCompare: *const nsIURI, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
    pub getRelativeSpec: unsafe extern "C" fn (this: *const nsIURL, aURIToCompare: *const nsIURI, _retval: *mut nsACString) -> nsresult,

}


impl nsIURL {
    /* attribute AUTF8String directory; */
    #[inline]
    pub unsafe fn get_directory(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_directory)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_directory(&self, aDirectory: &[u8]) -> Result<(), nsresult> {
        let aDirectory = nsCString::from(aDirectory);
        match ((*self.vtable).set_directory)(self as *const _, &*aDirectory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String fileName; */
    #[inline]
    pub unsafe fn get_fileName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fileName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fileName(&self, aFileName: &[u8]) -> Result<(), nsresult> {
        let aFileName = nsCString::from(aFileName);
        match ((*self.vtable).set_fileName)(self as *const _, &*aFileName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String fileBaseName; */
    #[inline]
    pub unsafe fn get_fileBaseName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fileBaseName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fileBaseName(&self, aFileBaseName: &[u8]) -> Result<(), nsresult> {
        let aFileBaseName = nsCString::from(aFileBaseName);
        match ((*self.vtable).set_fileBaseName)(self as *const _, &*aFileBaseName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String fileExtension; */
    #[inline]
    pub unsafe fn get_fileExtension(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_fileExtension)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fileExtension(&self, aFileExtension: &[u8]) -> Result<(), nsresult> {
        let aFileExtension = nsCString::from(aFileExtension);
        match ((*self.vtable).set_fileExtension)(self as *const _, &*aFileExtension) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
    #[inline]
    pub unsafe fn getCommonBaseSpec(&self, aURIToCompare: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getCommonBaseSpec)(self as *const _, aURIToCompare.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
    #[inline]
    pub unsafe fn getRelativeSpec(&self, aURIToCompare: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getRelativeSpec)(self as *const _, aURIToCompare.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


