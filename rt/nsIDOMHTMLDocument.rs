//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLDocument.idl
//


#[repr(C)]
pub struct nsIDOMHTMLDocument {
    vtable: *const nsIDOMHTMLDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLDocument {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcd31e61f, 0xcfc2, 0x4b91,
            [0x93, 0x85, 0x17, 0xb6, 0xa2, 0xd0, 0x63, 0x3d])
    }
}

unsafe impl RefCounted for nsIDOMHTMLDocument {
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
pub trait nsIDOMHTMLDocumentCoerce {
    fn coerce_from(v: &nsIDOMHTMLDocument) -> &Self;
}

impl nsIDOMHTMLDocumentCoerce for nsIDOMHTMLDocument {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLDocument) -> &Self {
        v
    }
}

impl nsIDOMHTMLDocument {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLDocument {
    type Target = nsIDOMDocument;
    #[inline]
    fn deref(&self) -> &nsIDOMDocument {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMDocumentCoerce> nsIDOMHTMLDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLDocument) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLDocumentVTable {
    pub __base: nsIDOMDocumentVTable,

    /* attribute DOMString domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aDomain: *mut nsAString) -> nsresult,
    pub set_domain: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aDomain: *const nsAString) -> nsresult,

    /* attribute DOMString cookie; */
    pub get_cookie: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aCookie: *mut nsAString) -> nsresult,
    pub set_cookie: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aCookie: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLHeadElement head; */
    pub get_head: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aHead: *mut *const nsIDOMHTMLHeadElement) -> nsresult,

    /* attribute nsIDOMHTMLElement body; */
    pub get_body: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aBody: *mut *const nsIDOMHTMLElement) -> nsresult,
    pub set_body: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aBody: *const nsIDOMHTMLElement) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection images; */
    pub get_images: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aImages: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection embeds; */
    pub get_embeds: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aEmbeds: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection plugins; */
    pub get_plugins: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aPlugins: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection links; */
    pub get_links: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aLinks: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection forms; */
    pub get_forms: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aForms: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection scripts; */
    pub get_scripts: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aScripts: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* nsIDOMNodeList getElementsByName (in DOMString elementName); */
    pub getElementsByName: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, elementName: *const nsAString, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* [implicit_jscontext,optional_argc] nsISupports open ([optional] in DOMString aContentTypeOrUrl, [optional] in DOMString aReplaceOrName, [optional] in DOMString aFeatures); */
    /// Unable to call function as its signature contains a non-rust type
    pub open: *const ::libc::c_void,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument) -> nsresult,

    /* [implicit_jscontext] void write ([optional, Null (Stringify)] in DOMString text); */
    /// Unable to call function as its signature contains a non-rust type
    pub write: *const ::libc::c_void,

    /* [implicit_jscontext] void writeln ([optional, Null (Stringify)] in DOMString text); */
    /// Unable to call function as its signature contains a non-rust type
    pub writeln: *const ::libc::c_void,

    /* attribute DOMString designMode; */
    pub get_designMode: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aDesignMode: *mut nsAString) -> nsresult,
    pub set_designMode: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aDesignMode: *const nsAString) -> nsresult,

    /* boolean queryCommandIndeterm (in DOMString commandID); */
    pub queryCommandIndeterm: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, commandID: *const nsAString, _retval: *mut bool) -> nsresult,

    /* boolean queryCommandState (in DOMString commandID); */
    pub queryCommandState: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, commandID: *const nsAString, _retval: *mut bool) -> nsresult,

    /* DOMString queryCommandValue (in DOMString commandID); */
    pub queryCommandValue: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, commandID: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* attribute DOMString fgColor; */
    pub get_fgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aFgColor: *mut nsAString) -> nsresult,
    pub set_fgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aFgColor: *const nsAString) -> nsresult,

    /* attribute DOMString bgColor; */
    pub get_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aBgColor: *mut nsAString) -> nsresult,
    pub set_bgColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aBgColor: *const nsAString) -> nsresult,

    /* attribute DOMString linkColor; */
    pub get_linkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aLinkColor: *mut nsAString) -> nsresult,
    pub set_linkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aLinkColor: *const nsAString) -> nsresult,

    /* attribute DOMString vlinkColor; */
    pub get_vlinkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aVlinkColor: *mut nsAString) -> nsresult,
    pub set_vlinkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aVlinkColor: *const nsAString) -> nsresult,

    /* attribute DOMString alinkColor; */
    pub get_alinkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aAlinkColor: *mut nsAString) -> nsresult,
    pub set_alinkColor: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aAlinkColor: *const nsAString) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection anchors; */
    pub get_anchors: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aAnchors: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* readonly attribute nsIDOMHTMLCollection applets; */
    pub get_applets: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, aApplets: *mut *const nsIDOMHTMLCollection) -> nsresult,

    /* void clear (); */
    pub clear: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument) -> nsresult,

    /* nsISelection getSelection (); */
    pub getSelection: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument, _retval: *mut *const nsISelection) -> nsresult,

    /* void captureEvents (); */
    pub captureEvents: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument) -> nsresult,

    /* void releaseEvents (); */
    pub releaseEvents: unsafe extern "C" fn (this: *const nsIDOMHTMLDocument) -> nsresult,

}


impl nsIDOMHTMLDocument {
    /* attribute DOMString domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_domain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_domain(&self, aDomain: &[u16]) -> Result<(), nsresult> {
        let aDomain = nsString::from(aDomain);
        match ((*self.vtable).set_domain)(self as *const _, &*aDomain) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString cookie; */
    #[inline]
    pub unsafe fn get_cookie(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cookie)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_cookie(&self, aCookie: &[u16]) -> Result<(), nsresult> {
        let aCookie = nsString::from(aCookie);
        match ((*self.vtable).set_cookie)(self as *const _, &*aCookie) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLHeadElement head; */
    #[inline]
    pub unsafe fn get_head(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLHeadElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_head)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIDOMHTMLElement body; */
    #[inline]
    pub unsafe fn get_body(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_body)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_body(&self, aBody: Option<&nsIDOMHTMLElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_body)(self as *const _, aBody.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLCollection images; */
    #[inline]
    pub unsafe fn get_images(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_images)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection embeds; */
    #[inline]
    pub unsafe fn get_embeds(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_embeds)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection plugins; */
    #[inline]
    pub unsafe fn get_plugins(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_plugins)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection links; */
    #[inline]
    pub unsafe fn get_links(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_links)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection forms; */
    #[inline]
    pub unsafe fn get_forms(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_forms)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection scripts; */
    #[inline]
    pub unsafe fn get_scripts(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_scripts)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getElementsByName (in DOMString elementName); */
    #[inline]
    pub unsafe fn getElementsByName(&self, elementName: &[u16]) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let elementName = nsString::from(elementName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementsByName)(self as *const _, &*elementName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext,optional_argc] nsISupports open ([optional] in DOMString aContentTypeOrUrl, [optional] in DOMString aReplaceOrName, [optional] in DOMString aFeatures); */


    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] void write ([optional, Null (Stringify)] in DOMString text); */


    /* [implicit_jscontext] void writeln ([optional, Null (Stringify)] in DOMString text); */


    /* attribute DOMString designMode; */
    #[inline]
    pub unsafe fn get_designMode(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_designMode)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_designMode(&self, aDesignMode: &[u16]) -> Result<(), nsresult> {
        let aDesignMode = nsString::from(aDesignMode);
        match ((*self.vtable).set_designMode)(self as *const _, &*aDesignMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean queryCommandIndeterm (in DOMString commandID); */
    #[inline]
    pub unsafe fn queryCommandIndeterm(&self, commandID: &[u16]) -> Result<bool, nsresult> {
        let commandID = nsString::from(commandID);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).queryCommandIndeterm)(self as *const _, &*commandID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean queryCommandState (in DOMString commandID); */
    #[inline]
    pub unsafe fn queryCommandState(&self, commandID: &[u16]) -> Result<bool, nsresult> {
        let commandID = nsString::from(commandID);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).queryCommandState)(self as *const _, &*commandID, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* DOMString queryCommandValue (in DOMString commandID); */
    #[inline]
    pub unsafe fn queryCommandValue(&self, commandID: &[u16]) -> Result<nsString, nsresult> {
        let commandID = nsString::from(commandID);
        let mut _retval = nsString::new();
        match ((*self.vtable).queryCommandValue)(self as *const _, &*commandID, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute DOMString fgColor; */
    #[inline]
    pub unsafe fn get_fgColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_fgColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fgColor(&self, aFgColor: &[u16]) -> Result<(), nsresult> {
        let aFgColor = nsString::from(aFgColor);
        match ((*self.vtable).set_fgColor)(self as *const _, &*aFgColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString bgColor; */
    #[inline]
    pub unsafe fn get_bgColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_bgColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_bgColor(&self, aBgColor: &[u16]) -> Result<(), nsresult> {
        let aBgColor = nsString::from(aBgColor);
        match ((*self.vtable).set_bgColor)(self as *const _, &*aBgColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString linkColor; */
    #[inline]
    pub unsafe fn get_linkColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_linkColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_linkColor(&self, aLinkColor: &[u16]) -> Result<(), nsresult> {
        let aLinkColor = nsString::from(aLinkColor);
        match ((*self.vtable).set_linkColor)(self as *const _, &*aLinkColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString vlinkColor; */
    #[inline]
    pub unsafe fn get_vlinkColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_vlinkColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_vlinkColor(&self, aVlinkColor: &[u16]) -> Result<(), nsresult> {
        let aVlinkColor = nsString::from(aVlinkColor);
        match ((*self.vtable).set_vlinkColor)(self as *const _, &*aVlinkColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString alinkColor; */
    #[inline]
    pub unsafe fn get_alinkColor(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alinkColor)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alinkColor(&self, aAlinkColor: &[u16]) -> Result<(), nsresult> {
        let aAlinkColor = nsString::from(aAlinkColor);
        match ((*self.vtable).set_alinkColor)(self as *const _, &*aAlinkColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMHTMLCollection anchors; */
    #[inline]
    pub unsafe fn get_anchors(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_anchors)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMHTMLCollection applets; */
    #[inline]
    pub unsafe fn get_applets(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLCollection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_applets)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void clear (); */
    #[inline]
    pub unsafe fn clear(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clear)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISelection getSelection (); */
    #[inline]
    pub unsafe fn getSelection(&self, ) -> Result<Option<RefPtr<nsISelection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void captureEvents (); */
    #[inline]
    pub unsafe fn captureEvents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).captureEvents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void releaseEvents (); */
    #[inline]
    pub unsafe fn releaseEvents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).releaseEvents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


