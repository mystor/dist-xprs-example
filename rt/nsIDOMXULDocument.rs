//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULDocument.idl
//


#[repr(C)]
pub struct nsIDOMXULDocument {
    vtable: *const nsIDOMXULDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7790d4c3, 0xe8f0, 0x4e29,
            [0x98, 0x87, 0xd6, 0x83, 0xed, 0x2b, 0x2a, 0x44])
    }
}

unsafe impl RefCounted for nsIDOMXULDocument {
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
pub trait nsIDOMXULDocumentCoerce {
    fn coerce_from(v: &nsIDOMXULDocument) -> &Self;
}

impl nsIDOMXULDocumentCoerce for nsIDOMXULDocument {
    #[inline]
    fn coerce_from(v: &nsIDOMXULDocument) -> &Self {
        v
    }
}

impl nsIDOMXULDocument {
    #[inline]
    pub fn coerce<T: nsIDOMXULDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULDocument {
    type Target = nsIDOMDocument;
    #[inline]
    fn deref(&self) -> &nsIDOMDocument {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMDocumentCoerce> nsIDOMXULDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULDocumentVTable {
    pub __base: nsIDOMDocumentVTable,

    /* attribute nsIDOMNode popupNode; */
    pub get_popupNode: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aPopupNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_popupNode: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aPopupNode: *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMNode popupRangeParent; */
    pub get_popupRangeParent: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aPopupRangeParent: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute long popupRangeOffset; */
    pub get_popupRangeOffset: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aPopupRangeOffset: *mut libc::int32_t) -> nsresult,

    /* attribute nsIDOMNode tooltipNode; */
    pub get_tooltipNode: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aTooltipNode: *mut *const nsIDOMNode) -> nsresult,
    pub set_tooltipNode: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aTooltipNode: *const nsIDOMNode) -> nsresult,

    /* readonly attribute nsIDOMXULCommandDispatcher commandDispatcher; */
    pub get_commandDispatcher: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aCommandDispatcher: *mut *const nsIDOMXULCommandDispatcher) -> nsresult,

    /* readonly attribute long width; */
    pub get_width: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long height; */
    pub get_height: unsafe extern "C" fn (this: *const nsIDOMXULDocument, aHeight: *mut libc::int32_t) -> nsresult,

    /* nsIDOMNodeList getElementsByAttribute (in DOMString name, in DOMString value); */
    pub getElementsByAttribute: unsafe extern "C" fn (this: *const nsIDOMXULDocument, name: *const nsAString, value: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* nsIDOMNodeList getElementsByAttributeNS (in DOMString namespaceURI, in DOMString name, in DOMString value); */
    pub getElementsByAttributeNS: unsafe extern "C" fn (this: *const nsIDOMXULDocument, namespaceURI: *const nsAString, name: *const nsAString, value: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* void addBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
    pub addBroadcastListenerFor: unsafe extern "C" fn (this: *const nsIDOMXULDocument, broadcaster: *const nsIDOMElement, observer: *const nsIDOMElement, attr: *const nsAString) -> nsresult,

    /* void removeBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
    pub removeBroadcastListenerFor: unsafe extern "C" fn (this: *const nsIDOMXULDocument, broadcaster: *const nsIDOMElement, observer: *const nsIDOMElement, attr: *const nsAString) -> nsresult,

    /* void persist (in DOMString id, in DOMString attr); */
    pub persist: unsafe extern "C" fn (this: *const nsIDOMXULDocument, id: *const nsAString, attr: *const nsAString) -> nsresult,

    /* nsIBoxObject getBoxObjectFor (in nsIDOMElement elt); */
    pub getBoxObjectFor: unsafe extern "C" fn (this: *const nsIDOMXULDocument, elt: *const nsIDOMElement, _retval: *mut *const nsIBoxObject) -> nsresult,

    /* void loadOverlay (in DOMString url, in nsIObserver aObserver); */
    pub loadOverlay: unsafe extern "C" fn (this: *const nsIDOMXULDocument, url: *const nsAString, aObserver: *const nsIObserver) -> nsresult,

}


impl nsIDOMXULDocument {
    /* attribute nsIDOMNode popupNode; */
    #[inline]
    pub unsafe fn get_popupNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_popupNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_popupNode(&self, aPopupNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).set_popupNode)(self as *const _, aPopupNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMNode popupRangeParent; */
    #[inline]
    pub unsafe fn get_popupRangeParent(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_popupRangeParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long popupRangeOffset; */
    #[inline]
    pub unsafe fn get_popupRangeOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_popupRangeOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIDOMNode tooltipNode; */
    #[inline]
    pub unsafe fn get_tooltipNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tooltipNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_tooltipNode(&self, aTooltipNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).set_tooltipNode)(self as *const _, aTooltipNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMXULCommandDispatcher commandDispatcher; */
    #[inline]
    pub unsafe fn get_commandDispatcher(&self, ) -> Result<Option<RefPtr<nsIDOMXULCommandDispatcher>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_commandDispatcher)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long width; */
    #[inline]
    pub unsafe fn get_width(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_width)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long height; */
    #[inline]
    pub unsafe fn get_height(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_height)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMNodeList getElementsByAttribute (in DOMString name, in DOMString value); */
    #[inline]
    pub unsafe fn getElementsByAttribute(&self, name: &[u16], value: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let name = nsString::from(name);
        let value = nsString::from(value);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByAttribute)(self as *const _, &*name, &*value, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getElementsByAttributeNS (in DOMString namespaceURI, in DOMString name, in DOMString value); */
    #[inline]
    pub unsafe fn getElementsByAttributeNS(&self, namespaceURI: &[u16], name: &[u16], value: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let namespaceURI = nsString::from(namespaceURI);
        let name = nsString::from(name);
        let value = nsString::from(value);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByAttributeNS)(self as *const _, &*namespaceURI, &*name, &*value, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
    #[inline]
    pub unsafe fn addBroadcastListenerFor(&self, broadcaster: Option<&nsIDOMElement>, observer: Option<&nsIDOMElement>, attr: &[u16]) -> Result<(), nsresult> {
        let attr = nsString::from(attr);
        match ((*self.vtable).addBroadcastListenerFor)(self as *const _, broadcaster.map_or(::std::ptr::null(), |x| x as *const _), observer.map_or(::std::ptr::null(), |x| x as *const _), &*attr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeBroadcastListenerFor (in nsIDOMElement broadcaster, in nsIDOMElement observer, in DOMString attr); */
    #[inline]
    pub unsafe fn removeBroadcastListenerFor(&self, broadcaster: Option<&nsIDOMElement>, observer: Option<&nsIDOMElement>, attr: &[u16]) -> Result<(), nsresult> {
        let attr = nsString::from(attr);
        match ((*self.vtable).removeBroadcastListenerFor)(self as *const _, broadcaster.map_or(::std::ptr::null(), |x| x as *const _), observer.map_or(::std::ptr::null(), |x| x as *const _), &*attr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void persist (in DOMString id, in DOMString attr); */
    #[inline]
    pub unsafe fn persist(&self, id: &[u16], attr: &[u16]) -> Result<(), nsresult> {
        let id = nsString::from(id);
        let attr = nsString::from(attr);
        match ((*self.vtable).persist)(self as *const _, &*id, &*attr) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIBoxObject getBoxObjectFor (in nsIDOMElement elt); */
    #[inline]
    pub unsafe fn getBoxObjectFor(&self, elt: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIBoxObject>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBoxObjectFor)(self as *const _, elt.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void loadOverlay (in DOMString url, in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn loadOverlay(&self, url: &[u16], aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {
        let url = nsString::from(url);
        match ((*self.vtable).loadOverlay)(self as *const _, &*url, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


