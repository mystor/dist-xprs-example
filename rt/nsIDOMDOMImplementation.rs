//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDOMImplementation.idl
//


#[repr(C)]
pub struct nsIDOMDOMImplementation {
    vtable: *const nsIDOMDOMImplementationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDOMImplementation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x03a6f574, 0x99ec, 0x42f8,
            [0x9e, 0x6c, 0x81, 0x2a, 0x4a, 0x9b, 0xcb, 0xf7])
    }
}

unsafe impl RefCounted for nsIDOMDOMImplementation {
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
pub trait nsIDOMDOMImplementationCoerce {
    fn coerce_from(v: &nsIDOMDOMImplementation) -> &Self;
}

impl nsIDOMDOMImplementationCoerce for nsIDOMDOMImplementation {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMImplementation) -> &Self {
        v
    }
}

impl nsIDOMDOMImplementation {
    #[inline]
    pub fn coerce<T: nsIDOMDOMImplementationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDOMImplementation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDOMImplementationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDOMImplementation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDOMImplementationVTable {
    pub __base: nsISupportsVTable,

    /* boolean hasFeature (in DOMString feature, in DOMString version); */
    pub hasFeature: unsafe extern "C" fn (this: *const nsIDOMDOMImplementation, feature: *const nsAString, version: *const nsAString, _retval: *mut bool) -> nsresult,

    /* nsIDOMDocumentType createDocumentType (in DOMString qualifiedName, in DOMString publicId, in DOMString systemId) raises (DOMException); */
    pub createDocumentType: unsafe extern "C" fn (this: *const nsIDOMDOMImplementation, qualifiedName: *const nsAString, publicId: *const nsAString, systemId: *const nsAString, _retval: *mut *const nsIDOMDocumentType) -> nsresult,

    /* nsIDOMDocument createDocument (in DOMString namespaceURI, in DOMString qualifiedName, in nsIDOMDocumentType doctype) raises (DOMException); */
    pub createDocument: unsafe extern "C" fn (this: *const nsIDOMDOMImplementation, namespaceURI: *const nsAString, qualifiedName: *const nsAString, doctype: *const nsIDOMDocumentType, _retval: *mut *const nsIDOMDocument) -> nsresult,

    /* nsIDOMDocument createHTMLDocument ([Null (Stringify)] in DOMString title); */
    pub createHTMLDocument: unsafe extern "C" fn (this: *const nsIDOMDOMImplementation, title: *const nsAString, _retval: *mut *const nsIDOMDocument) -> nsresult,

}


impl nsIDOMDOMImplementation {
    /* boolean hasFeature (in DOMString feature, in DOMString version); */
    #[inline]
    pub unsafe fn hasFeature(&self, feature: &[u16], version: &[u16]) -> Result<bool, nsresult> {
        let feature = nsString::from(feature);
        let version = nsString::from(version);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasFeature)(self as *const _, &*feature, &*version, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMDocumentType createDocumentType (in DOMString qualifiedName, in DOMString publicId, in DOMString systemId) raises (DOMException); */
    #[inline]
    pub unsafe fn createDocumentType(&self, qualifiedName: &[u16], publicId: &[u16], systemId: &[u16]) -> Result<Option<RefPtr<nsIDOMDocumentType>>, nsresult> {
        let qualifiedName = nsString::from(qualifiedName);
        let publicId = nsString::from(publicId);
        let systemId = nsString::from(systemId);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createDocumentType)(self as *const _, &*qualifiedName, &*publicId, &*systemId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocument createDocument (in DOMString namespaceURI, in DOMString qualifiedName, in nsIDOMDocumentType doctype) raises (DOMException); */
    #[inline]
    pub unsafe fn createDocument(&self, namespaceURI: &[u16], qualifiedName: &[u16], doctype: Option<&nsIDOMDocumentType>) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let qualifiedName = nsString::from(qualifiedName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createDocument)(self as *const _, &*namespaceURI, &*qualifiedName, doctype.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocument createHTMLDocument ([Null (Stringify)] in DOMString title); */
    #[inline]
    pub unsafe fn createHTMLDocument(&self, title: &[u16]) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let title = nsString::from(title);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createHTMLDocument)(self as *const _, &*title, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


