//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHTMLEditor.idl
//


pub mod nsIHTMLEditor_consts {
    pub const eLeft: i64 = 0;
    pub const eCenter: i64 = 1;
    pub const eRight: i64 = 2;
    pub const eJustify: i64 = 3;
}


#[repr(C)]
pub struct nsIHTMLEditor {
    vtable: *const nsIHTMLEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHTMLEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87ee993e, 0x985f, 0x4a43,
            [0xa9, 0x74, 0x0d, 0x95, 0x12, 0xda, 0x2f, 0xb0])
    }
}

unsafe impl RefCounted for nsIHTMLEditor {
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
pub trait nsIHTMLEditorCoerce {
    fn coerce_from(v: &nsIHTMLEditor) -> &Self;
}

impl nsIHTMLEditorCoerce for nsIHTMLEditor {
    #[inline]
    fn coerce_from(v: &nsIHTMLEditor) -> &Self {
        v
    }
}

impl nsIHTMLEditor {
    #[inline]
    pub fn coerce<T: nsIHTMLEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHTMLEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHTMLEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHTMLEditorVTable {
    pub __base: nsISupportsVTable,

    /* void addDefaultProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    pub addDefaultProperty: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString, aValue: *const nsAString) -> nsresult,

    /* void removeDefaultProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    pub removeDefaultProperty: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString, aValue: *const nsAString) -> nsresult,

    /* void removeAllDefaultProperties (); */
    pub removeAllDefaultProperties: unsafe extern "C" fn (this: *const nsIHTMLEditor) -> nsresult,

    /* void setInlineProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    pub setInlineProperty: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString, aValue: *const nsAString) -> nsresult,

    /* void getInlineProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    pub getInlineProperty: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString, aValue: *const nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool) -> nsresult,

    /* AString getInlinePropertyWithAttrValue (in nsIAtom aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    pub getInlinePropertyWithAttrValue: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString, aValue: *const nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* void removeAllInlineProperties (); */
    pub removeAllInlineProperties: unsafe extern "C" fn (this: *const nsIHTMLEditor) -> nsresult,

    /* void removeInlineProperty (in nsIAtom aProperty, in AString aAttribute); */
    pub removeInlineProperty: unsafe extern "C" fn (this: *const nsIHTMLEditor, aProperty: *const nsIAtom, aAttribute: *const nsAString) -> nsresult,

    /* void increaseFontSize (); */
    pub increaseFontSize: unsafe extern "C" fn (this: *const nsIHTMLEditor) -> nsresult,

    /* void decreaseFontSize (); */
    pub decreaseFontSize: unsafe extern "C" fn (this: *const nsIHTMLEditor) -> nsresult,

    /* boolean nodeIsBlock (in nsIDOMNode node); */
    pub nodeIsBlock: unsafe extern "C" fn (this: *const nsIHTMLEditor, node: *const nsIDOMNode, _retval: *mut bool) -> nsresult,

    /* void insertHTML (in AString aInputString); */
    pub insertHTML: unsafe extern "C" fn (this: *const nsIHTMLEditor, aInputString: *const nsAString) -> nsresult,

    /* void pasteNoFormatting (in long aSelectionType); */
    pub pasteNoFormatting: unsafe extern "C" fn (this: *const nsIHTMLEditor, aSelectionType: libc::int32_t) -> nsresult,

    /* void rebuildDocumentFromSource (in AString aSourceString); */
    pub rebuildDocumentFromSource: unsafe extern "C" fn (this: *const nsIHTMLEditor, aSourceString: *const nsAString) -> nsresult,

    /* void insertHTMLWithContext (in AString aInputString, in AString aContextStr, in AString aInfoStr, in AString aFlavor, in nsIDOMDocument aSourceDoc, in nsIDOMNode aDestinationNode, in long aDestinationOffset, in boolean aDeleteSelection); */
    pub insertHTMLWithContext: unsafe extern "C" fn (this: *const nsIHTMLEditor, aInputString: *const nsAString, aContextStr: *const nsAString, aInfoStr: *const nsAString, aFlavor: *const nsAString, aSourceDoc: *const nsIDOMDocument, aDestinationNode: *const nsIDOMNode, aDestinationOffset: libc::int32_t, aDeleteSelection: bool) -> nsresult,

    /* void insertElementAtSelection (in nsIDOMElement aElement, in boolean aDeleteSelection); */
    pub insertElementAtSelection: unsafe extern "C" fn (this: *const nsIHTMLEditor, aElement: *const nsIDOMElement, aDeleteSelection: bool) -> nsresult,

    /* void updateBaseURL (); */
    pub updateBaseURL: unsafe extern "C" fn (this: *const nsIHTMLEditor) -> nsresult,

    /* void selectElement (in nsIDOMElement aElement); */
    pub selectElement: unsafe extern "C" fn (this: *const nsIHTMLEditor, aElement: *const nsIDOMElement) -> nsresult,

    /* void setCaretAfterElement (in nsIDOMElement aElement); */
    pub setCaretAfterElement: unsafe extern "C" fn (this: *const nsIHTMLEditor, aElement: *const nsIDOMElement) -> nsresult,

    /* void setParagraphFormat (in AString aParagraphFormat); */
    pub setParagraphFormat: unsafe extern "C" fn (this: *const nsIHTMLEditor, aParagraphFormat: *const nsAString) -> nsresult,

    /* AString getParagraphState (out boolean aMixed); */
    pub getParagraphState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* AString getFontFaceState (out boolean aMixed); */
    pub getFontFaceState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* AString getFontColorState (out boolean aMixed); */
    pub getFontColorState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* AString getBackgroundColorState (out boolean aMixed); */
    pub getBackgroundColorState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* AString getHighlightColorState (out boolean aMixed); */
    pub getHighlightColorState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut nsAString) -> nsresult,

    /* void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL); */
    pub getListState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aOL: *mut bool, aUL: *mut bool, aDL: *mut bool) -> nsresult,

    /* void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD); */
    pub getListItemState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aLI: *mut bool, aDT: *mut bool, aDD: *mut bool) -> nsresult,

    /* void getAlignment (out boolean aMixed, out short aAlign); */
    pub getAlignment: unsafe extern "C" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aAlign: *mut libc::int16_t) -> nsresult,

    /* void getIndentState (out boolean aCanIndent, out boolean aCanOutdent); */
    pub getIndentState: unsafe extern "C" fn (this: *const nsIHTMLEditor, aCanIndent: *mut bool, aCanOutdent: *mut bool) -> nsresult,

    /* void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType); */
    pub makeOrChangeList: unsafe extern "C" fn (this: *const nsIHTMLEditor, aListType: *const nsAString, entireList: bool, aBulletType: *const nsAString) -> nsresult,

    /* void removeList (in AString aListType); */
    pub removeList: unsafe extern "C" fn (this: *const nsIHTMLEditor, aListType: *const nsAString) -> nsresult,

    /* void indent (in AString aIndent); */
    pub indent: unsafe extern "C" fn (this: *const nsIHTMLEditor, aIndent: *const nsAString) -> nsresult,

    /* void align (in AString aAlign); */
    pub align: unsafe extern "C" fn (this: *const nsIHTMLEditor, aAlign: *const nsAString) -> nsresult,

    /* nsIDOMElement getElementOrParentByTagName (in AString aTagName, in nsIDOMNode aNode); */
    pub getElementOrParentByTagName: unsafe extern "C" fn (this: *const nsIHTMLEditor, aTagName: *const nsAString, aNode: *const nsIDOMNode, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* nsIDOMElement getSelectedElement (in AString aTagName); */
    pub getSelectedElement: unsafe extern "C" fn (this: *const nsIHTMLEditor, aTagName: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* AString getHeadContentsAsHTML (); */
    pub getHeadContentsAsHTML: unsafe extern "C" fn (this: *const nsIHTMLEditor, _retval: *mut nsAString) -> nsresult,

    /* void replaceHeadContentsWithHTML (in AString aSourceToInsert); */
    pub replaceHeadContentsWithHTML: unsafe extern "C" fn (this: *const nsIHTMLEditor, aSourceToInsert: *const nsAString) -> nsresult,

    /* nsIDOMElement createElementWithDefaults (in AString aTagName); */
    pub createElementWithDefaults: unsafe extern "C" fn (this: *const nsIHTMLEditor, aTagName: *const nsAString, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void insertLinkAroundSelection (in nsIDOMElement aAnchorElement); */
    pub insertLinkAroundSelection: unsafe extern "C" fn (this: *const nsIHTMLEditor, aAnchorElement: *const nsIDOMElement) -> nsresult,

    /* void setBackgroundColor (in AString aColor); */
    pub setBackgroundColor: unsafe extern "C" fn (this: *const nsIHTMLEditor, aColor: *const nsAString) -> nsresult,

    /* void setBodyAttribute (in AString aAttr, in AString aValue); */
    pub setBodyAttribute: unsafe extern "C" fn (this: *const nsIHTMLEditor, aAttr: *const nsAString, aValue: *const nsAString) -> nsresult,

    /* nsIArray getLinkedObjects (); */
    pub getLinkedObjects: unsafe extern "C" fn (this: *const nsIHTMLEditor, _retval: *mut *const nsIArray) -> nsresult,

    /* attribute boolean isCSSEnabled; */
    pub get_isCSSEnabled: unsafe extern "C" fn (this: *const nsIHTMLEditor, aIsCSSEnabled: *mut bool) -> nsresult,
    pub set_isCSSEnabled: unsafe extern "C" fn (this: *const nsIHTMLEditor, aIsCSSEnabled: bool) -> nsresult,

    /* void addInsertionListener (in nsIContentFilter inFilter); */
    pub addInsertionListener: unsafe extern "C" fn (this: *const nsIHTMLEditor, inFilter: *const nsIContentFilter) -> nsresult,

    /* void removeInsertionListener (in nsIContentFilter inFilter); */
    pub removeInsertionListener: unsafe extern "C" fn (this: *const nsIHTMLEditor, inFilter: *const nsIContentFilter) -> nsresult,

    /* nsIDOMElement getSelectionContainer (); */
    pub getSelectionContainer: unsafe extern "C" fn (this: *const nsIHTMLEditor, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void checkSelectionStateForAnonymousButtons (in nsISelection aSelection); */
    pub checkSelectionStateForAnonymousButtons: unsafe extern "C" fn (this: *const nsIHTMLEditor, aSelection: *const nsISelection) -> nsresult,

    /* boolean isAnonymousElement (in nsIDOMElement aElement); */
    pub isAnonymousElement: unsafe extern "C" fn (this: *const nsIHTMLEditor, aElement: *const nsIDOMElement, _retval: *mut bool) -> nsresult,

    /* attribute boolean returnInParagraphCreatesNewParagraph; */
    pub get_returnInParagraphCreatesNewParagraph: unsafe extern "C" fn (this: *const nsIHTMLEditor, aReturnInParagraphCreatesNewParagraph: *mut bool) -> nsresult,
    pub set_returnInParagraphCreatesNewParagraph: unsafe extern "C" fn (this: *const nsIHTMLEditor, aReturnInParagraphCreatesNewParagraph: bool) -> nsresult,

    /* [noscript,notxpcom] Element GetActiveEditingHost (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetActiveEditingHost: *const ::libc::c_void,

}


impl nsIHTMLEditor {
    /* void addDefaultProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    #[inline]
    pub unsafe fn addDefaultProperty(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16], aValue: &[u16]) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).addDefaultProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDefaultProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    #[inline]
    pub unsafe fn removeDefaultProperty(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16], aValue: &[u16]) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).removeDefaultProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAllDefaultProperties (); */
    #[inline]
    pub unsafe fn removeAllDefaultProperties(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllDefaultProperties)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setInlineProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue); */
    #[inline]
    pub unsafe fn setInlineProperty(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16], aValue: &[u16]) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setInlineProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getInlineProperty (in nsIAtom aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    #[inline]
    pub unsafe fn getInlineProperty(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16], aValue: &[u16]) -> Result<(bool, bool, bool), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        let aValue = nsString::from(aValue);
        let mut aFirst: bool = ::std::mem::zeroed();
        let mut aAny: bool = ::std::mem::zeroed();
        let mut aAll: bool = ::std::mem::zeroed();
        match ((*self.vtable).getInlineProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute, &*aValue, &mut aFirst as *mut _, &mut aAny as *mut _, &mut aAll as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFirst, aAny, aAll))
    }

    /* AString getInlinePropertyWithAttrValue (in nsIAtom aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    #[inline]
    pub unsafe fn getInlinePropertyWithAttrValue(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16], aValue: &[u16]) -> Result<(bool, bool, bool, nsString), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        let aValue = nsString::from(aValue);
        let mut aFirst: bool = ::std::mem::zeroed();
        let mut aAny: bool = ::std::mem::zeroed();
        let mut aAll: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getInlinePropertyWithAttrValue)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute, &*aValue, &mut aFirst as *mut _, &mut aAny as *mut _, &mut aAll as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFirst, aAny, aAll, _retval))
    }

    /* void removeAllInlineProperties (); */
    #[inline]
    pub unsafe fn removeAllInlineProperties(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeAllInlineProperties)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeInlineProperty (in nsIAtom aProperty, in AString aAttribute); */
    #[inline]
    pub unsafe fn removeInlineProperty(&self, aProperty: Option<&nsIAtom>, aAttribute: &[u16]) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        match ((*self.vtable).removeInlineProperty)(self as *const _, aProperty.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void increaseFontSize (); */
    #[inline]
    pub unsafe fn increaseFontSize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).increaseFontSize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void decreaseFontSize (); */
    #[inline]
    pub unsafe fn decreaseFontSize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).decreaseFontSize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean nodeIsBlock (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn nodeIsBlock(&self, node: Option<&nsIDOMNode>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).nodeIsBlock)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void insertHTML (in AString aInputString); */
    #[inline]
    pub unsafe fn insertHTML(&self, aInputString: &[u16]) -> Result<(), nsresult> {
        let aInputString = nsString::from(aInputString);
        match ((*self.vtable).insertHTML)(self as *const _, &*aInputString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pasteNoFormatting (in long aSelectionType); */
    #[inline]
    pub unsafe fn pasteNoFormatting(&self, aSelectionType: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).pasteNoFormatting)(self as *const _, aSelectionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rebuildDocumentFromSource (in AString aSourceString); */
    #[inline]
    pub unsafe fn rebuildDocumentFromSource(&self, aSourceString: &[u16]) -> Result<(), nsresult> {
        let aSourceString = nsString::from(aSourceString);
        match ((*self.vtable).rebuildDocumentFromSource)(self as *const _, &*aSourceString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertHTMLWithContext (in AString aInputString, in AString aContextStr, in AString aInfoStr, in AString aFlavor, in nsIDOMDocument aSourceDoc, in nsIDOMNode aDestinationNode, in long aDestinationOffset, in boolean aDeleteSelection); */
    #[inline]
    pub unsafe fn insertHTMLWithContext(&self, aInputString: &[u16], aContextStr: &[u16], aInfoStr: &[u16], aFlavor: &[u16], aSourceDoc: Option<&nsIDOMDocument>, aDestinationNode: Option<&nsIDOMNode>, aDestinationOffset: libc::int32_t, aDeleteSelection: bool) -> Result<(), nsresult> {
        let aInputString = nsString::from(aInputString);
        let aContextStr = nsString::from(aContextStr);
        let aInfoStr = nsString::from(aInfoStr);
        let aFlavor = nsString::from(aFlavor);
        match ((*self.vtable).insertHTMLWithContext)(self as *const _, &*aInputString, &*aContextStr, &*aInfoStr, &*aFlavor, aSourceDoc.map_or(::std::ptr::null(), |x| x as *const _), aDestinationNode.map_or(::std::ptr::null(), |x| x as *const _), aDestinationOffset, aDeleteSelection) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertElementAtSelection (in nsIDOMElement aElement, in boolean aDeleteSelection); */
    #[inline]
    pub unsafe fn insertElementAtSelection(&self, aElement: Option<&nsIDOMElement>, aDeleteSelection: bool) -> Result<(), nsresult> {

        match ((*self.vtable).insertElementAtSelection)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aDeleteSelection) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateBaseURL (); */
    #[inline]
    pub unsafe fn updateBaseURL(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).updateBaseURL)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectElement (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn selectElement(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).selectElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCaretAfterElement (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn setCaretAfterElement(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).setCaretAfterElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setParagraphFormat (in AString aParagraphFormat); */
    #[inline]
    pub unsafe fn setParagraphFormat(&self, aParagraphFormat: &[u16]) -> Result<(), nsresult> {
        let aParagraphFormat = nsString::from(aParagraphFormat);
        match ((*self.vtable).setParagraphFormat)(self as *const _, &*aParagraphFormat) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString getParagraphState (out boolean aMixed); */
    #[inline]
    pub unsafe fn getParagraphState(&self, ) -> Result<(bool, nsString), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getParagraphState)(self as *const _, &mut aMixed as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, _retval))
    }

    /* AString getFontFaceState (out boolean aMixed); */
    #[inline]
    pub unsafe fn getFontFaceState(&self, ) -> Result<(bool, nsString), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getFontFaceState)(self as *const _, &mut aMixed as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, _retval))
    }

    /* AString getFontColorState (out boolean aMixed); */
    #[inline]
    pub unsafe fn getFontColorState(&self, ) -> Result<(bool, nsString), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getFontColorState)(self as *const _, &mut aMixed as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, _retval))
    }

    /* AString getBackgroundColorState (out boolean aMixed); */
    #[inline]
    pub unsafe fn getBackgroundColorState(&self, ) -> Result<(bool, nsString), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getBackgroundColorState)(self as *const _, &mut aMixed as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, _retval))
    }

    /* AString getHighlightColorState (out boolean aMixed); */
    #[inline]
    pub unsafe fn getHighlightColorState(&self, ) -> Result<(bool, nsString), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getHighlightColorState)(self as *const _, &mut aMixed as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, _retval))
    }

    /* void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL); */
    #[inline]
    pub unsafe fn getListState(&self, ) -> Result<(bool, bool, bool, bool), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut aOL: bool = ::std::mem::zeroed();
        let mut aUL: bool = ::std::mem::zeroed();
        let mut aDL: bool = ::std::mem::zeroed();
        match ((*self.vtable).getListState)(self as *const _, &mut aMixed as *mut _, &mut aOL as *mut _, &mut aUL as *mut _, &mut aDL as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, aOL, aUL, aDL))
    }

    /* void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD); */
    #[inline]
    pub unsafe fn getListItemState(&self, ) -> Result<(bool, bool, bool, bool), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut aLI: bool = ::std::mem::zeroed();
        let mut aDT: bool = ::std::mem::zeroed();
        let mut aDD: bool = ::std::mem::zeroed();
        match ((*self.vtable).getListItemState)(self as *const _, &mut aMixed as *mut _, &mut aLI as *mut _, &mut aDT as *mut _, &mut aDD as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, aLI, aDT, aDD))
    }

    /* void getAlignment (out boolean aMixed, out short aAlign); */
    #[inline]
    pub unsafe fn getAlignment(&self, ) -> Result<(bool, libc::int16_t), nsresult> {
        let mut aMixed: bool = ::std::mem::zeroed();
        let mut aAlign: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getAlignment)(self as *const _, &mut aMixed as *mut _, &mut aAlign as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aMixed, aAlign))
    }

    /* void getIndentState (out boolean aCanIndent, out boolean aCanOutdent); */
    #[inline]
    pub unsafe fn getIndentState(&self, ) -> Result<(bool, bool), nsresult> {
        let mut aCanIndent: bool = ::std::mem::zeroed();
        let mut aCanOutdent: bool = ::std::mem::zeroed();
        match ((*self.vtable).getIndentState)(self as *const _, &mut aCanIndent as *mut _, &mut aCanOutdent as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aCanIndent, aCanOutdent))
    }

    /* void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType); */
    #[inline]
    pub unsafe fn makeOrChangeList(&self, aListType: &[u16], entireList: bool, aBulletType: &[u16]) -> Result<(), nsresult> {
        let aListType = nsString::from(aListType);
        let aBulletType = nsString::from(aBulletType);
        match ((*self.vtable).makeOrChangeList)(self as *const _, &*aListType, entireList, &*aBulletType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeList (in AString aListType); */
    #[inline]
    pub unsafe fn removeList(&self, aListType: &[u16]) -> Result<(), nsresult> {
        let aListType = nsString::from(aListType);
        match ((*self.vtable).removeList)(self as *const _, &*aListType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void indent (in AString aIndent); */
    #[inline]
    pub unsafe fn indent(&self, aIndent: &[u16]) -> Result<(), nsresult> {
        let aIndent = nsString::from(aIndent);
        match ((*self.vtable).indent)(self as *const _, &*aIndent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void align (in AString aAlign); */
    #[inline]
    pub unsafe fn align(&self, aAlign: &[u16]) -> Result<(), nsresult> {
        let aAlign = nsString::from(aAlign);
        match ((*self.vtable).align)(self as *const _, &*aAlign) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement getElementOrParentByTagName (in AString aTagName, in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn getElementOrParentByTagName(&self, aTagName: &[u16], aNode: Option<&nsIDOMNode>) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let aTagName = nsString::from(aTagName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getElementOrParentByTagName)(self as *const _, &*aTagName, aNode.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMElement getSelectedElement (in AString aTagName); */
    #[inline]
    pub unsafe fn getSelectedElement(&self, aTagName: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let aTagName = nsString::from(aTagName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelectedElement)(self as *const _, &*aTagName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AString getHeadContentsAsHTML (); */
    #[inline]
    pub unsafe fn getHeadContentsAsHTML(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getHeadContentsAsHTML)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void replaceHeadContentsWithHTML (in AString aSourceToInsert); */
    #[inline]
    pub unsafe fn replaceHeadContentsWithHTML(&self, aSourceToInsert: &[u16]) -> Result<(), nsresult> {
        let aSourceToInsert = nsString::from(aSourceToInsert);
        match ((*self.vtable).replaceHeadContentsWithHTML)(self as *const _, &*aSourceToInsert) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement createElementWithDefaults (in AString aTagName); */
    #[inline]
    pub unsafe fn createElementWithDefaults(&self, aTagName: &[u16]) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let aTagName = nsString::from(aTagName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createElementWithDefaults)(self as *const _, &*aTagName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void insertLinkAroundSelection (in nsIDOMElement aAnchorElement); */
    #[inline]
    pub unsafe fn insertLinkAroundSelection(&self, aAnchorElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).insertLinkAroundSelection)(self as *const _, aAnchorElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setBackgroundColor (in AString aColor); */
    #[inline]
    pub unsafe fn setBackgroundColor(&self, aColor: &[u16]) -> Result<(), nsresult> {
        let aColor = nsString::from(aColor);
        match ((*self.vtable).setBackgroundColor)(self as *const _, &*aColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setBodyAttribute (in AString aAttr, in AString aValue); */
    #[inline]
    pub unsafe fn setBodyAttribute(&self, aAttr: &[u16], aValue: &[u16]) -> Result<(), nsresult> {
        let aAttr = nsString::from(aAttr);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setBodyAttribute)(self as *const _, &*aAttr, &*aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIArray getLinkedObjects (); */
    #[inline]
    pub unsafe fn getLinkedObjects(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLinkedObjects)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean isCSSEnabled; */
    #[inline]
    pub unsafe fn get_isCSSEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isCSSEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isCSSEnabled(&self, aIsCSSEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isCSSEnabled)(self as *const _, aIsCSSEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addInsertionListener (in nsIContentFilter inFilter); */
    #[inline]
    pub unsafe fn addInsertionListener(&self, inFilter: Option<&nsIContentFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).addInsertionListener)(self as *const _, inFilter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeInsertionListener (in nsIContentFilter inFilter); */
    #[inline]
    pub unsafe fn removeInsertionListener(&self, inFilter: Option<&nsIContentFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).removeInsertionListener)(self as *const _, inFilter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement getSelectionContainer (); */
    #[inline]
    pub unsafe fn getSelectionContainer(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelectionContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void checkSelectionStateForAnonymousButtons (in nsISelection aSelection); */
    #[inline]
    pub unsafe fn checkSelectionStateForAnonymousButtons(&self, aSelection: Option<&nsISelection>) -> Result<(), nsresult> {

        match ((*self.vtable).checkSelectionStateForAnonymousButtons)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isAnonymousElement (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn isAnonymousElement(&self, aElement: Option<&nsIDOMElement>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAnonymousElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean returnInParagraphCreatesNewParagraph; */
    #[inline]
    pub unsafe fn get_returnInParagraphCreatesNewParagraph(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_returnInParagraphCreatesNewParagraph)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_returnInParagraphCreatesNewParagraph(&self, aReturnInParagraphCreatesNewParagraph: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_returnInParagraphCreatesNewParagraph)(self as *const _, aReturnInParagraphCreatesNewParagraph) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] Element GetActiveEditingHost (); */


}


