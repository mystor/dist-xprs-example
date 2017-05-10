//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISAXContentHandler.idl
//


#[repr(C)]
pub struct nsISAXContentHandler {
    vtable: *const nsISAXContentHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISAXContentHandler {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2a99c757, 0xdfee, 0x4806,
            [0xbf, 0xf3, 0xf7, 0x21, 0x44, 0x04, 0x12, 0xe0])
    }
}

unsafe impl RefCounted for nsISAXContentHandler {
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
pub trait nsISAXContentHandlerCoerce {
    fn coerce_from(v: &nsISAXContentHandler) -> &Self;
}

impl nsISAXContentHandlerCoerce for nsISAXContentHandler {
    #[inline]
    fn coerce_from(v: &nsISAXContentHandler) -> &Self {
        v
    }
}

impl nsISAXContentHandler {
    #[inline]
    pub fn coerce<T: nsISAXContentHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISAXContentHandler {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISAXContentHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISAXContentHandler) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISAXContentHandlerVTable {
    pub __base: nsISupportsVTable,

    /* void startDocument (); */
    pub startDocument: unsafe extern "C" fn (this: *const nsISAXContentHandler) -> nsresult,

    /* void endDocument (); */
    pub endDocument: unsafe extern "C" fn (this: *const nsISAXContentHandler) -> nsresult,

    /* void startElement (in AString uri, in AString localName, in AString qName, in nsISAXAttributes attributes); */
    pub startElement: unsafe extern "C" fn (this: *const nsISAXContentHandler, uri: *const nsAString, localName: *const nsAString, qName: *const nsAString, attributes: *const nsISAXAttributes) -> nsresult,

    /* void endElement (in AString uri, in AString localName, in AString qName); */
    pub endElement: unsafe extern "C" fn (this: *const nsISAXContentHandler, uri: *const nsAString, localName: *const nsAString, qName: *const nsAString) -> nsresult,

    /* void characters (in AString value); */
    pub characters: unsafe extern "C" fn (this: *const nsISAXContentHandler, value: *const nsAString) -> nsresult,

    /* void processingInstruction (in AString target, in AString data); */
    pub processingInstruction: unsafe extern "C" fn (this: *const nsISAXContentHandler, target: *const nsAString, data: *const nsAString) -> nsresult,

    /* void ignorableWhitespace (in AString whitespace); */
    pub ignorableWhitespace: unsafe extern "C" fn (this: *const nsISAXContentHandler, whitespace: *const nsAString) -> nsresult,

    /* void startPrefixMapping (in AString prefix, in AString uri); */
    pub startPrefixMapping: unsafe extern "C" fn (this: *const nsISAXContentHandler, prefix: *const nsAString, uri: *const nsAString) -> nsresult,

    /* void endPrefixMapping (in AString prefix); */
    pub endPrefixMapping: unsafe extern "C" fn (this: *const nsISAXContentHandler, prefix: *const nsAString) -> nsresult,

}


impl nsISAXContentHandler {
    /* void startDocument (); */
    #[inline]
    pub unsafe fn startDocument(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startDocument)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endDocument (); */
    #[inline]
    pub unsafe fn endDocument(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endDocument)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startElement (in AString uri, in AString localName, in AString qName, in nsISAXAttributes attributes); */
    #[inline]
    pub unsafe fn startElement(&self, uri: &[u16], localName: &[u16], qName: &[u16], attributes: Option<&nsISAXAttributes>) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let qName = nsString::from(qName);
        match ((*self.vtable).startElement)(self as *const _, &*uri, &*localName, &*qName, attributes.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endElement (in AString uri, in AString localName, in AString qName); */
    #[inline]
    pub unsafe fn endElement(&self, uri: &[u16], localName: &[u16], qName: &[u16]) -> Result<(), nsresult> {
        let uri = nsString::from(uri);
        let localName = nsString::from(localName);
        let qName = nsString::from(qName);
        match ((*self.vtable).endElement)(self as *const _, &*uri, &*localName, &*qName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void characters (in AString value); */
    #[inline]
    pub unsafe fn characters(&self, value: &[u16]) -> Result<(), nsresult> {
        let value = nsString::from(value);
        match ((*self.vtable).characters)(self as *const _, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void processingInstruction (in AString target, in AString data); */
    #[inline]
    pub unsafe fn processingInstruction(&self, target: &[u16], data: &[u16]) -> Result<(), nsresult> {
        let target = nsString::from(target);
        let data = nsString::from(data);
        match ((*self.vtable).processingInstruction)(self as *const _, &*target, &*data) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ignorableWhitespace (in AString whitespace); */
    #[inline]
    pub unsafe fn ignorableWhitespace(&self, whitespace: &[u16]) -> Result<(), nsresult> {
        let whitespace = nsString::from(whitespace);
        match ((*self.vtable).ignorableWhitespace)(self as *const _, &*whitespace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void startPrefixMapping (in AString prefix, in AString uri); */
    #[inline]
    pub unsafe fn startPrefixMapping(&self, prefix: &[u16], uri: &[u16]) -> Result<(), nsresult> {
        let prefix = nsString::from(prefix);
        let uri = nsString::from(uri);
        match ((*self.vtable).startPrefixMapping)(self as *const _, &*prefix, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endPrefixMapping (in AString prefix); */
    #[inline]
    pub unsafe fn endPrefixMapping(&self, prefix: &[u16]) -> Result<(), nsresult> {
        let prefix = nsString::from(prefix);
        match ((*self.vtable).endPrefixMapping)(self as *const _, &*prefix) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


