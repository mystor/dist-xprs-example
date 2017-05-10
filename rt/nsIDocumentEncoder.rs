//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocumentEncoder.idl
//


#[repr(C)]
pub struct nsIDocumentEncoderNodeFixup {
    vtable: *const nsIDocumentEncoderNodeFixupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocumentEncoderNodeFixup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3d9371d8, 0xa2ad, 0x403e,
            [0x8b, 0x0e, 0x88, 0x85, 0xad, 0x35, 0x62, 0xe3])
    }
}

unsafe impl RefCounted for nsIDocumentEncoderNodeFixup {
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
pub trait nsIDocumentEncoderNodeFixupCoerce {
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self;
}

impl nsIDocumentEncoderNodeFixupCoerce for nsIDocumentEncoderNodeFixup {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self {
        v
    }
}

impl nsIDocumentEncoderNodeFixup {
    #[inline]
    pub fn coerce<T: nsIDocumentEncoderNodeFixupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocumentEncoderNodeFixup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocumentEncoderNodeFixupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocumentEncoderNodeFixupVTable {
    pub __base: nsISupportsVTable,

    /* nsIDOMNode fixupNode (in nsIDOMNode aNode, out boolean aSerializeCloneKids); */
    pub fixupNode: unsafe extern "C" fn (this: *const nsIDocumentEncoderNodeFixup, aNode: *const nsIDOMNode, aSerializeCloneKids: *mut bool, _retval: *mut *const nsIDOMNode) -> nsresult,

}


impl nsIDocumentEncoderNodeFixup {
    /* nsIDOMNode fixupNode (in nsIDOMNode aNode, out boolean aSerializeCloneKids); */
    #[inline]
    pub unsafe fn fixupNode(&self, aNode: Option<&nsIDOMNode>) -> Result<(bool, Option<RefPtr<nsIDOMNode>>), nsresult> {
        let mut aSerializeCloneKids: bool = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).fixupNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), &mut aSerializeCloneKids as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aSerializeCloneKids, _retval.refptr()))
    }

}


pub mod nsIDocumentEncoder_consts {
    pub const OutputSelectionOnly: i64 = 1;
    pub const OutputFormatted: i64 = 2;
    pub const OutputRaw: i64 = 4;
    pub const OutputBodyOnly: i64 = 8;
    pub const OutputPreformatted: i64 = 16;
    pub const OutputWrap: i64 = 32;
    pub const OutputFormatFlowed: i64 = 64;
    pub const OutputAbsoluteLinks: i64 = 128;
    pub const OutputEncodeW3CEntities: i64 = 256;
    pub const OutputCRLineBreak: i64 = 512;
    pub const OutputLFLineBreak: i64 = 1024;
    pub const OutputNoScriptContent: i64 = 2048;
    pub const OutputNoFramesContent: i64 = 4096;
    pub const OutputNoFormattingInPre: i64 = 8192;
    pub const OutputEncodeBasicEntities: i64 = 16384;
    pub const OutputEncodeLatin1Entities: i64 = 32768;
    pub const OutputEncodeHTMLEntities: i64 = 65536;
    pub const OutputPersistNBSP: i64 = 131072;
    pub const OutputDontRewriteEncodingDeclaration: i64 = 262144;
    pub const SkipInvisibleContent: i64 = 524288;
    pub const OutputFormatDelSp: i64 = 1048576;
    pub const OutputDropInvisibleBreak: i64 = 2097152;
    pub const OutputIgnoreMozDirty: i64 = 4194304;
    pub const OutputNonTextContentAsPlaceholder: i64 = 8388608;
    pub const OutputDontRemoveLineEndingSpaces: i64 = 16777216;
    pub const OutputForPlainTextClipboardCopy: i64 = 33554432;
    pub const OutputRubyAnnotation: i64 = 67108864;
    pub const OutputDisallowLineBreaking: i64 = 134217728;
    pub const RequiresReinitAfterOutput: i64 = 268435456;
}


#[repr(C)]
pub struct nsIDocumentEncoder {
    vtable: *const nsIDocumentEncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocumentEncoder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x21f112df, 0xd96f, 0x47da,
            [0xbf, 0xcb, 0x53, 0x31, 0x27, 0x30, 0x03, 0xd1])
    }
}

unsafe impl RefCounted for nsIDocumentEncoder {
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
pub trait nsIDocumentEncoderCoerce {
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self;
}

impl nsIDocumentEncoderCoerce for nsIDocumentEncoder {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self {
        v
    }
}

impl nsIDocumentEncoder {
    #[inline]
    pub fn coerce<T: nsIDocumentEncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocumentEncoder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDocumentEncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocumentEncoderVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIDOMDocument aDocument, in AString aMimeType, in unsigned long aFlags); */
    pub init: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aDocument: *const nsIDOMDocument, aMimeType: *const nsAString, aFlags: libc::uint32_t) -> nsresult,

    /* [noscript] void nativeInit (in nsIDocumentPtr aDocument, in AString aMimeType, in unsigned long aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub nativeInit: *const ::libc::c_void,

    /* void setSelection (in nsISelection aSelection); */
    pub setSelection: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aSelection: *const nsISelection) -> nsresult,

    /* void setRange (in nsIDOMRange aRange); */
    pub setRange: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aRange: *const nsIDOMRange) -> nsresult,

    /* void setNode (in nsIDOMNode aNode); */
    pub setNode: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aNode: *const nsIDOMNode) -> nsresult,

    /* [noscript] void setNativeNode (in nsINodePtr aNode); */
    /// Unable to call function as its signature contains a non-rust type
    pub setNativeNode: *const ::libc::c_void,

    /* void setContainerNode (in nsIDOMNode aContainer); */
    pub setContainerNode: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aContainer: *const nsIDOMNode) -> nsresult,

    /* [noscript] void setNativeContainerNode (in nsINodePtr aContainer); */
    /// Unable to call function as its signature contains a non-rust type
    pub setNativeContainerNode: *const ::libc::c_void,

    /* void setCharset (in ACString aCharset); */
    pub setCharset: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aCharset: *const nsACString) -> nsresult,

    /* void setWrapColumn (in unsigned long aWrapColumn); */
    pub setWrapColumn: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aWrapColumn: libc::uint32_t) -> nsresult,

    /* readonly attribute AString mimeType; */
    pub get_mimeType: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aMimeType: *mut nsAString) -> nsresult,

    /* void encodeToStream (in nsIOutputStream aStream); */
    pub encodeToStream: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aStream: *const nsIOutputStream) -> nsresult,

    /* AString encodeToString (); */
    pub encodeToString: unsafe extern "C" fn (this: *const nsIDocumentEncoder, _retval: *mut nsAString) -> nsresult,

    /* AString encodeToStringWithContext (out AString aContextString, out AString aInfoString); */
    pub encodeToStringWithContext: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aContextString: *mut nsAString, aInfoString: *mut nsAString, _retval: *mut nsAString) -> nsresult,

    /* AString encodeToStringWithMaxLength (in unsigned long aMaxLength); */
    pub encodeToStringWithMaxLength: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aMaxLength: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup); */
    pub setNodeFixup: unsafe extern "C" fn (this: *const nsIDocumentEncoder, aFixup: *const nsIDocumentEncoderNodeFixup) -> nsresult,

}


impl nsIDocumentEncoder {
    /* void init (in nsIDOMDocument aDocument, in AString aMimeType, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn init(&self, aDocument: Option<&nsIDOMDocument>, aMimeType: &[u16], aFlags: libc::uint32_t) -> Result<(), nsresult> {
        let aMimeType = nsString::from(aMimeType);
        match ((*self.vtable).init)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void nativeInit (in nsIDocumentPtr aDocument, in AString aMimeType, in unsigned long aFlags); */


    /* void setSelection (in nsISelection aSelection); */
    #[inline]
    pub unsafe fn setSelection(&self, aSelection: Option<&nsISelection>) -> Result<(), nsresult> {

        match ((*self.vtable).setSelection)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRange (in nsIDOMRange aRange); */
    #[inline]
    pub unsafe fn setRange(&self, aRange: Option<&nsIDOMRange>) -> Result<(), nsresult> {

        match ((*self.vtable).setRange)(self as *const _, aRange.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setNode (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn setNode(&self, aNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setNativeNode (in nsINodePtr aNode); */


    /* void setContainerNode (in nsIDOMNode aContainer); */
    #[inline]
    pub unsafe fn setContainerNode(&self, aContainer: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).setContainerNode)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setNativeContainerNode (in nsINodePtr aContainer); */


    /* void setCharset (in ACString aCharset); */
    #[inline]
    pub unsafe fn setCharset(&self, aCharset: &[u8]) -> Result<(), nsresult> {
        let aCharset = nsCString::from(aCharset);
        match ((*self.vtable).setCharset)(self as *const _, &*aCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setWrapColumn (in unsigned long aWrapColumn); */
    #[inline]
    pub unsafe fn setWrapColumn(&self, aWrapColumn: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setWrapColumn)(self as *const _, aWrapColumn) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AString mimeType; */
    #[inline]
    pub unsafe fn get_mimeType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_mimeType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void encodeToStream (in nsIOutputStream aStream); */
    #[inline]
    pub unsafe fn encodeToStream(&self, aStream: Option<&nsIOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).encodeToStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString encodeToString (); */
    #[inline]
    pub unsafe fn encodeToString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).encodeToString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString encodeToStringWithContext (out AString aContextString, out AString aInfoString); */
    #[inline]
    pub unsafe fn encodeToStringWithContext(&self, ) -> Result<(nsString, nsString, nsString), nsresult> {
        let mut aContextString = nsString::new();
        let mut aInfoString = nsString::new();
        let mut _retval = nsString::new();
        match ((*self.vtable).encodeToStringWithContext)(self as *const _, &mut *aContextString, &mut *aInfoString, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aContextString, aInfoString, _retval))
    }

    /* AString encodeToStringWithMaxLength (in unsigned long aMaxLength); */
    #[inline]
    pub unsafe fn encodeToStringWithMaxLength(&self, aMaxLength: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).encodeToStringWithMaxLength)(self as *const _, aMaxLength, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup); */
    #[inline]
    pub unsafe fn setNodeFixup(&self, aFixup: Option<&nsIDocumentEncoderNodeFixup>) -> Result<(), nsresult> {

        match ((*self.vtable).setNodeFixup)(self as *const _, aFixup.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


