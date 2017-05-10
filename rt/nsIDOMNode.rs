//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMNode.idl
//


pub mod nsIDOMNode_consts {
    pub const ELEMENT_NODE: i64 = 1;
    pub const ATTRIBUTE_NODE: i64 = 2;
    pub const TEXT_NODE: i64 = 3;
    pub const CDATA_SECTION_NODE: i64 = 4;
    pub const ENTITY_REFERENCE_NODE: i64 = 5;
    pub const ENTITY_NODE: i64 = 6;
    pub const PROCESSING_INSTRUCTION_NODE: i64 = 7;
    pub const COMMENT_NODE: i64 = 8;
    pub const DOCUMENT_NODE: i64 = 9;
    pub const DOCUMENT_TYPE_NODE: i64 = 10;
    pub const DOCUMENT_FRAGMENT_NODE: i64 = 11;
    pub const NOTATION_NODE: i64 = 12;
    pub const DOCUMENT_POSITION_DISCONNECTED: i64 = 1;
    pub const DOCUMENT_POSITION_PRECEDING: i64 = 2;
    pub const DOCUMENT_POSITION_FOLLOWING: i64 = 4;
    pub const DOCUMENT_POSITION_CONTAINS: i64 = 8;
    pub const DOCUMENT_POSITION_CONTAINED_BY: i64 = 16;
    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: i64 = 32;
}


#[repr(C)]
pub struct nsIDOMNode {
    vtable: *const nsIDOMNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMNode {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcc35b412, 0x009b, 0x46a3,
            [0x9b, 0xe0, 0x76, 0x44, 0x8f, 0x12, 0x54, 0x8d])
    }
}

unsafe impl RefCounted for nsIDOMNode {
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
pub trait nsIDOMNodeCoerce {
    fn coerce_from(v: &nsIDOMNode) -> &Self;
}

impl nsIDOMNodeCoerce for nsIDOMNode {
    #[inline]
    fn coerce_from(v: &nsIDOMNode) -> &Self {
        v
    }
}

impl nsIDOMNode {
    #[inline]
    pub fn coerce<T: nsIDOMNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMNode {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMNode) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMNodeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute DOMString nodeName; */
    pub get_nodeName: unsafe extern "C" fn (this: *const nsIDOMNode, aNodeName: *mut nsAString) -> nsresult,

    /* attribute DOMString nodeValue; */
    pub get_nodeValue: unsafe extern "C" fn (this: *const nsIDOMNode, aNodeValue: *mut nsAString) -> nsresult,
    pub set_nodeValue: unsafe extern "C" fn (this: *const nsIDOMNode, aNodeValue: *const nsAString) -> nsresult,

    /* readonly attribute unsigned short nodeType; */
    pub get_nodeType: unsafe extern "C" fn (this: *const nsIDOMNode, aNodeType: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute nsIDOMNode parentNode; */
    pub get_parentNode: unsafe extern "C" fn (this: *const nsIDOMNode, aParentNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMNodeList childNodes; */
    pub get_childNodes: unsafe extern "C" fn (this: *const nsIDOMNode, aChildNodes: *mut *const nsIDOMNodeList) -> nsresult,

    /* readonly attribute nsIDOMNode firstChild; */
    pub get_firstChild: unsafe extern "C" fn (this: *const nsIDOMNode, aFirstChild: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMNode lastChild; */
    pub get_lastChild: unsafe extern "C" fn (this: *const nsIDOMNode, aLastChild: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMNode previousSibling; */
    pub get_previousSibling: unsafe extern "C" fn (this: *const nsIDOMNode, aPreviousSibling: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMNode nextSibling; */
    pub get_nextSibling: unsafe extern "C" fn (this: *const nsIDOMNode, aNextSibling: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMDocument ownerDocument; */
    pub get_ownerDocument: unsafe extern "C" fn (this: *const nsIDOMNode, aOwnerDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* nsIDOMNode removeChild (in nsIDOMNode oldChild) raises (DOMException); */
    pub removeChild: unsafe extern "C" fn (this: *const nsIDOMNode, oldChild: *const nsIDOMNode, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* boolean hasChildNodes (); */
    pub hasChildNodes: unsafe extern "C" fn (this: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

    /* [optional_argc] nsIDOMNode cloneNode ([optional] in boolean deep); */
    /// Unable to call function as its signature contains a non-rust type
    pub cloneNode: *const ::libc::c_void,

    /* readonly attribute DOMString namespaceURI; */
    pub get_namespaceURI: unsafe extern "C" fn (this: *const nsIDOMNode, aNamespaceURI: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString prefix; */
    pub get_prefix: unsafe extern "C" fn (this: *const nsIDOMNode, aPrefix: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString localName; */
    pub get_localName: unsafe extern "C" fn (this: *const nsIDOMNode, aLocalName: *mut nsAString) -> nsresult,

    /* attribute DOMString textContent; */
    pub get_textContent: unsafe extern "C" fn (this: *const nsIDOMNode, aTextContent: *mut nsAString) -> nsresult,
    pub set_textContent: unsafe extern "C" fn (this: *const nsIDOMNode, aTextContent: *const nsAString) -> nsresult,

}


impl nsIDOMNode {
    /* readonly attribute DOMString nodeName; */
    #[inline]
    pub unsafe fn get_nodeName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_nodeName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString nodeValue; */
    #[inline]
    pub unsafe fn get_nodeValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_nodeValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_nodeValue(&self, aNodeValue: &[u16]) -> Result<(), nsresult> {
        let aNodeValue = nsString::from(aNodeValue);
        match ((*self.vtable).set_nodeValue)(self as *const _, &*aNodeValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short nodeType; */
    #[inline]
    pub unsafe fn get_nodeType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_nodeType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMNode parentNode; */
    #[inline]
    pub unsafe fn get_parentNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parentNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNodeList childNodes; */
    #[inline]
    pub unsafe fn get_childNodes(&self, ) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_childNodes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode firstChild; */
    #[inline]
    pub unsafe fn get_firstChild(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_firstChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode lastChild; */
    #[inline]
    pub unsafe fn get_lastChild(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_lastChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode previousSibling; */
    #[inline]
    pub unsafe fn get_previousSibling(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_previousSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode nextSibling; */
    #[inline]
    pub unsafe fn get_nextSibling(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_nextSibling)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMDocument ownerDocument; */
    #[inline]
    pub unsafe fn get_ownerDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNode removeChild (in nsIDOMNode oldChild) raises (DOMException); */
    #[inline]
    pub unsafe fn removeChild(&self, oldChild: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).removeChild)(self as *const _, oldChild.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean hasChildNodes (); */
    #[inline]
    pub unsafe fn hasChildNodes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasChildNodes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [optional_argc] nsIDOMNode cloneNode ([optional] in boolean deep); */


    /* readonly attribute DOMString namespaceURI; */
    #[inline]
    pub unsafe fn get_namespaceURI(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_namespaceURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString prefix; */
    #[inline]
    pub unsafe fn get_prefix(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_prefix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString localName; */
    #[inline]
    pub unsafe fn get_localName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_localName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString textContent; */
    #[inline]
    pub unsafe fn get_textContent(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_textContent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_textContent(&self, aTextContent: &[u16]) -> Result<(), nsresult> {
        let aTextContent = nsString::from(aTextContent);
        match ((*self.vtable).set_textContent)(self as *const _, &*aTextContent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


