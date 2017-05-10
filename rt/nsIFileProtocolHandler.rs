//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFileProtocolHandler.idl
//


#[repr(C)]
pub struct nsIFileProtocolHandler {
    vtable: *const nsIFileProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFileProtocolHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1fb25bd5, 0x4354, 0x4dcd,
            [0x8d, 0x97, 0x62, 0x1b, 0x7b, 0x3e, 0xd2, 0xe4])
    }
}

unsafe impl RefCounted for nsIFileProtocolHandler {
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
pub trait nsIFileProtocolHandlerCoerce {
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self;
}

impl nsIFileProtocolHandlerCoerce for nsIFileProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self {
        v
    }
}

impl nsIFileProtocolHandler {
    #[inline]
    pub fn coerce<T: nsIFileProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFileProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolHandlerCoerce> nsIFileProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFileProtocolHandlerVTable {
    pub __base: nsIProtocolHandlerVTable,

    /* nsIURI newFileURI (in nsIFile aFile); */
    pub newFileURI: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, aFile: *const nsIFile, _retval: *mut *const nsIURI) -> nsresult,

    /* AUTF8String getURLSpecFromFile (in nsIFile file); */
    pub getURLSpecFromFile: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
    pub getURLSpecFromActualFile: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String getURLSpecFromDir (in nsIFile file); */
    pub getURLSpecFromDir: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* nsIFile getFileFromURLSpec (in AUTF8String url); */
    pub getFileFromURLSpec: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, url: *const nsACString, _retval: *mut *const nsIFile) -> nsresult,

    /* nsIURI readURLFile (in nsIFile file); */
    pub readURLFile: unsafe extern "C" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut *const nsIURI) -> nsresult,

}


impl nsIFileProtocolHandler {
    /* nsIURI newFileURI (in nsIFile aFile); */
    #[inline]
    pub unsafe fn newFileURI(&self, aFile: Option<&nsIFile>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).newFileURI)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AUTF8String getURLSpecFromFile (in nsIFile file); */
    #[inline]
    pub unsafe fn getURLSpecFromFile(&self, file: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getURLSpecFromFile)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
    #[inline]
    pub unsafe fn getURLSpecFromActualFile(&self, file: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getURLSpecFromActualFile)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getURLSpecFromDir (in nsIFile file); */
    #[inline]
    pub unsafe fn getURLSpecFromDir(&self, file: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getURLSpecFromDir)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIFile getFileFromURLSpec (in AUTF8String url); */
    #[inline]
    pub unsafe fn getFileFromURLSpec(&self, url: &[u8]) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let url = nsCString::from(url);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFileFromURLSpec)(self as *const _, &*url, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIURI readURLFile (in nsIFile file); */
    #[inline]
    pub unsafe fn readURLFile(&self, file: Option<&nsIFile>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).readURLFile)(self as *const _, file.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


