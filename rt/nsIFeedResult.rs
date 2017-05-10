//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFeedResult.idl
//


#[repr(C)]
pub struct nsIFeedResult {
    vtable: *const nsIFeedResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFeedResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a180b78, 0x0f46, 0x4569,
            [0x8c, 0x22, 0xf3, 0xd7, 0x20, 0xea, 0x1c, 0x57])
    }
}

unsafe impl RefCounted for nsIFeedResult {
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
pub trait nsIFeedResultCoerce {
    fn coerce_from(v: &nsIFeedResult) -> &Self;
}

impl nsIFeedResultCoerce for nsIFeedResult {
    #[inline]
    fn coerce_from(v: &nsIFeedResult) -> &Self {
        v
    }
}

impl nsIFeedResult {
    #[inline]
    pub fn coerce<T: nsIFeedResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFeedResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFeedResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFeedResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFeedResultVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean bozo; */
    pub get_bozo: unsafe extern "C" fn (this: *const nsIFeedResult, aBozo: *mut bool) -> nsresult,
    pub set_bozo: unsafe extern "C" fn (this: *const nsIFeedResult, aBozo: bool) -> nsresult,

    /* attribute nsIFeedContainer doc; */
    pub get_doc: unsafe extern "C" fn (this: *const nsIFeedResult, aDoc: *mut *const nsIFeedContainer) -> nsresult,
    pub set_doc: unsafe extern "C" fn (this: *const nsIFeedResult, aDoc: *const nsIFeedContainer) -> nsresult,

    /* attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsIFeedResult, aUri: *mut *const nsIURI) -> nsresult,
    pub set_uri: unsafe extern "C" fn (this: *const nsIFeedResult, aUri: *const nsIURI) -> nsresult,

    /* attribute AString version; */
    pub get_version: unsafe extern "C" fn (this: *const nsIFeedResult, aVersion: *mut nsAString) -> nsresult,
    pub set_version: unsafe extern "C" fn (this: *const nsIFeedResult, aVersion: *const nsAString) -> nsresult,

    /* attribute nsIURI stylesheet; */
    pub get_stylesheet: unsafe extern "C" fn (this: *const nsIFeedResult, aStylesheet: *mut *const nsIURI) -> nsresult,
    pub set_stylesheet: unsafe extern "C" fn (this: *const nsIFeedResult, aStylesheet: *const nsIURI) -> nsresult,

    /* attribute nsIProperties headers; */
    pub get_headers: unsafe extern "C" fn (this: *const nsIFeedResult, aHeaders: *mut *const nsIProperties) -> nsresult,
    pub set_headers: unsafe extern "C" fn (this: *const nsIFeedResult, aHeaders: *const nsIProperties) -> nsresult,

    /* void registerExtensionPrefix (in AString aNamespace, in AString aPrefix); */
    pub registerExtensionPrefix: unsafe extern "C" fn (this: *const nsIFeedResult, aNamespace: *const nsAString, aPrefix: *const nsAString) -> nsresult,

}


impl nsIFeedResult {
    /* attribute boolean bozo; */
    #[inline]
    pub unsafe fn get_bozo(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_bozo)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_bozo(&self, aBozo: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_bozo)(self as *const _, aBozo) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIFeedContainer doc; */
    #[inline]
    pub unsafe fn get_doc(&self, ) -> Result<Option<RefPtr<nsIFeedContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_doc)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_doc(&self, aDoc: Option<&nsIFeedContainer>) -> Result<(), nsresult> {

        match ((*self.vtable).set_doc)(self as *const _, aDoc.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_uri)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_uri(&self, aUri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_uri)(self as *const _, aUri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString version; */
    #[inline]
    pub unsafe fn get_version(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_version)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_version(&self, aVersion: &[u16]) -> Result<(), nsresult> {
        let aVersion = nsString::from(aVersion);
        match ((*self.vtable).set_version)(self as *const _, &*aVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI stylesheet; */
    #[inline]
    pub unsafe fn get_stylesheet(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_stylesheet)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_stylesheet(&self, aStylesheet: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_stylesheet)(self as *const _, aStylesheet.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIProperties headers; */
    #[inline]
    pub unsafe fn get_headers(&self, ) -> Result<Option<RefPtr<nsIProperties>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_headers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_headers(&self, aHeaders: Option<&nsIProperties>) -> Result<(), nsresult> {

        match ((*self.vtable).set_headers)(self as *const _, aHeaders.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void registerExtensionPrefix (in AString aNamespace, in AString aPrefix); */
    #[inline]
    pub unsafe fn registerExtensionPrefix(&self, aNamespace: &[u16], aPrefix: &[u16]) -> Result<(), nsresult> {
        let aNamespace = nsString::from(aNamespace);
        let aPrefix = nsString::from(aPrefix);
        match ((*self.vtable).registerExtensionPrefix)(self as *const _, &*aNamespace, &*aPrefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


