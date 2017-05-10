//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocumentXBL.idl
//


#[repr(C)]
pub struct nsIDOMDocumentXBL {
    vtable: *const nsIDOMDocumentXBLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDocumentXBL {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xaf628000, 0xe3fa, 0x40d2,
            [0x91, 0x18, 0xfb, 0xaa, 0x9f, 0x3e, 0xc6, 0xb9])
    }
}

unsafe impl RefCounted for nsIDOMDocumentXBL {
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
pub trait nsIDOMDocumentXBLCoerce {
    fn coerce_from(v: &nsIDOMDocumentXBL) -> &Self;
}

impl nsIDOMDocumentXBLCoerce for nsIDOMDocumentXBL {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentXBL) -> &Self {
        v
    }
}

impl nsIDOMDocumentXBL {
    #[inline]
    pub fn coerce<T: nsIDOMDocumentXBLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDocumentXBL {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMDocumentXBLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDocumentXBL) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDocumentXBLVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMNodeList getAnonymousNodes (in nsIDOMElement elt); */
    pub getAnonymousNodes: unsafe extern "C" fn (this: *const nsIDOMDocumentXBL, elt: *const nsIDOMElement, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* nsIDOMElement getAnonymousElementByAttribute (in nsIDOMElement elt, in DOMString attrName, in DOMString attrValue); */
    pub getAnonymousElementByAttribute: unsafe extern "C" fn (this: *const nsIDOMDocumentXBL, elt: *const nsIDOMElement, attrName: *const nsAString, attrValue: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMElement getBindingParent (in nsIDOMNode node); */
    pub getBindingParent: unsafe extern "C" fn (this: *const nsIDOMDocumentXBL, node: *const nsIDOMNode, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void loadBindingDocument (in DOMString documentURL); */
    pub loadBindingDocument: unsafe extern "C" fn (this: *const nsIDOMDocumentXBL, documentURL: *const nsAString) -> nsresult,

}


impl nsIDOMDocumentXBL {
    /* nsIDOMNodeList getAnonymousNodes (in nsIDOMElement elt); */
    #[inline]
    pub unsafe fn getAnonymousNodes(&self, elt: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAnonymousNodes)(self as *const _, elt.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement getAnonymousElementByAttribute (in nsIDOMElement elt, in DOMString attrName, in DOMString attrValue); */
    #[inline]
    pub unsafe fn getAnonymousElementByAttribute(&self, elt: Option<&nsIDOMElement>, attrName: &[u16], attrValue: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let attrName = nsString::from(attrName);
        let attrValue = nsString::from(attrValue);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAnonymousElementByAttribute)(self as *const _, elt.map_or(::std::ptr::null(), |x| x as *const _), &*attrName, &*attrValue, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement getBindingParent (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn getBindingParent(&self, node: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBindingParent)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void loadBindingDocument (in DOMString documentURL); */
    #[inline]
    pub unsafe fn loadBindingDocument(&self, documentURL: &[u16]) -> Result<(), nsresult> {
        let documentURL = nsString::from(documentURL);
        match ((*self.vtable).loadBindingDocument)(self as *const _, &*documentURL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


