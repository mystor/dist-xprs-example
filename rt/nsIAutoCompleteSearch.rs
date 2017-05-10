//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAutoCompleteSearch.idl
//


#[repr(C)]
pub struct nsIAutoCompleteSearch {
    vtable: *const nsIAutoCompleteSearchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteSearch {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde8db85f, 0xc1de, 0x4d87,
            [0x94, 0xba, 0x78, 0x44, 0x89, 0x0f, 0x91, 0xfe])
    }
}

unsafe impl RefCounted for nsIAutoCompleteSearch {
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
pub trait nsIAutoCompleteSearchCoerce {
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self;
}

impl nsIAutoCompleteSearchCoerce for nsIAutoCompleteSearch {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self {
        v
    }
}

impl nsIAutoCompleteSearch {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSearchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteSearch {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteSearchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearch) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteSearchVTable {
    pub __base: nsISupportsVTable,

    /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener); */
    pub startSearch: unsafe extern "C" fn (this: *const nsIAutoCompleteSearch, searchString: *const nsAString, searchParam: *const nsAString, previousResult: *const nsIAutoCompleteResult, listener: *const nsIAutoCompleteObserver) -> nsresult,

    /* void stopSearch (); */
    pub stopSearch: unsafe extern "C" fn (this: *const nsIAutoCompleteSearch) -> nsresult,

}


impl nsIAutoCompleteSearch {
    /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener); */
    #[inline]
    pub unsafe fn startSearch(&self, searchString: &[u16], searchParam: &[u16], previousResult: Option<&nsIAutoCompleteResult>, listener: Option<&nsIAutoCompleteObserver>) -> Result<(), nsresult> {
        let searchString = nsString::from(searchString);
        let searchParam = nsString::from(searchParam);
        match ((*self.vtable).startSearch)(self as *const _, &*searchString, &*searchParam, previousResult.map_or(::std::ptr::null(), |x| x as *const _), listener.map_or(::std::ptr::null(), |x| x as *const _)) {
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

}


#[repr(C)]
pub struct nsIAutoCompleteObserver {
    vtable: *const nsIAutoCompleteObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8bd1dbbc, 0xdcce, 0x4007,
            [0x9a, 0xfa, 0xb5, 0x51, 0xeb, 0x68, 0x7b, 0x61])
    }
}

unsafe impl RefCounted for nsIAutoCompleteObserver {
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
pub trait nsIAutoCompleteObserverCoerce {
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self;
}

impl nsIAutoCompleteObserverCoerce for nsIAutoCompleteObserver {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self {
        v
    }
}

impl nsIAutoCompleteObserver {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
    pub onSearchResult: unsafe extern "C" fn (this: *const nsIAutoCompleteObserver, search: *const nsIAutoCompleteSearch, result: *const nsIAutoCompleteResult) -> nsresult,

    /* void onUpdateSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
    pub onUpdateSearchResult: unsafe extern "C" fn (this: *const nsIAutoCompleteObserver, search: *const nsIAutoCompleteSearch, result: *const nsIAutoCompleteResult) -> nsresult,

}


impl nsIAutoCompleteObserver {
    /* void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
    #[inline]
    pub unsafe fn onSearchResult(&self, search: Option<&nsIAutoCompleteSearch>, result: Option<&nsIAutoCompleteResult>) -> Result<(), nsresult> {

        match ((*self.vtable).onSearchResult)(self as *const _, search.map_or(::std::ptr::null(), |x| x as *const _), result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onUpdateSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
    #[inline]
    pub unsafe fn onUpdateSearchResult(&self, search: Option<&nsIAutoCompleteSearch>, result: Option<&nsIAutoCompleteResult>) -> Result<(), nsresult> {

        match ((*self.vtable).onUpdateSearchResult)(self as *const _, search.map_or(::std::ptr::null(), |x| x as *const _), result.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIAutoCompleteSearchDescriptor_consts {
    pub const SEARCH_TYPE_DELAYED: i64 = 0;
    pub const SEARCH_TYPE_IMMEDIATE: i64 = 1;
}


#[repr(C)]
pub struct nsIAutoCompleteSearchDescriptor {
    vtable: *const nsIAutoCompleteSearchDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAutoCompleteSearchDescriptor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4c3e7462, 0xfbfb, 0x4310,
            [0x8f, 0x4b, 0x23, 0x92, 0x38, 0x39, 0x2b, 0x75])
    }
}

unsafe impl RefCounted for nsIAutoCompleteSearchDescriptor {
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
pub trait nsIAutoCompleteSearchDescriptorCoerce {
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self;
}

impl nsIAutoCompleteSearchDescriptorCoerce for nsIAutoCompleteSearchDescriptor {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self {
        v
    }
}

impl nsIAutoCompleteSearchDescriptor {
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSearchDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAutoCompleteSearchDescriptor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAutoCompleteSearchDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSearchDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAutoCompleteSearchDescriptorVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short searchType; */
    pub get_searchType: unsafe extern "C" fn (this: *const nsIAutoCompleteSearchDescriptor, aSearchType: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute boolean clearingAutoFillSearchesAgain; */
    pub get_clearingAutoFillSearchesAgain: unsafe extern "C" fn (this: *const nsIAutoCompleteSearchDescriptor, aClearingAutoFillSearchesAgain: *mut bool) -> nsresult,

}


impl nsIAutoCompleteSearchDescriptor {
    /* readonly attribute unsigned short searchType; */
    #[inline]
    pub unsafe fn get_searchType(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_searchType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean clearingAutoFillSearchesAgain; */
    #[inline]
    pub unsafe fn get_clearingAutoFillSearchesAgain(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_clearingAutoFillSearchesAgain)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


