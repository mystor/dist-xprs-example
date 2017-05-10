//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleText.idl
//


pub type AccessibleTextBoundary = libc::int32_t;


pub mod nsIAccessibleText_consts {
    pub const TEXT_OFFSET_END_OF_TEXT: i64 = -1;
    pub const TEXT_OFFSET_CARET: i64 = -2;
    pub const BOUNDARY_CHAR: i64 = 0;
    pub const BOUNDARY_WORD_START: i64 = 1;
    pub const BOUNDARY_WORD_END: i64 = 2;
    pub const BOUNDARY_SENTENCE_START: i64 = 3;
    pub const BOUNDARY_SENTENCE_END: i64 = 4;
    pub const BOUNDARY_LINE_START: i64 = 5;
    pub const BOUNDARY_LINE_END: i64 = 6;
}


#[repr(C)]
pub struct nsIAccessibleText {
    vtable: *const nsIAccessibleTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleText {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x93ad2ca1, 0xf12b, 0x4ab9,
            [0xa7, 0x93, 0x95, 0xd9, 0xfa, 0x9d, 0x17, 0x74])
    }
}

unsafe impl RefCounted for nsIAccessibleText {
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
pub trait nsIAccessibleTextCoerce {
    fn coerce_from(v: &nsIAccessibleText) -> &Self;
}

impl nsIAccessibleTextCoerce for nsIAccessibleText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleText) -> &Self {
        v
    }
}

impl nsIAccessibleText {
    #[inline]
    pub fn coerce<T: nsIAccessibleTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleText {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleText) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTextVTable {
    pub __base: nsISupportsVTable,

    /* attribute long caretOffset; */
    pub get_caretOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, aCaretOffset: *mut libc::int32_t) -> nsresult,
    pub set_caretOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, aCaretOffset: libc::int32_t) -> nsresult,

    /* readonly attribute long characterCount; */
    pub get_characterCount: unsafe extern "C" fn (this: *const nsIAccessibleText, aCharacterCount: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long selectionCount; */
    pub get_selectionCount: unsafe extern "C" fn (this: *const nsIAccessibleText, aSelectionCount: *mut libc::int32_t) -> nsresult,

    /* AString getText (in long startOffset, in long endOffset); */
    pub getText: unsafe extern "C" fn (this: *const nsIAccessibleText, startOffset: libc::int32_t, endOffset: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub getTextAfterOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, offset: libc::int32_t, boundaryType: AccessibleTextBoundary, startOffset: *mut libc::int32_t, endOffset: *mut libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub getTextAtOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, offset: libc::int32_t, boundaryType: AccessibleTextBoundary, startOffset: *mut libc::int32_t, endOffset: *mut libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub getTextBeforeOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, offset: libc::int32_t, boundaryType: AccessibleTextBoundary, startOffset: *mut libc::int32_t, endOffset: *mut libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* wchar getCharacterAtOffset (in long offset); */
    pub getCharacterAtOffset: unsafe extern "C" fn (this: *const nsIAccessibleText, offset: libc::int32_t, _retval: *mut libc::int16_t) -> nsresult,

    /* nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset); */
    pub getTextAttributes: unsafe extern "C" fn (this: *const nsIAccessibleText, includeDefAttrs: bool, offset: libc::int32_t, rangeStartOffset: *mut libc::int32_t, rangeEndOffset: *mut libc::int32_t, _retval: *mut *const nsIPersistentProperties) -> nsresult,

    /* readonly attribute nsIPersistentProperties defaultTextAttributes; */
    pub get_defaultTextAttributes: unsafe extern "C" fn (this: *const nsIAccessibleText, aDefaultTextAttributes: *mut *const nsIPersistentProperties) -> nsresult,

    /* void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    pub getCharacterExtents: unsafe extern "C" fn (this: *const nsIAccessibleText, offset: libc::int32_t, x: *mut libc::int32_t, y: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t, coordType: libc::uint32_t) -> nsresult,

    /* void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    pub getRangeExtents: unsafe extern "C" fn (this: *const nsIAccessibleText, startOffset: libc::int32_t, endOffset: libc::int32_t, x: *mut libc::int32_t, y: *mut libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t, coordType: libc::uint32_t) -> nsresult,

    /* long getOffsetAtPoint (in long x, in long y, in unsigned long coordType); */
    pub getOffsetAtPoint: unsafe extern "C" fn (this: *const nsIAccessibleText, x: libc::int32_t, y: libc::int32_t, coordType: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset); */
    pub getSelectionBounds: unsafe extern "C" fn (this: *const nsIAccessibleText, selectionNum: libc::int32_t, startOffset: *mut libc::int32_t, endOffset: *mut libc::int32_t) -> nsresult,

    /* void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset); */
    pub setSelectionBounds: unsafe extern "C" fn (this: *const nsIAccessibleText, selectionNum: libc::int32_t, startOffset: libc::int32_t, endOffset: libc::int32_t) -> nsresult,

    /* void addSelection (in long startOffset, in long endOffset); */
    pub addSelection: unsafe extern "C" fn (this: *const nsIAccessibleText, startOffset: libc::int32_t, endOffset: libc::int32_t) -> nsresult,

    /* void removeSelection (in long selectionNum); */
    pub removeSelection: unsafe extern "C" fn (this: *const nsIAccessibleText, selectionNum: libc::int32_t) -> nsresult,

    /* void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType); */
    pub scrollSubstringTo: unsafe extern "C" fn (this: *const nsIAccessibleText, startIndex: libc::int32_t, endIndex: libc::int32_t, scrollType: libc::uint32_t) -> nsresult,

    /* void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y); */
    pub scrollSubstringToPoint: unsafe extern "C" fn (this: *const nsIAccessibleText, startIndex: libc::int32_t, endIndex: libc::int32_t, coordinateType: libc::uint32_t, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* readonly attribute nsIAccessibleTextRange enclosingRange; */
    pub get_enclosingRange: unsafe extern "C" fn (this: *const nsIAccessibleText, aEnclosingRange: *mut *const nsIAccessibleTextRange) -> nsresult,

    /* readonly attribute nsIArray selectionRanges; */
    pub get_selectionRanges: unsafe extern "C" fn (this: *const nsIAccessibleText, aSelectionRanges: *mut *const nsIArray) -> nsresult,

    /* readonly attribute nsIArray visibleRanges; */
    pub get_visibleRanges: unsafe extern "C" fn (this: *const nsIAccessibleText, aVisibleRanges: *mut *const nsIArray) -> nsresult,

    /* nsIAccessibleTextRange getRangeByChild (in nsIAccessible child); */
    pub getRangeByChild: unsafe extern "C" fn (this: *const nsIAccessibleText, child: *const nsIAccessible, _retval: *mut *const nsIAccessibleTextRange) -> nsresult,

    /* nsIAccessibleTextRange getRangeAtPoint (in long x, in long y); */
    pub getRangeAtPoint: unsafe extern "C" fn (this: *const nsIAccessibleText, x: libc::int32_t, y: libc::int32_t, _retval: *mut *const nsIAccessibleTextRange) -> nsresult,

}


impl nsIAccessibleText {
    /* attribute long caretOffset; */
    #[inline]
    pub unsafe fn get_caretOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_caretOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_caretOffset(&self, aCaretOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_caretOffset)(self as *const _, aCaretOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long characterCount; */
    #[inline]
    pub unsafe fn get_characterCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_characterCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long selectionCount; */
    #[inline]
    pub unsafe fn get_selectionCount(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getText (in long startOffset, in long endOffset); */
    #[inline]
    pub unsafe fn getText(&self, startOffset: libc::int32_t, endOffset: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getText)(self as *const _, startOffset, endOffset, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    #[inline]
    pub unsafe fn getTextAfterOffset(&self, offset: libc::int32_t, boundaryType: AccessibleTextBoundary) -> Result<(libc::int32_t, libc::int32_t, nsString), nsresult> {
        let mut startOffset: libc::int32_t = ::std::mem::zeroed();
        let mut endOffset: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getTextAfterOffset)(self as *const _, offset, boundaryType, &mut startOffset as *mut _, &mut endOffset as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((startOffset, endOffset, _retval))
    }

    /* AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    #[inline]
    pub unsafe fn getTextAtOffset(&self, offset: libc::int32_t, boundaryType: AccessibleTextBoundary) -> Result<(libc::int32_t, libc::int32_t, nsString), nsresult> {
        let mut startOffset: libc::int32_t = ::std::mem::zeroed();
        let mut endOffset: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getTextAtOffset)(self as *const _, offset, boundaryType, &mut startOffset as *mut _, &mut endOffset as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((startOffset, endOffset, _retval))
    }

    /* AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    #[inline]
    pub unsafe fn getTextBeforeOffset(&self, offset: libc::int32_t, boundaryType: AccessibleTextBoundary) -> Result<(libc::int32_t, libc::int32_t, nsString), nsresult> {
        let mut startOffset: libc::int32_t = ::std::mem::zeroed();
        let mut endOffset: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = nsString::new();
        match ((*self.vtable).getTextBeforeOffset)(self as *const _, offset, boundaryType, &mut startOffset as *mut _, &mut endOffset as *mut _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((startOffset, endOffset, _retval))
    }

    /* wchar getCharacterAtOffset (in long offset); */
    #[inline]
    pub unsafe fn getCharacterAtOffset(&self, offset: libc::int32_t) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getCharacterAtOffset)(self as *const _, offset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset); */
    #[inline]
    pub unsafe fn getTextAttributes(&self, includeDefAttrs: bool, offset: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t, Option<RefPtr<nsIPersistentProperties>>), nsresult> {
        let mut rangeStartOffset: libc::int32_t = ::std::mem::zeroed();
        let mut rangeEndOffset: libc::int32_t = ::std::mem::zeroed();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getTextAttributes)(self as *const _, includeDefAttrs, offset, &mut rangeStartOffset as *mut _, &mut rangeEndOffset as *mut _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((rangeStartOffset, rangeEndOffset, _retval.refptr()))
    }

    /* readonly attribute nsIPersistentProperties defaultTextAttributes; */
    #[inline]
    pub unsafe fn get_defaultTextAttributes(&self, ) -> Result<Option<RefPtr<nsIPersistentProperties>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultTextAttributes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    #[inline]
    pub unsafe fn getCharacterExtents(&self, offset: libc::int32_t, coordType: libc::uint32_t) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCharacterExtents)(self as *const _, offset, &mut x as *mut _, &mut y as *mut _, &mut width as *mut _, &mut height as *mut _, coordType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, width, height))
    }

    /* void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    #[inline]
    pub unsafe fn getRangeExtents(&self, startOffset: libc::int32_t, endOffset: libc::int32_t, coordType: libc::uint32_t) -> Result<(libc::int32_t, libc::int32_t, libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getRangeExtents)(self as *const _, startOffset, endOffset, &mut x as *mut _, &mut y as *mut _, &mut width as *mut _, &mut height as *mut _, coordType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y, width, height))
    }

    /* long getOffsetAtPoint (in long x, in long y, in unsigned long coordType); */
    #[inline]
    pub unsafe fn getOffsetAtPoint(&self, x: libc::int32_t, y: libc::int32_t, coordType: libc::uint32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getOffsetAtPoint)(self as *const _, x, y, coordType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset); */
    #[inline]
    pub unsafe fn getSelectionBounds(&self, selectionNum: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut startOffset: libc::int32_t = ::std::mem::zeroed();
        let mut endOffset: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getSelectionBounds)(self as *const _, selectionNum, &mut startOffset as *mut _, &mut endOffset as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((startOffset, endOffset))
    }

    /* void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset); */
    #[inline]
    pub unsafe fn setSelectionBounds(&self, selectionNum: libc::int32_t, startOffset: libc::int32_t, endOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setSelectionBounds)(self as *const _, selectionNum, startOffset, endOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addSelection (in long startOffset, in long endOffset); */
    #[inline]
    pub unsafe fn addSelection(&self, startOffset: libc::int32_t, endOffset: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).addSelection)(self as *const _, startOffset, endOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSelection (in long selectionNum); */
    #[inline]
    pub unsafe fn removeSelection(&self, selectionNum: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeSelection)(self as *const _, selectionNum) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType); */
    #[inline]
    pub unsafe fn scrollSubstringTo(&self, startIndex: libc::int32_t, endIndex: libc::int32_t, scrollType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollSubstringTo)(self as *const _, startIndex, endIndex, scrollType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y); */
    #[inline]
    pub unsafe fn scrollSubstringToPoint(&self, startIndex: libc::int32_t, endIndex: libc::int32_t, coordinateType: libc::uint32_t, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollSubstringToPoint)(self as *const _, startIndex, endIndex, coordinateType, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIAccessibleTextRange enclosingRange; */
    #[inline]
    pub unsafe fn get_enclosingRange(&self, ) -> Result<Option<RefPtr<nsIAccessibleTextRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_enclosingRange)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray selectionRanges; */
    #[inline]
    pub unsafe fn get_selectionRanges(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectionRanges)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray visibleRanges; */
    #[inline]
    pub unsafe fn get_visibleRanges(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_visibleRanges)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessibleTextRange getRangeByChild (in nsIAccessible child); */
    #[inline]
    pub unsafe fn getRangeByChild(&self, child: Option<&nsIAccessible>) -> Result<Option<RefPtr<nsIAccessibleTextRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRangeByChild)(self as *const _, child.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessibleTextRange getRangeAtPoint (in long x, in long y); */
    #[inline]
    pub unsafe fn getRangeAtPoint(&self, x: libc::int32_t, y: libc::int32_t) -> Result<Option<RefPtr<nsIAccessibleTextRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getRangeAtPoint)(self as *const _, x, y, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


