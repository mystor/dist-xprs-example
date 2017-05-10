//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozILocaleService.idl
//


pub mod mozILocaleService_consts {
    pub const langNegStrategyFiltering: i64 = 0;
    pub const langNegStrategyMatching: i64 = 1;
    pub const langNegStrategyLookup: i64 = 2;
}


#[repr(C)]
pub struct mozILocaleService {
    vtable: *const mozILocaleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozILocaleService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc27f8983, 0xb48b, 0x4d1a,
            [0x92, 0xd7, 0xfe, 0xb8, 0x10, 0x6f, 0x21, 0x2d])
    }
}

unsafe impl RefCounted for mozILocaleService {
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
pub trait mozILocaleServiceCoerce {
    fn coerce_from(v: &mozILocaleService) -> &Self;
}

impl mozILocaleServiceCoerce for mozILocaleService {
    #[inline]
    fn coerce_from(v: &mozILocaleService) -> &Self {
        v
    }
}

impl mozILocaleService {
    #[inline]
    pub fn coerce<T: mozILocaleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozILocaleService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozILocaleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozILocaleService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozILocaleServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString defaultLocale; */
    pub get_defaultLocale: unsafe extern "C" fn (this: *const mozILocaleService, aDefaultLocale: *mut nsACString) -> nsresult,

    /* void getAppLocalesAsLangTags ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAppLocalesAsLangTags: *const ::libc::c_void,

    /* void getAppLocalesAsBCP47 ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAppLocalesAsBCP47: *const ::libc::c_void,

    /* void negotiateLanguages ([array, size_is (aRequestedCount)] in string aRequested, [array, size_is (aAvailableCount)] in string aAvailable, [optional] in string aDefaultLocale, [optional] in long langNegStrategy, [optional] in unsigned long aRequestedCount, [optional] in unsigned long aAvailableCount, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */
    /// Unable to call function as its signature contains a non-rust type
    pub negotiateLanguages: *const ::libc::c_void,

    /* ACString getAppLocaleAsLangTag (); */
    pub getAppLocaleAsLangTag: unsafe extern "C" fn (this: *const mozILocaleService, _retval: *mut nsACString) -> nsresult,

    /* ACString getAppLocaleAsBCP47 (); */
    pub getAppLocaleAsBCP47: unsafe extern "C" fn (this: *const mozILocaleService, _retval: *mut nsACString) -> nsresult,

    /* void getRequestedLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getRequestedLocales: *const ::libc::c_void,

    /* ACString getRequestedLocale (); */
    pub getRequestedLocale: unsafe extern "C" fn (this: *const mozILocaleService, _retval: *mut nsACString) -> nsresult,

    /* void setRequestedLocales ([array, size_is (aRequestedCount)] in string aRequested, [optional] in unsigned long aRequestedCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub setRequestedLocales: *const ::libc::c_void,

    /* void getAvailableLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAvailableLocales: *const ::libc::c_void,

    /* readonly attribute boolean isAppLocaleRTL; */
    pub get_isAppLocaleRTL: unsafe extern "C" fn (this: *const mozILocaleService, aIsAppLocaleRTL: *mut bool) -> nsresult,

}


impl mozILocaleService {
    /* readonly attribute ACString defaultLocale; */
    #[inline]
    pub unsafe fn get_defaultLocale(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_defaultLocale)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getAppLocalesAsLangTags ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */


    /* void getAppLocalesAsBCP47 ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */


    /* void negotiateLanguages ([array, size_is (aRequestedCount)] in string aRequested, [array, size_is (aAvailableCount)] in string aAvailable, [optional] in string aDefaultLocale, [optional] in long langNegStrategy, [optional] in unsigned long aRequestedCount, [optional] in unsigned long aAvailableCount, [optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */


    /* ACString getAppLocaleAsLangTag (); */
    #[inline]
    pub unsafe fn getAppLocaleAsLangTag(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAppLocaleAsLangTag)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getAppLocaleAsBCP47 (); */
    #[inline]
    pub unsafe fn getAppLocaleAsBCP47(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getAppLocaleAsBCP47)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getRequestedLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */


    /* ACString getRequestedLocale (); */
    #[inline]
    pub unsafe fn getRequestedLocale(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getRequestedLocale)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setRequestedLocales ([array, size_is (aRequestedCount)] in string aRequested, [optional] in unsigned long aRequestedCount); */


    /* void getAvailableLocales ([optional] out unsigned long aCount, [array, size_is (aCount), retval] out string aLocales); */


    /* readonly attribute boolean isAppLocaleRTL; */
    #[inline]
    pub unsafe fn get_isAppLocaleRTL(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isAppLocaleRTL)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


