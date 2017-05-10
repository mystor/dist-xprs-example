//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXXMLReader.idl
//


#[repr(C)]
pub struct nsISAXXMLReader {
    vtable: *const nsISAXXMLReaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXXMLReader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5b1de802, 0x9091, 0x454f,
            [0x99, 0x72, 0x57, 0x53, 0xc0, 0xd0, 0xc7, 0x0e])
    }
}

unsafe impl RefCounted for nsISAXXMLReader {
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
pub trait nsISAXXMLReaderCoerce {
    fn coerce_from(v: &nsISAXXMLReader) -> &Self;
}

impl nsISAXXMLReaderCoerce for nsISAXXMLReader {
    #[inline]
    fn coerce_from(v: &nsISAXXMLReader) -> &Self {
        v
    }
}

impl nsISAXXMLReader {
    #[inline]
    pub fn coerce<T: nsISAXXMLReaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXXMLReader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsISAXXMLReaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXXMLReader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXXMLReaderVTable {
    pub __base: nsIStreamListenerVTable,

    /* attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsISAXXMLReader, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsISAXXMLReader, aBaseURI: *const nsIURI) -> nsresult,

    /* attribute nsISAXContentHandler contentHandler; */
    pub get_contentHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aContentHandler: *mut *const nsISAXContentHandler) -> nsresult,
    pub set_contentHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aContentHandler: *const nsISAXContentHandler) -> nsresult,

    /* attribute nsISAXDTDHandler dtdHandler; */
    pub get_dtdHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aDtdHandler: *mut *const nsISAXDTDHandler) -> nsresult,
    pub set_dtdHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aDtdHandler: *const nsISAXDTDHandler) -> nsresult,

    /* attribute nsISAXErrorHandler errorHandler; */
    pub get_errorHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aErrorHandler: *mut *const nsISAXErrorHandler) -> nsresult,
    pub set_errorHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aErrorHandler: *const nsISAXErrorHandler) -> nsresult,

    /* attribute nsIMozSAXXMLDeclarationHandler declarationHandler; */
    pub get_declarationHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aDeclarationHandler: *mut *const nsIMozSAXXMLDeclarationHandler) -> nsresult,
    pub set_declarationHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aDeclarationHandler: *const nsIMozSAXXMLDeclarationHandler) -> nsresult,

    /* attribute nsISAXLexicalHandler lexicalHandler; */
    pub get_lexicalHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aLexicalHandler: *mut *const nsISAXLexicalHandler) -> nsresult,
    pub set_lexicalHandler: unsafe extern "C" fn (this: *const nsISAXXMLReader, aLexicalHandler: *const nsISAXLexicalHandler) -> nsresult,

    /* void setFeature (in AString name, in boolean value); */
    pub setFeature: unsafe extern "C" fn (this: *const nsISAXXMLReader, name: *const nsAString, value: bool) -> nsresult,

    /* boolean getFeature (in AString name); */
    pub getFeature: unsafe extern "C" fn (this: *const nsISAXXMLReader, name: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void setProperty (in AString name, in nsISupports value); */
    pub setProperty: unsafe extern "C" fn (this: *const nsISAXXMLReader, name: *const nsAString, value: *const nsISupports) -> nsresult,

    /* boolean getProperty (in AString name); */
    pub getProperty: unsafe extern "C" fn (this: *const nsISAXXMLReader, name: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void parseFromString (in AString str, in string contentType); */
    pub parseFromString: unsafe extern "C" fn (this: *const nsISAXXMLReader, str: *const nsAString, contentType: *const libc::c_char) -> nsresult,

    /* void parseFromStream (in nsIInputStream stream, in string charset, in string contentType); */
    pub parseFromStream: unsafe extern "C" fn (this: *const nsISAXXMLReader, stream: *const nsIInputStream, charset: *const libc::c_char, contentType: *const libc::c_char) -> nsresult,

    /* void parseAsync (in nsIRequestObserver observer); */
    pub parseAsync: unsafe extern "C" fn (this: *const nsISAXXMLReader, observer: *const nsIRequestObserver) -> nsresult,

}


impl nsISAXXMLReader {
    /* attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseURI(&self, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURI)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISAXContentHandler contentHandler; */
    #[inline]
    pub unsafe fn get_contentHandler(&self, ) -> Result<Option<RefPtr<nsISAXContentHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_contentHandler(&self, aContentHandler: Option<&nsISAXContentHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentHandler)(self as *const _, aContentHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISAXDTDHandler dtdHandler; */
    #[inline]
    pub unsafe fn get_dtdHandler(&self, ) -> Result<Option<RefPtr<nsISAXDTDHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dtdHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_dtdHandler(&self, aDtdHandler: Option<&nsISAXDTDHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).set_dtdHandler)(self as *const _, aDtdHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISAXErrorHandler errorHandler; */
    #[inline]
    pub unsafe fn get_errorHandler(&self, ) -> Result<Option<RefPtr<nsISAXErrorHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_errorHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_errorHandler(&self, aErrorHandler: Option<&nsISAXErrorHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).set_errorHandler)(self as *const _, aErrorHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIMozSAXXMLDeclarationHandler declarationHandler; */
    #[inline]
    pub unsafe fn get_declarationHandler(&self, ) -> Result<Option<RefPtr<nsIMozSAXXMLDeclarationHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_declarationHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_declarationHandler(&self, aDeclarationHandler: Option<&nsIMozSAXXMLDeclarationHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).set_declarationHandler)(self as *const _, aDeclarationHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISAXLexicalHandler lexicalHandler; */
    #[inline]
    pub unsafe fn get_lexicalHandler(&self, ) -> Result<Option<RefPtr<nsISAXLexicalHandler>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_lexicalHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_lexicalHandler(&self, aLexicalHandler: Option<&nsISAXLexicalHandler>) -> Result<(), nsresult> {

        match ((*self.vtable).set_lexicalHandler)(self as *const _, aLexicalHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFeature (in AString name, in boolean value); */
    #[inline]
    pub unsafe fn setFeature(&self, name: &[u16], value: bool) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).setFeature)(self as *const _, &*name, value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getFeature (in AString name); */
    #[inline]
    pub unsafe fn getFeature(&self, name: &[u16]) -> Result<bool, nsresult> {
        let name = nsString::from(name);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getFeature)(self as *const _, &*name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setProperty (in AString name, in nsISupports value); */
    #[inline]
    pub unsafe fn setProperty(&self, name: &[u16], value: Option<&nsISupports>) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).setProperty)(self as *const _, &*name, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getProperty (in AString name); */
    #[inline]
    pub unsafe fn getProperty(&self, name: &[u16]) -> Result<bool, nsresult> {
        let name = nsString::from(name);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getProperty)(self as *const _, &*name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void parseFromString (in AString str, in string contentType); */
    #[inline]
    pub unsafe fn parseFromString(&self, str: &[u16], contentType: *const libc::c_char) -> Result<(), nsresult> {
        let str = nsString::from(str);
        match ((*self.vtable).parseFromString)(self as *const _, &*str, contentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseFromStream (in nsIInputStream stream, in string charset, in string contentType); */
    #[inline]
    pub unsafe fn parseFromStream(&self, stream: Option<&nsIInputStream>, charset: *const libc::c_char, contentType: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).parseFromStream)(self as *const _, stream.map_or(::std::ptr::null(), |x| x as *const _), charset, contentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseAsync (in nsIRequestObserver observer); */
    #[inline]
    pub unsafe fn parseAsync(&self, observer: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).parseAsync)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


