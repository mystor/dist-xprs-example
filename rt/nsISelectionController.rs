//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionController.idl
//


pub mod nsISelectionController_consts {
    pub const SELECTION_NONE: i64 = 0;
    pub const SELECTION_NORMAL: i64 = 1;
    pub const SELECTION_SPELLCHECK: i64 = 2;
    pub const SELECTION_IME_RAWINPUT: i64 = 4;
    pub const SELECTION_IME_SELECTEDRAWTEXT: i64 = 8;
    pub const SELECTION_IME_CONVERTEDTEXT: i64 = 16;
    pub const SELECTION_IME_SELECTEDCONVERTEDTEXT: i64 = 32;
    pub const SELECTION_ACCESSIBILITY: i64 = 64;
    pub const SELECTION_FIND: i64 = 128;
    pub const SELECTION_URLSECONDARY: i64 = 256;
    pub const SELECTION_URLSTRIKEOUT: i64 = 512;
    pub const NUM_SELECTIONTYPES: i64 = 11;
    pub const SELECTION_ANCHOR_REGION: i64 = 0;
    pub const SELECTION_FOCUS_REGION: i64 = 1;
    pub const SELECTION_WHOLE_SELECTION: i64 = 2;
    pub const NUM_SELECTION_REGIONS: i64 = 3;
    pub const SELECTION_OFF: i64 = 0;
    pub const SELECTION_HIDDEN: i64 = 1;
    pub const SELECTION_ON: i64 = 2;
    pub const SELECTION_DISABLED: i64 = 3;
    pub const SELECTION_ATTENTION: i64 = 4;
    pub const SCROLL_SYNCHRONOUS: i64 = 2;
    pub const SCROLL_FIRST_ANCESTOR_ONLY: i64 = 4;
    pub const SCROLL_CENTER_VERTICALLY: i64 = 16;
    pub const SCROLL_OVERFLOW_HIDDEN: i64 = 32;
    pub const SCROLL_FOR_CARET_MOVE: i64 = 64;
    pub const MOVE_LEFT: i64 = 0;
    pub const MOVE_RIGHT: i64 = 1;
    pub const MOVE_UP: i64 = 2;
    pub const MOVE_DOWN: i64 = 3;
}


#[repr(C)]
pub struct nsISelectionController {
    vtable: *const nsISelectionControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISelectionController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3801c9d4, 0x8e69, 0x4bfc,
            [0x9e, 0xdb, 0xb5, 0x82, 0x78, 0x62, 0x1f, 0x8f])
    }
}

unsafe impl RefCounted for nsISelectionController {
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
pub trait nsISelectionControllerCoerce {
    fn coerce_from(v: &nsISelectionController) -> &Self;
}

impl nsISelectionControllerCoerce for nsISelectionController {
    #[inline]
    fn coerce_from(v: &nsISelectionController) -> &Self {
        v
    }
}

impl nsISelectionController {
    #[inline]
    pub fn coerce<T: nsISelectionControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISelectionController {
    type Target = nsISelectionDisplay;
    #[inline]
    fn deref(&self) -> &nsISelectionDisplay {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISelectionDisplayCoerce> nsISelectionControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISelectionControllerVTable {
    pub __base: nsISelectionDisplayVTable,

    /* void setDisplaySelection (in short toggle); */
    pub setDisplaySelection: unsafe extern "C" fn (this: *const nsISelectionController, toggle: libc::int16_t) -> nsresult,

    /* short getDisplaySelection (); */
    pub getDisplaySelection: unsafe extern "C" fn (this: *const nsISelectionController, _retval: *mut libc::int16_t) -> nsresult,

    /* nsISelection getSelection (in short type); */
    pub getSelection: unsafe extern "C" fn (this: *const nsISelectionController, type_: libc::int16_t, _retval: *mut *const nsISelection) -> nsresult,

    /* void scrollSelectionIntoView (in short type, in short region, in short flags); */
    pub scrollSelectionIntoView: unsafe extern "C" fn (this: *const nsISelectionController, type_: libc::int16_t, region: libc::int16_t, flags: libc::int16_t) -> nsresult,

    /* void repaintSelection (in short type); */
    pub repaintSelection: unsafe extern "C" fn (this: *const nsISelectionController, type_: libc::int16_t) -> nsresult,

    /* void setCaretEnabled (in boolean enabled); */
    pub setCaretEnabled: unsafe extern "C" fn (this: *const nsISelectionController, enabled: bool) -> nsresult,

    /* void setCaretReadOnly (in boolean readOnly); */
    pub setCaretReadOnly: unsafe extern "C" fn (this: *const nsISelectionController, readOnly: bool) -> nsresult,

    /* boolean getCaretEnabled (); */
    pub getCaretEnabled: unsafe extern "C" fn (this: *const nsISelectionController, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean caretVisible; */
    pub get_caretVisible: unsafe extern "C" fn (this: *const nsISelectionController, aCaretVisible: *mut bool) -> nsresult,

    /* void setCaretVisibilityDuringSelection (in boolean visibility); */
    pub setCaretVisibilityDuringSelection: unsafe extern "C" fn (this: *const nsISelectionController, visibility: bool) -> nsresult,

    /* void characterMove (in boolean forward, in boolean extend); */
    pub characterMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* void physicalMove (in short direction, in short amount, in boolean extend); */
    pub physicalMove: unsafe extern "C" fn (this: *const nsISelectionController, direction: libc::int16_t, amount: libc::int16_t, extend: bool) -> nsresult,

    /* [noscript] void characterExtendForDelete (); */
    pub characterExtendForDelete: unsafe extern "C" fn (this: *const nsISelectionController) -> nsresult,

    /* [noscript] void characterExtendForBackspace (); */
    pub characterExtendForBackspace: unsafe extern "C" fn (this: *const nsISelectionController) -> nsresult,

    /* void wordMove (in boolean forward, in boolean extend); */
    pub wordMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* [noscript] void wordExtendForDelete (in boolean forward); */
    pub wordExtendForDelete: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool) -> nsresult,

    /* void lineMove (in boolean forward, in boolean extend); */
    pub lineMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* void intraLineMove (in boolean forward, in boolean extend); */
    pub intraLineMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* void pageMove (in boolean forward, in boolean extend); */
    pub pageMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* void completeScroll (in boolean forward); */
    pub completeScroll: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool) -> nsresult,

    /* void completeMove (in boolean forward, in boolean extend); */
    pub completeMove: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> nsresult,

    /* void scrollPage (in boolean forward); */
    pub scrollPage: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool) -> nsresult,

    /* void scrollLine (in boolean forward); */
    pub scrollLine: unsafe extern "C" fn (this: *const nsISelectionController, forward: bool) -> nsresult,

    /* void scrollCharacter (in boolean right); */
    pub scrollCharacter: unsafe extern "C" fn (this: *const nsISelectionController, right: bool) -> nsresult,

    /* void selectAll (); */
    pub selectAll: unsafe extern "C" fn (this: *const nsISelectionController) -> nsresult,

    /* boolean checkVisibility (in nsIDOMNode node, in short startOffset, in short endOffset); */
    pub checkVisibility: unsafe extern "C" fn (this: *const nsISelectionController, node: *const nsIDOMNode, startOffset: libc::int16_t, endOffset: libc::int16_t, _retval: *mut bool) -> nsresult,

    /* [noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset); */
    pub checkVisibilityContent: unsafe extern "C" fn (this: *const nsISelectionController, node: *const nsIContent, startOffset: libc::int16_t, endOffset: libc::int16_t, _retval: *mut bool) -> nsresult,

}


impl nsISelectionController {
    /* void setDisplaySelection (in short toggle); */
    #[inline]
    pub unsafe fn setDisplaySelection(&self, toggle: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDisplaySelection)(self as *const _, toggle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* short getDisplaySelection (); */
    #[inline]
    pub unsafe fn getDisplaySelection(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).getDisplaySelection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsISelection getSelection (in short type); */
    #[inline]
    pub unsafe fn getSelection(&self, type_: libc::int16_t) -> Result<Option<RefPtr<nsISelection>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSelection)(self as *const _, type_, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void scrollSelectionIntoView (in short type, in short region, in short flags); */
    #[inline]
    pub unsafe fn scrollSelectionIntoView(&self, type_: libc::int16_t, region: libc::int16_t, flags: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollSelectionIntoView)(self as *const _, type_, region, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void repaintSelection (in short type); */
    #[inline]
    pub unsafe fn repaintSelection(&self, type_: libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).repaintSelection)(self as *const _, type_) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCaretEnabled (in boolean enabled); */
    #[inline]
    pub unsafe fn setCaretEnabled(&self, enabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setCaretEnabled)(self as *const _, enabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCaretReadOnly (in boolean readOnly); */
    #[inline]
    pub unsafe fn setCaretReadOnly(&self, readOnly: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setCaretReadOnly)(self as *const _, readOnly) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getCaretEnabled (); */
    #[inline]
    pub unsafe fn getCaretEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getCaretEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean caretVisible; */
    #[inline]
    pub unsafe fn get_caretVisible(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_caretVisible)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setCaretVisibilityDuringSelection (in boolean visibility); */
    #[inline]
    pub unsafe fn setCaretVisibilityDuringSelection(&self, visibility: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setCaretVisibilityDuringSelection)(self as *const _, visibility) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void characterMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn characterMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).characterMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void physicalMove (in short direction, in short amount, in boolean extend); */
    #[inline]
    pub unsafe fn physicalMove(&self, direction: libc::int16_t, amount: libc::int16_t, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).physicalMove)(self as *const _, direction, amount, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void characterExtendForDelete (); */
    #[inline]
    pub unsafe fn characterExtendForDelete(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).characterExtendForDelete)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void characterExtendForBackspace (); */
    #[inline]
    pub unsafe fn characterExtendForBackspace(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).characterExtendForBackspace)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void wordMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn wordMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).wordMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void wordExtendForDelete (in boolean forward); */
    #[inline]
    pub unsafe fn wordExtendForDelete(&self, forward: bool) -> Result<(), nsresult> {

        match ((*self.vtable).wordExtendForDelete)(self as *const _, forward) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void lineMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn lineMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).lineMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void intraLineMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn intraLineMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).intraLineMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void pageMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn pageMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).pageMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completeScroll (in boolean forward); */
    #[inline]
    pub unsafe fn completeScroll(&self, forward: bool) -> Result<(), nsresult> {

        match ((*self.vtable).completeScroll)(self as *const _, forward) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completeMove (in boolean forward, in boolean extend); */
    #[inline]
    pub unsafe fn completeMove(&self, forward: bool, extend: bool) -> Result<(), nsresult> {

        match ((*self.vtable).completeMove)(self as *const _, forward, extend) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollPage (in boolean forward); */
    #[inline]
    pub unsafe fn scrollPage(&self, forward: bool) -> Result<(), nsresult> {

        match ((*self.vtable).scrollPage)(self as *const _, forward) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollLine (in boolean forward); */
    #[inline]
    pub unsafe fn scrollLine(&self, forward: bool) -> Result<(), nsresult> {

        match ((*self.vtable).scrollLine)(self as *const _, forward) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void scrollCharacter (in boolean right); */
    #[inline]
    pub unsafe fn scrollCharacter(&self, right: bool) -> Result<(), nsresult> {

        match ((*self.vtable).scrollCharacter)(self as *const _, right) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* boolean checkVisibility (in nsIDOMNode node, in short startOffset, in short endOffset); */
    #[inline]
    pub unsafe fn checkVisibility(&self, node: Option<&nsIDOMNode>, startOffset: libc::int16_t, endOffset: libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkVisibility)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), startOffset, endOffset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset); */
    #[inline]
    pub unsafe fn checkVisibilityContent(&self, node: Option<&nsIContent>, startOffset: libc::int16_t, endOffset: libc::int16_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).checkVisibilityContent)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _), startOffset, endOffset, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


