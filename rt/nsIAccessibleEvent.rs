//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAccessibleEvent.idl
//


pub mod nsIAccessibleEvent_consts {
    pub const EVENT_SHOW: i64 = 1;
    pub const EVENT_HIDE: i64 = 2;
    pub const EVENT_REORDER: i64 = 3;
    pub const EVENT_ACTIVE_DECENDENT_CHANGED: i64 = 4;
    pub const EVENT_FOCUS: i64 = 5;
    pub const EVENT_STATE_CHANGE: i64 = 6;
    pub const EVENT_LOCATION_CHANGE: i64 = 7;
    pub const EVENT_NAME_CHANGE: i64 = 8;
    pub const EVENT_DESCRIPTION_CHANGE: i64 = 9;
    pub const EVENT_VALUE_CHANGE: i64 = 10;
    pub const EVENT_HELP_CHANGE: i64 = 11;
    pub const EVENT_DEFACTION_CHANGE: i64 = 12;
    pub const EVENT_ACTION_CHANGE: i64 = 13;
    pub const EVENT_ACCELERATOR_CHANGE: i64 = 14;
    pub const EVENT_SELECTION: i64 = 15;
    pub const EVENT_SELECTION_ADD: i64 = 16;
    pub const EVENT_SELECTION_REMOVE: i64 = 17;
    pub const EVENT_SELECTION_WITHIN: i64 = 18;
    pub const EVENT_ALERT: i64 = 19;
    pub const EVENT_FOREGROUND: i64 = 20;
    pub const EVENT_MENU_START: i64 = 21;
    pub const EVENT_MENU_END: i64 = 22;
    pub const EVENT_MENUPOPUP_START: i64 = 23;
    pub const EVENT_MENUPOPUP_END: i64 = 24;
    pub const EVENT_CAPTURE_START: i64 = 25;
    pub const EVENT_CAPTURE_END: i64 = 26;
    pub const EVENT_MOVESIZE_START: i64 = 27;
    pub const EVENT_MOVESIZE_END: i64 = 28;
    pub const EVENT_CONTEXTHELP_START: i64 = 29;
    pub const EVENT_CONTEXTHELP_END: i64 = 30;
    pub const EVENT_DRAGDROP_START: i64 = 31;
    pub const EVENT_DRAGDROP_END: i64 = 32;
    pub const EVENT_DIALOG_START: i64 = 33;
    pub const EVENT_DIALOG_END: i64 = 34;
    pub const EVENT_SCROLLING_START: i64 = 35;
    pub const EVENT_SCROLLING_END: i64 = 36;
    pub const EVENT_MINIMIZE_START: i64 = 37;
    pub const EVENT_MINIMIZE_END: i64 = 38;
    pub const EVENT_DOCUMENT_LOAD_COMPLETE: i64 = 39;
    pub const EVENT_DOCUMENT_RELOAD: i64 = 40;
    pub const EVENT_DOCUMENT_LOAD_STOPPED: i64 = 41;
    pub const EVENT_DOCUMENT_ATTRIBUTES_CHANGED: i64 = 42;
    pub const EVENT_DOCUMENT_CONTENT_CHANGED: i64 = 43;
    pub const EVENT_PROPERTY_CHANGED: i64 = 44;
    pub const EVENT_PAGE_CHANGED: i64 = 45;
    pub const EVENT_TEXT_ATTRIBUTE_CHANGED: i64 = 46;
    pub const EVENT_TEXT_CARET_MOVED: i64 = 47;
    pub const EVENT_TEXT_CHANGED: i64 = 48;
    pub const EVENT_TEXT_INSERTED: i64 = 49;
    pub const EVENT_TEXT_REMOVED: i64 = 50;
    pub const EVENT_TEXT_UPDATED: i64 = 51;
    pub const EVENT_TEXT_SELECTION_CHANGED: i64 = 52;
    pub const EVENT_VISIBLE_DATA_CHANGED: i64 = 53;
    pub const EVENT_TEXT_COLUMN_CHANGED: i64 = 54;
    pub const EVENT_SECTION_CHANGED: i64 = 55;
    pub const EVENT_TABLE_CAPTION_CHANGED: i64 = 56;
    pub const EVENT_TABLE_MODEL_CHANGED: i64 = 57;
    pub const EVENT_TABLE_SUMMARY_CHANGED: i64 = 58;
    pub const EVENT_TABLE_ROW_DESCRIPTION_CHANGED: i64 = 59;
    pub const EVENT_TABLE_ROW_HEADER_CHANGED: i64 = 60;
    pub const EVENT_TABLE_ROW_INSERT: i64 = 61;
    pub const EVENT_TABLE_ROW_DELETE: i64 = 62;
    pub const EVENT_TABLE_ROW_REORDER: i64 = 63;
    pub const EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED: i64 = 64;
    pub const EVENT_TABLE_COLUMN_HEADER_CHANGED: i64 = 65;
    pub const EVENT_TABLE_COLUMN_INSERT: i64 = 66;
    pub const EVENT_TABLE_COLUMN_DELETE: i64 = 67;
    pub const EVENT_TABLE_COLUMN_REORDER: i64 = 68;
    pub const EVENT_WINDOW_ACTIVATE: i64 = 69;
    pub const EVENT_WINDOW_CREATE: i64 = 70;
    pub const EVENT_WINDOW_DEACTIVATE: i64 = 71;
    pub const EVENT_WINDOW_DESTROY: i64 = 72;
    pub const EVENT_WINDOW_MAXIMIZE: i64 = 73;
    pub const EVENT_WINDOW_MINIMIZE: i64 = 74;
    pub const EVENT_WINDOW_RESIZE: i64 = 75;
    pub const EVENT_WINDOW_RESTORE: i64 = 76;
    pub const EVENT_HYPERLINK_END_INDEX_CHANGED: i64 = 77;
    pub const EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED: i64 = 78;
    pub const EVENT_HYPERLINK_SELECTED_LINK_CHANGED: i64 = 79;
    pub const EVENT_HYPERTEXT_LINK_ACTIVATED: i64 = 80;
    pub const EVENT_HYPERTEXT_LINK_SELECTED: i64 = 81;
    pub const EVENT_HYPERLINK_START_INDEX_CHANGED: i64 = 82;
    pub const EVENT_HYPERTEXT_CHANGED: i64 = 83;
    pub const EVENT_HYPERTEXT_NLINKS_CHANGED: i64 = 84;
    pub const EVENT_OBJECT_ATTRIBUTE_CHANGED: i64 = 85;
    pub const EVENT_VIRTUALCURSOR_CHANGED: i64 = 86;
    pub const EVENT_TEXT_VALUE_CHANGE: i64 = 87;
    pub const EVENT_LAST_ENTRY: i64 = 88;
}


#[repr(C)]
pub struct nsIAccessibleEvent {
    vtable: *const nsIAccessibleEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAccessibleEvent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x20c69a40, 0x6c2c, 0x42a3,
            [0xa5, 0x78, 0x6f, 0x44, 0x73, 0xaa, 0xb9, 0xdd])
    }
}

unsafe impl RefCounted for nsIAccessibleEvent {
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
pub trait nsIAccessibleEventCoerce {
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self;
}

impl nsIAccessibleEventCoerce for nsIAccessibleEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self {
        v
    }
}

impl nsIAccessibleEvent {
    #[inline]
    pub fn coerce<T: nsIAccessibleEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAccessibleEvent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAccessibleEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAccessibleEventVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long eventType; */
    pub get_eventType: unsafe extern "C" fn (this: *const nsIAccessibleEvent, aEventType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIAccessible accessible; */
    pub get_accessible: unsafe extern "C" fn (this: *const nsIAccessibleEvent, aAccessible: *mut *const nsIAccessible) -> nsresult,

    /* readonly attribute nsIAccessibleDocument accessibleDocument; */
    pub get_accessibleDocument: unsafe extern "C" fn (this: *const nsIAccessibleEvent, aAccessibleDocument: *mut *const nsIAccessibleDocument) -> nsresult,

    /* readonly attribute nsIDOMNode DOMNode; */
    pub get_DOMNode: unsafe extern "C" fn (this: *const nsIAccessibleEvent, aDOMNode: *mut *const nsIDOMNode) -> nsresult,

    /* readonly attribute boolean isFromUserInput; */
    pub get_isFromUserInput: unsafe extern "C" fn (this: *const nsIAccessibleEvent, aIsFromUserInput: *mut bool) -> nsresult,

}


impl nsIAccessibleEvent {
    /* readonly attribute unsigned long eventType; */
    #[inline]
    pub unsafe fn get_eventType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_eventType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIAccessible accessible; */
    #[inline]
    pub unsafe fn get_accessible(&self, ) -> Result<Option<RefPtr<nsIAccessible>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_accessible)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIAccessibleDocument accessibleDocument; */
    #[inline]
    pub unsafe fn get_accessibleDocument(&self, ) -> Result<Option<RefPtr<nsIAccessibleDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_accessibleDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIDOMNode DOMNode; */
    #[inline]
    pub unsafe fn get_DOMNode(&self, ) -> Result<Option<RefPtr<nsIDOMNode>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DOMNode)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean isFromUserInput; */
    #[inline]
    pub unsafe fn get_isFromUserInput(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isFromUserInput)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


