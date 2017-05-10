//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFService.idl
//


#[repr(C)]
pub struct nsIRDFService {
    vtable: *const nsIRDFServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbfd05261, 0x834c, 0x11d2,
            [0x8e, 0xac, 0x00, 0x80, 0x5f, 0x29, 0xf3, 0x70])
    }
}

unsafe impl RefCounted for nsIRDFService {
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
pub trait nsIRDFServiceCoerce {
    fn coerce_from(v: &nsIRDFService) -> &Self;
}

impl nsIRDFServiceCoerce for nsIRDFService {
    #[inline]
    fn coerce_from(v: &nsIRDFService) -> &Self {
        v
    }
}

impl nsIRDFService {
    #[inline]
    pub fn coerce<T: nsIRDFServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIRDFResource GetResource (in AUTF8String aURI); */
    pub GetResource: unsafe extern "C" fn (this: *const nsIRDFService, aURI: *const nsACString, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* nsIRDFResource GetUnicodeResource (in AString aURI); */
    pub GetUnicodeResource: unsafe extern "C" fn (this: *const nsIRDFService, aURI: *const nsAString, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* nsIRDFResource GetAnonymousResource (); */
    pub GetAnonymousResource: unsafe extern "C" fn (this: *const nsIRDFService, _retval: *mut *const nsIRDFResource) -> nsresult,

    /* nsIRDFLiteral GetLiteral (in wstring aValue); */
    pub GetLiteral: unsafe extern "C" fn (this: *const nsIRDFService, aValue: *const libc::int16_t, _retval: *mut *const nsIRDFLiteral) -> nsresult,

    /* nsIRDFDate GetDateLiteral (in PRTime aValue); */
    pub GetDateLiteral: unsafe extern "C" fn (this: *const nsIRDFService, aValue: PRTime, _retval: *mut *const nsIRDFDate) -> nsresult,

    /* nsIRDFInt GetIntLiteral (in long aValue); */
    pub GetIntLiteral: unsafe extern "C" fn (this: *const nsIRDFService, aValue: libc::int32_t, _retval: *mut *const nsIRDFInt) -> nsresult,

    /* [noscript] nsIRDFBlob getBlobLiteral (in const_octet_ptr aValue, in long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub getBlobLiteral: *const ::libc::c_void,

    /* boolean IsAnonymousResource (in nsIRDFResource aResource); */
    pub IsAnonymousResource: unsafe extern "C" fn (this: *const nsIRDFService, aResource: *const nsIRDFResource, _retval: *mut bool) -> nsresult,

    /* void RegisterResource (in nsIRDFResource aResource, in boolean aReplace); */
    pub RegisterResource: unsafe extern "C" fn (this: *const nsIRDFService, aResource: *const nsIRDFResource, aReplace: bool) -> nsresult,

    /* void UnregisterResource (in nsIRDFResource aResource); */
    pub UnregisterResource: unsafe extern "C" fn (this: *const nsIRDFService, aResource: *const nsIRDFResource) -> nsresult,

    /* void RegisterDataSource (in nsIRDFDataSource aDataSource, in boolean aReplace); */
    pub RegisterDataSource: unsafe extern "C" fn (this: *const nsIRDFService, aDataSource: *const nsIRDFDataSource, aReplace: bool) -> nsresult,

    /* void UnregisterDataSource (in nsIRDFDataSource aDataSource); */
    pub UnregisterDataSource: unsafe extern "C" fn (this: *const nsIRDFService, aDataSource: *const nsIRDFDataSource) -> nsresult,

    /* nsIRDFDataSource GetDataSource (in string aURI); */
    pub GetDataSource: unsafe extern "C" fn (this: *const nsIRDFService, aURI: *const libc::c_char, _retval: *mut *const nsIRDFDataSource) -> nsresult,

    /* nsIRDFDataSource GetDataSourceBlocking (in string aURI); */
    pub GetDataSourceBlocking: unsafe extern "C" fn (this: *const nsIRDFService, aURI: *const libc::c_char, _retval: *mut *const nsIRDFDataSource) -> nsresult,

}


impl nsIRDFService {
    /* nsIRDFResource GetResource (in AUTF8String aURI); */
    #[inline]
    pub unsafe fn GetResource(&self, aURI: &[u8]) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let aURI = nsCString::from(aURI);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetResource)(self as *const _, &*aURI, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFResource GetUnicodeResource (in AString aURI); */
    #[inline]
    pub unsafe fn GetUnicodeResource(&self, aURI: &[u16]) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let aURI = nsString::from(aURI);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetUnicodeResource)(self as *const _, &*aURI, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFResource GetAnonymousResource (); */
    #[inline]
    pub unsafe fn GetAnonymousResource(&self, ) -> Result<Option<RefPtr<nsIRDFResource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetAnonymousResource)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFLiteral GetLiteral (in wstring aValue); */
    #[inline]
    pub unsafe fn GetLiteral(&self, aValue: *const libc::int16_t) -> Result<Option<RefPtr<nsIRDFLiteral>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetLiteral)(self as *const _, aValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFDate GetDateLiteral (in PRTime aValue); */
    #[inline]
    pub unsafe fn GetDateLiteral(&self, aValue: PRTime) -> Result<Option<RefPtr<nsIRDFDate>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetDateLiteral)(self as *const _, aValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFInt GetIntLiteral (in long aValue); */
    #[inline]
    pub unsafe fn GetIntLiteral(&self, aValue: libc::int32_t) -> Result<Option<RefPtr<nsIRDFInt>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetIntLiteral)(self as *const _, aValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsIRDFBlob getBlobLiteral (in const_octet_ptr aValue, in long aLength); */


    /* boolean IsAnonymousResource (in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn IsAnonymousResource(&self, aResource: Option<&nsIRDFResource>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).IsAnonymousResource)(self as *const _, aResource.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void RegisterResource (in nsIRDFResource aResource, in boolean aReplace); */
    #[inline]
    pub unsafe fn RegisterResource(&self, aResource: Option<&nsIRDFResource>, aReplace: bool) -> Result<(), nsresult> {

        match ((*self.vtable).RegisterResource)(self as *const _, aResource.map_or(::std::ptr::null(), |x| x as *const _), aReplace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void UnregisterResource (in nsIRDFResource aResource); */
    #[inline]
    pub unsafe fn UnregisterResource(&self, aResource: Option<&nsIRDFResource>) -> Result<(), nsresult> {

        match ((*self.vtable).UnregisterResource)(self as *const _, aResource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RegisterDataSource (in nsIRDFDataSource aDataSource, in boolean aReplace); */
    #[inline]
    pub unsafe fn RegisterDataSource(&self, aDataSource: Option<&nsIRDFDataSource>, aReplace: bool) -> Result<(), nsresult> {

        match ((*self.vtable).RegisterDataSource)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _), aReplace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void UnregisterDataSource (in nsIRDFDataSource aDataSource); */
    #[inline]
    pub unsafe fn UnregisterDataSource(&self, aDataSource: Option<&nsIRDFDataSource>) -> Result<(), nsresult> {

        match ((*self.vtable).UnregisterDataSource)(self as *const _, aDataSource.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIRDFDataSource GetDataSource (in string aURI); */
    #[inline]
    pub unsafe fn GetDataSource(&self, aURI: *const libc::c_char) -> Result<Option<RefPtr<nsIRDFDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetDataSource)(self as *const _, aURI, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIRDFDataSource GetDataSourceBlocking (in string aURI); */
    #[inline]
    pub unsafe fn GetDataSourceBlocking(&self, aURI: *const libc::c_char) -> Result<Option<RefPtr<nsIRDFDataSource>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).GetDataSourceBlocking)(self as *const _, aURI, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


