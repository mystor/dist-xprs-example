//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAnnotationService.idl
//


#[repr(C)]
pub struct nsIAnnotationObserver {
    vtable: *const nsIAnnotationObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAnnotationObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x63fe98e0, 0x6889, 0x4c2c,
            [0xac, 0x9f, 0x70, 0x3e, 0x4b, 0xc2, 0x50, 0x27])
    }
}

unsafe impl RefCounted for nsIAnnotationObserver {
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
pub trait nsIAnnotationObserverCoerce {
    fn coerce_from(v: &nsIAnnotationObserver) -> &Self;
}

impl nsIAnnotationObserverCoerce for nsIAnnotationObserver {
    #[inline]
    fn coerce_from(v: &nsIAnnotationObserver) -> &Self {
        v
    }
}

impl nsIAnnotationObserver {
    #[inline]
    pub fn coerce<T: nsIAnnotationObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAnnotationObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAnnotationObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAnnotationObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAnnotationObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onPageAnnotationSet (in nsIURI aPage, in AUTF8String aName); */
    pub onPageAnnotationSet: unsafe extern "C" fn (this: *const nsIAnnotationObserver, aPage: *const nsIURI, aName: *const nsACString) -> nsresult,

    /* void onItemAnnotationSet (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
    pub onItemAnnotationSet: unsafe extern "C" fn (this: *const nsIAnnotationObserver, aItemId: libc::int64_t, aName: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* void onPageAnnotationRemoved (in nsIURI aURI, in AUTF8String aName); */
    pub onPageAnnotationRemoved: unsafe extern "C" fn (this: *const nsIAnnotationObserver, aURI: *const nsIURI, aName: *const nsACString) -> nsresult,

    /* void onItemAnnotationRemoved (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
    pub onItemAnnotationRemoved: unsafe extern "C" fn (this: *const nsIAnnotationObserver, aItemId: libc::int64_t, aName: *const nsACString, aSource: libc::uint16_t) -> nsresult,

}


impl nsIAnnotationObserver {
    /* void onPageAnnotationSet (in nsIURI aPage, in AUTF8String aName); */
    #[inline]
    pub unsafe fn onPageAnnotationSet(&self, aPage: Option<&nsIURI>, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).onPageAnnotationSet)(self as *const _, aPage.map_or(::std::ptr::null(), |x| x as *const _), &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemAnnotationSet (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemAnnotationSet(&self, aItemId: libc::int64_t, aName: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).onItemAnnotationSet)(self as *const _, aItemId, &*aName, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPageAnnotationRemoved (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn onPageAnnotationRemoved(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).onPageAnnotationRemoved)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onItemAnnotationRemoved (in long long aItemId, in AUTF8String aName, in unsigned short aSource); */
    #[inline]
    pub unsafe fn onItemAnnotationRemoved(&self, aItemId: libc::int64_t, aName: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).onItemAnnotationRemoved)(self as *const _, aItemId, &*aName, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIAnnotationService_consts {
    pub const EXPIRE_SESSION: i64 = 0;
    pub const EXPIRE_WEEKS: i64 = 2;
    pub const EXPIRE_MONTHS: i64 = 3;
    pub const EXPIRE_NEVER: i64 = 4;
    pub const EXPIRE_WITH_HISTORY: i64 = 5;
    pub const EXPIRE_DAYS: i64 = 6;
    pub const TYPE_INT32: i64 = 1;
    pub const TYPE_DOUBLE: i64 = 2;
    pub const TYPE_STRING: i64 = 3;
    pub const TYPE_INT64: i64 = 5;
}


#[repr(C)]
pub struct nsIAnnotationService {
    vtable: *const nsIAnnotationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAnnotationService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd4cdaab1, 0x8eec, 0x47a8,
            [0xb4, 0x20, 0xad, 0x7c, 0xb3, 0x33, 0x05, 0x6a])
    }
}

unsafe impl RefCounted for nsIAnnotationService {
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
pub trait nsIAnnotationServiceCoerce {
    fn coerce_from(v: &nsIAnnotationService) -> &Self;
}

impl nsIAnnotationServiceCoerce for nsIAnnotationService {
    #[inline]
    fn coerce_from(v: &nsIAnnotationService) -> &Self {
        v
    }
}

impl nsIAnnotationService {
    #[inline]
    pub fn coerce<T: nsIAnnotationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAnnotationService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAnnotationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAnnotationService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAnnotationServiceVTable {
    pub __base: nsISupportsVTable,

    /* void setPageAnnotation (in nsIURI aURI, in AUTF8String aName, in nsIVariant aValue, in long aFlags, in unsigned short aExpiration); */
    pub setPageAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aValue: *const nsIVariant, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> nsresult,

    /* void setItemAnnotation (in long long aItemId, in AUTF8String aName, in nsIVariant aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    pub setItemAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aValue: *const nsIVariant, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> nsresult,

    /* [noscript] void setPageAnnotationString (in nsIURI aURI, in AUTF8String aName, in AString aValue, in long aFlags, in unsigned short aExpiration); */
    pub setPageAnnotationString: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aValue: *const nsAString, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> nsresult,

    /* [noscript] void setItemAnnotationString (in long long aItemId, in AUTF8String aName, in AString aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    pub setItemAnnotationString: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aValue: *const nsAString, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> nsresult,

    /* [noscript] void setPageAnnotationInt32 (in nsIURI aURI, in AUTF8String aName, in long aValue, in long aFlags, in unsigned short aExpiration); */
    pub setPageAnnotationInt32: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aValue: libc::int32_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> nsresult,

    /* [noscript] void setItemAnnotationInt32 (in long long aItemId, in AUTF8String aName, in long aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    pub setItemAnnotationInt32: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aValue: libc::int32_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> nsresult,

    /* [noscript] void setPageAnnotationInt64 (in nsIURI aURI, in AUTF8String aName, in long long aValue, in long aFlags, in unsigned short aExpiration); */
    pub setPageAnnotationInt64: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aValue: libc::int64_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> nsresult,

    /* [noscript] void setItemAnnotationInt64 (in long long aItemId, in AUTF8String aName, in long long aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    pub setItemAnnotationInt64: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aValue: libc::int64_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> nsresult,

    /* [noscript] void setPageAnnotationDouble (in nsIURI aURI, in AUTF8String aName, in double aValue, in long aFlags, in unsigned short aExpiration); */
    pub setPageAnnotationDouble: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aValue: libc::c_double, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> nsresult,

    /* [noscript] void setItemAnnotationDouble (in long long aItemId, in AUTF8String aName, in double aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    pub setItemAnnotationDouble: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aValue: libc::c_double, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> nsresult,

    /* nsIVariant getPageAnnotation (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut *const nsIVariant) -> nsresult,

    /* nsIVariant getItemAnnotation (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut *const nsIVariant) -> nsresult,

    /* [noscript] AString getPageAnnotationString (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotationString: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* [noscript] AString getItemAnnotationString (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotationString: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut nsAString) -> nsresult,

    /* [noscript] long getPageAnnotationInt32 (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotationInt32: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut libc::int32_t) -> nsresult,

    /* [noscript] long getItemAnnotationInt32 (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotationInt32: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut libc::int32_t) -> nsresult,

    /* [noscript] long long getPageAnnotationInt64 (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotationInt64: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut libc::int64_t) -> nsresult,

    /* [noscript] long long getItemAnnotationInt64 (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotationInt64: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut libc::int64_t) -> nsresult,

    /* [noscript] double getPageAnnotationDouble (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotationDouble: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut libc::c_double) -> nsresult,

    /* [noscript] double getItemAnnotationDouble (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotationDouble: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut libc::c_double) -> nsresult,

    /* void getPageAnnotationInfo (in nsIURI aURI, in AUTF8String aName, out int32_t aFlags, out unsigned short aExpiration, out unsigned short aType); */
    pub getPageAnnotationInfo: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, aFlags: *mut int32_t, aExpiration: *mut libc::uint16_t, aType: *mut libc::uint16_t) -> nsresult,

    /* void getItemAnnotationInfo (in long long aItemId, in AUTF8String aName, out long aFlags, out unsigned short aExpiration, out unsigned short aType); */
    pub getItemAnnotationInfo: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aFlags: *mut libc::int32_t, aExpiration: *mut libc::uint16_t, aType: *mut libc::uint16_t) -> nsresult,

    /* uint16_t getPageAnnotationType (in nsIURI aURI, in AUTF8String aName); */
    pub getPageAnnotationType: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut uint16_t) -> nsresult,

    /* uint16_t getItemAnnotationType (in long long aItemId, in AUTF8String aName); */
    pub getItemAnnotationType: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut uint16_t) -> nsresult,

    /* void getPagesWithAnnotation (in AUTF8String name, [optional] out unsigned long resultCount, [array, size_is (resultCount), retval] out nsIURI results); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPagesWithAnnotation: *const ::libc::c_void,

    /* void getItemsWithAnnotation (in AUTF8String name, [optional] out unsigned long resultCount, [array, size_is (resultCount), retval] out long long results); */
    /// Unable to call function as its signature contains a non-rust type
    pub getItemsWithAnnotation: *const ::libc::c_void,

    /* void getAnnotationsWithName (in AUTF8String name, [optional] out unsigned long count, [array, size_is (count), retval] out mozIAnnotatedResult results); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAnnotationsWithName: *const ::libc::c_void,

    /* void getPageAnnotationNames (in nsIURI aURI, [optional] out unsigned long count, [array, size_is (count), retval] out nsIVariant result); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPageAnnotationNames: *const ::libc::c_void,

    /* void getItemAnnotationNames (in long long aItemId, [optional] out unsigned long count, [array, size_is (count), retval] out nsIVariant result); */
    /// Unable to call function as its signature contains a non-rust type
    pub getItemAnnotationNames: *const ::libc::c_void,

    /* boolean pageHasAnnotation (in nsIURI aURI, in AUTF8String aName); */
    pub pageHasAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString, _retval: *mut bool) -> nsresult,

    /* boolean itemHasAnnotation (in long long aItemId, in AUTF8String aName); */
    pub itemHasAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void removePageAnnotation (in nsIURI aURI, in AUTF8String aName); */
    pub removePageAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI, aName: *const nsACString) -> nsresult,

    /* void removeItemAnnotation (in long long aItemId, in AUTF8String aName, [optional] in unsigned short aSource); */
    pub removeItemAnnotation: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aName: *const nsACString, aSource: libc::uint16_t) -> nsresult,

    /* void removePageAnnotations (in nsIURI aURI); */
    pub removePageAnnotations: unsafe extern "C" fn (this: *const nsIAnnotationService, aURI: *const nsIURI) -> nsresult,

    /* void removeItemAnnotations (in long long aItemId, [optional] in unsigned short aSource); */
    pub removeItemAnnotations: unsafe extern "C" fn (this: *const nsIAnnotationService, aItemId: libc::int64_t, aSource: libc::uint16_t) -> nsresult,

    /* void copyPageAnnotations (in nsIURI aSourceURI, in nsIURI aDestURI, in boolean aOverwriteDest); */
    pub copyPageAnnotations: unsafe extern "C" fn (this: *const nsIAnnotationService, aSourceURI: *const nsIURI, aDestURI: *const nsIURI, aOverwriteDest: bool) -> nsresult,

    /* void copyItemAnnotations (in long long aSourceItemId, in long long aDestItemId, in boolean aOverwriteDest, [optional] in unsigned short aSource); */
    pub copyItemAnnotations: unsafe extern "C" fn (this: *const nsIAnnotationService, aSourceItemId: libc::int64_t, aDestItemId: libc::int64_t, aOverwriteDest: bool, aSource: libc::uint16_t) -> nsresult,

    /* void addObserver (in nsIAnnotationObserver aObserver); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIAnnotationService, aObserver: *const nsIAnnotationObserver) -> nsresult,

    /* void removeObserver (in nsIAnnotationObserver aObserver); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIAnnotationService, aObserver: *const nsIAnnotationObserver) -> nsresult,

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsIAnnotationObserver observers); */
    /// Unable to call function as its signature contains a non-rust type
    pub getObservers: *const ::libc::c_void,

}


impl nsIAnnotationService {
    /* void setPageAnnotation (in nsIURI aURI, in AUTF8String aName, in nsIVariant aValue, in long aFlags, in unsigned short aExpiration); */
    #[inline]
    pub unsafe fn setPageAnnotation(&self, aURI: Option<&nsIURI>, aName: &[u8], aValue: Option<&nsIVariant>, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setPageAnnotation)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aExpiration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setItemAnnotation (in long long aItemId, in AUTF8String aName, in nsIVariant aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemAnnotation(&self, aItemId: libc::int64_t, aName: &[u8], aValue: Option<&nsIVariant>, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setItemAnnotation)(self as *const _, aItemId, &*aName, aValue.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aExpiration, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setPageAnnotationString (in nsIURI aURI, in AUTF8String aName, in AString aValue, in long aFlags, in unsigned short aExpiration); */
    #[inline]
    pub unsafe fn setPageAnnotationString(&self, aURI: Option<&nsIURI>, aName: &[u8], aValue: &[u16], aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setPageAnnotationString)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &*aValue, aFlags, aExpiration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setItemAnnotationString (in long long aItemId, in AUTF8String aName, in AString aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemAnnotationString(&self, aItemId: libc::int64_t, aName: &[u8], aValue: &[u16], aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        let aValue = nsString::from(aValue);
        match ((*self.vtable).setItemAnnotationString)(self as *const _, aItemId, &*aName, &*aValue, aFlags, aExpiration, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setPageAnnotationInt32 (in nsIURI aURI, in AUTF8String aName, in long aValue, in long aFlags, in unsigned short aExpiration); */
    #[inline]
    pub unsafe fn setPageAnnotationInt32(&self, aURI: Option<&nsIURI>, aName: &[u8], aValue: libc::int32_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setPageAnnotationInt32)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aValue, aFlags, aExpiration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setItemAnnotationInt32 (in long long aItemId, in AUTF8String aName, in long aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemAnnotationInt32(&self, aItemId: libc::int64_t, aName: &[u8], aValue: libc::int32_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setItemAnnotationInt32)(self as *const _, aItemId, &*aName, aValue, aFlags, aExpiration, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setPageAnnotationInt64 (in nsIURI aURI, in AUTF8String aName, in long long aValue, in long aFlags, in unsigned short aExpiration); */
    #[inline]
    pub unsafe fn setPageAnnotationInt64(&self, aURI: Option<&nsIURI>, aName: &[u8], aValue: libc::int64_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setPageAnnotationInt64)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aValue, aFlags, aExpiration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setItemAnnotationInt64 (in long long aItemId, in AUTF8String aName, in long long aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemAnnotationInt64(&self, aItemId: libc::int64_t, aName: &[u8], aValue: libc::int64_t, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setItemAnnotationInt64)(self as *const _, aItemId, &*aName, aValue, aFlags, aExpiration, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setPageAnnotationDouble (in nsIURI aURI, in AUTF8String aName, in double aValue, in long aFlags, in unsigned short aExpiration); */
    #[inline]
    pub unsafe fn setPageAnnotationDouble(&self, aURI: Option<&nsIURI>, aName: &[u8], aValue: libc::c_double, aFlags: libc::int32_t, aExpiration: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setPageAnnotationDouble)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, aValue, aFlags, aExpiration) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setItemAnnotationDouble (in long long aItemId, in AUTF8String aName, in double aValue, in long aFlags, in unsigned short aExpiration, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn setItemAnnotationDouble(&self, aItemId: libc::int64_t, aName: &[u8], aValue: libc::c_double, aFlags: libc::int32_t, aExpiration: libc::uint16_t, aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setItemAnnotationDouble)(self as *const _, aItemId, &*aName, aValue, aFlags, aExpiration, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIVariant getPageAnnotation (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotation(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPageAnnotation)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIVariant getItemAnnotation (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotation(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getItemAnnotation)(self as *const _, aItemId, &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] AString getPageAnnotationString (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotationString(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<nsString, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getPageAnnotationString)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] AString getItemAnnotationString (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotationString(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<nsString, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = nsString::new();
        match ((*self.vtable).getItemAnnotationString)(self as *const _, aItemId, &*aName, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] long getPageAnnotationInt32 (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotationInt32(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<libc::int32_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPageAnnotationInt32)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] long getItemAnnotationInt32 (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotationInt32(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<libc::int32_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemAnnotationInt32)(self as *const _, aItemId, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] long long getPageAnnotationInt64 (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotationInt64(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<libc::int64_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getPageAnnotationInt64)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] long long getItemAnnotationInt64 (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotationInt64(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<libc::int64_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemAnnotationInt64)(self as *const _, aItemId, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] double getPageAnnotationDouble (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotationDouble(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<libc::c_double, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getPageAnnotationDouble)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] double getItemAnnotationDouble (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotationDouble(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<libc::c_double, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: libc::c_double = ::std::mem::zeroed();
        match ((*self.vtable).getItemAnnotationDouble)(self as *const _, aItemId, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getPageAnnotationInfo (in nsIURI aURI, in AUTF8String aName, out int32_t aFlags, out unsigned short aExpiration, out unsigned short aType); */
    #[inline]
    pub unsafe fn getPageAnnotationInfo(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<(int32_t, libc::uint16_t, libc::uint16_t), nsresult> {
        let aName = nsCString::from(aName);
        let mut aFlags: int32_t = ::std::mem::zeroed();
        let mut aExpiration: libc::uint16_t = ::std::mem::zeroed();
        let mut aType: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getPageAnnotationInfo)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut aFlags as *mut _, &mut aExpiration as *mut _, &mut aType as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFlags, aExpiration, aType))
    }

    /* void getItemAnnotationInfo (in long long aItemId, in AUTF8String aName, out long aFlags, out unsigned short aExpiration, out unsigned short aType); */
    #[inline]
    pub unsafe fn getItemAnnotationInfo(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<(libc::int32_t, libc::uint16_t, libc::uint16_t), nsresult> {
        let aName = nsCString::from(aName);
        let mut aFlags: libc::int32_t = ::std::mem::zeroed();
        let mut aExpiration: libc::uint16_t = ::std::mem::zeroed();
        let mut aType: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemAnnotationInfo)(self as *const _, aItemId, &*aName, &mut aFlags as *mut _, &mut aExpiration as *mut _, &mut aType as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFlags, aExpiration, aType))
    }

    /* uint16_t getPageAnnotationType (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getPageAnnotationType(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<uint16_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getPageAnnotationType)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint16_t getItemAnnotationType (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn getItemAnnotationType(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<uint16_t, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).getItemAnnotationType)(self as *const _, aItemId, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getPagesWithAnnotation (in AUTF8String name, [optional] out unsigned long resultCount, [array, size_is (resultCount), retval] out nsIURI results); */


    /* void getItemsWithAnnotation (in AUTF8String name, [optional] out unsigned long resultCount, [array, size_is (resultCount), retval] out long long results); */


    /* void getAnnotationsWithName (in AUTF8String name, [optional] out unsigned long count, [array, size_is (count), retval] out mozIAnnotatedResult results); */


    /* void getPageAnnotationNames (in nsIURI aURI, [optional] out unsigned long count, [array, size_is (count), retval] out nsIVariant result); */


    /* void getItemAnnotationNames (in long long aItemId, [optional] out unsigned long count, [array, size_is (count), retval] out nsIVariant result); */


    /* boolean pageHasAnnotation (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn pageHasAnnotation(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<bool, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).pageHasAnnotation)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean itemHasAnnotation (in long long aItemId, in AUTF8String aName); */
    #[inline]
    pub unsafe fn itemHasAnnotation(&self, aItemId: libc::int64_t, aName: &[u8]) -> Result<bool, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).itemHasAnnotation)(self as *const _, aItemId, &*aName, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void removePageAnnotation (in nsIURI aURI, in AUTF8String aName); */
    #[inline]
    pub unsafe fn removePageAnnotation(&self, aURI: Option<&nsIURI>, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).removePageAnnotation)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeItemAnnotation (in long long aItemId, in AUTF8String aName, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn removeItemAnnotation(&self, aItemId: libc::int64_t, aName: &[u8], aSource: libc::uint16_t) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).removeItemAnnotation)(self as *const _, aItemId, &*aName, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removePageAnnotations (in nsIURI aURI); */
    #[inline]
    pub unsafe fn removePageAnnotations(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).removePageAnnotations)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeItemAnnotations (in long long aItemId, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn removeItemAnnotations(&self, aItemId: libc::int64_t, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).removeItemAnnotations)(self as *const _, aItemId, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyPageAnnotations (in nsIURI aSourceURI, in nsIURI aDestURI, in boolean aOverwriteDest); */
    #[inline]
    pub unsafe fn copyPageAnnotations(&self, aSourceURI: Option<&nsIURI>, aDestURI: Option<&nsIURI>, aOverwriteDest: bool) -> Result<(), nsresult> {

        match ((*self.vtable).copyPageAnnotations)(self as *const _, aSourceURI.map_or(::std::ptr::null(), |x| x as *const _), aDestURI.map_or(::std::ptr::null(), |x| x as *const _), aOverwriteDest) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void copyItemAnnotations (in long long aSourceItemId, in long long aDestItemId, in boolean aOverwriteDest, [optional] in unsigned short aSource); */
    #[inline]
    pub unsafe fn copyItemAnnotations(&self, aSourceItemId: libc::int64_t, aDestItemId: libc::int64_t, aOverwriteDest: bool, aSource: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).copyItemAnnotations)(self as *const _, aSourceItemId, aDestItemId, aOverwriteDest, aSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserver (in nsIAnnotationObserver aObserver); */
    #[inline]
    pub unsafe fn addObserver(&self, aObserver: Option<&nsIAnnotationObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIAnnotationObserver aObserver); */
    #[inline]
    pub unsafe fn removeObserver(&self, aObserver: Option<&nsIAnnotationObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getObservers ([optional] out unsigned long count, [array, size_is (count), retval] out nsIAnnotationObserver observers); */


}


#[repr(C)]
pub struct mozIAnnotatedResult {
    vtable: *const mozIAnnotatedResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIAnnotatedResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x81fd0188, 0xdb6a, 0x492e,
            [0x80, 0xb6, 0xf6, 0x41, 0x49, 0x13, 0xb3, 0x96])
    }
}

unsafe impl RefCounted for mozIAnnotatedResult {
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
pub trait mozIAnnotatedResultCoerce {
    fn coerce_from(v: &mozIAnnotatedResult) -> &Self;
}

impl mozIAnnotatedResultCoerce for mozIAnnotatedResult {
    #[inline]
    fn coerce_from(v: &mozIAnnotatedResult) -> &Self {
        v
    }
}

impl mozIAnnotatedResult {
    #[inline]
    pub fn coerce<T: mozIAnnotatedResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIAnnotatedResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIAnnotatedResultCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIAnnotatedResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIAnnotatedResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String guid; */
    pub get_guid: unsafe extern "C" fn (this: *const mozIAnnotatedResult, aGuid: *mut nsACString) -> nsresult,

    /* readonly attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const mozIAnnotatedResult, aUri: *mut *const nsIURI) -> nsresult,

    /* readonly attribute long long itemId; */
    pub get_itemId: unsafe extern "C" fn (this: *const mozIAnnotatedResult, aItemId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute AUTF8String annotationName; */
    pub get_annotationName: unsafe extern "C" fn (this: *const mozIAnnotatedResult, aAnnotationName: *mut nsACString) -> nsresult,

    /* readonly attribute nsIVariant annotationValue; */
    pub get_annotationValue: unsafe extern "C" fn (this: *const mozIAnnotatedResult, aAnnotationValue: *mut *const nsIVariant) -> nsresult,

}


impl mozIAnnotatedResult {
    /* readonly attribute AUTF8String guid; */
    #[inline]
    pub unsafe fn get_guid(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_guid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI uri; */
    #[inline]
    pub unsafe fn get_uri(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_uri)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long long itemId; */
    #[inline]
    pub unsafe fn get_itemId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_itemId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String annotationName; */
    #[inline]
    pub unsafe fn get_annotationName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_annotationName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIVariant annotationValue; */
    #[inline]
    pub unsafe fn get_annotationValue(&self, ) -> Result<Option<RefPtr<nsIVariant>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_annotationValue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


