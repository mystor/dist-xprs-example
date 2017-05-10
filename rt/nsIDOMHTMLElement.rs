//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMHTMLElement.idl
//


#[repr(C)]
pub struct nsIDOMHTMLElement {
    vtable: *const nsIDOMHTMLElementVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMHTMLElement {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb0c42392, 0xd0e7, 0x4f6a,
            [0xbe, 0xb5, 0xa6, 0x98, 0xce, 0x64, 0x89, 0x45])
    }
}

unsafe impl RefCounted for nsIDOMHTMLElement {
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
pub trait nsIDOMHTMLElementCoerce {
    fn coerce_from(v: &nsIDOMHTMLElement) -> &Self;
}

impl nsIDOMHTMLElementCoerce for nsIDOMHTMLElement {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLElement) -> &Self {
        v
    }
}

impl nsIDOMHTMLElement {
    #[inline]
    pub fn coerce<T: nsIDOMHTMLElementCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMHTMLElement {
    type Target = nsIDOMElement;
    #[inline]
    fn deref(&self) -> &nsIDOMElement {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDOMElementCoerce> nsIDOMHTMLElementCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMHTMLElement) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMHTMLElementVTable {
    pub __base: nsIDOMElementVTable,

    /* attribute DOMString title; */
    pub get_title: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aTitle: *mut nsAString) -> nsresult,
    pub set_title: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aTitle: *const nsAString) -> nsresult,

    /* attribute DOMString lang; */
    pub get_lang: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aLang: *mut nsAString) -> nsresult,
    pub set_lang: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aLang: *const nsAString) -> nsresult,

    /* attribute DOMString dir; */
    pub get_dir: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aDir: *mut nsAString) -> nsresult,
    pub set_dir: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aDir: *const nsAString) -> nsresult,

    /* readonly attribute nsISupports dataset; */
    pub get_dataset: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aDataset: *mut *const nsISupports) -> nsresult,

    /* attribute boolean hidden; */
    pub get_hidden: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aHidden: *mut bool) -> nsresult,
    pub set_hidden: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aHidden: bool) -> nsresult,

    /* attribute long tabIndex; */
    pub get_tabIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aTabIndex: *mut libc::int32_t) -> nsresult,
    pub set_tabIndex: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aTabIndex: libc::int32_t) -> nsresult,

    /* void focus (); */
    pub focus: unsafe extern "C" fn (this: *const nsIDOMHTMLElement) -> nsresult,

    /* [binaryname(DOMBlur)] void blur (); */
    pub DOMBlur: unsafe extern "C" fn (this: *const nsIDOMHTMLElement) -> nsresult,

    /* attribute DOMString accessKey; */
    pub get_accessKey: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aAccessKey: *mut nsAString) -> nsresult,
    pub set_accessKey: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aAccessKey: *const nsAString) -> nsresult,

    /* readonly attribute DOMString accessKeyLabel; */
    pub get_accessKeyLabel: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aAccessKeyLabel: *mut nsAString) -> nsresult,

    /* attribute boolean draggable; */
    pub get_draggable: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aDraggable: *mut bool) -> nsresult,
    pub set_draggable: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aDraggable: bool) -> nsresult,

    /* attribute DOMString contentEditable; */
    pub get_contentEditable: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aContentEditable: *mut nsAString) -> nsresult,
    pub set_contentEditable: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aContentEditable: *const nsAString) -> nsresult,

    /* readonly attribute boolean isContentEditable; */
    pub get_isContentEditable: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aIsContentEditable: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMHTMLMenuElement contextMenu; */
    pub get_contextMenu: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aContextMenu: *mut *const nsIDOMHTMLMenuElement) -> nsresult,

    /* attribute boolean spellcheck; */
    pub get_spellcheck: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aSpellcheck: *mut bool) -> nsresult,
    pub set_spellcheck: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aSpellcheck: bool) -> nsresult,

    /* attribute DOMString innerHTML; */
    pub get_innerHTML: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aInnerHTML: *mut nsAString) -> nsresult,
    pub set_innerHTML: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aInnerHTML: *const nsAString) -> nsresult,

    /* attribute DOMString outerHTML; */
    pub get_outerHTML: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOuterHTML: *mut nsAString) -> nsresult,
    pub set_outerHTML: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOuterHTML: *const nsAString) -> nsresult,

    /* void insertAdjacentHTML (in DOMString position, in DOMString text); */
    pub insertAdjacentHTML: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, position: *const nsAString, text: *const nsAString) -> nsresult,

    /* [optional_argc] void scrollIntoView ([optional] in boolean top); */
    /// Unable to call function as its signature contains a non-rust type
    pub scrollIntoView: *const ::libc::c_void,

    /* readonly attribute nsIDOMElement offsetParent; */
    pub get_offsetParent: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOffsetParent: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute long offsetTop; */
    pub get_offsetTop: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOffsetTop: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long offsetLeft; */
    pub get_offsetLeft: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOffsetLeft: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long offsetWidth; */
    pub get_offsetWidth: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOffsetWidth: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long offsetHeight; */
    pub get_offsetHeight: unsafe extern "C" fn (this: *const nsIDOMHTMLElement, aOffsetHeight: *mut libc::int32_t) -> nsresult,

}


impl nsIDOMHTMLElement {
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

    /* attribute DOMString lang; */
    #[inline]
    pub unsafe fn get_lang(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lang)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lang(&self, aLang: &[u16]) -> Result<(), nsresult> {
        let aLang = nsString::from(aLang);
        match ((*self.vtable).set_lang)(self as *const _, &*aLang) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* readonly attribute nsISupports dataset; */
    #[inline]
    pub unsafe fn get_dataset(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_dataset)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean hidden; */
    #[inline]
    pub unsafe fn get_hidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hidden(&self, aHidden: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_hidden)(self as *const _, aHidden) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long tabIndex; */
    #[inline]
    pub unsafe fn get_tabIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_tabIndex(&self, aTabIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_tabIndex)(self as *const _, aTabIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void focus (); */
    #[inline]
    pub unsafe fn focus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).focus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(DOMBlur)] void blur (); */
    #[inline]
    pub unsafe fn DOMBlur(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).DOMBlur)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString accessKey; */
    #[inline]
    pub unsafe fn get_accessKey(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessKey)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_accessKey(&self, aAccessKey: &[u16]) -> Result<(), nsresult> {
        let aAccessKey = nsString::from(aAccessKey);
        match ((*self.vtable).set_accessKey)(self as *const _, &*aAccessKey) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute DOMString accessKeyLabel; */
    #[inline]
    pub unsafe fn get_accessKeyLabel(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_accessKeyLabel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean draggable; */
    #[inline]
    pub unsafe fn get_draggable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_draggable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_draggable(&self, aDraggable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_draggable)(self as *const _, aDraggable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString contentEditable; */
    #[inline]
    pub unsafe fn get_contentEditable(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_contentEditable)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentEditable(&self, aContentEditable: &[u16]) -> Result<(), nsresult> {
        let aContentEditable = nsString::from(aContentEditable);
        match ((*self.vtable).set_contentEditable)(self as *const _, &*aContentEditable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isContentEditable; */
    #[inline]
    pub unsafe fn get_isContentEditable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isContentEditable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMHTMLMenuElement contextMenu; */
    #[inline]
    pub unsafe fn get_contextMenu(&self, ) -> Result<Option<RefPtr<nsIDOMHTMLMenuElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contextMenu)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean spellcheck; */
    #[inline]
    pub unsafe fn get_spellcheck(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_spellcheck)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_spellcheck(&self, aSpellcheck: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_spellcheck)(self as *const _, aSpellcheck) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString innerHTML; */
    #[inline]
    pub unsafe fn get_innerHTML(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_innerHTML)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_innerHTML(&self, aInnerHTML: &[u16]) -> Result<(), nsresult> {
        let aInnerHTML = nsString::from(aInnerHTML);
        match ((*self.vtable).set_innerHTML)(self as *const _, &*aInnerHTML) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString outerHTML; */
    #[inline]
    pub unsafe fn get_outerHTML(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_outerHTML)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_outerHTML(&self, aOuterHTML: &[u16]) -> Result<(), nsresult> {
        let aOuterHTML = nsString::from(aOuterHTML);
        match ((*self.vtable).set_outerHTML)(self as *const _, &*aOuterHTML) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertAdjacentHTML (in DOMString position, in DOMString text); */
    #[inline]
    pub unsafe fn insertAdjacentHTML(&self, position: &[u16], text: &[u16]) -> Result<(), nsresult> {
        let position = nsString::from(position);
        let text = nsString::from(text);
        match ((*self.vtable).insertAdjacentHTML)(self as *const _, &*position, &*text) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [optional_argc] void scrollIntoView ([optional] in boolean top); */


    /* readonly attribute nsIDOMElement offsetParent; */
    #[inline]
    pub unsafe fn get_offsetParent(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_offsetParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long offsetTop; */
    #[inline]
    pub unsafe fn get_offsetTop(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offsetTop)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long offsetLeft; */
    #[inline]
    pub unsafe fn get_offsetLeft(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offsetLeft)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long offsetWidth; */
    #[inline]
    pub unsafe fn get_offsetWidth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offsetWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long offsetHeight; */
    #[inline]
    pub unsafe fn get_offsetHeight(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_offsetHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


