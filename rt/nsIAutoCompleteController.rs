//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteController.idl
//


pub mod nsIAutoCompleteController_consts {
    pub const STATUS_NONE: i64 = 1;
    pub const STATUS_SEARCHING: i64 = 2;
    pub const STATUS_COMPLETE_NO_MATCH: i64 = 3;
    pub const STATUS_COMPLETE_MATCH: i64 = 4;
}


#[repr(C)]
pub struct nsIAutoCompleteController {
    vtable: *const nsIAutoCompleteControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xff9f8465, 0x204a, 0x47a6,
            [0xb3, 0xc9, 0x06, 0x28, 0xb3, 0x85, 0x66, 0x84])
    }
}

unsafe impl RefCounted for nsIAutoCompleteController {
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
pub trait nsIAutoCompleteControllerCoerce {
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self;
}

impl nsIAutoCompleteControllerCoerce for nsIAutoCompleteController {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self {
        v
    }
}

impl nsIAutoCompleteController {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteControllerVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIAutoCompleteInput input; */
    pub get_input: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aInput: *mut *const nsIAutoCompleteInput) -> nsresult,
    pub set_input: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aInput: *const nsIAutoCompleteInput) -> nsresult,

    /* readonly attribute unsigned short searchStatus; */
    pub get_searchStatus: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aSearchStatus: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute unsigned long matchCount; */
    pub get_matchCount: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aMatchCount: *mut libc::uint32_t) -> nsresult,

    /* void startSearch (in AString searchString); */
    pub startSearch: unsafe extern "C" fn (this: *const nsIAutoCompleteController, searchString: *const nsAString) -> nsresult,

    /* void stopSearch (); */
    pub stopSearch: unsafe extern "C" fn (this: *const nsIAutoCompleteController) -> nsresult,

    /* boolean handleText (); */
    pub handleText: unsafe extern "C" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> nsresult,

    /* boolean handleEnter (in boolean aIsPopupSelection, [optional] in nsIDOMEvent aEvent); */
    pub handleEnter: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aIsPopupSelection: bool, aEvent: *const nsIDOMEvent, _retval: *mut bool) -> nsresult,

    /* boolean handleEscape (); */
    pub handleEscape: unsafe extern "C" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> nsresult,

    /* void handleStartComposition (); */
    pub handleStartComposition: unsafe extern "C" fn (this: *const nsIAutoCompleteController) -> nsresult,

    /* void handleEndComposition (); */
    pub handleEndComposition: unsafe extern "C" fn (this: *const nsIAutoCompleteController) -> nsresult,

    /* void handleTab (); */
    pub handleTab: unsafe extern "C" fn (this: *const nsIAutoCompleteController) -> nsresult,

    /* boolean handleKeyNavigation (in unsigned long key); */
    pub handleKeyNavigation: unsafe extern "C" fn (this: *const nsIAutoCompleteController, key: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean handleDelete (); */
    pub handleDelete: unsafe extern "C" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> nsresult,

    /* AString getValueAt (in long index); */
    pub getValueAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getLabelAt (in long index); */
    pub getLabelAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getCommentAt (in long index); */
    pub getCommentAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getStyleAt (in long index); */
    pub getStyleAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getImageAt (in long index); */
    pub getImageAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getFinalCompleteValueAt (in long index); */
    pub getFinalCompleteValueAt: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* attribute AString searchString; */
    pub get_searchString: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aSearchString: *mut nsAString) -> nsresult,
    pub set_searchString: unsafe extern "C" fn (this: *const nsIAutoCompleteController, aSearchString: *const nsAString) -> nsresult,

    /* void setInitiallySelectedIndex (in long index); */
    pub setInitiallySelectedIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteController, index: libc::int32_t) -> nsresult,

}


impl nsIAutoCompleteController {
    /* attribute nsIAutoCompleteInput input; */
    #[inline]
    pub unsafe fn get_input(&self, ) -> Result<Option<RefPtr<nsIAutoCompleteInput>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_input)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_input(&self, aInput: Option<&nsIAutoCompleteInput>) -> Result<(), nsresult> {

        match ((*self.vtable).set_input)(self as *const _, aInput.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned short searchStatus; */
    #[inline]
    pub unsafe fn get_searchStatus(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_searchStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long matchCount; */
    #[inline]
    pub unsafe fn get_matchCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_matchCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void startSearch (in AString searchString); */
    #[inline]
    pub unsafe fn startSearch(&self, searchString: &[u16]) -> Result<(), nsresult> {
        let searchString = nsString::from(searchString);
        match ((*self.vtable).startSearch)(self as *const _, &*searchString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopSearch (); */
    #[inline]
    pub unsafe fn stopSearch(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopSearch)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean handleText (); */
    #[inline]
    pub unsafe fn handleText(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleText)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean handleEnter (in boolean aIsPopupSelection, [optional] in nsIDOMEvent aEvent); */
    #[inline]
    pub unsafe fn handleEnter(&self, aIsPopupSelection: bool, aEvent: Option<&nsIDOMEvent>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleEnter)(self as *const _, aIsPopupSelection, aEvent.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean handleEscape (); */
    #[inline]
    pub unsafe fn handleEscape(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleEscape)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void handleStartComposition (); */
    #[inline]
    pub unsafe fn handleStartComposition(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).handleStartComposition)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleEndComposition (); */
    #[inline]
    pub unsafe fn handleEndComposition(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).handleEndComposition)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleTab (); */
    #[inline]
    pub unsafe fn handleTab(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).handleTab)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean handleKeyNavigation (in unsigned long key); */
    #[inline]
    pub unsafe fn handleKeyNavigation(&self, key: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleKeyNavigation)(self as *const _, key, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean handleDelete (); */
    #[inline]
    pub unsafe fn handleDelete(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).handleDelete)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getValueAt (in long index); */
    #[inline]
    pub unsafe fn getValueAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getValueAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getLabelAt (in long index); */
    #[inline]
    pub unsafe fn getLabelAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getLabelAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getCommentAt (in long index); */
    #[inline]
    pub unsafe fn getCommentAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getCommentAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getStyleAt (in long index); */
    #[inline]
    pub unsafe fn getStyleAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getStyleAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getImageAt (in long index); */
    #[inline]
    pub unsafe fn getImageAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getImageAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getFinalCompleteValueAt (in long index); */
    #[inline]
    pub unsafe fn getFinalCompleteValueAt(&self, index: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getFinalCompleteValueAt)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString searchString; */
    #[inline]
    pub unsafe fn get_searchString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_searchString(&self, aSearchString: &[u16]) -> Result<(), nsresult> {
        let aSearchString = nsString::from(aSearchString);
        match ((*self.vtable).set_searchString)(self as *const _, &*aSearchString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setInitiallySelectedIndex (in long index); */
    #[inline]
    pub unsafe fn setInitiallySelectedIndex(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setInitiallySelectedIndex)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


