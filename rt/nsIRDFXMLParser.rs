//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRDFXMLParser.idl
//


#[repr(C)]
pub struct nsIRDFXMLParser {
    vtable: *const nsIRDFXMLParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRDFXMLParser {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1831dd2e, 0x1dd2, 0x11b2,
            [0xbd, 0xb3, 0x86, 0xb7, 0xb5, 0x0b, 0x70, 0xb5])
    }
}

unsafe impl RefCounted for nsIRDFXMLParser {
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
pub trait nsIRDFXMLParserCoerce {
    fn coerce_from(v: &nsIRDFXMLParser) -> &Self;
}

impl nsIRDFXMLParserCoerce for nsIRDFXMLParser {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLParser) -> &Self {
        v
    }
}

impl nsIRDFXMLParser {
    #[inline]
    pub fn coerce<T: nsIRDFXMLParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRDFXMLParser {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRDFXMLParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRDFXMLParser) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRDFXMLParserVTable {
    pub __base: nsISupportsVTable,

    /* nsIStreamListener parseAsync (in nsIRDFDataSource aSink, in nsIURI aBaseURI); */
    pub parseAsync: unsafe extern "C" fn (this: *const nsIRDFXMLParser, aSink: *const nsIRDFDataSource, aBaseURI: *const nsIURI, _retval: *mut *const nsIStreamListener) -> nsresult,

    /* void parseString (in nsIRDFDataSource aSink, in nsIURI aBaseURI, in AUTF8String aSource); */
    pub parseString: unsafe extern "C" fn (this: *const nsIRDFXMLParser, aSink: *const nsIRDFDataSource, aBaseURI: *const nsIURI, aSource: *const nsACString) -> nsresult,

}


impl nsIRDFXMLParser {
    /* nsIStreamListener parseAsync (in nsIRDFDataSource aSink, in nsIURI aBaseURI); */
    #[inline]
    pub unsafe fn parseAsync(&self, aSink: Option<&nsIRDFDataSource>, aBaseURI: Option<&nsIURI>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseAsync)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _), aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void parseString (in nsIRDFDataSource aSink, in nsIURI aBaseURI, in AUTF8String aSource); */
    #[inline]
    pub unsafe fn parseString(&self, aSink: Option<&nsIRDFDataSource>, aBaseURI: Option<&nsIURI>, aSource: &[u8]) -> Result<(), nsresult> {
        let aSource = nsCString::from(aSource);
        match ((*self.vtable).parseString)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _), aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), &*aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


