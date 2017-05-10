//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIEditorSpellCheck.idl
//


#[repr(C)]
pub struct nsIEditorSpellCheck {
    vtable: *const nsIEditorSpellCheckVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorSpellCheck {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa171c25f, 0xe4a8, 0x4d08,
            [0xad, 0xef, 0xb7, 0x97, 0xe6, 0x37, 0x7b, 0xdc])
    }
}

unsafe impl RefCounted for nsIEditorSpellCheck {
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
pub trait nsIEditorSpellCheckCoerce {
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self;
}

impl nsIEditorSpellCheckCoerce for nsIEditorSpellCheck {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self {
        v
    }
}

impl nsIEditorSpellCheck {
    #[inline]
    pub fn coerce<T: nsIEditorSpellCheckCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorSpellCheck {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorSpellCheckCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorSpellCheckVTable {
    pub __base: nsISupportsVTable,

    /* boolean canSpellCheck (); */
    pub canSpellCheck: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, _retval: *mut bool) -> nsresult,

    /* void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback); */
    pub InitSpellChecker: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, editor: *const nsIEditor, enableSelectionChecking: bool, callback: *const nsIEditorSpellCheckCallback) -> nsresult,

    /* wstring GetNextMisspelledWord (); */
    pub GetNextMisspelledWord: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, _retval: *mut *const libc::int16_t) -> nsresult,

    /* wstring GetSuggestedWord (); */
    pub GetSuggestedWord: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, _retval: *mut *const libc::int16_t) -> nsresult,

    /* boolean CheckCurrentWord (in wstring suggestedWord); */
    pub CheckCurrentWord: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, suggestedWord: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* void ReplaceWord (in wstring misspelledWord, in wstring replaceWord, in boolean allOccurrences); */
    pub ReplaceWord: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, misspelledWord: *const libc::int16_t, replaceWord: *const libc::int16_t, allOccurrences: bool) -> nsresult,

    /* void IgnoreWordAllOccurrences (in wstring word); */
    pub IgnoreWordAllOccurrences: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, word: *const libc::int16_t) -> nsresult,

    /* void GetPersonalDictionary (); */
    pub GetPersonalDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck) -> nsresult,

    /* wstring GetPersonalDictionaryWord (); */
    pub GetPersonalDictionaryWord: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, _retval: *mut *const libc::int16_t) -> nsresult,

    /* void AddWordToDictionary (in wstring word); */
    pub AddWordToDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, word: *const libc::int16_t) -> nsresult,

    /* void RemoveWordFromDictionary (in wstring word); */
    pub RemoveWordFromDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, word: *const libc::int16_t) -> nsresult,

    /* void GetDictionaryList ([array, size_is (count)] out wstring dictionaryList, out uint32_t count); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetDictionaryList: *const ::libc::c_void,

    /* AString GetCurrentDictionary (); */
    pub GetCurrentDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, _retval: *mut nsAString) -> nsresult,

    /* void SetCurrentDictionary (in AString dictionary); */
    pub SetCurrentDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, dictionary: *const nsAString) -> nsresult,

    /* void UninitSpellChecker (); */
    pub UninitSpellChecker: unsafe extern "C" fn (this: *const nsIEditorSpellCheck) -> nsresult,

    /* void setFilter (in nsITextServicesFilter filter); */
    pub setFilter: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, filter: *const nsITextServicesFilter) -> nsresult,

    /* boolean CheckCurrentWordNoSuggest (in wstring suggestedWord); */
    pub CheckCurrentWordNoSuggest: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, suggestedWord: *const libc::int16_t, _retval: *mut bool) -> nsresult,

    /* void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback); */
    pub UpdateCurrentDictionary: unsafe extern "C" fn (this: *const nsIEditorSpellCheck, callback: *const nsIEditorSpellCheckCallback) -> nsresult,

}


impl nsIEditorSpellCheck {
    /* boolean canSpellCheck (); */
    #[inline]
    pub unsafe fn canSpellCheck(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canSpellCheck)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback); */
    #[inline]
    pub unsafe fn InitSpellChecker(&self, editor: Option<&nsIEditor>, enableSelectionChecking: bool, callback: Option<&nsIEditorSpellCheckCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).InitSpellChecker)(self as *const _, editor.map_or(::std::ptr::null(), |x| x as *const _), enableSelectionChecking, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring GetNextMisspelledWord (); */
    #[inline]
    pub unsafe fn GetNextMisspelledWord(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetNextMisspelledWord)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* wstring GetSuggestedWord (); */
    #[inline]
    pub unsafe fn GetSuggestedWord(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetSuggestedWord)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean CheckCurrentWord (in wstring suggestedWord); */
    #[inline]
    pub unsafe fn CheckCurrentWord(&self, suggestedWord: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).CheckCurrentWord)(self as *const _, suggestedWord, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void ReplaceWord (in wstring misspelledWord, in wstring replaceWord, in boolean allOccurrences); */
    #[inline]
    pub unsafe fn ReplaceWord(&self, misspelledWord: *const libc::int16_t, replaceWord: *const libc::int16_t, allOccurrences: bool) -> Result<(), nsresult> {

        match ((*self.vtable).ReplaceWord)(self as *const _, misspelledWord, replaceWord, allOccurrences) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void IgnoreWordAllOccurrences (in wstring word); */
    #[inline]
    pub unsafe fn IgnoreWordAllOccurrences(&self, word: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).IgnoreWordAllOccurrences)(self as *const _, word) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void GetPersonalDictionary (); */
    #[inline]
    pub unsafe fn GetPersonalDictionary(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).GetPersonalDictionary)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* wstring GetPersonalDictionaryWord (); */
    #[inline]
    pub unsafe fn GetPersonalDictionaryWord(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).GetPersonalDictionaryWord)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void AddWordToDictionary (in wstring word); */
    #[inline]
    pub unsafe fn AddWordToDictionary(&self, word: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).AddWordToDictionary)(self as *const _, word) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void RemoveWordFromDictionary (in wstring word); */
    #[inline]
    pub unsafe fn RemoveWordFromDictionary(&self, word: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).RemoveWordFromDictionary)(self as *const _, word) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void GetDictionaryList ([array, size_is (count)] out wstring dictionaryList, out uint32_t count); */


    /* AString GetCurrentDictionary (); */
    #[inline]
    pub unsafe fn GetCurrentDictionary(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).GetCurrentDictionary)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void SetCurrentDictionary (in AString dictionary); */
    #[inline]
    pub unsafe fn SetCurrentDictionary(&self, dictionary: &[u16]) -> Result<(), nsresult> {
        let dictionary = nsString::from(dictionary);
        match ((*self.vtable).SetCurrentDictionary)(self as *const _, &*dictionary) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void UninitSpellChecker (); */
    #[inline]
    pub unsafe fn UninitSpellChecker(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).UninitSpellChecker)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setFilter (in nsITextServicesFilter filter); */
    #[inline]
    pub unsafe fn setFilter(&self, filter: Option<&nsITextServicesFilter>) -> Result<(), nsresult> {

        match ((*self.vtable).setFilter)(self as *const _, filter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean CheckCurrentWordNoSuggest (in wstring suggestedWord); */
    #[inline]
    pub unsafe fn CheckCurrentWordNoSuggest(&self, suggestedWord: *const libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).CheckCurrentWordNoSuggest)(self as *const _, suggestedWord, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback); */
    #[inline]
    pub unsafe fn UpdateCurrentDictionary(&self, callback: Option<&nsIEditorSpellCheckCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).UpdateCurrentDictionary)(self as *const _, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIEditorSpellCheckCallback {
    vtable: *const nsIEditorSpellCheckCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIEditorSpellCheckCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5f0a4bab, 0x8538, 0x4074,
            [0x89, 0xd3, 0x2f, 0x0e, 0x86, 0x6a, 0x1c, 0x0b])
    }
}

unsafe impl RefCounted for nsIEditorSpellCheckCallback {
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
pub trait nsIEditorSpellCheckCallbackCoerce {
    fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self;
}

impl nsIEditorSpellCheckCallbackCoerce for nsIEditorSpellCheckCallback {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self {
        v
    }
}

impl nsIEditorSpellCheckCallback {
    #[inline]
    pub fn coerce<T: nsIEditorSpellCheckCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIEditorSpellCheckCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIEditorSpellCheckCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIEditorSpellCheckCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void editorSpellCheckDone (); */
    pub editorSpellCheckDone: unsafe extern "C" fn (this: *const nsIEditorSpellCheckCallback) -> nsresult,

}


impl nsIEditorSpellCheckCallback {
    /* void editorSpellCheckDone (); */
    #[inline]
    pub unsafe fn editorSpellCheckDone(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).editorSpellCheckDone)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


