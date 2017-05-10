//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExpatSink.idl
//


#[repr(C)]
pub struct nsIExpatSink {
    vtable: *const nsIExpatSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExpatSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x01f681af, 0x0f22, 0x4725,
            [0xa9, 0x14, 0x0d, 0x39, 0x61, 0x14, 0xda, 0xf0])
    }
}

unsafe impl RefCounted for nsIExpatSink {
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
pub trait nsIExpatSinkCoerce {
    fn coerce_from(v: &nsIExpatSink) -> &Self;
}

impl nsIExpatSinkCoerce for nsIExpatSink {
    #[inline]
    fn coerce_from(v: &nsIExpatSink) -> &Self {
        v
    }
}

impl nsIExpatSink {
    #[inline]
    pub fn coerce<T: nsIExpatSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExpatSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExpatSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExpatSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExpatSinkVTable {
    pub __base: nsISupportsVTable,

    /* void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber); */
    /// Unable to call function as its signature contains a non-rust type
    pub HandleStartElement: *const ::libc::c_void,

    /* void HandleEndElement (in wstring aName); */
    pub HandleEndElement: unsafe extern "C" fn (this: *const nsIExpatSink, aName: *const libc::int16_t) -> nsresult,

    /* void HandleComment (in wstring aCommentText); */
    pub HandleComment: unsafe extern "C" fn (this: *const nsIExpatSink, aCommentText: *const libc::int16_t) -> nsresult,

    /* void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    pub HandleCDataSection: unsafe extern "C" fn (this: *const nsIExpatSink, aData: *const libc::int16_t, aLength: libc::uint32_t) -> nsresult,

    /* void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData); */
    pub HandleDoctypeDecl: unsafe extern "C" fn (this: *const nsIExpatSink, aSubset: *const nsAString, aName: *const nsAString, aSystemId: *const nsAString, aPublicId: *const nsAString, aCatalogData: *const nsISupports) -> nsresult,

    /* void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    pub HandleCharacterData: unsafe extern "C" fn (this: *const nsIExpatSink, aData: *const libc::int16_t, aLength: libc::uint32_t) -> nsresult,

    /* void HandleProcessingInstruction (in wstring aTarget, in wstring aData); */
    pub HandleProcessingInstruction: unsafe extern "C" fn (this: *const nsIExpatSink, aTarget: *const libc::int16_t, aData: *const libc::int16_t) -> nsresult,

    /* void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone); */
    pub HandleXMLDeclaration: unsafe extern "C" fn (this: *const nsIExpatSink, aVersion: *const libc::int16_t, aEncoding: *const libc::int16_t, aStandalone: libc::int32_t) -> nsresult,

    /* boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError); */
    pub ReportError: unsafe extern "C" fn (this: *const nsIExpatSink, aErrorText: *const libc::int16_t, aSourceText: *const libc::int16_t, aError: *const nsIScriptError, _retval: *mut bool) -> nsresult,

}


impl nsIExpatSink {
    /* void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber); */


    /* void HandleEndElement (in wstring aName); */
    #[inline]
    pub unsafe fn HandleEndElement(&self, aName: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleEndElement)(self as *const _, aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleComment (in wstring aCommentText); */
    #[inline]
    pub unsafe fn HandleComment(&self, aCommentText: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleComment)(self as *const _, aCommentText) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    #[inline]
    pub unsafe fn HandleCDataSection(&self, aData: *const libc::int16_t, aLength: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleCDataSection)(self as *const _, aData, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData); */
    #[inline]
    pub unsafe fn HandleDoctypeDecl(&self, aSubset: &[u16], aName: &[u16], aSystemId: &[u16], aPublicId: &[u16], aCatalogData: Option<&nsISupports>) -> Result<(), nsresult> {
        let aSubset = nsString::from(aSubset);
        let aName = nsString::from(aName);
        let aSystemId = nsString::from(aSystemId);
        let aPublicId = nsString::from(aPublicId);
        match ((*self.vtable).HandleDoctypeDecl)(self as *const _, &*aSubset, &*aName, &*aSystemId, &*aPublicId, aCatalogData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    #[inline]
    pub unsafe fn HandleCharacterData(&self, aData: *const libc::int16_t, aLength: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleCharacterData)(self as *const _, aData, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleProcessingInstruction (in wstring aTarget, in wstring aData); */
    #[inline]
    pub unsafe fn HandleProcessingInstruction(&self, aTarget: *const libc::int16_t, aData: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleProcessingInstruction)(self as *const _, aTarget, aData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone); */
    #[inline]
    pub unsafe fn HandleXMLDeclaration(&self, aVersion: *const libc::int16_t, aEncoding: *const libc::int16_t, aStandalone: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).HandleXMLDeclaration)(self as *const _, aVersion, aEncoding, aStandalone) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError); */
    #[inline]
    pub unsafe fn ReportError(&self, aErrorText: *const libc::int16_t, aSourceText: *const libc::int16_t, aError: Option<&nsIScriptError>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).ReportError)(self as *const _, aErrorText, aSourceText, aError.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


