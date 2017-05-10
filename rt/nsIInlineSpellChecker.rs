//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIInlineSpellChecker.idl
//


#[repr(C)]
pub struct nsIInlineSpellChecker {
    vtable: *const nsIInlineSpellCheckerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInlineSpellChecker {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb7b7a77c, 0x40c4, 0x4196,
            [0xb0, 0xb7, 0xb0, 0x33, 0x82, 0x43, 0xb3, 0xfe])
    }
}

unsafe impl RefCounted for nsIInlineSpellChecker {
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
pub trait nsIInlineSpellCheckerCoerce {
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self;
}

impl nsIInlineSpellCheckerCoerce for nsIInlineSpellChecker {
    #[inline]
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self {
        v
    }
}

impl nsIInlineSpellChecker {
    #[inline]
    pub fn coerce<T: nsIInlineSpellCheckerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInlineSpellChecker {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInlineSpellCheckerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInlineSpellCheckerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIEditorSpellCheck spellChecker; */
    pub get_spellChecker: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aSpellChecker: *mut *const nsIEditorSpellCheck) -> nsresult,

    /* void init (in nsIEditor aEditor); */
    pub init: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aEditor: *const nsIEditor) -> nsresult,

    /* void cleanup (in boolean aDestroyingFrames); */
    pub cleanup: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aDestroyingFrames: bool) -> nsresult,

    /* attribute boolean enableRealTimeSpell; */
    pub get_enableRealTimeSpell: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aEnableRealTimeSpell: *mut bool) -> nsresult,
    pub set_enableRealTimeSpell: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aEnableRealTimeSpell: bool) -> nsresult,

    /* void spellCheckAfterEditorChange (in long aAction, in nsISelection aSelection, in nsIDOMNode aPreviousSelectedNode, in long aPreviousSelectedOffset, in nsIDOMNode aStartNode, in long aStartOffset, in nsIDOMNode aEndNode, in long aEndOffset); */
    pub spellCheckAfterEditorChange: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aAction: libc::int32_t, aSelection: *const nsISelection, aPreviousSelectedNode: *const nsIDOMNode, aPreviousSelectedOffset: libc::int32_t, aStartNode: *const nsIDOMNode, aStartOffset: libc::int32_t, aEndNode: *const nsIDOMNode, aEndOffset: libc::int32_t) -> nsresult,

    /* void spellCheckRange (in nsIDOMRange aSelection); */
    pub spellCheckRange: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aSelection: *const nsIDOMRange) -> nsresult,

    /* nsIDOMRange getMisspelledWord (in nsIDOMNode aNode, in long aOffset); */
    pub getMisspelledWord: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aNode: *const nsIDOMNode, aOffset: libc::int32_t, _retval: *mut *const nsIDOMRange) -> nsresult,

    /* void replaceWord (in nsIDOMNode aNode, in long aOffset, in AString aNewword); */
    pub replaceWord: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aNode: *const nsIDOMNode, aOffset: libc::int32_t, aNewword: *const nsAString) -> nsresult,

    /* void addWordToDictionary (in AString aWord); */
    pub addWordToDictionary: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aWord: *const nsAString) -> nsresult,

    /* void removeWordFromDictionary (in AString aWord); */
    pub removeWordFromDictionary: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aWord: *const nsAString) -> nsresult,

    /* void ignoreWord (in AString aWord); */
    pub ignoreWord: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aWord: *const nsAString) -> nsresult,

    /* void ignoreWords ([array, size_is (aCount)] in wstring aWordsToIgnore, in unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub ignoreWords: *const ::libc::c_void,

    /* void updateCurrentDictionary (); */
    pub updateCurrentDictionary: unsafe extern "C" fn (this: *const nsIInlineSpellChecker) -> nsresult,

    /* readonly attribute boolean spellCheckPending; */
    pub get_spellCheckPending: unsafe extern "C" fn (this: *const nsIInlineSpellChecker, aSpellCheckPending: *mut bool) -> nsresult,

}


impl nsIInlineSpellChecker {
    /* readonly attribute nsIEditorSpellCheck spellChecker; */
    #[inline]
    pub unsafe fn get_spellChecker(&self, ) -> Result<Option<RefPtr<nsIEditorSpellCheck>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_spellChecker)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void init (in nsIEditor aEditor); */
    #[inline]
    pub unsafe fn init(&self, aEditor: Option<&nsIEditor>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aEditor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cleanup (in boolean aDestroyingFrames); */
    #[inline]
    pub unsafe fn cleanup(&self, aDestroyingFrames: bool) -> Result<(), nsresult> {

        match ((*self.vtable).cleanup)(self as *const _, aDestroyingFrames) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean enableRealTimeSpell; */
    #[inline]
    pub unsafe fn get_enableRealTimeSpell(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enableRealTimeSpell)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enableRealTimeSpell(&self, aEnableRealTimeSpell: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_enableRealTimeSpell)(self as *const _, aEnableRealTimeSpell) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void spellCheckAfterEditorChange (in long aAction, in nsISelection aSelection, in nsIDOMNode aPreviousSelectedNode, in long aPreviousSelectedOffset, in nsIDOMNode aStartNode, in long aStartOffset, in nsIDOMNode aEndNode, in long aEndOffset); */
    #[inline]
    pub unsafe fn spellCheckAfterEditorChange(&self, aAction: libc::int32_t, aSelection: Option<&nsISelection>, aPreviousSelectedNode: Option<&nsIDOMNode>, aPreviousSelectedOffset: libc::int32_t, aStartNode: Option<&nsIDOMNode>, aStartOffset: libc::int32_t, aEndNode: Option<&nsIDOMNode>, aEndOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).spellCheckAfterEditorChange)(self as *const _, aAction, aSelection.map_or(::std::ptr::null(), |x| x as *const _), aPreviousSelectedNode.map_or(::std::ptr::null(), |x| x as *const _), aPreviousSelectedOffset, aStartNode.map_or(::std::ptr::null(), |x| x as *const _), aStartOffset, aEndNode.map_or(::std::ptr::null(), |x| x as *const _), aEndOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void spellCheckRange (in nsIDOMRange aSelection); */
    #[inline]
    pub unsafe fn spellCheckRange(&self, aSelection: Option<&nsIDOMRange>) -> Result<(), nsresult> {

        match ((*self.vtable).spellCheckRange)(self as *const _, aSelection.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMRange getMisspelledWord (in nsIDOMNode aNode, in long aOffset); */
    #[inline]
    pub unsafe fn getMisspelledWord(&self, aNode: Option<&nsIDOMNode>, aOffset: libc::int32_t) -> Result<Option<RefPtr<nsIDOMRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getMisspelledWord)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void replaceWord (in nsIDOMNode aNode, in long aOffset, in AString aNewword); */
    #[inline]
    pub unsafe fn replaceWord(&self, aNode: Option<&nsIDOMNode>, aOffset: libc::int32_t, aNewword: &[u16]) -> Result<(), nsresult> {
        let aNewword = nsString::from(aNewword);
        match ((*self.vtable).replaceWord)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aOffset, &*aNewword) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addWordToDictionary (in AString aWord); */
    #[inline]
    pub unsafe fn addWordToDictionary(&self, aWord: &[u16]) -> Result<(), nsresult> {
        let aWord = nsString::from(aWord);
        match ((*self.vtable).addWordToDictionary)(self as *const _, &*aWord) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWordFromDictionary (in AString aWord); */
    #[inline]
    pub unsafe fn removeWordFromDictionary(&self, aWord: &[u16]) -> Result<(), nsresult> {
        let aWord = nsString::from(aWord);
        match ((*self.vtable).removeWordFromDictionary)(self as *const _, &*aWord) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ignoreWord (in AString aWord); */
    #[inline]
    pub unsafe fn ignoreWord(&self, aWord: &[u16]) -> Result<(), nsresult> {
        let aWord = nsString::from(aWord);
        match ((*self.vtable).ignoreWord)(self as *const _, &*aWord) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ignoreWords ([array, size_is (aCount)] in wstring aWordsToIgnore, in unsigned long aCount); */


    /* void updateCurrentDictionary (); */
    #[inline]
    pub unsafe fn updateCurrentDictionary(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).updateCurrentDictionary)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean spellCheckPending; */
    #[inline]
    pub unsafe fn get_spellCheckPending(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_spellCheckPending)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


