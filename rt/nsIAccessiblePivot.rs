//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessiblePivot.idl
//


pub type TextBoundaryType = libc::int16_t;


pub type PivotMoveReason = libc::int16_t;


pub mod nsIAccessiblePivot_consts {
    pub const CHAR_BOUNDARY: i64 = 0;
    pub const WORD_BOUNDARY: i64 = 1;
    pub const LINE_BOUNDARY: i64 = 2;
    pub const ATTRIBUTE_RANGE_BOUNDARY: i64 = 3;
    pub const REASON_NONE: i64 = 0;
    pub const REASON_NEXT: i64 = 1;
    pub const REASON_PREV: i64 = 2;
    pub const REASON_FIRST: i64 = 3;
    pub const REASON_LAST: i64 = 4;
    pub const REASON_TEXT: i64 = 5;
    pub const REASON_POINT: i64 = 6;
}


#[repr(C)]
pub struct nsIAccessiblePivot {
    vtable: *const nsIAccessiblePivotVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessiblePivot {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x81fe5144, 0x059b, 0x42db,
            [0xbd, 0x3a, 0xf6, 0xce, 0x31, 0x58, 0xd5, 0xe9])
    }
}

unsafe impl RefCounted for nsIAccessiblePivot {
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
pub trait nsIAccessiblePivotCoerce {
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self;
}

impl nsIAccessiblePivotCoerce for nsIAccessiblePivot {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self {
        v
    }
}

impl nsIAccessiblePivot {
    #[inline]
    pub fn coerce<T: nsIAccessiblePivotCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessiblePivot {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessiblePivotCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessiblePivotVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIAccessible position; */
    pub get_position: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aPosition: *mut *const nsIAccessible) -> nsresult,
    pub set_position: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aPosition: *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessible root; */
    pub get_root: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aRoot: *mut *const nsIAccessible) -> nsresult,

    /* attribute nsIAccessible modalRoot; */
    pub get_modalRoot: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aModalRoot: *mut *const nsIAccessible) -> nsresult,
    pub set_modalRoot: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aModalRoot: *const nsIAccessible) -> nsresult,

    /* readonly attribute long startOffset; */
    pub get_startOffset: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aStartOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long endOffset; */
    pub get_endOffset: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aEndOffset: *mut libc::int32_t) -> nsresult,

    /* [optional_argc] void setTextRange (in nsIAccessibleText aTextAccessible, in long aStartOffset, in long aEndOffset, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub setTextRange: *const ::libc::c_void,

    /* [optional_argc] boolean moveNext (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub moveNext: *const ::libc::c_void,

    /* [optional_argc] boolean movePrevious (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub movePrevious: *const ::libc::c_void,

    /* [optional_argc] boolean moveFirst (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub moveFirst: *const ::libc::c_void,

    /* [optional_argc] boolean moveLast (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub moveLast: *const ::libc::c_void,

    /* [optional_argc] boolean moveNextByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub moveNextByText: *const ::libc::c_void,

    /* [optional_argc] boolean movePreviousByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub movePreviousByText: *const ::libc::c_void,

    /* [optional_argc] boolean moveToPoint (in nsIAccessibleTraversalRule aRule, in long aX, in long aY, in boolean aIgnoreNoMatch, [optional] in boolean aIsFromUserInput); */
    /// Unable to call function as its signature contains a non-rust type
    pub moveToPoint: *const ::libc::c_void,

    /* void addObserver (in nsIAccessiblePivotObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aObserver: *const nsIAccessiblePivotObserver) -> nsresult,

    /* void removeObserver (in nsIAccessiblePivotObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIAccessiblePivot, aObserver: *const nsIAccessiblePivotObserver) -> nsresult,

}


impl nsIAccessiblePivot {
    /* attribute nsIAccessible position; */
    #[inline]
    pub unsafe fn get_position(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_position)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_position(&self, aPosition: Option<&nsIAccessible>) -> Result<(), nsresult> {

        match ((*self.vtable).set_position)(self as *const _, aPosition.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIAccessible root; */
    #[inline]
    pub unsafe fn get_root(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_root)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIAccessible modalRoot; */
    #[inline]
    pub unsafe fn get_modalRoot(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_modalRoot)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_modalRoot(&self, aModalRoot: Option<&nsIAccessible>) -> Result<(), nsresult> {

        match ((*self.vtable).set_modalRoot)(self as *const _, aModalRoot.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* [optional_argc] void setTextRange (in nsIAccessibleText aTextAccessible, in long aStartOffset, in long aEndOffset, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean moveNext (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean movePrevious (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean moveFirst (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean moveLast (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean moveNextByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean movePreviousByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */


    /* [optional_argc] boolean moveToPoint (in nsIAccessibleTraversalRule aRule, in long aX, in long aY, in boolean aIgnoreNoMatch, [optional] in boolean aIsFromUserInput); */


    /* void addObserver (in nsIAccessiblePivotObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsIAccessiblePivotObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIAccessiblePivotObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsIAccessiblePivotObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAccessiblePivotObserver {
    vtable: *const nsIAccessiblePivotObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessiblePivotObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6006e502, 0x3861, 0x49bd,
            [0xab, 0xa1, 0xfa, 0x6d, 0x2e, 0x74, 0xe2, 0x37])
    }
}

unsafe impl RefCounted for nsIAccessiblePivotObserver {
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
pub trait nsIAccessiblePivotObserverCoerce {
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self;
}

impl nsIAccessiblePivotObserverCoerce for nsIAccessiblePivotObserver {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self {
        v
    }
}

impl nsIAccessiblePivotObserver {
    #[inline]
    pub fn coerce<T: nsIAccessiblePivotObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessiblePivotObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessiblePivotObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessiblePivotObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in PivotMoveReason aReason, in boolean aIsFromUserInput); */
    pub onPivotChanged: unsafe extern "C" fn (this: *const nsIAccessiblePivotObserver, aPivot: *const nsIAccessiblePivot, aOldAccessible: *const nsIAccessible, aOldStart: libc::int32_t, aOldEnd: libc::int32_t, aReason: PivotMoveReason, aIsFromUserInput: bool) -> nsresult,

}


impl nsIAccessiblePivotObserver {
    /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in PivotMoveReason aReason, in boolean aIsFromUserInput); */
    #[inline]
    pub unsafe fn onPivotChanged(&self, aPivot: Option<&nsIAccessiblePivot>, aOldAccessible: Option<&nsIAccessible>, aOldStart: libc::int32_t, aOldEnd: libc::int32_t, aReason: PivotMoveReason, aIsFromUserInput: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onPivotChanged)(self as *const _, aPivot.map_or(::std::ptr::null(), |x| x as *const _), aOldAccessible.map_or(::std::ptr::null(), |x| x as *const _), aOldStart, aOldEnd, aReason, aIsFromUserInput) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIAccessibleTraversalRule_consts {
    pub const FILTER_IGNORE: i64 = 0;
    pub const FILTER_MATCH: i64 = 1;
    pub const FILTER_IGNORE_SUBTREE: i64 = 2;
    pub const PREFILTER_INVISIBLE: i64 = 1;
    pub const PREFILTER_OFFSCREEN: i64 = 2;
    pub const PREFILTER_NOT_FOCUSABLE: i64 = 4;
    pub const PREFILTER_ARIA_HIDDEN: i64 = 8;
    pub const PREFILTER_TRANSPARENT: i64 = 16;
}


#[repr(C)]
pub struct nsIAccessibleTraversalRule {
    vtable: *const nsIAccessibleTraversalRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleTraversalRule {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe197460d, 0x1eff, 0x4247,
            [0xb4, 0xbb, 0xa4, 0x3b, 0xe1, 0x84, 0x0d, 0xae])
    }
}

unsafe impl RefCounted for nsIAccessibleTraversalRule {
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
pub trait nsIAccessibleTraversalRuleCoerce {
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self;
}

impl nsIAccessibleTraversalRuleCoerce for nsIAccessibleTraversalRule {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self {
        v
    }
}

impl nsIAccessibleTraversalRule {
    #[inline]
    pub fn coerce<T: nsIAccessibleTraversalRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleTraversalRule {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleTraversalRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleTraversalRuleVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long preFilter; */
    pub get_preFilter: unsafe extern "C" fn (this: *const nsIAccessibleTraversalRule, aPreFilter: *mut libc::uint32_t) -> nsresult,

    /* void getMatchRoles ([array, size_is (aCount)] out unsigned long aRoles, [retval] out unsigned long aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub getMatchRoles: *const ::libc::c_void,

    /* unsigned short match (in nsIAccessible aAccessible); */
    pub match_: unsafe extern "C" fn (this: *const nsIAccessibleTraversalRule, aAccessible: *const nsIAccessible, _retval: *mut libc::uint16_t) -> nsresult,

}


impl nsIAccessibleTraversalRule {
    /* readonly attribute unsigned long preFilter; */
    #[inline]
    pub unsafe fn get_preFilter(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_preFilter)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getMatchRoles ([array, size_is (aCount)] out unsigned long aRoles, [retval] out unsigned long aCount); */


    /* unsigned short match (in nsIAccessible aAccessible); */
    #[inline]
    pub unsafe fn match_(&self, aAccessible: Option<&nsIAccessible>) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).match_)(self as *const _, aAccessible.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


