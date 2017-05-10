//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXSLTProcessor.idl
//


#[repr(C)]
pub struct nsIXSLTProcessor {
    vtable: *const nsIXSLTProcessorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXSLTProcessor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4a91aeb3, 0x4100, 0x43ee,
            [0xa2, 0x1e, 0x98, 0x66, 0x26, 0x87, 0x57, 0xc5])
    }
}

unsafe impl RefCounted for nsIXSLTProcessor {
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
pub trait nsIXSLTProcessorCoerce {
    fn coerce_from(v: &nsIXSLTProcessor) -> &Self;
}

impl nsIXSLTProcessorCoerce for nsIXSLTProcessor {
    #[inline]
    fn coerce_from(v: &nsIXSLTProcessor) -> &Self {
        v
    }
}

impl nsIXSLTProcessor {
    #[inline]
    pub fn coerce<T: nsIXSLTProcessorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXSLTProcessor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXSLTProcessorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXSLTProcessor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXSLTProcessorVTable {
    pub __base: nsISupportsVTable,

    /* void importStylesheet (in nsIDOMNode style); */
    pub importStylesheet: unsafe extern "C" fn (this: *const nsIXSLTProcessor, style: *const nsIDOMNode) -> nsresult,

    /* nsIDOMDocumentFragment transformToFragment (in nsIDOMNode source, in nsIDOMDocument output); */
    pub transformToFragment: unsafe extern "C" fn (this: *const nsIXSLTProcessor, source: *const nsIDOMNode, output: *const nsIDOMDocument, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* nsIDOMDocument transformToDocument (in nsIDOMNode source); */
    pub transformToDocument: unsafe extern "C" fn (this: *const nsIXSLTProcessor, source: *const nsIDOMNode, _retval: *mut *const nsIDOMDocument) -> nsresult,

    /* void setParameter (in DOMString namespaceURI, in DOMString localName, in nsIVariant value); */
    pub setParameter: unsafe extern "C" fn (this: *const nsIXSLTProcessor, namespaceURI: *const nsAString, localName: *const nsAString, value: *const nsIVariant) -> nsresult,

    /* nsIVariant getParameter (in DOMString namespaceURI, in DOMString localName); */
    pub getParameter: unsafe extern "C" fn (this: *const nsIXSLTProcessor, namespaceURI: *const nsAString, localName: *const nsAString, _retval: *mut *const nsIVariant) -> nsresult,

    /* void removeParameter (in DOMString namespaceURI, in DOMString localName); */
    pub removeParameter: unsafe extern "C" fn (this: *const nsIXSLTProcessor, namespaceURI: *const nsAString, localName: *const nsAString) -> nsresult,

    /* void clearParameters (); */
    pub clearParameters: unsafe extern "C" fn (this: *const nsIXSLTProcessor) -> nsresult,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsIXSLTProcessor) -> nsresult,

}


impl nsIXSLTProcessor {
    /* void importStylesheet (in nsIDOMNode style); */
    #[inline]
    pub unsafe fn importStylesheet(&self, style: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).importStylesheet)(self as *const _, style.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMDocumentFragment transformToFragment (in nsIDOMNode source, in nsIDOMDocument output); */
    #[inline]
    pub unsafe fn transformToFragment(&self, source: Option<&nsIDOMNode>, output: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).transformToFragment)(self as *const _, source.map_or(::std::ptr::null(), |x| x as *const _), output.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocument transformToDocument (in nsIDOMNode source); */
    #[inline]
    pub unsafe fn transformToDocument(&self, source: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).transformToDocument)(self as *const _, source.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setParameter (in DOMString namespaceURI, in DOMString localName, in nsIVariant value); */
    #[inline]
    pub unsafe fn setParameter(&self, namespaceURI: &[u16], localName: &[u16], value: Option<&nsIVariant>) -> Result<(), nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        match ((*self.vtable).setParameter)(self as *const _, &*namespaceURI, &*localName, value.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIVariant getParameter (in DOMString namespaceURI, in DOMString localName); */
    #[inline]
    pub unsafe fn getParameter(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getParameter)(self as *const _, &*namespaceURI, &*localName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void removeParameter (in DOMString namespaceURI, in DOMString localName); */
    #[inline]
    pub unsafe fn removeParameter(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<(), nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        match ((*self.vtable).removeParameter)(self as *const _, &*namespaceURI, &*localName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearParameters (); */
    #[inline]
    pub unsafe fn clearParameters(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearParameters)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


