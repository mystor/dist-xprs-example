//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMDocument.idl
//


#[repr(C)]
pub struct nsIDOMDocument {
    vtable: *const nsIDOMDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb15fa0f4, 0x97c1, 0x4388,
            [0xaf, 0x62, 0x2c, 0xef, 0xf7, 0xa8, 0x9b, 0xdf])
    }
}

unsafe impl RefCounted for nsIDOMDocument {
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
pub trait nsIDOMDocumentCoerce {
    fn coerce_from(v: &nsIDOMDocument) -> &Self;
}

impl nsIDOMDocumentCoerce for nsIDOMDocument {
    #[inline]
    fn coerce_from(v: &nsIDOMDocument) -> &Self {
        v
    }
}

impl nsIDOMDocument {
    #[inline]
    pub fn coerce<T: nsIDOMDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMDocument {
    type Target = nsIDOMNode;
    #[inline]
    fn deref(&self) -> &nsIDOMNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMNodeCoerce> nsIDOMDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMDocumentVTable {
    pub __base: nsIDOMNodeVTable,

    /* readonly attribute nsIDOMDocumentType doctype; */
    pub get_doctype: unsafe extern "C" fn (this: *const nsIDOMDocument, aDoctype: *mut *const nsIDOMDocumentType) -> nsresult,

    /* readonly attribute nsIDOMDOMImplementation implementation; */
    pub get_implementation: unsafe extern "C" fn (this: *const nsIDOMDocument, aImplementation: *mut *const nsIDOMDOMImplementation) -> nsresult,

    /* readonly attribute nsIDOMElement documentElement; */
    pub get_documentElement: unsafe extern "C" fn (this: *const nsIDOMDocument, aDocumentElement: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMElement createElement ([Null (Stringify)] in DOMString tagName) raises (DOMException); */
    pub createElement: unsafe extern "C" fn (this: *const nsIDOMDocument, tagName: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMDocumentFragment createDocumentFragment (); */
    pub createDocumentFragment: unsafe extern "C" fn (this: *const nsIDOMDocument, _retval: *mut *const nsIDOMDocumentFragment) -> nsresult,

    /* nsIDOMText createTextNode (in DOMString data); */
    pub createTextNode: unsafe extern "C" fn (this: *const nsIDOMDocument, data: *const nsAString, _retval: *mut *const nsIDOMText) -> nsresult,

    /* nsIDOMComment createComment (in DOMString data); */
    pub createComment: unsafe extern "C" fn (this: *const nsIDOMDocument, data: *const nsAString, _retval: *mut *const nsIDOMComment) -> nsresult,

    /* nsIDOMCDATASection createCDATASection (in DOMString data) raises (DOMException); */
    pub createCDATASection: unsafe extern "C" fn (this: *const nsIDOMDocument, data: *const nsAString, _retval: *mut *const nsIDOMCDATASection) -> nsresult,

    /* nsIDOMProcessingInstruction createProcessingInstruction (in DOMString target, in DOMString data) raises (DOMException); */
    pub createProcessingInstruction: unsafe extern "C" fn (this: *const nsIDOMDocument, target: *const nsAString, data: *const nsAString, _retval: *mut *const nsIDOMProcessingInstruction) -> nsresult,

    /* nsIDOMAttr createAttribute (in DOMString name) raises (DOMException); */
    pub createAttribute: unsafe extern "C" fn (this: *const nsIDOMDocument, name: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMNodeList getElementsByTagName (in DOMString tagname); */
    pub getElementsByTagName: unsafe extern "C" fn (this: *const nsIDOMDocument, tagname: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* [optional_argc] nsIDOMNode importNode (in nsIDOMNode importedNode, [optional] in boolean deep) raises (DOMException); */
    /// Unable to call function as its signature contains a non-rust type
    pub importNode: *const ::libc::c_void,

    /* nsIDOMElement createElementNS (in DOMString namespaceURI, [Null (Stringify)] in DOMString qualifiedName) raises (DOMException); */
    pub createElementNS: unsafe extern "C" fn (this: *const nsIDOMDocument, namespaceURI: *const nsAString, qualifiedName: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMAttr createAttributeNS (in DOMString namespaceURI, in DOMString qualifiedName) raises (DOMException); */
    pub createAttributeNS: unsafe extern "C" fn (this: *const nsIDOMDocument, namespaceURI: *const nsAString, qualifiedName: *const nsAString, _retval: *mut *const nsIDOMAttr) -> nsresult,

    /* nsIDOMNodeList getElementsByTagNameNS (in DOMString namespaceURI, in DOMString localName); */
    pub getElementsByTagNameNS: unsafe extern "C" fn (this: *const nsIDOMDocument, namespaceURI: *const nsAString, localName: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* nsIDOMElement getElementById (in DOMString elementId); */
    pub getElementById: unsafe extern "C" fn (this: *const nsIDOMDocument, elementId: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute DOMString inputEncoding; */
    pub get_inputEncoding: unsafe extern "C" fn (this: *const nsIDOMDocument, aInputEncoding: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString documentURI; */
    pub get_documentURI: unsafe extern "C" fn (this: *const nsIDOMDocument, aDocumentURI: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString URL; */
    pub get_URL: unsafe extern "C" fn (this: *const nsIDOMDocument, aURL: *mut nsAString) -> nsresult,

    /* nsIDOMNode adoptNode (in nsIDOMNode source) raises (DOMException); */
    pub adoptNode: unsafe extern "C" fn (this: *const nsIDOMDocument, source: *const nsIDOMNode, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMRange createRange (); */
    pub createRange: unsafe extern "C" fn (this: *const nsIDOMDocument, _retval: *mut *const nsIDOMRange) -> nsresult,

    /* [optional_argc] nsIDOMNodeIterator createNodeIterator (in nsIDOMNode root, [optional] in unsigned long whatToShow, [optional] in nsIDOMNodeFilter filter) raises (DOMException); */
    /// Unable to call function as its signature contains a non-rust type
    pub createNodeIterator: *const ::libc::c_void,

    /* [optional_argc] nsIDOMTreeWalker createTreeWalker (in nsIDOMNode root, [optional] in unsigned long whatToShow, [optional] in nsIDOMNodeFilter filter) raises (DOMException); */
    /// Unable to call function as its signature contains a non-rust type
    pub createTreeWalker: *const ::libc::c_void,

    /* nsIDOMEvent createEvent (in DOMString eventType) raises (DOMException); */
    pub createEvent: unsafe extern "C" fn (this: *const nsIDOMDocument, eventType: *const nsAString, _retval: *mut *const nsIDOMEvent) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy defaultView; */
    pub get_defaultView: unsafe extern "C" fn (this: *const nsIDOMDocument, aDefaultView: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute DOMString characterSet; */
    pub get_characterSet: unsafe extern "C" fn (this: *const nsIDOMDocument, aCharacterSet: *mut nsAString) -> nsresult,

    /* attribute DOMString dir; */
    pub get_dir: unsafe extern "C" fn (this: *const nsIDOMDocument, aDir: *mut nsAString) -> nsresult,
    pub set_dir: unsafe extern "C" fn (this: *const nsIDOMDocument, aDir: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMLocation location; */
    pub get_location: unsafe extern "C" fn (this: *const nsIDOMDocument, aLocation: *mut *const nsIDOMLocation) -> nsresult,

    /* attribute DOMString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIDOMDocument, aTitle: *mut nsAString) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIDOMDocument, aTitle: *const nsAString) -> nsresult,

    /* readonly attribute DOMString readyState; */
    pub get_readyState: unsafe extern "C" fn (this: *const nsIDOMDocument, aReadyState: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString lastModified; */
    pub get_lastModified: unsafe extern "C" fn (this: *const nsIDOMDocument, aLastModified: *mut nsAString) -> nsresult,

    /* readonly attribute DOMString referrer; */
    pub get_referrer: unsafe extern "C" fn (this: *const nsIDOMDocument, aReferrer: *mut nsAString) -> nsresult,

    /* boolean hasFocus (); */
    pub hasFocus: unsafe extern "C" fn (this: *const nsIDOMDocument, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMElement activeElement; */
    pub get_activeElement: unsafe extern "C" fn (this: *const nsIDOMDocument, aActiveElement: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMNodeList getElementsByClassName (in DOMString classes); */
    pub getElementsByClassName: unsafe extern "C" fn (this: *const nsIDOMDocument, classes: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* readonly attribute nsIDOMStyleSheetList styleSheets; */
    pub get_styleSheets: unsafe extern "C" fn (this: *const nsIDOMDocument, aStyleSheets: *mut *const nsIDOMStyleSheetList) -> nsresult,

    /* readonly attribute DOMString preferredStyleSheetSet; */
    pub get_preferredStyleSheetSet: unsafe extern "C" fn (this: *const nsIDOMDocument, aPreferredStyleSheetSet: *mut nsAString) -> nsresult,

    /* [binaryname(MozSelectedStyleSheetSet)] attribute DOMString selectedStyleSheetSet; */
    pub get_MozSelectedStyleSheetSet: unsafe extern "C" fn (this: *const nsIDOMDocument, aSelectedStyleSheetSet: *mut nsAString) -> nsresult,
    pub set_MozSelectedStyleSheetSet: unsafe extern "C" fn (this: *const nsIDOMDocument, aSelectedStyleSheetSet: *const nsAString) -> nsresult,

    /* readonly attribute DOMString lastStyleSheetSet; */
    pub get_lastStyleSheetSet: unsafe extern "C" fn (this: *const nsIDOMDocument, aLastStyleSheetSet: *mut nsAString) -> nsresult,

    /* readonly attribute nsISupports styleSheetSets; */
    pub get_styleSheetSets: unsafe extern "C" fn (this: *const nsIDOMDocument, aStyleSheetSets: *mut *const nsISupports) -> nsresult,

    /* [binaryname(MozEnableStyleSheetsForSet)] void enableStyleSheetsForSet (in DOMString name); */
    pub MozEnableStyleSheetsForSet: unsafe extern "C" fn (this: *const nsIDOMDocument, name: *const nsAString) -> nsresult,

    /* nsIDOMElement elementFromPoint (in float x, in float y); */
    pub elementFromPoint: unsafe extern "C" fn (this: *const nsIDOMDocument, x: libc::c_float, y: libc::c_float, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute DOMString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsIDOMDocument, aContentType: *mut nsAString) -> nsresult,

    /* readonly attribute boolean mozSyntheticDocument; */
    pub get_mozSyntheticDocument: unsafe extern "C" fn (this: *const nsIDOMDocument, aMozSyntheticDocument: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMElement currentScript; */
    pub get_currentScript: unsafe extern "C" fn (this: *const nsIDOMDocument, aCurrentScript: *mut *const nsIDOMElement) -> nsresult,

    /* void mozSetImageElement (in DOMString aImageElementId, in nsIDOMElement aImageElement); */
    pub mozSetImageElement: unsafe extern "C" fn (this: *const nsIDOMDocument, aImageElementId: *const nsAString, aImageElement: *const nsIDOMElement) -> nsresult,

    /* nsISupports caretPositionFromPoint (in float x, in float y); */
    pub caretPositionFromPoint: unsafe extern "C" fn (this: *const nsIDOMDocument, x: libc::c_float, y: libc::c_float, _retval: *mut *const nsISupports) -> nsresult,

    /* readonly attribute boolean hidden; */
    pub get_hidden: unsafe extern "C" fn (this: *const nsIDOMDocument, aHidden: *mut bool) -> nsresult,

    /* readonly attribute DOMString visibilityState; */
    pub get_visibilityState: unsafe extern "C" fn (this: *const nsIDOMDocument, aVisibilityState: *mut nsAString) -> nsresult,

}


impl nsIDOMDocument {
    /* readonly attribute nsIDOMDocumentType doctype; */
    #[inline]
    pub unsafe fn get_doctype(&self, ) -> Result<Option<RefPtr<nsIDOMDocumentType>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_doctype)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMDOMImplementation implementation; */
    #[inline]
    pub unsafe fn get_implementation(&self, ) -> Result<Option<RefPtr<nsIDOMDOMImplementation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_implementation)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement documentElement; */
    #[inline]
    pub unsafe fn get_documentElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_documentElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement createElement ([Null (Stringify)] in DOMString tagName) raises (DOMException); */
    #[inline]
    pub unsafe fn createElement(&self, tagName: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let tagName = nsString::from(tagName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createElement)(self as *const _, &*tagName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMDocumentFragment createDocumentFragment (); */
    #[inline]
    pub unsafe fn createDocumentFragment(&self, ) -> Result<Option<RefPtr<nsIDOMDocumentFragment>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createDocumentFragment)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMText createTextNode (in DOMString data); */
    #[inline]
    pub unsafe fn createTextNode(&self, data: &[u16]) -> Result<Option<RefPtr<nsIDOMText>>, nsresult> {
        let data = nsString::from(data);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createTextNode)(self as *const _, &*data, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMComment createComment (in DOMString data); */
    #[inline]
    pub unsafe fn createComment(&self, data: &[u16]) -> Result<Option<RefPtr<nsIDOMComment>>, nsresult> {
        let data = nsString::from(data);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createComment)(self as *const _, &*data, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMCDATASection createCDATASection (in DOMString data) raises (DOMException); */
    #[inline]
    pub unsafe fn createCDATASection(&self, data: &[u16]) -> Result<Option<RefPtr<nsIDOMCDATASection>>, nsresult> {
        let data = nsString::from(data);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createCDATASection)(self as *const _, &*data, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMProcessingInstruction createProcessingInstruction (in DOMString target, in DOMString data) raises (DOMException); */
    #[inline]
    pub unsafe fn createProcessingInstruction(&self, target: &[u16], data: &[u16]) -> Result<Option<RefPtr<nsIDOMProcessingInstruction>>, nsresult> {
        let target = nsString::from(target);
        let data = nsString::from(data);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createProcessingInstruction)(self as *const _, &*target, &*data, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr createAttribute (in DOMString name) raises (DOMException); */
    #[inline]
    pub unsafe fn createAttribute(&self, name: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let name = nsString::from(name);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAttribute)(self as *const _, &*name, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getElementsByTagName (in DOMString tagname); */
    #[inline]
    pub unsafe fn getElementsByTagName(&self, tagname: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let tagname = nsString::from(tagname);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByTagName)(self as *const _, &*tagname, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [optional_argc] nsIDOMNode importNode (in nsIDOMNode importedNode, [optional] in boolean deep) raises (DOMException); */


    /* nsIDOMElement createElementNS (in DOMString namespaceURI, [Null (Stringify)] in DOMString qualifiedName) raises (DOMException); */
    #[inline]
    pub unsafe fn createElementNS(&self, namespaceURI: &[u16], qualifiedName: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let qualifiedName = nsString::from(qualifiedName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createElementNS)(self as *const _, &*namespaceURI, &*qualifiedName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMAttr createAttributeNS (in DOMString namespaceURI, in DOMString qualifiedName) raises (DOMException); */
    #[inline]
    pub unsafe fn createAttributeNS(&self, namespaceURI: &[u16], qualifiedName: &[u16]) -> Result<Option<RefPtr<nsIDOMAttr>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let qualifiedName = nsString::from(qualifiedName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAttributeNS)(self as *const _, &*namespaceURI, &*qualifiedName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getElementsByTagNameNS (in DOMString namespaceURI, in DOMString localName); */
    #[inline]
    pub unsafe fn getElementsByTagNameNS(&self, namespaceURI: &[u16], localName: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let localName = nsString::from(localName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByTagNameNS)(self as *const _, &*namespaceURI, &*localName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement getElementById (in DOMString elementId); */
    #[inline]
    pub unsafe fn getElementById(&self, elementId: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let elementId = nsString::from(elementId);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementById)(self as *const _, &*elementId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString inputEncoding; */
    #[inline]
    pub unsafe fn get_inputEncoding(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_inputEncoding)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString documentURI; */
    #[inline]
    pub unsafe fn get_documentURI(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_documentURI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString URL; */
    #[inline]
    pub unsafe fn get_URL(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_URL)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMNode adoptNode (in nsIDOMNode source) raises (DOMException); */
    #[inline]
    pub unsafe fn adoptNode(&self, source: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).adoptNode)(self as *const _, source.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMRange createRange (); */
    #[inline]
    pub unsafe fn createRange(&self, ) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createRange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [optional_argc] nsIDOMNodeIterator createNodeIterator (in nsIDOMNode root, [optional] in unsigned long whatToShow, [optional] in nsIDOMNodeFilter filter) raises (DOMException); */


    /* [optional_argc] nsIDOMTreeWalker createTreeWalker (in nsIDOMNode root, [optional] in unsigned long whatToShow, [optional] in nsIDOMNodeFilter filter) raises (DOMException); */


    /* nsIDOMEvent createEvent (in DOMString eventType) raises (DOMException); */
    #[inline]
    pub unsafe fn createEvent(&self, eventType: &[u16]) -> Result<Option<RefPtr<nsIDOMEvent>>, nsresult> {
        let eventType = nsString::from(eventType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createEvent)(self as *const _, &*eventType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy defaultView; */
    #[inline]
    pub unsafe fn get_defaultView(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultView)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString characterSet; */
    #[inline]
    pub unsafe fn get_characterSet(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_characterSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString dir; */
    #[inline]
    pub unsafe fn get_dir(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_dir)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_dir(&self, aDir: &[u16]) -> Result<(), nsresult> {
        let aDir = nsString::from(aDir);
        match ((*self.vtable).set_dir)(self as *const _, &*aDir) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMLocation location; */
    #[inline]
    pub unsafe fn get_location(&self, ) -> Result<Option<RefPtr<nsIDOMLocation>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_location)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute DOMString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_title(&self, aTitle: &[u16]) -> Result<(), nsresult> {
        let aTitle = nsString::from(aTitle);
        match ((*self.vtable).set_title)(self as *const _, &*aTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString readyState; */
    #[inline]
    pub unsafe fn get_readyState(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_readyState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString lastModified; */
    #[inline]
    pub unsafe fn get_lastModified(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lastModified)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString referrer; */
    #[inline]
    pub unsafe fn get_referrer(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_referrer)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasFocus (); */
    #[inline]
    pub unsafe fn hasFocus(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasFocus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMElement activeElement; */
    #[inline]
    pub unsafe fn get_activeElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getElementsByClassName (in DOMString classes); */
    #[inline]
    pub unsafe fn getElementsByClassName(&self, classes: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let classes = nsString::from(classes);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByClassName)(self as *const _, &*classes, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMStyleSheetList styleSheets; */
    #[inline]
    pub unsafe fn get_styleSheets(&self, ) -> Result<Option<RefPtr<nsIDOMStyleSheetList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_styleSheets)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString preferredStyleSheetSet; */
    #[inline]
    pub unsafe fn get_preferredStyleSheetSet(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_preferredStyleSheetSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [binaryname(MozSelectedStyleSheetSet)] attribute DOMString selectedStyleSheetSet; */
    #[inline]
    pub unsafe fn get_MozSelectedStyleSheetSet(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_MozSelectedStyleSheetSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_MozSelectedStyleSheetSet(&self, aSelectedStyleSheetSet: &[u16]) -> Result<(), nsresult> {
        let aSelectedStyleSheetSet = nsString::from(aSelectedStyleSheetSet);
        match ((*self.vtable).set_MozSelectedStyleSheetSet)(self as *const _, &*aSelectedStyleSheetSet) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString lastStyleSheetSet; */
    #[inline]
    pub unsafe fn get_lastStyleSheetSet(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lastStyleSheetSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsISupports styleSheetSets; */
    #[inline]
    pub unsafe fn get_styleSheetSets(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_styleSheetSets)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(MozEnableStyleSheetsForSet)] void enableStyleSheetsForSet (in DOMString name); */
    #[inline]
    pub unsafe fn MozEnableStyleSheetsForSet(&self, name: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        match ((*self.vtable).MozEnableStyleSheetsForSet)(self as *const _, &*name) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement elementFromPoint (in float x, in float y); */
    #[inline]
    pub unsafe fn elementFromPoint(&self, x: libc::c_float, y: libc::c_float) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).elementFromPoint)(self as *const _, x, y, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute DOMString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean mozSyntheticDocument; */
    #[inline]
    pub unsafe fn get_mozSyntheticDocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mozSyntheticDocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMElement currentScript; */
    #[inline]
    pub unsafe fn get_currentScript(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentScript)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void mozSetImageElement (in DOMString aImageElementId, in nsIDOMElement aImageElement); */
    #[inline]
    pub unsafe fn mozSetImageElement(&self, aImageElementId: &[u16], aImageElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {
        let aImageElementId = nsString::from(aImageElementId);
        match ((*self.vtable).mozSetImageElement)(self as *const _, &*aImageElementId, aImageElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISupports caretPositionFromPoint (in float x, in float y); */
    #[inline]
    pub unsafe fn caretPositionFromPoint(&self, x: libc::c_float, y: libc::c_float) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).caretPositionFromPoint)(self as *const _, x, y, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean hidden; */
    #[inline]
    pub unsafe fn get_hidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString visibilityState; */
    #[inline]
    pub unsafe fn get_visibilityState(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_visibilityState)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


