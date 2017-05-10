//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompletePopup.idl
//


pub mod nsIAutoCompletePopup_consts {
    pub const INVALIDATE_REASON_NEW_RESULT: i64 = 0;
    pub const INVALIDATE_REASON_DELETE: i64 = 1;
}


#[repr(C)]
pub struct nsIAutoCompletePopup {
    vtable: *const nsIAutoCompletePopupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompletePopup {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbd3c2662, 0xa988, 0x41ab,
            [0x8c, 0x94, 0xc1, 0x5e, 0xd0, 0xe6, 0xac, 0x7d])
    }
}

unsafe impl RefCounted for nsIAutoCompletePopup {
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
pub trait nsIAutoCompletePopupCoerce {
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self;
}

impl nsIAutoCompletePopupCoerce for nsIAutoCompletePopup {
    #[inline]
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self {
        v
    }
}

impl nsIAutoCompletePopup {
    #[inline]
    pub fn coerce<T: nsIAutoCompletePopupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompletePopup {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompletePopupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompletePopupVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAutoCompleteInput input; */
    pub get_input: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, aInput: *mut *const nsIAutoCompleteInput) -> nsresult,

    /* readonly attribute AString overrideValue; */
    pub get_overrideValue: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, aOverrideValue: *mut nsAString) -> nsresult,

    /* attribute long selectedIndex; */
    pub get_selectedIndex: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, aSelectedIndex: *mut libc::int32_t) -> nsresult,
    pub set_selectedIndex: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, aSelectedIndex: libc::int32_t) -> nsresult,

    /* readonly attribute boolean popupOpen; */
    pub get_popupOpen: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, aPopupOpen: *mut bool) -> nsresult,

    /* void openAutocompletePopup (in nsIAutoCompleteInput input, in nsIDOMElement element); */
    pub openAutocompletePopup: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, input: *const nsIAutoCompleteInput, element: *const nsIDOMElement) -> nsresult,

    /* void closePopup (); */
    pub closePopup: unsafe extern "C" fn (this: *const nsIAutoCompletePopup) -> nsresult,

    /* void invalidate (in unsigned short reason); */
    pub invalidate: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, reason: libc::uint16_t) -> nsresult,

    /* void selectBy (in boolean reverse, in boolean page); */
    pub selectBy: unsafe extern "C" fn (this: *const nsIAutoCompletePopup, reverse: bool, page: bool) -> nsresult,

}


impl nsIAutoCompletePopup {
    /* readonly attribute nsIAutoCompleteInput input; */
    #[inline]
    pub unsafe fn get_input(&self, ) -> Result<Option<RefPtr<nsIAutoCompleteInput>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_input)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString overrideValue; */
    #[inline]
    pub unsafe fn get_overrideValue(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_overrideValue)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute long selectedIndex; */
    #[inline]
    pub unsafe fn get_selectedIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_selectedIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_selectedIndex(&self, aSelectedIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedIndex)(self as *const _, aSelectedIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean popupOpen; */
    #[inline]
    pub unsafe fn get_popupOpen(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_popupOpen)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void openAutocompletePopup (in nsIAutoCompleteInput input, in nsIDOMElement element); */
    #[inline]
    pub unsafe fn openAutocompletePopup(&self, input: Option<&nsIAutoCompleteInput>, element: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).openAutocompletePopup)(self as *const _, input.map_or(::std::ptr::null(), |x| x as *const _), element.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void closePopup (); */
    #[inline]
    pub unsafe fn closePopup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).closePopup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void invalidate (in unsigned short reason); */
    #[inline]
    pub unsafe fn invalidate(&self, reason: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).invalidate)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void selectBy (in boolean reverse, in boolean page); */
    #[inline]
    pub unsafe fn selectBy(&self, reverse: bool, page: bool) -> Result<(), nsresult> {

        match ((*self.vtable).selectBy)(self as *const _, reverse, page) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


