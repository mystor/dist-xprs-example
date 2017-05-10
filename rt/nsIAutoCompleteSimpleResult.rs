//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteSimpleResult.idl
//


#[repr(C)]
pub struct nsIAutoCompleteSimpleResult {
    vtable: *const nsIAutoCompleteSimpleResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteSimpleResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x23de9c96, 0xbecb, 0x4d0d,
            [0xa9, 0xbb, 0x1d, 0x13, 0x1c, 0xe3, 0x61, 0xb5])
    }
}

unsafe impl RefCounted for nsIAutoCompleteSimpleResult {
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
pub trait nsIAutoCompleteSimpleResultCoerce {
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self;
}

impl nsIAutoCompleteSimpleResultCoerce for nsIAutoCompleteSimpleResult {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self {
        v
    }
}

impl nsIAutoCompleteSimpleResult {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSimpleResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteSimpleResult {
    type Target = nsIAutoCompleteResult;
    #[inline]
    fn deref(&self) -> &nsIAutoCompleteResult {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIAutoCompleteResultCoerce> nsIAutoCompleteSimpleResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteSimpleResultVTable {
    pub __base: nsIAutoCompleteResultVTable,

    /* void setSearchString (in AString aSearchString); */
    pub setSearchString: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aSearchString: *const nsAString) -> nsresult,

    /* void setErrorDescription (in AString aErrorDescription); */
    pub setErrorDescription: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aErrorDescription: *const nsAString) -> nsresult,

    /* void setDefaultIndex (in long aDefaultIndex); */
    pub setDefaultIndex: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aDefaultIndex: libc::int32_t) -> nsresult,

    /* void setSearchResult (in unsigned short aSearchResult); */
    pub setSearchResult: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aSearchResult: libc::uint16_t) -> nsresult,

    /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    pub insertMatchAt: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aIndex: libc::int32_t, aValue: *const nsAString, aComment: *const nsAString, aImage: *const nsAString, aStyle: *const nsAString, aFinalCompleteValue: *const nsAString, aLabel: *const nsAString) -> nsresult,

    /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    pub appendMatch: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aValue: *const nsAString, aComment: *const nsAString, aImage: *const nsAString, aStyle: *const nsAString, aFinalCompleteValue: *const nsAString, aLabel: *const nsAString) -> nsresult,

    /* nsIAutoCompleteSimpleResultListener getListener (); */
    pub getListener: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, _retval: *mut *const nsIAutoCompleteSimpleResultListener) -> nsresult,

    /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
    pub setListener: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResult, aListener: *const nsIAutoCompleteSimpleResultListener) -> nsresult,

}


impl nsIAutoCompleteSimpleResult {
    /* void setSearchString (in AString aSearchString); */
    #[inline]
    pub unsafe fn setSearchString(&self, aSearchString: &[u16]) -> Result<(), nsresult> {
        let aSearchString = nsString::from(aSearchString);
        match ((*self.vtable).setSearchString)(self as *const _, &*aSearchString) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setErrorDescription (in AString aErrorDescription); */
    #[inline]
    pub unsafe fn setErrorDescription(&self, aErrorDescription: &[u16]) -> Result<(), nsresult> {
        let aErrorDescription = nsString::from(aErrorDescription);
        match ((*self.vtable).setErrorDescription)(self as *const _, &*aErrorDescription) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDefaultIndex (in long aDefaultIndex); */
    #[inline]
    pub unsafe fn setDefaultIndex(&self, aDefaultIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDefaultIndex)(self as *const _, aDefaultIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSearchResult (in unsigned short aSearchResult); */
    #[inline]
    pub unsafe fn setSearchResult(&self, aSearchResult: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setSearchResult)(self as *const _, aSearchResult) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    #[inline]
    pub unsafe fn insertMatchAt(&self, aIndex: libc::int32_t, aValue: &[u16], aComment: &[u16], aImage: &[u16], aStyle: &[u16], aFinalCompleteValue: &[u16], aLabel: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        let aComment = nsString::from(aComment);
        let aImage = nsString::from(aImage);
        let aStyle = nsString::from(aStyle);
        let aFinalCompleteValue = nsString::from(aFinalCompleteValue);
        let aLabel = nsString::from(aLabel);
        match ((*self.vtable).insertMatchAt)(self as *const _, aIndex, &*aValue, &*aComment, &*aImage, &*aStyle, &*aFinalCompleteValue, &*aLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    #[inline]
    pub unsafe fn appendMatch(&self, aValue: &[u16], aComment: &[u16], aImage: &[u16], aStyle: &[u16], aFinalCompleteValue: &[u16], aLabel: &[u16]) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        let aComment = nsString::from(aComment);
        let aImage = nsString::from(aImage);
        let aStyle = nsString::from(aStyle);
        let aFinalCompleteValue = nsString::from(aFinalCompleteValue);
        let aLabel = nsString::from(aLabel);
        match ((*self.vtable).appendMatch)(self as *const _, &*aValue, &*aComment, &*aImage, &*aStyle, &*aFinalCompleteValue, &*aLabel) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIAutoCompleteSimpleResultListener getListener (); */
    #[inline]
    pub unsafe fn getListener(&self, ) -> Result<Option<RefPtr<nsIAutoCompleteSimpleResultListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getListener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
    #[inline]
    pub unsafe fn setListener(&self, aListener: Option<&nsIAutoCompleteSimpleResultListener>) -> Result<(), nsresult> {

        match ((*self.vtable).setListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIAutoCompleteSimpleResultListener {
    vtable: *const nsIAutoCompleteSimpleResultListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteSimpleResultListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x004efdc5, 0x1989, 0x4874,
            [0x8a, 0x7a, 0x34, 0x5b, 0xf2, 0xfa, 0x33, 0xaf])
    }
}

unsafe impl RefCounted for nsIAutoCompleteSimpleResultListener {
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
pub trait nsIAutoCompleteSimpleResultListenerCoerce {
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self;
}

impl nsIAutoCompleteSimpleResultListenerCoerce for nsIAutoCompleteSimpleResultListener {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self {
        v
    }
}

impl nsIAutoCompleteSimpleResultListener {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSimpleResultListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteSimpleResultListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteSimpleResultListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteSimpleResultListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue, in boolean aRemoveFromDb); */
    pub onValueRemoved: unsafe extern "C" fn (this: *const nsIAutoCompleteSimpleResultListener, aResult: *const nsIAutoCompleteSimpleResult, aValue: *const nsAString, aRemoveFromDb: bool) -> nsresult,

}


impl nsIAutoCompleteSimpleResultListener {
    /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue, in boolean aRemoveFromDb); */
    #[inline]
    pub unsafe fn onValueRemoved(&self, aResult: Option<&nsIAutoCompleteSimpleResult>, aValue: &[u16], aRemoveFromDb: bool) -> Result<(), nsresult> {
        let aValue = nsString::from(aValue);
        match ((*self.vtable).onValueRemoved)(self as *const _, aResult.map_or(::std::ptr::null(), |x| x as *const _), &*aValue, aRemoveFromDb) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


