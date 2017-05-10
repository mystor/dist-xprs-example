//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditor.idl
//


pub mod nsIEditor_consts {
    pub const eNone: i64 = 0;
    pub const eNext: i64 = 1;
    pub const ePrevious: i64 = 2;
    pub const eNextWord: i64 = 3;
    pub const ePreviousWord: i64 = 4;
    pub const eToBeginningOfLine: i64 = 5;
    pub const eToEndOfLine: i64 = 6;
    pub const eStrip: i64 = 0;
    pub const eNoStrip: i64 = 1;
}


#[repr(C)]
pub struct nsIEditor {
    vtable: *const nsIEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x094be624, 0xf0bf, 0x400f,
            [0x89, 0xe2, 0x6a, 0x84, 0xba, 0xab, 0x94, 0x74])
    }
}

unsafe impl RefCounted for nsIEditor {
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
pub trait nsIEditorCoerce {
    fn coerce_from(v: &nsIEditor) -> &Self;
}

impl nsIEditorCoerce for nsIEditor {
    #[inline]
    fn coerce_from(v: &nsIEditor) -> &Self {
        v
    }
}

impl nsIEditor {
    #[inline]
    pub fn coerce<T: nsIEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISelection selection; */
    pub get_selection: unsafe extern "C" fn (this: *const nsIEditor, aSelection: *mut *const nsISelection) -> nsresult,

    /* [noscript] void finalizeSelection (); */
    pub finalizeSelection: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* [noscript] void init (in nsIDOMDocument doc, in nsIContent aRoot, in nsISelectionController aSelCon, in unsigned long aFlags, in AString initialValue); */
    pub init: unsafe extern "C" fn (this: *const nsIEditor, doc: *const nsIDOMDocument, aRoot: *const nsIContent, aSelCon: *const nsISelectionController, aFlags: libc::uint32_t, initialValue: *const nsAString) -> nsresult,

    /* void setAttributeOrEquivalent (in nsIDOMElement element, in AString sourceAttrName, in AString sourceAttrValue, in boolean aSuppressTransaction); */
    pub setAttributeOrEquivalent: unsafe extern "C" fn (this: *const nsIEditor, element: *const nsIDOMElement, sourceAttrName: *const nsAString, sourceAttrValue: *const nsAString, aSuppressTransaction: bool) -> nsresult,

    /* void removeAttributeOrEquivalent (in nsIDOMElement element, in DOMString sourceAttrName, in boolean aSuppressTransaction); */
    pub removeAttributeOrEquivalent: unsafe extern "C" fn (this: *const nsIEditor, element: *const nsIDOMElement, sourceAttrName: *const nsAString, aSuppressTransaction: bool) -> nsresult,

    /* void postCreate (); */
    pub postCreate: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void preDestroy (in boolean aDestroyingFrames); */
    pub preDestroy: unsafe extern "C" fn (this: *const nsIEditor, aDestroyingFrames: bool) -> nsresult,

    /* attribute unsigned long flags; */
    pub get_flags: unsafe extern "C" fn (this: *const nsIEditor, aFlags: *mut libc::uint32_t) -> nsresult,
    pub set_flags: unsafe extern "C" fn (this: *const nsIEditor, aFlags: libc::uint32_t) -> nsresult,

    /* attribute string contentsMIMEType; */
    pub get_contentsMIMEType: unsafe extern "C" fn (this: *const nsIEditor, aContentsMIMEType: *mut *const libc::c_char) -> nsresult,
    pub set_contentsMIMEType: unsafe extern "C" fn (this: *const nsIEditor, aContentsMIMEType: *const libc::c_char) -> nsresult,

    /* readonly attribute boolean isDocumentEditable; */
    pub get_isDocumentEditable: unsafe extern "C" fn (this: *const nsIEditor, aIsDocumentEditable: *mut bool) -> nsresult,

    /* readonly attribute boolean isSelectionEditable; */
    pub get_isSelectionEditable: unsafe extern "C" fn (this: *const nsIEditor, aIsSelectionEditable: *mut bool) -> nsresult,

    /* readonly attribute nsIDOMDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const nsIEditor, aDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute nsIDOMElement rootElement; */
    pub get_rootElement: unsafe extern "C" fn (this: *const nsIEditor, aRootElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsISelectionController selectionController; */
    pub get_selectionController: unsafe extern "C" fn (this: *const nsIEditor, aSelectionController: *mut *const nsISelectionController) -> nsresult,

    /* void deleteSelection (in short action, in short stripWrappers); */
    pub deleteSelection: unsafe extern "C" fn (this: *const nsIEditor, action: libc::int16_t, stripWrappers: libc::int16_t) -> nsresult,

    /* readonly attribute boolean documentIsEmpty; */
    pub get_documentIsEmpty: unsafe extern "C" fn (this: *const nsIEditor, aDocumentIsEmpty: *mut bool) -> nsresult,

    /* readonly attribute boolean documentModified; */
    pub get_documentModified: unsafe extern "C" fn (this: *const nsIEditor, aDocumentModified: *mut bool) -> nsresult,

    /* attribute ACString documentCharacterSet; */
    pub get_documentCharacterSet: unsafe extern "C" fn (this: *const nsIEditor, aDocumentCharacterSet: *mut nsACString) -> nsresult,
    pub set_documentCharacterSet: unsafe extern "C" fn (this: *const nsIEditor, aDocumentCharacterSet: *const nsACString) -> nsresult,

    /* void resetModificationCount (); */
    pub resetModificationCount: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* long getModificationCount (); */
    pub getModificationCount: unsafe extern "C" fn (this: *const nsIEditor, _retval: *mut libc::int32_t) -> nsresult,

    /* void incrementModificationCount (in long aModCount); */
    pub incrementModificationCount: unsafe extern "C" fn (this: *const nsIEditor, aModCount: libc::int32_t) -> nsresult,

    /* readonly attribute nsITransactionManager transactionManager; */
    pub get_transactionManager: unsafe extern "C" fn (this: *const nsIEditor, aTransactionManager: *mut *const nsITransactionManager) -> nsresult,

    /* void doTransaction (in nsITransaction txn); */
    pub doTransaction: unsafe extern "C" fn (this: *const nsIEditor, txn: *const nsITransaction) -> nsresult,

    /* void enableUndo (in boolean enable); */
    pub enableUndo: unsafe extern "C" fn (this: *const nsIEditor, enable: bool) -> nsresult,

    /* readonly attribute long numberOfUndoItems; */
    pub get_numberOfUndoItems: unsafe extern "C" fn (this: *const nsIEditor, aNumberOfUndoItems: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long numberOfRedoItems; */
    pub get_numberOfRedoItems: unsafe extern "C" fn (this: *const nsIEditor, aNumberOfRedoItems: *mut libc::int32_t) -> nsresult,

    /* void undo (in unsigned long count); */
    pub undo: unsafe extern "C" fn (this: *const nsIEditor, count: libc::uint32_t) -> nsresult,

    /* void canUndo (out boolean isEnabled, out boolean canUndo); */
    pub canUndo: unsafe extern "C" fn (this: *const nsIEditor, isEnabled: *mut bool, canUndo: *mut bool) -> nsresult,

    /* void redo (in unsigned long count); */
    pub redo: unsafe extern "C" fn (this: *const nsIEditor, count: libc::uint32_t) -> nsresult,

    /* void canRedo (out boolean isEnabled, out boolean canRedo); */
    pub canRedo: unsafe extern "C" fn (this: *const nsIEditor, isEnabled: *mut bool, canRedo: *mut bool) -> nsresult,

    /* void beginTransaction (); */
    pub beginTransaction: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void endTransaction (); */
    pub endTransaction: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void beginPlaceHolderTransaction (in nsIAtom name); */
    pub beginPlaceHolderTransaction: unsafe extern "C" fn (this: *const nsIEditor, name: *const nsIAtom) -> nsresult,

    /* void endPlaceHolderTransaction (); */
    pub endPlaceHolderTransaction: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* boolean shouldTxnSetSelection (); */
    pub shouldTxnSetSelection: unsafe extern "C" fn (this: *const nsIEditor, _retval: *mut bool) -> nsresult,

    /* void setShouldTxnSetSelection (in boolean should); */
    pub setShouldTxnSetSelection: unsafe extern "C" fn (this: *const nsIEditor, should: bool) -> nsresult,

    /* nsIInlineSpellChecker getInlineSpellChecker (in boolean autoCreate); */
    pub getInlineSpellChecker: unsafe extern "C" fn (this: *const nsIEditor, autoCreate: bool, _retval: *mut *const nsIInlineSpellChecker) -> nsresult,

    /* void syncRealTimeSpell (); */
    pub syncRealTimeSpell: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void setSpellcheckUserOverride (in boolean enable); */
    pub setSpellcheckUserOverride: unsafe extern "C" fn (this: *const nsIEditor, enable: bool) -> nsresult,

    /* void cut (); */
    pub cut: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* boolean canCut (); */
    pub canCut: unsafe extern "C" fn (this: *const nsIEditor, _retval: *mut bool) -> nsresult,

    /* void copy (); */
    pub copy: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* boolean canCopy (); */
    pub canCopy: unsafe extern "C" fn (this: *const nsIEditor, _retval: *mut bool) -> nsresult,

    /* boolean canDelete (); */
    pub canDelete: unsafe extern "C" fn (this: *const nsIEditor, _retval: *mut bool) -> nsresult,

    /* void paste (in long aSelectionType); */
    pub paste: unsafe extern "C" fn (this: *const nsIEditor, aSelectionType: libc::int32_t) -> nsresult,

    /* void pasteTransferable (in nsITransferable aTransferable); */
    pub pasteTransferable: unsafe extern "C" fn (this: *const nsIEditor, aTransferable: *const nsITransferable) -> nsresult,

    /* boolean canPaste (in long aSelectionType); */
    pub canPaste: unsafe extern "C" fn (this: *const nsIEditor, aSelectionType: libc::int32_t, _retval: *mut bool) -> nsresult,

    /* boolean canPasteTransferable ([optional] in nsITransferable aTransferable); */
    pub canPasteTransferable: unsafe extern "C" fn (this: *const nsIEditor, aTransferable: *const nsITransferable, _retval: *mut bool) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void beginningOfDocument (); */
    pub beginningOfDocument: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void endOfDocument (); */
    pub endOfDocument: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void setAttribute (in nsIDOMElement aElement, in AString attributestr, in AString attvalue); */
    pub setAttribute: unsafe extern "C" fn (this: *const nsIEditor, aElement: *const nsIDOMElement, attributestr: *const nsAString, attvalue: *const nsAString) -> nsresult,

    /* boolean getAttributeValue (in nsIDOMElement aElement, in AString attributestr, out AString resultValue); */
    pub getAttributeValue: unsafe extern "C" fn (this: *const nsIEditor, aElement: *const nsIDOMElement, attributestr: *const nsAString, resultValue: *mut nsAString, _retval: *mut bool) -> nsresult,

    /* void removeAttribute (in nsIDOMElement aElement, in AString aAttribute); */
    pub removeAttribute: unsafe extern "C" fn (this: *const nsIEditor, aElement: *const nsIDOMElement, aAttribute: *const nsAString) -> nsresult,

    /* void cloneAttribute (in AString aAttribute, in nsIDOMNode aDestNode, in nsIDOMNode aSourceNode); */
    pub cloneAttribute: unsafe extern "C" fn (this: *const nsIEditor, aAttribute: *const nsAString, aDestNode: *const nsIDOMNode, aSourceNode: *const nsIDOMNode) -> nsresult,

    /* void cloneAttributes (in nsIDOMNode destNode, in nsIDOMNode sourceNode); */
    pub cloneAttributes: unsafe extern "C" fn (this: *const nsIEditor, destNode: *const nsIDOMNode, sourceNode: *const nsIDOMNode) -> nsresult,

    /* nsIDOMNode createNode (in AString tag, in nsIDOMNode parent, in long position); */
    pub createNode: unsafe extern "C" fn (this: *const nsIEditor, tag: *const nsAString, parent: *const nsIDOMNode, position: libc::int32_t, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* void insertNode (in nsIDOMNode node, in nsIDOMNode parent, in long aPosition); */
    pub insertNode: unsafe extern "C" fn (this: *const nsIEditor, node: *const nsIDOMNode, parent: *const nsIDOMNode, aPosition: libc::int32_t) -> nsresult,

    /* void splitNode (in nsIDOMNode existingRightNode, in long offset, out nsIDOMNode newLeftNode); */
    pub splitNode: unsafe extern "C" fn (this: *const nsIEditor, existingRightNode: *const nsIDOMNode, offset: libc::int32_t, newLeftNode: *mut *const nsIDOMNode) -> nsresult,

    /* void joinNodes (in nsIDOMNode leftNode, in nsIDOMNode rightNode, in nsIDOMNode parent); */
    pub joinNodes: unsafe extern "C" fn (this: *const nsIEditor, leftNode: *const nsIDOMNode, rightNode: *const nsIDOMNode, parent: *const nsIDOMNode) -> nsresult,

    /* void deleteNode (in nsIDOMNode child); */
    pub deleteNode: unsafe extern "C" fn (this: *const nsIEditor, child: *const nsIDOMNode) -> nsresult,

    /* [notxpcom] boolean outputsMozDirty (); */
    pub outputsMozDirty: unsafe extern "C" fn (this: *const nsIEditor) -> bool,

    /* void markNodeDirty (in nsIDOMNode node); */
    pub markNodeDirty: unsafe extern "C" fn (this: *const nsIEditor, node: *const nsIDOMNode) -> nsresult,

    /* void switchTextDirection (); */
    pub switchTextDirection: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* AString outputToString (in AString formatType, in unsigned long flags); */
    pub outputToString: unsafe extern "C" fn (this: *const nsIEditor, formatType: *const nsAString, flags: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* void outputToStream (in nsIOutputStream aStream, in AString formatType, in ACString charsetOverride, in unsigned long flags); */
    pub outputToStream: unsafe extern "C" fn (this: *const nsIEditor, aStream: *const nsIOutputStream, formatType: *const nsAString, charsetOverride: *const nsACString, flags: libc::uint32_t) -> nsresult,

    /* void addEditorObserver (in nsIEditorObserver observer); */
    pub addEditorObserver: unsafe extern "C" fn (this: *const nsIEditor, observer: *const nsIEditorObserver) -> nsresult,

    /* void removeEditorObserver (in nsIEditorObserver observer); */
    pub removeEditorObserver: unsafe extern "C" fn (this: *const nsIEditor, observer: *const nsIEditorObserver) -> nsresult,

    /* void addEditActionListener (in nsIEditActionListener listener); */
    pub addEditActionListener: unsafe extern "C" fn (this: *const nsIEditor, listener: *const nsIEditActionListener) -> nsresult,

    /* void removeEditActionListener (in nsIEditActionListener listener); */
    pub removeEditActionListener: unsafe extern "C" fn (this: *const nsIEditor, listener: *const nsIEditActionListener) -> nsresult,

    /* void addDocumentStateListener (in nsIDocumentStateListener listener); */
    pub addDocumentStateListener: unsafe extern "C" fn (this: *const nsIEditor, listener: *const nsIDocumentStateListener) -> nsresult,

    /* void removeDocumentStateListener (in nsIDocumentStateListener listener); */
    pub removeDocumentStateListener: unsafe extern "C" fn (this: *const nsIEditor, listener: *const nsIDocumentStateListener) -> nsresult,

    /* void dumpContentTree (); */
    pub dumpContentTree: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void debugDumpContent (); */
    pub debugDumpContent: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* void debugUnitTests (out long outNumTests, out long outNumTestsFailed); */
    pub debugUnitTests: unsafe extern "C" fn (this: *const nsIEditor, outNumTests: *mut libc::int32_t, outNumTestsFailed: *mut libc::int32_t) -> nsresult,

    /* [notxpcom] boolean isModifiableNode (in nsIDOMNode aNode); */
    pub isModifiableNode: unsafe extern "C" fn (this: *const nsIEditor, aNode: *const nsIDOMNode) -> bool,

    /* attribute boolean suppressDispatchingInputEvent; */
    pub get_suppressDispatchingInputEvent: unsafe extern "C" fn (this: *const nsIEditor, aSuppressDispatchingInputEvent: *mut bool) -> nsresult,
    pub set_suppressDispatchingInputEvent: unsafe extern "C" fn (this: *const nsIEditor, aSuppressDispatchingInputEvent: bool) -> nsresult,

    /* [noscript] readonly attribute boolean isInEditAction; */
    pub get_isInEditAction: unsafe extern "C" fn (this: *const nsIEditor, aIsInEditAction: *mut bool) -> nsresult,

    /* void forceCompositionEnd (); */
    pub forceCompositionEnd: unsafe extern "C" fn (this: *const nsIEditor) -> nsresult,

    /* [noscript] IMEState getPreferredIMEState (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPreferredIMEState: *const ::libc::c_void,

    /* readonly attribute boolean composing; */
    pub get_composing: unsafe extern "C" fn (this: *const nsIEditor, aComposing: *mut bool) -> nsresult,

}


impl nsIEditor {
    /* readonly attribute nsISelection selection; */
    #[inline]
    pub unsafe fn get_selection(&self, ) -> Result<Option<RefPtr<nsISelection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selection)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void finalizeSelection (); */
    #[inline]
    pub unsafe fn finalizeSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finalizeSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void init (in nsIDOMDocument doc, in nsIContent aRoot, in nsISelectionController aSelCon, in unsigned long aFlags, in AString initialValue); */
    #[inline]
    pub unsafe fn init(&self, doc: Option<&nsIDOMDocument>, aRoot: Option<&nsIContent>, aSelCon: Option<&nsISelectionController>, aFlags: libc::uint32_t, initialValue: &[u16]) -> Result<(), nsresult> {
        let initialValue = nsString::from(initialValue);
        match ((*self.vtable).init)(self as *const _, doc.map_or(::std::ptr::null(), |x| x as *const _), aRoot.map_or(::std::ptr::null(), |x| x as *const _), aSelCon.map_or(::std::ptr::null(), |x| x as *const _), aFlags, &*initialValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAttributeOrEquivalent (in nsIDOMElement element, in AString sourceAttrName, in AString sourceAttrValue, in boolean aSuppressTransaction); */
    #[inline]
    pub unsafe fn setAttributeOrEquivalent(&self, element: Option<&nsIDOMElement>, sourceAttrName: &[u16], sourceAttrValue: &[u16], aSuppressTransaction: bool) -> Result<(), nsresult> {
        let sourceAttrName = nsString::from(sourceAttrName);
        let sourceAttrValue = nsString::from(sourceAttrValue);
        match ((*self.vtable).setAttributeOrEquivalent)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), &*sourceAttrName, &*sourceAttrValue, aSuppressTransaction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeAttributeOrEquivalent (in nsIDOMElement element, in DOMString sourceAttrName, in boolean aSuppressTransaction); */
    #[inline]
    pub unsafe fn removeAttributeOrEquivalent(&self, element: Option<&nsIDOMElement>, sourceAttrName: &[u16], aSuppressTransaction: bool) -> Result<(), nsresult> {
        let sourceAttrName = nsString::from(sourceAttrName);
        match ((*self.vtable).removeAttributeOrEquivalent)(self as *const _, element.map_or(::std::ptr::null(), |x| x as *const _), &*sourceAttrName, aSuppressTransaction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void postCreate (); */
    #[inline]
    pub unsafe fn postCreate(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).postCreate)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void preDestroy (in boolean aDestroyingFrames); */
    #[inline]
    pub unsafe fn preDestroy(&self, aDestroyingFrames: bool) -> Result<(), nsresult> {

        match ((*self.vtable).preDestroy)(self as *const _, aDestroyingFrames) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long flags; */
    #[inline]
    pub unsafe fn get_flags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_flags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_flags(&self, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_flags)(self as *const _, aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute string contentsMIMEType; */
    #[inline]
    pub unsafe fn get_contentsMIMEType(&self, ) -> Result<*const libc::c_char, nsresult> {
        let mut _retval: *const libc::c_char = ::std::mem::zeroed();
        match ((*self.vtable).get_contentsMIMEType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentsMIMEType(&self, aContentsMIMEType: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentsMIMEType)(self as *const _, aContentsMIMEType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isDocumentEditable; */
    #[inline]
    pub unsafe fn get_isDocumentEditable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDocumentEditable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSelectionEditable; */
    #[inline]
    pub unsafe fn get_isSelectionEditable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSelectionEditable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDOMDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMElement rootElement; */
    #[inline]
    pub unsafe fn get_rootElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_rootElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISelectionController selectionController; */
    #[inline]
    pub unsafe fn get_selectionController(&self, ) -> Result<Option<RefPtr<nsISelectionController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectionController)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void deleteSelection (in short action, in short stripWrappers); */
    #[inline]
    pub unsafe fn deleteSelection(&self, action: libc::int16_t, stripWrappers: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).deleteSelection)(self as *const _, action, stripWrappers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean documentIsEmpty; */
    #[inline]
    pub unsafe fn get_documentIsEmpty(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_documentIsEmpty)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean documentModified; */
    #[inline]
    pub unsafe fn get_documentModified(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_documentModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute ACString documentCharacterSet; */
    #[inline]
    pub unsafe fn get_documentCharacterSet(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_documentCharacterSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_documentCharacterSet(&self, aDocumentCharacterSet: &[u8]) -> Result<(), nsresult> {
        let aDocumentCharacterSet = nsCString::from(aDocumentCharacterSet);
        match ((*self.vtable).set_documentCharacterSet)(self as *const _, &*aDocumentCharacterSet) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resetModificationCount (); */
    #[inline]
    pub unsafe fn resetModificationCount(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetModificationCount)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* long getModificationCount (); */
    #[inline]
    pub unsafe fn getModificationCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getModificationCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void incrementModificationCount (in long aModCount); */
    #[inline]
    pub unsafe fn incrementModificationCount(&self, aModCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).incrementModificationCount)(self as *const _, aModCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsITransactionManager transactionManager; */
    #[inline]
    pub unsafe fn get_transactionManager(&self, ) -> Result<Option<RefPtr<nsITransactionManager>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_transactionManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void doTransaction (in nsITransaction txn); */
    #[inline]
    pub unsafe fn doTransaction(&self, txn: Option<&nsITransaction>) -> Result<(), nsresult> {

        match ((*self.vtable).doTransaction)(self as *const _, txn.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void enableUndo (in boolean enable); */
    #[inline]
    pub unsafe fn enableUndo(&self, enable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).enableUndo)(self as *const _, enable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long numberOfUndoItems; */
    #[inline]
    pub unsafe fn get_numberOfUndoItems(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numberOfUndoItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long numberOfRedoItems; */
    #[inline]
    pub unsafe fn get_numberOfRedoItems(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_numberOfRedoItems)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void undo (in unsigned long count); */
    #[inline]
    pub unsafe fn undo(&self, count: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).undo)(self as *const _, count) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void canUndo (out boolean isEnabled, out boolean canUndo); */
    #[inline]
    pub unsafe fn canUndo(&self, ) -> Result<(bool, bool), nsresult> {
        let mut isEnabled: bool = ::std::mem::zeroed();
        let mut canUndo: bool = ::std::mem::zeroed();
        match ((*self.vtable).canUndo)(self as *const _, &mut isEnabled as *mut _, &mut canUndo as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((isEnabled, canUndo))
    }

    /* void redo (in unsigned long count); */
    #[inline]
    pub unsafe fn redo(&self, count: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).redo)(self as *const _, count) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void canRedo (out boolean isEnabled, out boolean canRedo); */
    #[inline]
    pub unsafe fn canRedo(&self, ) -> Result<(bool, bool), nsresult> {
        let mut isEnabled: bool = ::std::mem::zeroed();
        let mut canRedo: bool = ::std::mem::zeroed();
        match ((*self.vtable).canRedo)(self as *const _, &mut isEnabled as *mut _, &mut canRedo as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((isEnabled, canRedo))
    }

    /* void beginTransaction (); */
    #[inline]
    pub unsafe fn beginTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endTransaction (); */
    #[inline]
    pub unsafe fn endTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginPlaceHolderTransaction (in nsIAtom name); */
    #[inline]
    pub unsafe fn beginPlaceHolderTransaction(&self, name: Option<&nsIAtom>) -> Result<(), nsresult> {

        match ((*self.vtable).beginPlaceHolderTransaction)(self as *const _, name.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endPlaceHolderTransaction (); */
    #[inline]
    pub unsafe fn endPlaceHolderTransaction(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endPlaceHolderTransaction)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean shouldTxnSetSelection (); */
    #[inline]
    pub unsafe fn shouldTxnSetSelection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldTxnSetSelection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setShouldTxnSetSelection (in boolean should); */
    #[inline]
    pub unsafe fn setShouldTxnSetSelection(&self, should: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setShouldTxnSetSelection)(self as *const _, should) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIInlineSpellChecker getInlineSpellChecker (in boolean autoCreate); */
    #[inline]
    pub unsafe fn getInlineSpellChecker(&self, autoCreate: bool) -> Result<Option<RefPtr<nsIInlineSpellChecker>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getInlineSpellChecker)(self as *const _, autoCreate, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void syncRealTimeSpell (); */
    #[inline]
    pub unsafe fn syncRealTimeSpell(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).syncRealTimeSpell)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSpellcheckUserOverride (in boolean enable); */
    #[inline]
    pub unsafe fn setSpellcheckUserOverride(&self, enable: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSpellcheckUserOverride)(self as *const _, enable) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cut (); */
    #[inline]
    pub unsafe fn cut(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cut)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canCut (); */
    #[inline]
    pub unsafe fn canCut(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCut)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void copy (); */
    #[inline]
    pub unsafe fn copy(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).copy)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canCopy (); */
    #[inline]
    pub unsafe fn canCopy(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canCopy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canDelete (); */
    #[inline]
    pub unsafe fn canDelete(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canDelete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void paste (in long aSelectionType); */
    #[inline]
    pub unsafe fn paste(&self, aSelectionType: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).paste)(self as *const _, aSelectionType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pasteTransferable (in nsITransferable aTransferable); */
    #[inline]
    pub unsafe fn pasteTransferable(&self, aTransferable: Option<&nsITransferable>) -> Result<(), nsresult> {

        match ((*self.vtable).pasteTransferable)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean canPaste (in long aSelectionType); */
    #[inline]
    pub unsafe fn canPaste(&self, aSelectionType: libc::int32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canPaste)(self as *const _, aSelectionType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canPasteTransferable ([optional] in nsITransferable aTransferable); */
    #[inline]
    pub unsafe fn canPasteTransferable(&self, aTransferable: Option<&nsITransferable>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canPasteTransferable)(self as *const _, aTransferable.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void selectAll (); */
    #[inline]
    pub unsafe fn selectAll(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).selectAll)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginningOfDocument (); */
    #[inline]
    pub unsafe fn beginningOfDocument(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).beginningOfDocument)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void endOfDocument (); */
    #[inline]
    pub unsafe fn endOfDocument(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endOfDocument)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAttribute (in nsIDOMElement aElement, in AString attributestr, in AString attvalue); */
    #[inline]
    pub unsafe fn setAttribute(&self, aElement: Option<&nsIDOMElement>, attributestr: &[u16], attvalue: &[u16]) -> Result<(), nsresult> {
        let attributestr = nsString::from(attributestr);
        let attvalue = nsString::from(attvalue);
        match ((*self.vtable).setAttribute)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*attributestr, &*attvalue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getAttributeValue (in nsIDOMElement aElement, in AString attributestr, out AString resultValue); */
    #[inline]
    pub unsafe fn getAttributeValue(&self, aElement: Option<&nsIDOMElement>, attributestr: &[u16]) -> Result<(nsString, bool), nsresult> {
        let attributestr = nsString::from(attributestr);
        let mut resultValue = nsString::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getAttributeValue)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*attributestr, &mut *resultValue, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((resultValue, _retval))
    }

    /* void removeAttribute (in nsIDOMElement aElement, in AString aAttribute); */
    #[inline]
    pub unsafe fn removeAttribute(&self, aElement: Option<&nsIDOMElement>, aAttribute: &[u16]) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        match ((*self.vtable).removeAttribute)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aAttribute) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cloneAttribute (in AString aAttribute, in nsIDOMNode aDestNode, in nsIDOMNode aSourceNode); */
    #[inline]
    pub unsafe fn cloneAttribute(&self, aAttribute: &[u16], aDestNode: Option<&nsIDOMNode>, aSourceNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {
        let aAttribute = nsString::from(aAttribute);
        match ((*self.vtable).cloneAttribute)(self as *const _, &*aAttribute, aDestNode.map_or(::std::ptr::null(), |x| x as *const _), aSourceNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cloneAttributes (in nsIDOMNode destNode, in nsIDOMNode sourceNode); */
    #[inline]
    pub unsafe fn cloneAttributes(&self, destNode: Option<&nsIDOMNode>, sourceNode: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).cloneAttributes)(self as *const _, destNode.map_or(::std::ptr::null(), |x| x as *const _), sourceNode.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMNode createNode (in AString tag, in nsIDOMNode parent, in long position); */
    #[inline]
    pub unsafe fn createNode(&self, tag: &[u16], parent: Option<&nsIDOMNode>, position: libc::int32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let tag = nsString::from(tag);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createNode)(self as *const _, &*tag, parent.map_or(::std::ptr::null(), |x| x as *const _), position, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void insertNode (in nsIDOMNode node, in nsIDOMNode parent, in long aPosition); */
    #[inline]
    pub unsafe fn insertNode(&self, node: Option<&nsIDOMNode>, parent: Option<&nsIDOMNode>, aPosition: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).insertNode)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), parent.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void splitNode (in nsIDOMNode existingRightNode, in long offset, out nsIDOMNode newLeftNode); */
    #[inline]
    pub unsafe fn splitNode(&self, existingRightNode: Option<&nsIDOMNode>, offset: libc::int32_t) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut newLeftNode = GetterAddrefs::new();
        match ((*self.vtable).splitNode)(self as *const _, existingRightNode.map_or(::std::ptr::null(), |x| x as *const _), offset, newLeftNode.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(newLeftNode.refptr())
    }

    /* void joinNodes (in nsIDOMNode leftNode, in nsIDOMNode rightNode, in nsIDOMNode parent); */
    #[inline]
    pub unsafe fn joinNodes(&self, leftNode: Option<&nsIDOMNode>, rightNode: Option<&nsIDOMNode>, parent: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).joinNodes)(self as *const _, leftNode.map_or(::std::ptr::null(), |x| x as *const _), rightNode.map_or(::std::ptr::null(), |x| x as *const _), parent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteNode (in nsIDOMNode child); */
    #[inline]
    pub unsafe fn deleteNode(&self, child: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).deleteNode)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] boolean outputsMozDirty (); */
    #[inline]
    pub unsafe fn outputsMozDirty(&self, ) -> bool {

        let _retval = ((*self.vtable).outputsMozDirty)(self as *const _, );
        _retval
    }

    /* void markNodeDirty (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn markNodeDirty(&self, node: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).markNodeDirty)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void switchTextDirection (); */
    #[inline]
    pub unsafe fn switchTextDirection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).switchTextDirection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString outputToString (in AString formatType, in unsigned long flags); */
    #[inline]
    pub unsafe fn outputToString(&self, formatType: &[u16], flags: libc::uint32_t) -> Result<nsString, nsresult> {
        let formatType = nsString::from(formatType);
        let mut _retval = nsString::new();
        match ((*self.vtable).outputToString)(self as *const _, &*formatType, flags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void outputToStream (in nsIOutputStream aStream, in AString formatType, in ACString charsetOverride, in unsigned long flags); */
    #[inline]
    pub unsafe fn outputToStream(&self, aStream: Option<&nsIOutputStream>, formatType: &[u16], charsetOverride: &[u8], flags: libc::uint32_t) -> Result<(), nsresult> {
        let formatType = nsString::from(formatType);
        let charsetOverride = nsCString::from(charsetOverride);
        match ((*self.vtable).outputToStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), &*formatType, &*charsetOverride, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEditorObserver (in nsIEditorObserver observer); */
    #[inline]
    pub unsafe fn addEditorObserver(&self, observer: Option<&nsIEditorObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addEditorObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeEditorObserver (in nsIEditorObserver observer); */
    #[inline]
    pub unsafe fn removeEditorObserver(&self, observer: Option<&nsIEditorObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeEditorObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEditActionListener (in nsIEditActionListener listener); */
    #[inline]
    pub unsafe fn addEditActionListener(&self, listener: Option<&nsIEditActionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addEditActionListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeEditActionListener (in nsIEditActionListener listener); */
    #[inline]
    pub unsafe fn removeEditActionListener(&self, listener: Option<&nsIEditActionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeEditActionListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addDocumentStateListener (in nsIDocumentStateListener listener); */
    #[inline]
    pub unsafe fn addDocumentStateListener(&self, listener: Option<&nsIDocumentStateListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addDocumentStateListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDocumentStateListener (in nsIDocumentStateListener listener); */
    #[inline]
    pub unsafe fn removeDocumentStateListener(&self, listener: Option<&nsIDocumentStateListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeDocumentStateListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void dumpContentTree (); */
    #[inline]
    pub unsafe fn dumpContentTree(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dumpContentTree)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void debugDumpContent (); */
    #[inline]
    pub unsafe fn debugDumpContent(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).debugDumpContent)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void debugUnitTests (out long outNumTests, out long outNumTestsFailed); */
    #[inline]
    pub unsafe fn debugUnitTests(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut outNumTests: libc::int32_t = ::std::mem::zeroed();
        let mut outNumTestsFailed: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).debugUnitTests)(self as *const _, &mut outNumTests as *mut _, &mut outNumTestsFailed as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((outNumTests, outNumTestsFailed))
    }

    /* [notxpcom] boolean isModifiableNode (in nsIDOMNode aNode); */
    #[inline]
    pub unsafe fn isModifiableNode(&self, aNode: Option<&nsIDOMNode>) -> bool {

        let _retval = ((*self.vtable).isModifiableNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _));
        _retval
    }

    /* attribute boolean suppressDispatchingInputEvent; */
    #[inline]
    pub unsafe fn get_suppressDispatchingInputEvent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_suppressDispatchingInputEvent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_suppressDispatchingInputEvent(&self, aSuppressDispatchingInputEvent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_suppressDispatchingInputEvent)(self as *const _, aSuppressDispatchingInputEvent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute boolean isInEditAction; */
    #[inline]
    pub unsafe fn get_isInEditAction(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInEditAction)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void forceCompositionEnd (); */
    #[inline]
    pub unsafe fn forceCompositionEnd(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceCompositionEnd)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] IMEState getPreferredIMEState (); */


    /* readonly attribute boolean composing; */
    #[inline]
    pub unsafe fn get_composing(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_composing)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


