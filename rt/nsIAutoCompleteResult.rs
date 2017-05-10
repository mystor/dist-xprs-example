//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteResult.idl
//


pub mod nsIAutoCompleteResult_consts {
    pub const RESULT_IGNORED: i64 = 1;
    pub const RESULT_FAILURE: i64 = 2;
    pub const RESULT_NOMATCH: i64 = 3;
    pub const RESULT_SUCCESS: i64 = 4;
    pub const RESULT_NOMATCH_ONGOING: i64 = 5;
    pub const RESULT_SUCCESS_ONGOING: i64 = 6;
}


#[repr(C)]
pub struct nsIAutoCompleteResult {
    vtable: *const nsIAutoCompleteResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9203c031, 0xc4e7, 0x4537,
            [0xa4, 0xec, 0x81, 0x44, 0x3d, 0x62, 0x3d, 0x5a])
    }
}

unsafe impl RefCounted for nsIAutoCompleteResult {
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
pub trait nsIAutoCompleteResultCoerce {
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self;
}

impl nsIAutoCompleteResultCoerce for nsIAutoCompleteResult {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self {
        v
    }
}

impl nsIAutoCompleteResult {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString searchString; */
    pub get_searchString: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, aSearchString: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned short searchResult; */
    pub get_searchResult: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, aSearchResult: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute long defaultIndex; */
    pub get_defaultIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, aDefaultIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute AString errorDescription; */
    pub get_errorDescription: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, aErrorDescription: *mut nsAString) -> nsresult,

    /* readonly attribute unsigned long matchCount; */
    pub get_matchCount: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, aMatchCount: *mut libc::uint32_t) -> nsresult,

    /* AString getValueAt (in long index); */
    pub getValueAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getLabelAt (in long index); */
    pub getLabelAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getCommentAt (in long index); */
    pub getCommentAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getStyleAt (in long index); */
    pub getStyleAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getImageAt (in long index); */
    pub getImageAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* AString getFinalCompleteValueAt (in long index); */
    pub getFinalCompleteValueAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, index: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* void removeValueAt (in long rowIndex, in boolean removeFromDb); */
    pub removeValueAt: unsafe extern "C" fn (this: *const nsIAutoCompleteResult, rowIndex: libc::int32_t, removeFromDb: bool) -> nsresult,

}


impl nsIAutoCompleteResult {
    /* readonly attribute AString searchString; */
    #[inline]
    pub unsafe fn get_searchString(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchString)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned short searchResult; */
    #[inline]
    pub unsafe fn get_searchResult(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_searchResult)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long defaultIndex; */
    #[inline]
    pub unsafe fn get_defaultIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString errorDescription; */
    #[inline]
    pub unsafe fn get_errorDescription(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_errorDescription)(self as *const _, &mut *_retval) {
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

    /* void removeValueAt (in long rowIndex, in boolean removeFromDb); */
    #[inline]
    pub unsafe fn removeValueAt(&self, rowIndex: libc::int32_t, removeFromDb: bool) -> Result<(), nsresult> {

        match ((*self.vtable).removeValueAt)(self as *const _, rowIndex, removeFromDb) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


