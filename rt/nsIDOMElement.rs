//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMElement.idl
//


#[repr(C)]
pub struct nsIDOMElement {
    vtable: *const nsIDOMElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6289999b, 0x1008, 0x4269,
            [0xb4, 0x2a, 0x41, 0x3e, 0xc5, 0xa9, 0xd3, 0xf4])
    }
}

unsafe impl RefCounted for nsIDOMElement {
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
pub trait nsIDOMElementCoerce {
    fn coerce_from(v: &nsIDOMElement) -> &Self;
}

impl nsIDOMElementCoerce for nsIDOMElement {
    #[inline]
    fn coerce_from(v: &nsIDOMElement) -> &Self {
        v
    }
}

impl nsIDOMElement {
    #[inline]
    pub fn coerce<T: nsIDOMElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMElement {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMElementVTable {
    pub __base: nsIDOMNodeVTable,

    /* readonly attribute DOMString tagName; */
    pub get_tagName: unsafe extern "C" fn (this: *const nsIDOMElement, aTagName: *mut nsAString) -> nsresult,

    /* readonly attribute nsIDOMMozNamedAttrMap attributes; */
    pub get_attributes: unsafe extern "C" fn (this: *const nsIDOMElement, aAttributes: *mut *const nsIDOMMozNamedAttrMap) -> nsresult,

    /* DOMString getAttribute (in DOMString name); */
    pub getAttribute: unsafe extern "C" fn (this: *const nsIDOMElement, name: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* void setAttribute (in DOMString name, in DOMString value); */
    pub setAttribute: unsafe extern "C" fn (this: *const nsIDOMElement, name: *const nsAString, value: *const nsAString) -> nsresult,

    /* boolean hasAttribute (in DOMString name); */
    pub hasAttribute: unsafe extern "C" fn (this: *const nsIDOMElement, name: *const nsAString, _retval: *mut bool) -> nsresult,

    /* nsIDOMAttr getAttributeNode (in DOMString name); */
    pub getAttributeNode: unsafe extern "C" fn (this: *const nsIDOMElement, name: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMAttr getAttributeNodeNS (in DOMString namespaceURI, in DOMString localName); */
    pub getAttributeNodeNS: unsafe extern "C" fn (this: *const nsIDOMElement, namespaceURI: *const nsAString, localName: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

}


impl nsIDOMElement {
    /* readonly attribute DOMString tagName; */
    #[inline]
    pub unsafe fn get_tagName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_tagName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMMozNamedAttrMap attributes; */
    #[inline]
    pub unsafe fn get_attributes(&self, ) -> Result<Option<RefPtr<nsIDOMMozNamedAttrMap>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_attributes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* DOMString getAttribute (in DOMString name); */
    #[inline]
    pub unsafe fn getAttribute(&self, name: &[u16]) -> Result<nsString, nsresult> {
        let name = nsString::from(name);
        let mut _retval = nsString::new();
        match ((*self.vtable).getAttribute)(self as *const _, &*name, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setAttribute (in DOMString name, in DOMString value); */
    #[inline]
    pub unsafe fn setAttribute(&self, name: &[u16], value: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let value = nsString::from(value);
        match ((*self.vtable).setAttribute)(self as *const _, &*name, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasAttribute (in DOMString name); */
    #[inline]
    pub unsafe fn hasAttribute(&self, name: &[u16]) -> Result<bool, nsresult> {
        let name = nsString::from(name);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasAttribute)(self as *const _, &*name, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMAttr getAttributeNode (in DOMString name); */
    #[inline]
    pub unsafe fn getAttributeNode(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAttributeNode)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr getAttributeNodeNS (in DOMString namespaceURI, in DOMString localName); */
    #[inline]
    pub unsafe fn getAttributeNodeNS(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAttributeNodeNS)(self as *const _, &*namespaceURI, &*localName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


