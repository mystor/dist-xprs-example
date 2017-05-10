//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebContentConverterRegistrar.idl
//


#[repr(C)]
pub struct nsIWebContentHandlerInfo {
    vtable: *const nsIWebContentHandlerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebContentHandlerInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeb361098, 0x5158, 0x4b21,
            [0x8f, 0x98, 0x50, 0xb4, 0x45, 0xf1, 0xf0, 0xb2])
    }
}

unsafe impl RefCounted for nsIWebContentHandlerInfo {
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
pub trait nsIWebContentHandlerInfoCoerce {
    fn coerce_from(v: &nsIWebContentHandlerInfo) -> &Self;
}

impl nsIWebContentHandlerInfoCoerce for nsIWebContentHandlerInfo {
    #[inline]
    fn coerce_from(v: &nsIWebContentHandlerInfo) -> &Self {
        v
    }
}

impl nsIWebContentHandlerInfo {
    #[inline]
    pub fn coerce<T: nsIWebContentHandlerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebContentHandlerInfo {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIHandlerAppCoerce> nsIWebContentHandlerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebContentHandlerInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebContentHandlerInfoVTable {
    pub __base: nsIHandlerAppVTable,

    /* readonly attribute AString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIWebContentHandlerInfo, aContentType: *mut nsAString) -> nsresult,

    /* readonly attribute AString uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsIWebContentHandlerInfo, aUri: *mut nsAString) -> nsresult,

    /* AString getHandlerURI (in AString uri); */
    pub getHandlerURI: unsafe extern "C" fn (this: *const nsIWebContentHandlerInfo, uri: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIWebContentHandlerInfo {
    /* readonly attribute AString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_uri)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getHandlerURI (in AString uri); */
    #[inline]
    pub unsafe fn getHandlerURI(&self, uri: &[u16]) -> Result<nsString, nsresult> {
        let uri = nsString::from(uri);
        let mut _retval = nsString::new();
        match ((*self.vtable).getHandlerURI)(self as *const _, &*uri, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIWebContentConverterService {
    vtable: *const nsIWebContentConverterServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebContentConverterService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde7cc06e, 0xe778, 0x45cb,
            [0xb7, 0xdb, 0x7a, 0x11, 0x4e, 0x1e, 0x75, 0xb1])
    }
}

unsafe impl RefCounted for nsIWebContentConverterService {
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
pub trait nsIWebContentConverterServiceCoerce {
    fn coerce_from(v: &nsIWebContentConverterService) -> &Self;
}

impl nsIWebContentConverterServiceCoerce for nsIWebContentConverterService {
    #[inline]
    fn coerce_from(v: &nsIWebContentConverterService) -> &Self {
        v
    }
}

impl nsIWebContentConverterService {
    #[inline]
    pub fn coerce<T: nsIWebContentConverterServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebContentConverterService {
    type Target = nsIWebContentHandlerRegistrar;
    #[inline]
    fn deref(&self) -> &nsIWebContentHandlerRegistrar {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebContentHandlerRegistrarCoerce> nsIWebContentConverterServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebContentConverterService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebContentConverterServiceVTable {
    pub __base: nsIWebContentHandlerRegistrarVTable,

    /* void setAutoHandler (in AString contentType, in nsIWebContentHandlerInfo handler); */
    pub setAutoHandler: unsafe extern "C" fn (this: *const nsIWebContentConverterService, contentType: *const nsAString, handler: *const nsIWebContentHandlerInfo) -> nsresult,

    /* nsIWebContentHandlerInfo getAutoHandler (in AString contentType); */
    pub getAutoHandler: unsafe extern "C" fn (this: *const nsIWebContentConverterService, contentType: *const nsAString, _retval: *mut *const nsIWebContentHandlerInfo) -> nsresult,

    /* nsIWebContentHandlerInfo getWebContentHandlerByURI (in AString contentType, in AString uri); */
    pub getWebContentHandlerByURI: unsafe extern "C" fn (this: *const nsIWebContentConverterService, contentType: *const nsAString, uri: *const nsAString, _retval: *mut *const nsIWebContentHandlerInfo) -> nsresult,

    /* void loadPreferredHandler (in nsIRequest request); */
    pub loadPreferredHandler: unsafe extern "C" fn (this: *const nsIWebContentConverterService, request: *const nsIRequest) -> nsresult,

    /* void removeProtocolHandler (in AString protocol, in AString uri); */
    pub removeProtocolHandler: unsafe extern "C" fn (this: *const nsIWebContentConverterService, protocol: *const nsAString, uri: *const nsAString) -> nsresult,

    /* void removeContentHandler (in AString contentType, in AString uri); */
    pub removeContentHandler: unsafe extern "C" fn (this: *const nsIWebContentConverterService, contentType: *const nsAString, uri: *const nsAString) -> nsresult,

    /* void getContentHandlers (in AString contentType, [optional] out unsigned long count, [array, size_is (count), retval] out nsIWebContentHandlerInfo handlers); */
    /// Unable to call function as its signature contains a non-rust type
    pub getContentHandlers: *const ::libc::c_void,

    /* void resetHandlersForType (in AString contentType); */
    pub resetHandlersForType: unsafe extern "C" fn (this: *const nsIWebContentConverterService, contentType: *const nsAString) -> nsresult,

}


impl nsIWebContentConverterService {
    /* void setAutoHandler (in AString contentType, in nsIWebContentHandlerInfo handler); */
    #[inline]
    pub unsafe fn setAutoHandler(&self, contentType: &[u16], handler: Option<&nsIWebContentHandlerInfo>) -> Result<(), nsresult> {
        let contentType = nsString::from(contentType);
        match ((*self.vtable).setAutoHandler)(self as *const _, &*contentType, handler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIWebContentHandlerInfo getAutoHandler (in AString contentType); */
    #[inline]
    pub unsafe fn getAutoHandler(&self, contentType: &[u16]) -> Result<Option<RefPtr<nsIWebContentHandlerInfo>>, nsresult> {
        let contentType = nsString::from(contentType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAutoHandler)(self as *const _, &*contentType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIWebContentHandlerInfo getWebContentHandlerByURI (in AString contentType, in AString uri); */
    #[inline]
    pub unsafe fn getWebContentHandlerByURI(&self, contentType: &[u16], uri: &[u16]) -> Result<Option<RefPtr<nsIWebContentHandlerInfo>>, nsresult> {
        let contentType = nsString::from(contentType);
        let uri = nsString::from(uri);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWebContentHandlerByURI)(self as *const _, &*contentType, &*uri, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void loadPreferredHandler (in nsIRequest request); */
    #[inline]
    pub unsafe fn loadPreferredHandler(&self, request: Option<&nsIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).loadPreferredHandler)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeProtocolHandler (in AString protocol, in AString uri); */
    #[inline]
    pub unsafe fn removeProtocolHandler(&self, protocol: &[u16], uri: &[u16]) -> Result<(), nsresult> {
        let protocol = nsString::from(protocol);
        let uri = nsString::from(uri);
        match ((*self.vtable).removeProtocolHandler)(self as *const _, &*protocol, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeContentHandler (in AString contentType, in AString uri); */
    #[inline]
    pub unsafe fn removeContentHandler(&self, contentType: &[u16], uri: &[u16]) -> Result<(), nsresult> {
        let contentType = nsString::from(contentType);
        let uri = nsString::from(uri);
        match ((*self.vtable).removeContentHandler)(self as *const _, &*contentType, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getContentHandlers (in AString contentType, [optional] out unsigned long count, [array, size_is (count), retval] out nsIWebContentHandlerInfo handlers); */


    /* void resetHandlersForType (in AString contentType); */
    #[inline]
    pub unsafe fn resetHandlersForType(&self, contentType: &[u16]) -> Result<(), nsresult> {
        let contentType = nsString::from(contentType);
        match ((*self.vtable).resetHandlersForType)(self as *const _, &*contentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


