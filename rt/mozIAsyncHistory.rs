//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/mozIAsyncHistory.idl
//


#[repr(C)]
pub struct mozIVisitInfo {
    vtable: *const mozIVisitInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIVisitInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x41e4ccc9, 0xf0c8, 0x4cd7,
            [0x97, 0x53, 0x7a, 0x38, 0x51, 0x4b, 0x84, 0x88])
    }
}

unsafe impl RefCounted for mozIVisitInfo {
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
pub trait mozIVisitInfoCoerce {
    fn coerce_from(v: &mozIVisitInfo) -> &Self;
}

impl mozIVisitInfoCoerce for mozIVisitInfo {
    #[inline]
    fn coerce_from(v: &mozIVisitInfo) -> &Self {
        v
    }
}

impl mozIVisitInfo {
    #[inline]
    pub fn coerce<T: mozIVisitInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIVisitInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIVisitInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIVisitInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long long visitId; */
    pub get_visitId: unsafe extern "C" fn (this: *const mozIVisitInfo, aVisitId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute PRTime visitDate; */
    pub get_visitDate: unsafe extern "C" fn (this: *const mozIVisitInfo, aVisitDate: *mut PRTime) -> nsresult,

    /* readonly attribute unsigned long transitionType; */
    pub get_transitionType: unsafe extern "C" fn (this: *const mozIVisitInfo, aTransitionType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIURI referrerURI; */
    pub get_referrerURI: unsafe extern "C" fn (this: *const mozIVisitInfo, aReferrerURI: *mut *const nsIURI) -> nsresult,

}


impl mozIVisitInfo {
    /* readonly attribute long long visitId; */
    #[inline]
    pub unsafe fn get_visitId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_visitId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime visitDate; */
    #[inline]
    pub unsafe fn get_visitDate(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_visitDate)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long transitionType; */
    #[inline]
    pub unsafe fn get_transitionType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_transitionType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI referrerURI; */
    #[inline]
    pub unsafe fn get_referrerURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrerURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct mozIPlaceInfo {
    vtable: *const mozIPlaceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIPlaceInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xad83e137, 0xc92a, 0x4b7b,
            [0xb6, 0x7e, 0x0a, 0x31, 0x88, 0x11, 0xf9, 0x1e])
    }
}

unsafe impl RefCounted for mozIPlaceInfo {
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
pub trait mozIPlaceInfoCoerce {
    fn coerce_from(v: &mozIPlaceInfo) -> &Self;
}

impl mozIPlaceInfoCoerce for mozIPlaceInfo {
    #[inline]
    fn coerce_from(v: &mozIPlaceInfo) -> &Self {
        v
    }
}

impl mozIPlaceInfo {
    #[inline]
    pub fn coerce<T: mozIPlaceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIPlaceInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIPlaceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlaceInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIPlaceInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute long long placeId; */
    pub get_placeId: unsafe extern "C" fn (this: *const mozIPlaceInfo, aPlaceId: *mut libc::int64_t) -> nsresult,

    /* readonly attribute ACString guid; */
    pub get_guid: unsafe extern "C" fn (this: *const mozIPlaceInfo, aGuid: *mut nsACString) -> nsresult,

    /* readonly attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const mozIPlaceInfo, aUri: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AString title; */
    pub get_title: unsafe extern "C" fn (this: *const mozIPlaceInfo, aTitle: *mut nsAString) -> nsresult,

    /* readonly attribute long long frecency; */
    pub get_frecency: unsafe extern "C" fn (this: *const mozIPlaceInfo, aFrecency: *mut libc::int64_t) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval visits; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_visits: *const ::libc::c_void,

}


impl mozIPlaceInfo {
    /* readonly attribute long long placeId; */
    #[inline]
    pub unsafe fn get_placeId(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_placeId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString guid; */
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

    /* readonly attribute AString title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_title)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long long frecency; */
    #[inline]
    pub unsafe fn get_frecency(&self, ) -> Result<libc::int64_t, nsresult> {
        let mut _retval: libc::int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_frecency)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval visits; */


}


#[repr(C)]
pub struct mozIVisitInfoCallback {
    vtable: *const mozIVisitInfoCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIVisitInfoCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1f266877, 0x2859, 0x418b,
            [0xa1, 0x1b, 0xec, 0x3a, 0xe4, 0xf4, 0xf9, 0x3d])
    }
}

unsafe impl RefCounted for mozIVisitInfoCallback {
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
pub trait mozIVisitInfoCallbackCoerce {
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self;
}

impl mozIVisitInfoCallbackCoerce for mozIVisitInfoCallback {
    #[inline]
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self {
        v
    }
}

impl mozIVisitInfoCallback {
    #[inline]
    pub fn coerce<T: mozIVisitInfoCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIVisitInfoCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIVisitInfoCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitInfoCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIVisitInfoCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
    pub handleError: unsafe extern "C" fn (this: *const mozIVisitInfoCallback, aResultCode: nsresult, aPlaceInfo: *const mozIPlaceInfo) -> nsresult,

    /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
    pub handleResult: unsafe extern "C" fn (this: *const mozIVisitInfoCallback, aPlaceInfo: *const mozIPlaceInfo) -> nsresult,

    /* void handleCompletion (in unsigned long aUpdatedItems); */
    pub handleCompletion: unsafe extern "C" fn (this: *const mozIVisitInfoCallback, aUpdatedItems: libc::uint32_t) -> nsresult,

    /* readonly attribute bool ignoreResults; */
    pub get_ignoreResults: unsafe extern "C" fn (this: *const mozIVisitInfoCallback, aIgnoreResults: *mut bool) -> nsresult,

    /* readonly attribute bool ignoreErrors; */
    pub get_ignoreErrors: unsafe extern "C" fn (this: *const mozIVisitInfoCallback, aIgnoreErrors: *mut bool) -> nsresult,

}


impl mozIVisitInfoCallback {
    /* void handleError (in nsresult aResultCode, in mozIPlaceInfo aPlaceInfo); */
    #[inline]
    pub unsafe fn handleError(&self, aResultCode: nsresult, aPlaceInfo: Option<&mozIPlaceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).handleError)(self as *const _, aResultCode, aPlaceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleResult (in mozIPlaceInfo aPlaceInfo); */
    #[inline]
    pub unsafe fn handleResult(&self, aPlaceInfo: Option<&mozIPlaceInfo>) -> Result<(), nsresult> {

        match ((*self.vtable).handleResult)(self as *const _, aPlaceInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void handleCompletion (in unsigned long aUpdatedItems); */
    #[inline]
    pub unsafe fn handleCompletion(&self, aUpdatedItems: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).handleCompletion)(self as *const _, aUpdatedItems) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool ignoreResults; */
    #[inline]
    pub unsafe fn get_ignoreResults(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ignoreResults)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool ignoreErrors; */
    #[inline]
    pub unsafe fn get_ignoreErrors(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ignoreErrors)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct mozIVisitedStatusCallback {
    vtable: *const mozIVisitedStatusCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIVisitedStatusCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x994092bf, 0x936f, 0x449b,
            [0x8d, 0xd6, 0x09, 0x41, 0xe0, 0x24, 0x36, 0x0d])
    }
}

unsafe impl RefCounted for mozIVisitedStatusCallback {
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
pub trait mozIVisitedStatusCallbackCoerce {
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self;
}

impl mozIVisitedStatusCallbackCoerce for mozIVisitedStatusCallback {
    #[inline]
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self {
        v
    }
}

impl mozIVisitedStatusCallback {
    #[inline]
    pub fn coerce<T: mozIVisitedStatusCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIVisitedStatusCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIVisitedStatusCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIVisitedStatusCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIVisitedStatusCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
    pub isVisited: unsafe extern "C" fn (this: *const mozIVisitedStatusCallback, aURI: *const nsIURI, aVisitedStatus: bool) -> nsresult,

}


impl mozIVisitedStatusCallback {
    /* void isVisited (in nsIURI aURI, in boolean aVisitedStatus); */
    #[inline]
    pub unsafe fn isVisited(&self, aURI: Option<&nsIURI>, aVisitedStatus: bool) -> Result<(), nsresult> {

        match ((*self.vtable).isVisited)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aVisitedStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct mozIAsyncHistory {
    vtable: *const mozIAsyncHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for mozIAsyncHistory {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1643efd2, 0xa329, 0x4733,
            [0xa3, 0x9d, 0x17, 0x06, 0x9c, 0x8d, 0x3b, 0x2d])
    }
}

unsafe impl RefCounted for mozIAsyncHistory {
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
pub trait mozIAsyncHistoryCoerce {
    fn coerce_from(v: &mozIAsyncHistory) -> &Self;
}

impl mozIAsyncHistoryCoerce for mozIAsyncHistory {
    #[inline]
    fn coerce_from(v: &mozIAsyncHistory) -> &Self {
        v
    }
}

impl mozIAsyncHistory {
    #[inline]
    pub fn coerce<T: mozIAsyncHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for mozIAsyncHistory {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> mozIAsyncHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIAsyncHistory) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct mozIAsyncHistoryVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void getPlacesInfo (in jsval aPlaceIdentifiers, in mozIVisitInfoCallback aCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPlacesInfo: *const ::libc::c_void,

    /* [implicit_jscontext] void updatePlaces (in jsval aPlaceInfo, [optional] in mozIVisitInfoCallback aCallback, [optional] in bool aGroupNotifications); */
    /// Unable to call function as its signature contains a non-rust type
    pub updatePlaces: *const ::libc::c_void,

    /* void isURIVisited (in nsIURI aURI, in mozIVisitedStatusCallback aCallback); */
    pub isURIVisited: unsafe extern "C" fn (this: *const mozIAsyncHistory, aURI: *const nsIURI, aCallback: *const mozIVisitedStatusCallback) -> nsresult,

}


impl mozIAsyncHistory {
    /* [implicit_jscontext] void getPlacesInfo (in jsval aPlaceIdentifiers, in mozIVisitInfoCallback aCallback); */


    /* [implicit_jscontext] void updatePlaces (in jsval aPlaceInfo, [optional] in mozIVisitInfoCallback aCallback, [optional] in bool aGroupNotifications); */


    /* void isURIVisited (in nsIURI aURI, in mozIVisitedStatusCallback aCallback); */
    #[inline]
    pub unsafe fn isURIVisited(&self, aURI: Option<&nsIURI>, aCallback: Option<&mozIVisitedStatusCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).isURIVisited)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


