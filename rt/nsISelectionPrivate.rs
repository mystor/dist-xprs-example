//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISelectionPrivate.idl
//


pub mod nsISelectionPrivate_consts {
    pub const ENDOFPRECEDINGLINE: i64 = 0;
    pub const STARTOFNEXTLINE: i64 = 1;
    pub const TABLESELECTION_NONE: i64 = 0;
    pub const TABLESELECTION_CELL: i64 = 1;
    pub const TABLESELECTION_ROW: i64 = 2;
    pub const TABLESELECTION_COLUMN: i64 = 3;
    pub const TABLESELECTION_TABLE: i64 = 4;
    pub const TABLESELECTION_ALLCELLS: i64 = 5;
}


#[repr(C)]
pub struct nsISelectionPrivate {
    vtable: *const nsISelectionPrivateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISelectionPrivate {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0c9f4f74, 0xee7e, 0x4fe9,
            [0xbe, 0x6b, 0x0b, 0xa8, 0x56, 0x36, 0x81, 0x78])
    }
}

unsafe impl RefCounted for nsISelectionPrivate {
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
pub trait nsISelectionPrivateCoerce {
    fn coerce_from(v: &nsISelectionPrivate) -> &Self;
}

impl nsISelectionPrivateCoerce for nsISelectionPrivate {
    #[inline]
    fn coerce_from(v: &nsISelectionPrivate) -> &Self {
        v
    }
}

impl nsISelectionPrivate {
    #[inline]
    pub fn coerce<T: nsISelectionPrivateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISelectionPrivate {
    type Target = nsISelection;
    #[inline]
    fn deref(&self) -> &nsISelection {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISelectionCoerce> nsISelectionPrivateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionPrivate) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISelectionPrivateVTable {
    pub __base: nsISelectionVTable,

    /* attribute boolean interlinePosition; */
    pub get_interlinePosition: unsafe extern "C" fn (this: *const nsISelectionPrivate, aInterlinePosition: *mut bool) -> nsresult,
    pub set_interlinePosition: unsafe extern "C" fn (this: *const nsISelectionPrivate, aInterlinePosition: bool) -> nsresult,

    /* [noscript] attribute nsIContent ancestorLimiter; */
    pub get_ancestorLimiter: unsafe extern "C" fn (this: *const nsISelectionPrivate, aAncestorLimiter: *mut *const nsIContent) -> nsresult,
    pub set_ancestorLimiter: unsafe extern "C" fn (this: *const nsISelectionPrivate, aAncestorLimiter: *const nsIContent) -> nsresult,

    /* [noscript] void startBatchChanges (); */
    pub startBatchChanges: unsafe extern "C" fn (this: *const nsISelectionPrivate) -> nsresult,

    /* [noscript] void endBatchChanges (); */
    pub endBatchChanges: unsafe extern "C" fn (this: *const nsISelectionPrivate) -> nsresult,

    /* DOMString toStringWithFormat (in string formatType, in unsigned long flags, in int32_t wrapColumn); */
    pub toStringWithFormat: unsafe extern "C" fn (this: *const nsISelectionPrivate, formatType: *const libc::c_char, flags: libc::uint32_t, wrapColumn: int32_t, _retval: *mut nsAString) -> nsresult,

    /* void addSelectionListener (in nsISelectionListener newListener); */
    pub addSelectionListener: unsafe extern "C" fn (this: *const nsISelectionPrivate, newListener: *const nsISelectionListener) -> nsresult,

    /* void removeSelectionListener (in nsISelectionListener listenerToRemove); */
    pub removeSelectionListener: unsafe extern "C" fn (this: *const nsISelectionPrivate, listenerToRemove: *const nsISelectionListener) -> nsresult,

    /* [noscript] long getTableSelectionType (in nsIDOMRange range); */
    pub getTableSelectionType: unsafe extern "C" fn (this: *const nsISelectionPrivate, range: *const nsIDOMRange, _retval: *mut libc::int32_t) -> nsresult,

    /* [noscript] attribute boolean canCacheFrameOffset; */
    pub get_canCacheFrameOffset: unsafe extern "C" fn (this: *const nsISelectionPrivate, aCanCacheFrameOffset: *mut bool) -> nsresult,
    pub set_canCacheFrameOffset: unsafe extern "C" fn (this: *const nsISelectionPrivate, aCanCacheFrameOffset: bool) -> nsresult,

    /* [noscript] void getCachedFrameOffset (in nsIFrame aFrame, in int32_t inOffset, in nsPointRef aPoint); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCachedFrameOffset: *const ::libc::c_void,

    /* [noscript] void setTextRangeStyle (in nsIDOMRange range, in constTextRangeStyleRef textRangeStyle); */
    /// Unable to call function as its signature contains a non-rust type
    pub setTextRangeStyle: *const ::libc::c_void,

    /* [noscript,notxpcom] nsDirection getSelectionDirection (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSelectionDirection: *const ::libc::c_void,

    /* [noscript,notxpcom] void setSelectionDirection (in nsDirection aDirection); */
    /// Unable to call function as its signature contains a non-rust type
    pub setSelectionDirection: *const ::libc::c_void,

    /* readonly attribute short type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsISelectionPrivate, aType: *mut libc::int16_t) -> nsresult,

    /* void GetRangesForInterval (in nsIDOMNode beginNode, in int32_t beginOffset, in nsIDOMNode endNode, in int32_t endOffset, in boolean allowAdjacent, out uint32_t resultCount, [array, size_is (resultCount), retval] out nsIDOMRange results); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetRangesForInterval: *const ::libc::c_void,

    /* [noscript] void GetRangesForIntervalArray (in nsINode beginNode, in int32_t beginOffset, in nsINode endNode, in int32_t endOffset, in boolean allowAdjacent, in RangeArray results); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetRangesForIntervalArray: *const ::libc::c_void,

    /* void scrollIntoView (in short aRegion, in boolean aIsSynchronous, in int16_t aVPercent, in int16_t aHPercent); */
    pub scrollIntoView: unsafe extern "C" fn (this: *const nsISelectionPrivate, aRegion: libc::int16_t, aIsSynchronous: bool, aVPercent: int16_t, aHPercent: int16_t) -> nsresult,

    /* [noscript] void scrollIntoViewInternal (in short aRegion, in boolean aIsSynchronous, in ScrollAxis aVertical, in ScrollAxis aHorizontal); */
    /// Unable to call function as its signature contains a non-rust type
    pub scrollIntoViewInternal: *const ::libc::c_void,

    /* [noscript] void selectionLanguageChange (in boolean langRTL); */
    pub selectionLanguageChange: unsafe extern "C" fn (this: *const nsISelectionPrivate, langRTL: bool) -> nsresult,

}


impl nsISelectionPrivate {
    /* attribute boolean interlinePosition; */
    #[inline]
    pub unsafe fn get_interlinePosition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_interlinePosition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_interlinePosition(&self, aInterlinePosition: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_interlinePosition)(self as *const _, aInterlinePosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsIContent ancestorLimiter; */
    #[inline]
    pub unsafe fn get_ancestorLimiter(&self, ) -> Result<Option<RefPtr<nsIContent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ancestorLimiter)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_ancestorLimiter(&self, aAncestorLimiter: Option<&nsIContent>) -> Result<(), nsresult> {

        match ((*self.vtable).set_ancestorLimiter)(self as *const _, aAncestorLimiter.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void startBatchChanges (); */
    #[inline]
    pub unsafe fn startBatchChanges(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).startBatchChanges)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void endBatchChanges (); */
    #[inline]
    pub unsafe fn endBatchChanges(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).endBatchChanges)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMString toStringWithFormat (in string formatType, in unsigned long flags, in int32_t wrapColumn); */
    #[inline]
    pub unsafe fn toStringWithFormat(&self, formatType: *const libc::c_char, flags: libc::uint32_t, wrapColumn: int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toStringWithFormat)(self as *const _, formatType, flags, wrapColumn, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addSelectionListener (in nsISelectionListener newListener); */
    #[inline]
    pub unsafe fn addSelectionListener(&self, newListener: Option<&nsISelectionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addSelectionListener)(self as *const _, newListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeSelectionListener (in nsISelectionListener listenerToRemove); */
    #[inline]
    pub unsafe fn removeSelectionListener(&self, listenerToRemove: Option<&nsISelectionListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeSelectionListener)(self as *const _, listenerToRemove.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] long getTableSelectionType (in nsIDOMRange range); */
    #[inline]
    pub unsafe fn getTableSelectionType(&self, range: Option<&nsIDOMRange>) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTableSelectionType)(self as *const _, range.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] attribute boolean canCacheFrameOffset; */
    #[inline]
    pub unsafe fn get_canCacheFrameOffset(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canCacheFrameOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_canCacheFrameOffset(&self, aCanCacheFrameOffset: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_canCacheFrameOffset)(self as *const _, aCanCacheFrameOffset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getCachedFrameOffset (in nsIFrame aFrame, in int32_t inOffset, in nsPointRef aPoint); */


    /* [noscript] void setTextRangeStyle (in nsIDOMRange range, in constTextRangeStyleRef textRangeStyle); */


    /* [noscript,notxpcom] nsDirection getSelectionDirection (); */


    /* [noscript,notxpcom] void setSelectionDirection (in nsDirection aDirection); */


    /* readonly attribute short type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void GetRangesForInterval (in nsIDOMNode beginNode, in int32_t beginOffset, in nsIDOMNode endNode, in int32_t endOffset, in boolean allowAdjacent, out uint32_t resultCount, [array, size_is (resultCount), retval] out nsIDOMRange results); */


    /* [noscript] void GetRangesForIntervalArray (in nsINode beginNode, in int32_t beginOffset, in nsINode endNode, in int32_t endOffset, in boolean allowAdjacent, in RangeArray results); */


    /* void scrollIntoView (in short aRegion, in boolean aIsSynchronous, in int16_t aVPercent, in int16_t aHPercent); */
    #[inline]
    pub unsafe fn scrollIntoView(&self, aRegion: libc::int16_t, aIsSynchronous: bool, aVPercent: int16_t, aHPercent: int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).scrollIntoView)(self as *const _, aRegion, aIsSynchronous, aVPercent, aHPercent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void scrollIntoViewInternal (in short aRegion, in boolean aIsSynchronous, in ScrollAxis aVertical, in ScrollAxis aHorizontal); */


    /* [noscript] void selectionLanguageChange (in boolean langRTL); */
    #[inline]
    pub unsafe fn selectionLanguageChange(&self, langRTL: bool) -> Result<(), nsresult> {

        match ((*self.vtable).selectionLanguageChange)(self as *const _, langRTL) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


