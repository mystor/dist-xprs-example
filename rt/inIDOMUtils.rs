//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/inIDOMUtils.idl
//


pub mod inIDOMUtils_consts {
    pub const EXCLUDE_SHORTHANDS: i64 = 1;
    pub const INCLUDE_ALIASES: i64 = 2;
    pub const TYPE_LENGTH: i64 = 0;
    pub const TYPE_PERCENTAGE: i64 = 1;
    pub const TYPE_COLOR: i64 = 2;
    pub const TYPE_URL: i64 = 3;
    pub const TYPE_ANGLE: i64 = 4;
    pub const TYPE_FREQUENCY: i64 = 5;
    pub const TYPE_TIME: i64 = 6;
    pub const TYPE_GRADIENT: i64 = 7;
    pub const TYPE_TIMING_FUNCTION: i64 = 8;
    pub const TYPE_IMAGE_RECT: i64 = 9;
    pub const TYPE_NUMBER: i64 = 10;
}


#[repr(C)]
pub struct inIDOMUtils {
    vtable: *const inIDOMUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for inIDOMUtils {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x362e98c3, 0x82c2, 0x4ad8,
            [0x8d, 0xcb, 0x00, 0xe8, 0xe4, 0xea, 0xb4, 0x97])
    }
}

unsafe impl RefCounted for inIDOMUtils {
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
pub trait inIDOMUtilsCoerce {
    fn coerce_from(v: &inIDOMUtils) -> &Self;
}

impl inIDOMUtilsCoerce for inIDOMUtils {
    #[inline]
    fn coerce_from(v: &inIDOMUtils) -> &Self {
        v
    }
}

impl inIDOMUtils {
    #[inline]
    pub fn coerce<T: inIDOMUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for inIDOMUtils {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> inIDOMUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &inIDOMUtils) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct inIDOMUtilsVTable {
    pub __base: nsISupportsVTable,

    /* void getAllStyleSheets (in nsIDOMDocument aDoc, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out nsISupports aSheets); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAllStyleSheets: *const ::libc::c_void,

    /* nsIArrayExtensions getCSSStyleRules (in nsIDOMElement aElement, [optional] in DOMString aPseudo); */
    pub getCSSStyleRules: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aPseudo: *const nsAString, _retval: *mut *const nsIArrayExtensions) -> nsresult,

    /* unsigned long getRuleLine (in nsIDOMCSSRule aRule); */
    pub getRuleLine: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSRule, _retval: *mut libc::uint32_t) -> nsresult,

    /* unsigned long getRuleColumn (in nsIDOMCSSRule aRule); */
    pub getRuleColumn: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSRule, _retval: *mut libc::uint32_t) -> nsresult,

    /* unsigned long getRelativeRuleLine (in nsIDOMCSSRule aRule); */
    pub getRelativeRuleLine: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSRule, _retval: *mut libc::uint32_t) -> nsresult,

    /* [implicit_jscontext] jsval getCSSLexer (in DOMString aText); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCSSLexer: *const ::libc::c_void,

    /* unsigned long getSelectorCount (in nsIDOMCSSStyleRule aRule); */
    pub getSelectorCount: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSStyleRule, _retval: *mut libc::uint32_t) -> nsresult,

    /* AString getSelectorText (in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex); */
    pub getSelectorText: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSStyleRule, aSelectorIndex: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* unsigned long long getSpecificity (in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex); */
    pub getSpecificity: unsafe extern "C" fn (this: *const inIDOMUtils, aRule: *const nsIDOMCSSStyleRule, aSelectorIndex: libc::uint32_t, _retval: *mut libc::uint64_t) -> nsresult,

    /* bool selectorMatchesElement (in nsIDOMElement aElement, in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex, [optional] in DOMString aPseudo); */
    pub selectorMatchesElement: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aRule: *const nsIDOMCSSStyleRule, aSelectorIndex: libc::uint32_t, aPseudo: *const nsAString, _retval: *mut bool) -> nsresult,

    /* bool isInheritedProperty (in AString aPropertyName); */
    pub isInheritedProperty: unsafe extern "C" fn (this: *const inIDOMUtils, aPropertyName: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void getCSSPropertyNames ([optional] in unsigned long aFlags, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aProps); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCSSPropertyNames: *const ::libc::c_void,

    /* void getCSSValuesForProperty (in AString aProperty, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out wstring aValues); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCSSValuesForProperty: *const ::libc::c_void,

    /* [implicit_jscontext] jsval colorNameToRGB (in DOMString aColorName); */
    /// Unable to call function as its signature contains a non-rust type
    pub colorNameToRGB: *const ::libc::c_void,

    /* AString rgbToColorName (in octet aR, in octet aG, in octet aB); */
    pub rgbToColorName: unsafe extern "C" fn (this: *const inIDOMUtils, aR: libc::uint8_t, aG: libc::uint8_t, aB: libc::uint8_t, _retval: *mut nsAString) -> nsresult,

    /* [implicit_jscontext] jsval colorToRGBA (in DOMString aColorString); */
    /// Unable to call function as its signature contains a non-rust type
    pub colorToRGBA: *const ::libc::c_void,

    /* bool isValidCSSColor (in AString aColorString); */
    pub isValidCSSColor: unsafe extern "C" fn (this: *const inIDOMUtils, aColorString: *const nsAString, _retval: *mut bool) -> nsresult,

    /* bool cssPropertyIsValid (in AString aPropertyName, in AString aPropertyValue); */
    pub cssPropertyIsValid: unsafe extern "C" fn (this: *const inIDOMUtils, aPropertyName: *const nsAString, aPropertyValue: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void getSubpropertiesForCSSProperty (in AString aProperty, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out wstring aValues); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSubpropertiesForCSSProperty: *const ::libc::c_void,

    /* bool cssPropertyIsShorthand (in AString aProperty); */
    pub cssPropertyIsShorthand: unsafe extern "C" fn (this: *const inIDOMUtils, aProperty: *const nsAString, _retval: *mut bool) -> nsresult,

    /* bool cssPropertySupportsType (in AString aProperty, in unsigned long type); */
    pub cssPropertySupportsType: unsafe extern "C" fn (this: *const inIDOMUtils, aProperty: *const nsAString, type_: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean isIgnorableWhitespace (in nsIDOMCharacterData aDataNode); */
    pub isIgnorableWhitespace: unsafe extern "C" fn (this: *const inIDOMUtils, aDataNode: *const nsIDOMCharacterData, _retval: *mut bool) -> nsresult,

    /* nsIDOMNode getParentForNode (in nsIDOMNode aNode, in boolean aShowingAnonymousContent); */
    pub getParentForNode: unsafe extern "C" fn (this: *const inIDOMUtils, aNode: *const nsIDOMNode, aShowingAnonymousContent: bool, _retval: *mut *const nsIDOMNode) -> nsresult,

    /* nsIDOMNodeList getChildrenForNode (in nsIDOMNode aNode, in boolean aShowingAnonymousContent); */
    pub getChildrenForNode: unsafe extern "C" fn (this: *const inIDOMUtils, aNode: *const nsIDOMNode, aShowingAnonymousContent: bool, _retval: *mut *const nsIDOMNodeList) -> nsresult,

    /* nsIArray getBindingURLs (in nsIDOMElement aElement); */
    pub getBindingURLs: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, _retval: *mut *const nsIArray) -> nsresult,

    /* unsigned long long getContentState (in nsIDOMElement aElement); */
    pub getContentState: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, _retval: *mut libc::uint64_t) -> nsresult,

    /* bool setContentState (in nsIDOMElement aElement, in unsigned long long aState); */
    pub setContentState: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aState: libc::uint64_t, _retval: *mut bool) -> nsresult,

    /* bool removeContentState (in nsIDOMElement aElement, in unsigned long long aState); */
    pub removeContentState: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aState: libc::uint64_t, _retval: *mut bool) -> nsresult,

    /* nsIDOMFontFaceList getUsedFontFaces (in nsIDOMRange aRange); */
    pub getUsedFontFaces: unsafe extern "C" fn (this: *const inIDOMUtils, aRange: *const nsIDOMRange, _retval: *mut *const nsIDOMFontFaceList) -> nsresult,

    /* void getCSSPseudoElementNames ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aNames); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCSSPseudoElementNames: *const ::libc::c_void,

    /* [optional_argc] void addPseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass, [optional] in boolean aEnabled); */
    /// Unable to call function as its signature contains a non-rust type
    pub addPseudoClassLock: *const ::libc::c_void,

    /* void removePseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass); */
    pub removePseudoClassLock: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aPseudoClass: *const nsAString) -> nsresult,

    /* bool hasPseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass); */
    pub hasPseudoClassLock: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement, aPseudoClass: *const nsAString, _retval: *mut bool) -> nsresult,

    /* void clearPseudoClassLocks (in nsIDOMElement aElement); */
    pub clearPseudoClassLocks: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement) -> nsresult,

    /* void parseStyleSheet (in nsIDOMCSSStyleSheet aSheet, in DOMString aInput); */
    pub parseStyleSheet: unsafe extern "C" fn (this: *const inIDOMUtils, aSheet: *const nsIDOMCSSStyleSheet, aInput: *const nsAString) -> nsresult,

    /* void scrollElementIntoView (in nsIDOMElement aElement); */
    pub scrollElementIntoView: unsafe extern "C" fn (this: *const inIDOMUtils, aElement: *const nsIDOMElement) -> nsresult,

}


impl inIDOMUtils {
    /* void getAllStyleSheets (in nsIDOMDocument aDoc, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out nsISupports aSheets); */


    /* nsIArrayExtensions getCSSStyleRules (in nsIDOMElement aElement, [optional] in DOMString aPseudo); */
    #[inline]
    pub unsafe fn getCSSStyleRules(&self, aElement: Option<&nsIDOMElement>, aPseudo: &[u16]) -> Result<Option<RefPtr<nsIArrayExtensions>>, nsresult> {
        let aPseudo = nsString::from(aPseudo);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCSSStyleRules)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aPseudo, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getRuleLine (in nsIDOMCSSRule aRule); */
    #[inline]
    pub unsafe fn getRuleLine(&self, aRule: Option<&nsIDOMCSSRule>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRuleLine)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getRuleColumn (in nsIDOMCSSRule aRule); */
    #[inline]
    pub unsafe fn getRuleColumn(&self, aRule: Option<&nsIDOMCSSRule>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRuleColumn)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getRelativeRuleLine (in nsIDOMCSSRule aRule); */
    #[inline]
    pub unsafe fn getRelativeRuleLine(&self, aRule: Option<&nsIDOMCSSRule>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRelativeRuleLine)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval getCSSLexer (in DOMString aText); */


    /* unsigned long getSelectorCount (in nsIDOMCSSStyleRule aRule); */
    #[inline]
    pub unsafe fn getSelectorCount(&self, aRule: Option<&nsIDOMCSSStyleRule>) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getSelectorCount)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getSelectorText (in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex); */
    #[inline]
    pub unsafe fn getSelectorText(&self, aRule: Option<&nsIDOMCSSStyleRule>, aSelectorIndex: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getSelectorText)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), aSelectorIndex, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long long getSpecificity (in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex); */
    #[inline]
    pub unsafe fn getSpecificity(&self, aRule: Option<&nsIDOMCSSStyleRule>, aSelectorIndex: libc::uint32_t) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).getSpecificity)(self as *const _, aRule.map_or(::std::ptr::null(), |x| x as *const _), aSelectorIndex, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool selectorMatchesElement (in nsIDOMElement aElement, in nsIDOMCSSStyleRule aRule, in unsigned long aSelectorIndex, [optional] in DOMString aPseudo); */
    #[inline]
    pub unsafe fn selectorMatchesElement(&self, aElement: Option<&nsIDOMElement>, aRule: Option<&nsIDOMCSSStyleRule>, aSelectorIndex: libc::uint32_t, aPseudo: &[u16]) -> Result<bool, nsresult> {
        let aPseudo = nsString::from(aPseudo);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).selectorMatchesElement)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aRule.map_or(::std::ptr::null(), |x| x as *const _), aSelectorIndex, &*aPseudo, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool isInheritedProperty (in AString aPropertyName); */
    #[inline]
    pub unsafe fn isInheritedProperty(&self, aPropertyName: &[u16]) -> Result<bool, nsresult> {
        let aPropertyName = nsString::from(aPropertyName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isInheritedProperty)(self as *const _, &*aPropertyName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCSSPropertyNames ([optional] in unsigned long aFlags, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aProps); */


    /* void getCSSValuesForProperty (in AString aProperty, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out wstring aValues); */


    /* [implicit_jscontext] jsval colorNameToRGB (in DOMString aColorName); */


    /* AString rgbToColorName (in octet aR, in octet aG, in octet aB); */
    #[inline]
    pub unsafe fn rgbToColorName(&self, aR: libc::uint8_t, aG: libc::uint8_t, aB: libc::uint8_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).rgbToColorName)(self as *const _, aR, aG, aB, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval colorToRGBA (in DOMString aColorString); */


    /* bool isValidCSSColor (in AString aColorString); */
    #[inline]
    pub unsafe fn isValidCSSColor(&self, aColorString: &[u16]) -> Result<bool, nsresult> {
        let aColorString = nsString::from(aColorString);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isValidCSSColor)(self as *const _, &*aColorString, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool cssPropertyIsValid (in AString aPropertyName, in AString aPropertyValue); */
    #[inline]
    pub unsafe fn cssPropertyIsValid(&self, aPropertyName: &[u16], aPropertyValue: &[u16]) -> Result<bool, nsresult> {
        let aPropertyName = nsString::from(aPropertyName);
        let aPropertyValue = nsString::from(aPropertyValue);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).cssPropertyIsValid)(self as *const _, &*aPropertyName, &*aPropertyValue, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getSubpropertiesForCSSProperty (in AString aProperty, [optional] out unsigned long aLength, [array, size_is (aLength), retval] out wstring aValues); */


    /* bool cssPropertyIsShorthand (in AString aProperty); */
    #[inline]
    pub unsafe fn cssPropertyIsShorthand(&self, aProperty: &[u16]) -> Result<bool, nsresult> {
        let aProperty = nsString::from(aProperty);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).cssPropertyIsShorthand)(self as *const _, &*aProperty, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool cssPropertySupportsType (in AString aProperty, in unsigned long type); */
    #[inline]
    pub unsafe fn cssPropertySupportsType(&self, aProperty: &[u16], type_: libc::uint32_t) -> Result<bool, nsresult> {
        let aProperty = nsString::from(aProperty);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).cssPropertySupportsType)(self as *const _, &*aProperty, type_, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isIgnorableWhitespace (in nsIDOMCharacterData aDataNode); */
    #[inline]
    pub unsafe fn isIgnorableWhitespace(&self, aDataNode: Option<&nsIDOMCharacterData>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isIgnorableWhitespace)(self as *const _, aDataNode.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMNode getParentForNode (in nsIDOMNode aNode, in boolean aShowingAnonymousContent); */
    #[inline]
    pub unsafe fn getParentForNode(&self, aNode: Option<&nsIDOMNode>, aShowingAnonymousContent: bool) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getParentForNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aShowingAnonymousContent, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDOMNodeList getChildrenForNode (in nsIDOMNode aNode, in boolean aShowingAnonymousContent); */
    #[inline]
    pub unsafe fn getChildrenForNode(&self, aNode: Option<&nsIDOMNode>, aShowingAnonymousContent: bool) -> Result<Option<RefPtr<nsIDOMNodeList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildrenForNode)(self as *const _, aNode.map_or(::std::ptr::null(), |x| x as *const _), aShowingAnonymousContent, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIArray getBindingURLs (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getBindingURLs(&self, aElement: Option<&nsIDOMElement>) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getBindingURLs)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long long getContentState (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn getContentState(&self, aElement: Option<&nsIDOMElement>) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).getContentState)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool setContentState (in nsIDOMElement aElement, in unsigned long long aState); */
    #[inline]
    pub unsafe fn setContentState(&self, aElement: Option<&nsIDOMElement>, aState: libc::uint64_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).setContentState)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aState, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool removeContentState (in nsIDOMElement aElement, in unsigned long long aState); */
    #[inline]
    pub unsafe fn removeContentState(&self, aElement: Option<&nsIDOMElement>, aState: libc::uint64_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).removeContentState)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aState, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDOMFontFaceList getUsedFontFaces (in nsIDOMRange aRange); */
    #[inline]
    pub unsafe fn getUsedFontFaces(&self, aRange: Option<&nsIDOMRange>) -> Result<Option<RefPtr<nsIDOMFontFaceList>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getUsedFontFaces)(self as *const _, aRange.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getCSSPseudoElementNames ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out wstring aNames); */


    /* [optional_argc] void addPseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass, [optional] in boolean aEnabled); */


    /* void removePseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass); */
    #[inline]
    pub unsafe fn removePseudoClassLock(&self, aElement: Option<&nsIDOMElement>, aPseudoClass: &[u16]) -> Result<(), nsresult> {
        let aPseudoClass = nsString::from(aPseudoClass);
        match ((*self.vtable).removePseudoClassLock)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aPseudoClass) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool hasPseudoClassLock (in nsIDOMElement aElement, in DOMString aPseudoClass); */
    #[inline]
    pub unsafe fn hasPseudoClassLock(&self, aElement: Option<&nsIDOMElement>, aPseudoClass: &[u16]) -> Result<bool, nsresult> {
        let aPseudoClass = nsString::from(aPseudoClass);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasPseudoClassLock)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), &*aPseudoClass, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void clearPseudoClassLocks (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn clearPseudoClassLocks(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).clearPseudoClassLocks)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void parseStyleSheet (in nsIDOMCSSStyleSheet aSheet, in DOMString aInput); */
    #[inline]
    pub unsafe fn parseStyleSheet(&self, aSheet: Option<&nsIDOMCSSStyleSheet>, aInput: &[u16]) -> Result<(), nsresult> {
        let aInput = nsString::from(aInput);
        match ((*self.vtable).parseStyleSheet)(self as *const _, aSheet.map_or(::std::ptr::null(), |x| x as *const _), &*aInput) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollElementIntoView (in nsIDOMElement aElement); */
    #[inline]
    pub unsafe fn scrollElementIntoView(&self, aElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).scrollElementIntoView)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


