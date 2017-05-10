//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMIMEService.idl
//


#[repr(C)]
pub struct nsIMIMEService {
    vtable: *const nsIMIMEServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMIMEService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5b3675a1, 0x02db, 0x4f8f,
            [0xa5, 0x60, 0xb3, 0x47, 0x36, 0x63, 0x5f, 0x47])
    }
}

unsafe impl RefCounted for nsIMIMEService {
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
pub trait nsIMIMEServiceCoerce {
    fn coerce_from(v: &nsIMIMEService) -> &Self;
}

impl nsIMIMEServiceCoerce for nsIMIMEService {
    #[inline]
    fn coerce_from(v: &nsIMIMEService) -> &Self {
        v
    }
}

impl nsIMIMEService {
    #[inline]
    pub fn coerce<T: nsIMIMEServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMIMEService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMIMEServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMIMEServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    pub getFromTypeAndExtension: unsafe extern "C" fn (this: *const nsIMIMEService, aMIMEType: *const nsACString, aFileExt: *const nsACString, _retval: *mut *const nsIMIMEInfo) -> nsresult,

    /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
    pub getTypeFromExtension: unsafe extern "C" fn (this: *const nsIMIMEService, aFileExt: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* ACString getTypeFromURI (in nsIURI aURI); */
    pub getTypeFromURI: unsafe extern "C" fn (this: *const nsIMIMEService, aURI: *const nsIURI, _retval: *mut nsACString) -> nsresult,

    /* ACString getTypeFromFile (in nsIFile aFile); */
    pub getTypeFromFile: unsafe extern "C" fn (this: *const nsIMIMEService, aFile: *const nsIFile, _retval: *mut nsACString) -> nsresult,

    /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    pub getPrimaryExtension: unsafe extern "C" fn (this: *const nsIMIMEService, aMIMEType: *const nsACString, aFileExt: *const nsACString, _retval: *mut nsACString) -> nsresult,

}


impl nsIMIMEService {
    /* nsIMIMEInfo getFromTypeAndExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    #[inline]
    pub unsafe fn getFromTypeAndExtension(&self, aMIMEType: &[u8], aFileExt: &[u8]) -> Result<Option<RefPtr<nsIMIMEInfo>>, nsresult> {
        let aMIMEType = nsCString::from(aMIMEType);
        let aFileExt = nsCString::from(aFileExt);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFromTypeAndExtension)(self as *const _, &*aMIMEType, &*aFileExt, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* ACString getTypeFromExtension (in AUTF8String aFileExt); */
    #[inline]
    pub unsafe fn getTypeFromExtension(&self, aFileExt: &[u8]) -> Result<nsCString, nsresult> {
        let aFileExt = nsCString::from(aFileExt);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getTypeFromExtension)(self as *const _, &*aFileExt, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getTypeFromURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn getTypeFromURI(&self, aURI: Option<&nsIURI>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getTypeFromURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getTypeFromFile (in nsIFile aFile); */
    #[inline]
    pub unsafe fn getTypeFromFile(&self, aFile: Option<&nsIFile>) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getTypeFromFile)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AUTF8String getPrimaryExtension (in ACString aMIMEType, in AUTF8String aFileExt); */
    #[inline]
    pub unsafe fn getPrimaryExtension(&self, aMIMEType: &[u8], aFileExt: &[u8]) -> Result<nsCString, nsresult> {
        let aMIMEType = nsCString::from(aMIMEType);
        let aFileExt = nsCString::from(aFileExt);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPrimaryExtension)(self as *const _, &*aMIMEType, &*aFileExt, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


