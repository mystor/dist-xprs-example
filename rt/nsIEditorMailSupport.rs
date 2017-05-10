//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorMailSupport.idl
//


#[repr(C)]
pub struct nsIEditorMailSupport {
    vtable: *const nsIEditorMailSupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorMailSupport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfdf23301, 0x4a94, 0x11d3,
            [0x9c, 0xe4, 0x99, 0x60, 0x49, 0x6c, 0x41, 0xbc])
    }
}

unsafe impl RefCounted for nsIEditorMailSupport {
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
pub trait nsIEditorMailSupportCoerce {
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self;
}

impl nsIEditorMailSupportCoerce for nsIEditorMailSupport {
    #[inline]
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self {
        v
    }
}

impl nsIEditorMailSupport {
    #[inline]
    pub fn coerce<T: nsIEditorMailSupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorMailSupport {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorMailSupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorMailSupportVTable {
    pub __base: nsISupportsVTable,

    /* void pasteAsQuotation (in long aSelectionType); */
    pub pasteAsQuotation: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aSelectionType: libc::int32_t) -> nsresult,

    /* nsIDOMNode insertAsQuotation (in AString aQuotedText); */
    pub insertAsQuotation: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aQuotedText: *const nsAString, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void insertTextWithQuotations (in DOMString aStringToInsert); */
    pub insertTextWithQuotations: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aStringToInsert: *const nsAString) -> nsresult,

    /* void pasteAsCitedQuotation (in AString aCitation, in long aSelectionType); */
    pub pasteAsCitedQuotation: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aCitation: *const nsAString, aSelectionType: libc::int32_t) -> nsresult,

    /* nsIDOMNode insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
    pub insertAsCitedQuotation: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aQuotedText: *const nsAString, aCitation: *const nsAString, aInsertHTML: bool, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void rewrap (in boolean aRespectNewlines); */
    pub rewrap: unsafe extern "C" fn (this: *const nsIEditorMailSupport, aRespectNewlines: bool) -> nsresult,

    /* void stripCites (); */
    pub stripCites: unsafe extern "C" fn (this: *const nsIEditorMailSupport) -> nsresult,

    /* nsIArray getEmbeddedObjects (); */
    pub getEmbeddedObjects: unsafe extern "C" fn (this: *const nsIEditorMailSupport, _retval: *mut *const nsIArray) -> nsresult,

}


impl nsIEditorMailSupport {
    /* void pasteAsQuotation (in long aSelectionType); */
    #[inline]
    pub unsafe fn pasteAsQuotation(&self, aSelectionType: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).pasteAsQuotation)(self as *const _, aSelectionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode insertAsQuotation (in AString aQuotedText); */
    #[inline]
    pub unsafe fn insertAsQuotation(&self, aQuotedText: &[u16]) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let aQuotedText = nsString::from(aQuotedText);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).insertAsQuotation)(self as *const _, &*aQuotedText, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void insertTextWithQuotations (in DOMString aStringToInsert); */
    #[inline]
    pub unsafe fn insertTextWithQuotations(&self, aStringToInsert: &[u16]) -> Result<(), nsresult> {
        let aStringToInsert = nsString::from(aStringToInsert);
        match ((*self.vtable).insertTextWithQuotations)(self as *const _, &*aStringToInsert) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pasteAsCitedQuotation (in AString aCitation, in long aSelectionType); */
    #[inline]
    pub unsafe fn pasteAsCitedQuotation(&self, aCitation: &[u16], aSelectionType: libc::int32_t) -> Result<(), nsresult> {
        let aCitation = nsString::from(aCitation);
        match ((*self.vtable).pasteAsCitedQuotation)(self as *const _, &*aCitation, aSelectionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
    #[inline]
    pub unsafe fn insertAsCitedQuotation(&self, aQuotedText: &[u16], aCitation: &[u16], aInsertHTML: bool) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let aQuotedText = nsString::from(aQuotedText);
        let aCitation = nsString::from(aCitation);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).insertAsCitedQuotation)(self as *const _, &*aQuotedText, &*aCitation, aInsertHTML, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void rewrap (in boolean aRespectNewlines); */
    #[inline]
    pub unsafe fn rewrap(&self, aRespectNewlines: bool) -> Result<(), nsresult> {

        match ((*self.vtable).rewrap)(self as *const _, aRespectNewlines) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stripCites (); */
    #[inline]
    pub unsafe fn stripCites(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stripCites)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIArray getEmbeddedObjects (); */
    #[inline]
    pub unsafe fn getEmbeddedObjects(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEmbeddedObjects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


