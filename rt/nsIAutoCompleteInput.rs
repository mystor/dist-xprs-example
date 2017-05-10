//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteInput.idl
//


pub mod nsIAutoCompleteInput_consts {
    pub const TEXTVALUE_REASON_UNKNOWN: i64 = 0;
    pub const TEXTVALUE_REASON_COMPLETEDEFAULT: i64 = 1;
    pub const TEXTVALUE_REASON_COMPLETESELECTED: i64 = 2;
    pub const TEXTVALUE_REASON_REVERT: i64 = 3;
    pub const TEXTVALUE_REASON_ENTERMATCH: i64 = 4;
}


#[repr(C)]
pub struct nsIAutoCompleteInput {
    vtable: *const nsIAutoCompleteInputVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteInput {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb068e70f, 0xf82c, 0x4c12,
            [0xad, 0x87, 0x82, 0xe2, 0x71, 0xc5, 0xc1, 0x80])
    }
}

unsafe impl RefCounted for nsIAutoCompleteInput {
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
pub trait nsIAutoCompleteInputCoerce {
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self;
}

impl nsIAutoCompleteInputCoerce for nsIAutoCompleteInput {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self {
        v
    }
}

impl nsIAutoCompleteInput {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteInputCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteInput {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteInputCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteInputVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAutoCompletePopup popup; */
    pub get_popup: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aPopup: *mut *const nsIAutoCompletePopup) -> nsresult,

    /* readonly attribute nsIAutoCompleteController controller; */
    pub get_controller: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aController: *mut *const nsIAutoCompleteController) -> nsresult,

    /* attribute boolean popupOpen; */
    pub get_popupOpen: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aPopupOpen: *mut bool) -> nsresult,
    pub set_popupOpen: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aPopupOpen: bool) -> nsresult,

    /* attribute boolean disableAutoComplete; */
    pub get_disableAutoComplete: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aDisableAutoComplete: *mut bool) -> nsresult,
    pub set_disableAutoComplete: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aDisableAutoComplete: bool) -> nsresult,

    /* attribute boolean completeDefaultIndex; */
    pub get_completeDefaultIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aCompleteDefaultIndex: *mut bool) -> nsresult,
    pub set_completeDefaultIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aCompleteDefaultIndex: bool) -> nsresult,

    /* attribute boolean completeSelectedIndex; */
    pub get_completeSelectedIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aCompleteSelectedIndex: *mut bool) -> nsresult,
    pub set_completeSelectedIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aCompleteSelectedIndex: bool) -> nsresult,

    /* attribute boolean forceComplete; */
    pub get_forceComplete: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aForceComplete: *mut bool) -> nsresult,
    pub set_forceComplete: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aForceComplete: bool) -> nsresult,

    /* attribute unsigned long minResultsForPopup; */
    pub get_minResultsForPopup: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aMinResultsForPopup: *mut libc::uint32_t) -> nsresult,
    pub set_minResultsForPopup: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aMinResultsForPopup: libc::uint32_t) -> nsresult,

    /* attribute unsigned long maxRows; */
    pub get_maxRows: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aMaxRows: *mut libc::uint32_t) -> nsresult,
    pub set_maxRows: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aMaxRows: libc::uint32_t) -> nsresult,

    /* attribute boolean showCommentColumn; */
    pub get_showCommentColumn: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aShowCommentColumn: *mut bool) -> nsresult,
    pub set_showCommentColumn: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aShowCommentColumn: bool) -> nsresult,

    /* attribute boolean showImageColumn; */
    pub get_showImageColumn: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aShowImageColumn: *mut bool) -> nsresult,
    pub set_showImageColumn: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aShowImageColumn: bool) -> nsresult,

    /* attribute unsigned long timeout; */
    pub get_timeout: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aTimeout: *mut libc::uint32_t) -> nsresult,
    pub set_timeout: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aTimeout: libc::uint32_t) -> nsresult,

    /* attribute AString searchParam; */
    pub get_searchParam: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aSearchParam: *mut nsAString) -> nsresult,
    pub set_searchParam: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aSearchParam: *const nsAString) -> nsresult,

    /* readonly attribute unsigned long searchCount; */
    pub get_searchCount: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aSearchCount: *mut libc::uint32_t) -> nsresult,

    /* ACString getSearchAt (in unsigned long index); */
    pub getSearchAt: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, index: libc::uint32_t, _retval: *mut nsACString) -> nsresult,

    /* attribute AString textValue; */
    pub get_textValue: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aTextValue: *mut nsAString) -> nsresult,
    pub set_textValue: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aTextValue: *const nsAString) -> nsresult,

    /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
    pub setTextValueWithReason: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aValue: *const nsAString, aReason: libc::uint16_t) -> nsresult,

    /* readonly attribute long selectionStart; */
    pub get_selectionStart: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aSelectionStart: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long selectionEnd; */
    pub get_selectionEnd: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aSelectionEnd: *mut libc::int32_t) -> nsresult,

    /* void selectTextRange (in long startIndex, in long endIndex); */
    pub selectTextRange: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, startIndex: libc::int32_t, endIndex: libc::int32_t) -> nsresult,

    /* void onSearchBegin (); */
    pub onSearchBegin: unsafe extern "C" fn (this: *const nsIAutoCompleteInput) -> nsresult,

    /* void onSearchComplete (); */
    pub onSearchComplete: unsafe extern "C" fn (this: *const nsIAutoCompleteInput) -> nsresult,

    /* boolean onTextEntered ([optional] in nsIDOMEvent aEvent); */
    pub onTextEntered: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aEvent: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* boolean onTextReverted (); */
    pub onTextReverted: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean consumeRollupEvent; */
    pub get_consumeRollupEvent: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aConsumeRollupEvent: *mut bool) -> nsresult,

    /* readonly attribute boolean inPrivateContext; */
    pub get_inPrivateContext: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aInPrivateContext: *mut bool) -> nsresult,

    /* readonly attribute boolean noRollupOnCaretMove; */
    pub get_noRollupOnCaretMove: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aNoRollupOnCaretMove: *mut bool) -> nsresult,

    /* readonly attribute unsigned long userContextId; */
    pub get_userContextId: unsafe extern "C" fn (this: *const nsIAutoCompleteInput, aUserContextId: *mut libc::uint32_t) -> nsresult,

}


impl nsIAutoCompleteInput {
    /* readonly attribute nsIAutoCompletePopup popup; */
    #[inline]
    pub unsafe fn get_popup(&self, ) -> Result<Option<RefPtr<nsIAutoCompletePopup>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_popup)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAutoCompleteController controller; */
    #[inline]
    pub unsafe fn get_controller(&self, ) -> Result<Option<RefPtr<nsIAutoCompleteController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_controller)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute boolean popupOpen; */
    #[inline]
    pub unsafe fn get_popupOpen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_popupOpen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_popupOpen(&self, aPopupOpen: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_popupOpen)(self as *const _, aPopupOpen) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean disableAutoComplete; */
    #[inline]
    pub unsafe fn get_disableAutoComplete(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disableAutoComplete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_disableAutoComplete(&self, aDisableAutoComplete: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_disableAutoComplete)(self as *const _, aDisableAutoComplete) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean completeDefaultIndex; */
    #[inline]
    pub unsafe fn get_completeDefaultIndex(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_completeDefaultIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_completeDefaultIndex(&self, aCompleteDefaultIndex: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_completeDefaultIndex)(self as *const _, aCompleteDefaultIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean completeSelectedIndex; */
    #[inline]
    pub unsafe fn get_completeSelectedIndex(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_completeSelectedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_completeSelectedIndex(&self, aCompleteSelectedIndex: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_completeSelectedIndex)(self as *const _, aCompleteSelectedIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean forceComplete; */
    #[inline]
    pub unsafe fn get_forceComplete(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceComplete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_forceComplete(&self, aForceComplete: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_forceComplete)(self as *const _, aForceComplete) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long minResultsForPopup; */
    #[inline]
    pub unsafe fn get_minResultsForPopup(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_minResultsForPopup)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_minResultsForPopup(&self, aMinResultsForPopup: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_minResultsForPopup)(self as *const _, aMinResultsForPopup) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long maxRows; */
    #[inline]
    pub unsafe fn get_maxRows(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxRows)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_maxRows(&self, aMaxRows: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_maxRows)(self as *const _, aMaxRows) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showCommentColumn; */
    #[inline]
    pub unsafe fn get_showCommentColumn(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showCommentColumn)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showCommentColumn(&self, aShowCommentColumn: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showCommentColumn)(self as *const _, aShowCommentColumn) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean showImageColumn; */
    #[inline]
    pub unsafe fn get_showImageColumn(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_showImageColumn)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_showImageColumn(&self, aShowImageColumn: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_showImageColumn)(self as *const _, aShowImageColumn) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long timeout; */
    #[inline]
    pub unsafe fn get_timeout(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_timeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timeout(&self, aTimeout: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_timeout)(self as *const _, aTimeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AString searchParam; */
    #[inline]
    pub unsafe fn get_searchParam(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchParam)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchParam(&self, aSearchParam: &[u16]) -> Result<(), nsresult> {
        let aSearchParam = nsString::from(aSearchParam);
        match ((*self.vtable).set_searchParam)(self as *const _, &*aSearchParam) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long searchCount; */
    #[inline]
    pub unsafe fn get_searchCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_searchCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getSearchAt (in unsigned long index); */
    #[inline]
    pub unsafe fn getSearchAt(&self, index: libc::uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getSearchAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString textValue; */
    #[inline]
    pub unsafe fn get_textValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_textValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_textValue(&self, aTextValue: &[u16]) -> Result<(), nsresult> {
        let aTextValue = nsString::from(aTextValue);
        match ((*self.vtable).set_textValue)(self as *const _, &*aTextValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
    #[inline]
    pub unsafe fn setTextValueWithReason(&self, aValue: &[u16], aReason: libc::uint16_t) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setTextValueWithReason)(self as *const _, &*aValue, aReason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute long selectionStart; */
    #[inline]
    pub unsafe fn get_selectionStart(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionStart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long selectionEnd; */
    #[inline]
    pub unsafe fn get_selectionEnd(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectionEnd)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void selectTextRange (in long startIndex, in long endIndex); */
    #[inline]
    pub unsafe fn selectTextRange(&self, startIndex: libc::int32_t, endIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).selectTextRange)(self as *const _, startIndex, endIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSearchBegin (); */
    #[inline]
    pub unsafe fn onSearchBegin(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchBegin)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSearchComplete (); */
    #[inline]
    pub unsafe fn onSearchComplete(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchComplete)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean onTextEntered ([optional] in nsIDOMEvent aEvent); */
    #[inline]
    pub unsafe fn onTextEntered(&self, aEvent: Option<&nsIDOMEvent>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onTextEntered)(self as *const _, aEvent.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean onTextReverted (); */
    #[inline]
    pub unsafe fn onTextReverted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onTextReverted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean consumeRollupEvent; */
    #[inline]
    pub unsafe fn get_consumeRollupEvent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_consumeRollupEvent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean inPrivateContext; */
    #[inline]
    pub unsafe fn get_inPrivateContext(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inPrivateContext)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean noRollupOnCaretMove; */
    #[inline]
    pub unsafe fn get_noRollupOnCaretMove(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_noRollupOnCaretMove)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long userContextId; */
    #[inline]
    pub unsafe fn get_userContextId(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_userContextId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


