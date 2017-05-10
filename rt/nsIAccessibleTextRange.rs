//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleTextRange.idl
//


pub mod nsIAccessibleTextRange_consts {
    pub const EndPoint_Start: i64 = 1;
    pub const EndPoint_End: i64 = 2;
    pub const FormatUnit: i64 = 0;
    pub const WordUnit: i64 = 1;
    pub const LineUnit: i64 = 2;
    pub const ParagraphUnit: i64 = 3;
    pub const PageUnit: i64 = 4;
    pub const DocumentUnit: i64 = 5;
    pub const AnimationStyleAttr: i64 = 0;
    pub const AnnotationObjectsAttr: i64 = 1;
    pub const AnnotationTypesAttr: i64 = 2;
    pub const BackgroundColorAttr: i64 = 3;
    pub const BulletStyleAttr: i64 = 4;
    pub const CapStyleAttr: i64 = 5;
    pub const CaretBidiModeAttr: i64 = 6;
    pub const CaretPositionAttr: i64 = 7;
    pub const CultureAttr: i64 = 8;
    pub const FontNameAttr: i64 = 9;
    pub const FontSizeAttr: i64 = 10;
    pub const FontWeightAttr: i64 = 11;
    pub const ForegroundColorAttr: i64 = 12;
    pub const HorizontalTextAlignmentAttr: i64 = 13;
    pub const IndentationFirstLineAttr: i64 = 14;
    pub const IndentationLeadingAttr: i64 = 15;
    pub const IndentationTrailingAttr: i64 = 16;
    pub const IsActiveAttr: i64 = 17;
    pub const IsHiddenAttr: i64 = 18;
    pub const IsItalicAttr: i64 = 19;
    pub const IsReadOnlyAttr: i64 = 20;
    pub const IsSubscriptAttr: i64 = 21;
    pub const IsSuperscriptAttr: i64 = 22;
    pub const LinkAttr: i64 = 23;
    pub const MarginBottomAttr: i64 = 24;
    pub const MarginLeadingAttr: i64 = 25;
    pub const MarginTopAttr: i64 = 26;
    pub const MarginTrailingAttr: i64 = 27;
    pub const OutlineStylesAttr: i64 = 28;
    pub const OverlineColorAttr: i64 = 29;
    pub const OverlineStyleAttr: i64 = 30;
    pub const SelectionActiveEndAttr: i64 = 31;
    pub const StrikethroughColorAttr: i64 = 32;
    pub const StrikethroughStyleAttr: i64 = 33;
    pub const StyleIdAttr: i64 = 34;
    pub const StyleNameAttr: i64 = 35;
    pub const TabsAttr: i64 = 36;
    pub const TextFlowDirectionsAttr: i64 = 37;
    pub const UnderlineColorAttr: i64 = 38;
    pub const UnderlineStyleAttr: i64 = 39;
    pub const AlignToTop: i64 = 0;
    pub const AlignToBottom: i64 = 1;
}


#[repr(C)]
pub struct nsIAccessibleTextRange {
    vtable: *const nsIAccessibleTextRangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTextRange {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc4515623, 0x55f9, 0x4543,
            [0xa3, 0xd5, 0xc1, 0xe9, 0xaf, 0xa5, 0x88, 0xf4])
    }
}

unsafe impl RefCounted for nsIAccessibleTextRange {
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
pub trait nsIAccessibleTextRangeCoerce {
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self;
}

impl nsIAccessibleTextRangeCoerce for nsIAccessibleTextRange {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self {
        v
    }
}

impl nsIAccessibleTextRange {
    #[inline]
    pub fn coerce<T: nsIAccessibleTextRangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTextRange {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleTextRangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTextRangeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessibleText startContainer; */
    pub get_startContainer: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aStartContainer: *mut *const nsIAccessibleText) -> nsresult,

    /* readonly attribute long startOffset; */
    pub get_startOffset: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aStartOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIAccessibleText endContainer; */
    pub get_endContainer: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aEndContainer: *mut *const nsIAccessibleText) -> nsresult,

    /* readonly attribute long endOffset; */
    pub get_endOffset: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aEndOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIAccessible container; */
    pub get_container: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aContainer: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIArray embeddedChildren; */
    pub get_embeddedChildren: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aEmbeddedChildren: *mut *const nsIArray) -> nsresult,

    /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
    pub compare: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aOtherRange: *const nsIAccessibleTextRange, _retval: *mut bool) -> nsresult,

    /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
    pub compareEndPoints: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aEndPoint: libc::uint32_t, aOtherRange: *const nsIAccessibleTextRange, aOtherRangeEndPoint: libc::uint32_t, _retval: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AString text; */
    pub get_text: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aText: *mut nsAString) -> nsresult,

    /* readonly attribute nsIArray bounds; */
    pub get_bounds: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aBounds: *mut *const nsIArray) -> nsresult,

    /* void move (in unsigned long aUnit, in long aCount); */
    pub move_: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aUnit: libc::uint32_t, aCount: libc::int32_t) -> nsresult,

    /* void moveStart (in unsigned long aUnit, in long aCount); */
    pub moveStart: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aUnit: libc::uint32_t, aCount: libc::int32_t) -> nsresult,

    /* void moveEnd (in unsigned long aUnit, in long aCount); */
    pub moveEnd: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aUnit: libc::uint32_t, aCount: libc::int32_t) -> nsresult,

    /* void normalize (in unsigned long aUnit); */
    pub normalize: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aUnit: libc::uint32_t) -> nsresult,

    /* boolean crop (in nsIAccessible aContainer); */
    pub crop: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aContainer: *const nsIAccessible, _retval: *mut bool) -> nsresult,

    /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
    pub findText: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aText: *const nsAString, aIsBackward: bool, aIsIgnoreCase: bool, _retval: *mut *const nsIAccessibleTextRange) -> nsresult,

    /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
    pub findAttr: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aAttr: libc::uint32_t, aValue: *const nsIVariant, aIsBackward: bool, _retval: *mut *const nsIAccessibleTextRange) -> nsresult,

    /* void addToSelection (); */
    pub addToSelection: unsafe extern "C" fn (this: *const nsIAccessibleTextRange) -> nsresult,

    /* void removeFromSelection (); */
    pub removeFromSelection: unsafe extern "C" fn (this: *const nsIAccessibleTextRange) -> nsresult,

    /* void select (); */
    pub select: unsafe extern "C" fn (this: *const nsIAccessibleTextRange) -> nsresult,

    /* void scrollIntoView (in unsigned long aHow); */
    pub scrollIntoView: unsafe extern "C" fn (this: *const nsIAccessibleTextRange, aHow: libc::uint32_t) -> nsresult,

}


impl nsIAccessibleTextRange {
    /* readonly attribute nsIAccessibleText startContainer; */
    #[inline]
    pub unsafe fn get_startContainer(&self, ) -> Result<Option<RefPtr<nsIAccessibleText>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_startContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long startOffset; */
    #[inline]
    pub unsafe fn get_startOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_startOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessibleText endContainer; */
    #[inline]
    pub unsafe fn get_endContainer(&self, ) -> Result<Option<RefPtr<nsIAccessibleText>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_endContainer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long endOffset; */
    #[inline]
    pub unsafe fn get_endOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_endOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessible container; */
    #[inline]
    pub unsafe fn get_container(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_container)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIArray embeddedChildren; */
    #[inline]
    pub unsafe fn get_embeddedChildren(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_embeddedChildren)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
    #[inline]
    pub unsafe fn compare(&self, aOtherRange: Option<&nsIAccessibleTextRange>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).compare)(self as *const _, aOtherRange.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
    #[inline]
    pub unsafe fn compareEndPoints(&self, aEndPoint: libc::uint32_t, aOtherRange: Option<&nsIAccessibleTextRange>, aOtherRangeEndPoint: libc::uint32_t) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).compareEndPoints)(self as *const _, aEndPoint, aOtherRange.map_or(::std::ptr::null(), |x| x as *const _), aOtherRangeEndPoint, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString text; */
    #[inline]
    pub unsafe fn get_text(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_text)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray bounds; */
    #[inline]
    pub unsafe fn get_bounds(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_bounds)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void move (in unsigned long aUnit, in long aCount); */
    #[inline]
    pub unsafe fn move_(&self, aUnit: libc::uint32_t, aCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).move_)(self as *const _, aUnit, aCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void moveStart (in unsigned long aUnit, in long aCount); */
    #[inline]
    pub unsafe fn moveStart(&self, aUnit: libc::uint32_t, aCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).moveStart)(self as *const _, aUnit, aCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void moveEnd (in unsigned long aUnit, in long aCount); */
    #[inline]
    pub unsafe fn moveEnd(&self, aUnit: libc::uint32_t, aCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).moveEnd)(self as *const _, aUnit, aCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void normalize (in unsigned long aUnit); */
    #[inline]
    pub unsafe fn normalize(&self, aUnit: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).normalize)(self as *const _, aUnit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean crop (in nsIAccessible aContainer); */
    #[inline]
    pub unsafe fn crop(&self, aContainer: Option<&nsIAccessible>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).crop)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
    #[inline]
    pub unsafe fn findText(&self, aText: &[u16], aIsBackward: bool, aIsIgnoreCase: bool) -> Result<Option<RefPtr<nsIAccessibleTextRange>>, nsresult> {
        let aText = nsString::from(aText);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findText)(self as *const _, &*aText, aIsBackward, aIsIgnoreCase, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
    #[inline]
    pub unsafe fn findAttr(&self, aAttr: libc::uint32_t, aValue: Option<&nsIVariant>, aIsBackward: bool) -> Result<Option<RefPtr<nsIAccessibleTextRange>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findAttr)(self as *const _, aAttr, aValue.map_or(::std::ptr::null(), |x| x as *const _), aIsBackward, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addToSelection (); */
    #[inline]
    pub unsafe fn addToSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).addToSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFromSelection (); */
    #[inline]
    pub unsafe fn removeFromSelection(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeFromSelection)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void select (); */
    #[inline]
    pub unsafe fn select(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).select)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollIntoView (in unsigned long aHow); */
    #[inline]
    pub unsafe fn scrollIntoView(&self, aHow: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollIntoView)(self as *const _, aHow) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


