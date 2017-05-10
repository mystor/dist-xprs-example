//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBrowserSearchService.idl
//


#[repr(C)]
pub struct nsISearchSubmission {
    vtable: *const nsISearchSubmissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISearchSubmission {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5799251f, 0x5b55, 0x4df7,
            [0xa9, 0xe7, 0x0c, 0x27, 0x81, 0x2c, 0x46, 0x9a])
    }
}

unsafe impl RefCounted for nsISearchSubmission {
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
pub trait nsISearchSubmissionCoerce {
    fn coerce_from(v: &nsISearchSubmission) -> &Self;
}

impl nsISearchSubmissionCoerce for nsISearchSubmission {
    #[inline]
    fn coerce_from(v: &nsISearchSubmission) -> &Self {
        v
    }
}

impl nsISearchSubmission {
    #[inline]
    pub fn coerce<T: nsISearchSubmissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISearchSubmission {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISearchSubmissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchSubmission) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISearchSubmissionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIInputStream postData; */
    pub get_postData: unsafe extern "C" fn (this: *const nsISearchSubmission, aPostData: *mut *const nsIInputStream) -> nsresult,

    /* readonly attribute nsIURI uri; */
    pub get_uri: unsafe extern "C" fn (this: *const nsISearchSubmission, aUri: *mut *const nsIURI) -> nsresult,

}


impl nsISearchSubmission {
    /* readonly attribute nsIInputStream postData; */
    #[inline]
    pub unsafe fn get_postData(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_postData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
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

}


#[repr(C)]
pub struct nsISearchEngine {
    vtable: *const nsISearchEngineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISearchEngine {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x620bd920, 0x0491, 0x48c8,
            [0x99, 0xa8, 0xd6, 0x04, 0x7e, 0x64, 0x80, 0x2d])
    }
}

unsafe impl RefCounted for nsISearchEngine {
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
pub trait nsISearchEngineCoerce {
    fn coerce_from(v: &nsISearchEngine) -> &Self;
}

impl nsISearchEngineCoerce for nsISearchEngine {
    #[inline]
    fn coerce_from(v: &nsISearchEngine) -> &Self {
        v
    }
}

impl nsISearchEngine {
    #[inline]
    pub fn coerce<T: nsISearchEngineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISearchEngine {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISearchEngineCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchEngine) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISearchEngineVTable {
    pub __base: nsISupportsVTable,

    /* nsISearchSubmission getSubmission (in AString data, [optional] in AString responseType, [optional] in AString purpose); */
    pub getSubmission: unsafe extern "C" fn (this: *const nsISearchEngine, data: *const nsAString, responseType: *const nsAString, purpose: *const nsAString, _retval: *mut *const nsISearchSubmission) -> nsresult,

    /* void addParam (in AString name, in AString value, in AString responseType); */
    pub addParam: unsafe extern "C" fn (this: *const nsISearchEngine, name: *const nsAString, value: *const nsAString, responseType: *const nsAString) -> nsresult,

    /* boolean supportsResponseType (in AString responseType); */
    pub supportsResponseType: unsafe extern "C" fn (this: *const nsISearchEngine, responseType: *const nsAString, _retval: *mut bool) -> nsresult,

    /* AString getIconURLBySize (in long width, in long height); */
    pub getIconURLBySize: unsafe extern "C" fn (this: *const nsISearchEngine, width: libc::int32_t, height: libc::int32_t, _retval: *mut nsAString) -> nsresult,

    /* jsval getIcons (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIcons: *const ::libc::c_void,

    /* void speculativeConnect (in jsval options); */
    /// Unable to call function as its signature contains a non-rust type
    pub speculativeConnect: *const ::libc::c_void,

    /* attribute AString alias; */
    pub get_alias: unsafe extern "C" fn (this: *const nsISearchEngine, aAlias: *mut nsAString) -> nsresult,
    pub set_alias: unsafe extern "C" fn (this: *const nsISearchEngine, aAlias: *const nsAString) -> nsresult,

    /* readonly attribute AString description; */
    pub get_description: unsafe extern "C" fn (this: *const nsISearchEngine, aDescription: *mut nsAString) -> nsresult,

    /* attribute boolean hidden; */
    pub get_hidden: unsafe extern "C" fn (this: *const nsISearchEngine, aHidden: *mut bool) -> nsresult,
    pub set_hidden: unsafe extern "C" fn (this: *const nsISearchEngine, aHidden: bool) -> nsresult,

    /* readonly attribute nsIURI iconURI; */
    pub get_iconURI: unsafe extern "C" fn (this: *const nsISearchEngine, aIconURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AString name; */
    pub get_name: unsafe extern "C" fn (this: *const nsISearchEngine, aName: *mut nsAString) -> nsresult,

    /* readonly attribute AString searchForm; */
    pub get_searchForm: unsafe extern "C" fn (this: *const nsISearchEngine, aSearchForm: *mut nsAString) -> nsresult,

    /* readonly attribute AString identifier; */
    pub get_identifier: unsafe extern "C" fn (this: *const nsISearchEngine, aIdentifier: *mut nsAString) -> nsresult,

    /* AString getResultDomain ([optional] in AString responseType); */
    pub getResultDomain: unsafe extern "C" fn (this: *const nsISearchEngine, responseType: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsISearchEngine {
    /* nsISearchSubmission getSubmission (in AString data, [optional] in AString responseType, [optional] in AString purpose); */
    #[inline]
    pub unsafe fn getSubmission(&self, data: &[u16], responseType: &[u16], purpose: &[u16]) -> Result<Option<RefPtr<nsISearchSubmission>>, nsresult> {
        let data = nsString::from(data);
        let responseType = nsString::from(responseType);
        let purpose = nsString::from(purpose);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSubmission)(self as *const _, &*data, &*responseType, &*purpose, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addParam (in AString name, in AString value, in AString responseType); */
    #[inline]
    pub unsafe fn addParam(&self, name: &[u16], value: &[u16], responseType: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let value = nsString::from(value);
        let responseType = nsString::from(responseType);
        match ((*self.vtable).addParam)(self as *const _, &*name, &*value, &*responseType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean supportsResponseType (in AString responseType); */
    #[inline]
    pub unsafe fn supportsResponseType(&self, responseType: &[u16]) -> Result<bool, nsresult> {
        let responseType = nsString::from(responseType);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).supportsResponseType)(self as *const _, &*responseType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getIconURLBySize (in long width, in long height); */
    #[inline]
    pub unsafe fn getIconURLBySize(&self, width: libc::int32_t, height: libc::int32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getIconURLBySize)(self as *const _, width, height, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* jsval getIcons (); */


    /* void speculativeConnect (in jsval options); */


    /* attribute AString alias; */
    #[inline]
    pub unsafe fn get_alias(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_alias)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_alias(&self, aAlias: &[u16]) -> Result<(), nsresult> {
        let aAlias = nsString::from(aAlias);
        match ((*self.vtable).set_alias)(self as *const _, &*aAlias) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AString description; */
    #[inline]
    pub unsafe fn get_description(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_description)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean hidden; */
    #[inline]
    pub unsafe fn get_hidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hidden(&self, aHidden: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_hidden)(self as *const _, aHidden) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIURI iconURI; */
    #[inline]
    pub unsafe fn get_iconURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_iconURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString searchForm; */
    #[inline]
    pub unsafe fn get_searchForm(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_searchForm)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString identifier; */
    #[inline]
    pub unsafe fn get_identifier(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_identifier)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getResultDomain ([optional] in AString responseType); */
    #[inline]
    pub unsafe fn getResultDomain(&self, responseType: &[u16]) -> Result<nsString, nsresult> {
        let responseType = nsString::from(responseType);
        let mut _retval = nsString::new();
        match ((*self.vtable).getResultDomain)(self as *const _, &*responseType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsISearchParseSubmissionResult {
    vtable: *const nsISearchParseSubmissionResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISearchParseSubmissionResult {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0dc93e51, 0xa7bf, 0x4a16,
            [0x86, 0x2d, 0x4b, 0x34, 0x69, 0xff, 0x62, 0x06])
    }
}

unsafe impl RefCounted for nsISearchParseSubmissionResult {
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
pub trait nsISearchParseSubmissionResultCoerce {
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self;
}

impl nsISearchParseSubmissionResultCoerce for nsISearchParseSubmissionResult {
    #[inline]
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self {
        v
    }
}

impl nsISearchParseSubmissionResult {
    #[inline]
    pub fn coerce<T: nsISearchParseSubmissionResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISearchParseSubmissionResult {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISearchParseSubmissionResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchParseSubmissionResult) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISearchParseSubmissionResultVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISearchEngine engine; */
    pub get_engine: unsafe extern "C" fn (this: *const nsISearchParseSubmissionResult, aEngine: *mut *const nsISearchEngine) -> nsresult,

    /* readonly attribute AString terms; */
    pub get_terms: unsafe extern "C" fn (this: *const nsISearchParseSubmissionResult, aTerms: *mut nsAString) -> nsresult,

    /* readonly attribute long termsOffset; */
    pub get_termsOffset: unsafe extern "C" fn (this: *const nsISearchParseSubmissionResult, aTermsOffset: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long termsLength; */
    pub get_termsLength: unsafe extern "C" fn (this: *const nsISearchParseSubmissionResult, aTermsLength: *mut libc::int32_t) -> nsresult,

}


impl nsISearchParseSubmissionResult {
    /* readonly attribute nsISearchEngine engine; */
    #[inline]
    pub unsafe fn get_engine(&self, ) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_engine)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString terms; */
    #[inline]
    pub unsafe fn get_terms(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_terms)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long termsOffset; */
    #[inline]
    pub unsafe fn get_termsOffset(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_termsOffset)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long termsLength; */
    #[inline]
    pub unsafe fn get_termsLength(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_termsLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


pub mod nsISearchInstallCallback_consts {
    pub const ERROR_UNKNOWN_FAILURE: i64 = 1;
    pub const ERROR_DUPLICATE_ENGINE: i64 = 2;
}


#[repr(C)]
pub struct nsISearchInstallCallback {
    vtable: *const nsISearchInstallCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISearchInstallCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9fc39136, 0xf08b, 0x46d3,
            [0xb2, 0x32, 0x96, 0xf4, 0xb7, 0xb0, 0xe2, 0x35])
    }
}

unsafe impl RefCounted for nsISearchInstallCallback {
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
pub trait nsISearchInstallCallbackCoerce {
    fn coerce_from(v: &nsISearchInstallCallback) -> &Self;
}

impl nsISearchInstallCallbackCoerce for nsISearchInstallCallback {
    #[inline]
    fn coerce_from(v: &nsISearchInstallCallback) -> &Self {
        v
    }
}

impl nsISearchInstallCallback {
    #[inline]
    pub fn coerce<T: nsISearchInstallCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISearchInstallCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISearchInstallCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchInstallCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISearchInstallCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onSuccess (in nsISearchEngine engine); */
    pub onSuccess: unsafe extern "C" fn (this: *const nsISearchInstallCallback, engine: *const nsISearchEngine) -> nsresult,

    /* void onError (in unsigned long errorCode); */
    pub onError: unsafe extern "C" fn (this: *const nsISearchInstallCallback, errorCode: libc::uint32_t) -> nsresult,

}


impl nsISearchInstallCallback {
    /* void onSuccess (in nsISearchEngine engine); */
    #[inline]
    pub unsafe fn onSuccess(&self, engine: Option<&nsISearchEngine>) -> Result<(), nsresult> {

        match ((*self.vtable).onSuccess)(self as *const _, engine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in unsigned long errorCode); */
    #[inline]
    pub unsafe fn onError(&self, errorCode: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, errorCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIBrowserSearchInitObserver {
    vtable: *const nsIBrowserSearchInitObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserSearchInitObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x02256156, 0x16e4, 0x47f1,
            [0x99, 0x79, 0x76, 0xff, 0x98, 0xce, 0xb5, 0x90])
    }
}

unsafe impl RefCounted for nsIBrowserSearchInitObserver {
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
pub trait nsIBrowserSearchInitObserverCoerce {
    fn coerce_from(v: &nsIBrowserSearchInitObserver) -> &Self;
}

impl nsIBrowserSearchInitObserverCoerce for nsIBrowserSearchInitObserver {
    #[inline]
    fn coerce_from(v: &nsIBrowserSearchInitObserver) -> &Self {
        v
    }
}

impl nsIBrowserSearchInitObserver {
    #[inline]
    pub fn coerce<T: nsIBrowserSearchInitObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserSearchInitObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserSearchInitObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserSearchInitObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserSearchInitObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onInitComplete (in nsresult aStatus); */
    pub onInitComplete: unsafe extern "C" fn (this: *const nsIBrowserSearchInitObserver, aStatus: nsresult) -> nsresult,

}


impl nsIBrowserSearchInitObserver {
    /* void onInitComplete (in nsresult aStatus); */
    #[inline]
    pub unsafe fn onInitComplete(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onInitComplete)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIBrowserSearchService {
    vtable: *const nsIBrowserSearchServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBrowserSearchService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x150ef720, 0xbbe2, 0x4169,
            [0xb9, 0xf3, 0xef, 0x7e, 0xc0, 0x65, 0x4c, 0xed])
    }
}

unsafe impl RefCounted for nsIBrowserSearchService {
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
pub trait nsIBrowserSearchServiceCoerce {
    fn coerce_from(v: &nsIBrowserSearchService) -> &Self;
}

impl nsIBrowserSearchServiceCoerce for nsIBrowserSearchService {
    #[inline]
    fn coerce_from(v: &nsIBrowserSearchService) -> &Self {
        v
    }
}

impl nsIBrowserSearchService {
    #[inline]
    pub fn coerce<T: nsIBrowserSearchServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBrowserSearchService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBrowserSearchServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserSearchService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBrowserSearchServiceVTable {
    pub __base: nsISupportsVTable,

    /* void init ([optional] in nsIBrowserSearchInitObserver aObserver); */
    pub init: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aObserver: *const nsIBrowserSearchInitObserver) -> nsresult,

    /* readonly attribute bool isInitialized; */
    pub get_isInitialized: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aIsInitialized: *mut bool) -> nsresult,

    /* void resetToOriginalDefaultEngine (); */
    pub resetToOriginalDefaultEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService) -> nsresult,

    /* boolean hasEngineWithURL (in AString method, in AString url, in jsval formData); */
    /// Unable to call function as its signature contains a non-rust type
    pub hasEngineWithURL: *const ::libc::c_void,

    /* void addEngine (in AString engineURL, in long dataType, in AString iconURL, in boolean confirm, [optional] in nsISearchInstallCallback callback); */
    pub addEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, engineURL: *const nsAString, dataType: libc::int32_t, iconURL: *const nsAString, confirm: bool, callback: *const nsISearchInstallCallback) -> nsresult,

    /* void addEngineWithDetails (in AString name, in AString iconURL, in AString alias, in AString description, in AString method, in AString url, [optional] in AString extensionID); */
    pub addEngineWithDetails: unsafe extern "C" fn (this: *const nsIBrowserSearchService, name: *const nsAString, iconURL: *const nsAString, alias: *const nsAString, description: *const nsAString, method: *const nsAString, url: *const nsAString, extensionID: *const nsAString) -> nsresult,

    /* void restoreDefaultEngines (); */
    pub restoreDefaultEngines: unsafe extern "C" fn (this: *const nsIBrowserSearchService) -> nsresult,

    /* nsISearchEngine getEngineByAlias (in AString alias); */
    pub getEngineByAlias: unsafe extern "C" fn (this: *const nsIBrowserSearchService, alias: *const nsAString, _retval: *mut *const nsISearchEngine) -> nsresult,

    /* nsISearchEngine getEngineByName (in AString aEngineName); */
    pub getEngineByName: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aEngineName: *const nsAString, _retval: *mut *const nsISearchEngine) -> nsresult,

    /* void getEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */
    /// Unable to call function as its signature contains a non-rust type
    pub getEngines: *const ::libc::c_void,

    /* void getVisibleEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */
    /// Unable to call function as its signature contains a non-rust type
    pub getVisibleEngines: *const ::libc::c_void,

    /* void getDefaultEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDefaultEngines: *const ::libc::c_void,

    /* void moveEngine (in nsISearchEngine engine, in long newIndex); */
    pub moveEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, engine: *const nsISearchEngine, newIndex: libc::int32_t) -> nsresult,

    /* void removeEngine (in nsISearchEngine engine); */
    pub removeEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, engine: *const nsISearchEngine) -> nsresult,

    /* readonly attribute nsISearchEngine originalDefaultEngine; */
    pub get_originalDefaultEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aOriginalDefaultEngine: *mut *const nsISearchEngine) -> nsresult,

    /* attribute nsISearchEngine defaultEngine; */
    pub get_defaultEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aDefaultEngine: *mut *const nsISearchEngine) -> nsresult,
    pub set_defaultEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aDefaultEngine: *const nsISearchEngine) -> nsresult,

    /* attribute nsISearchEngine currentEngine; */
    pub get_currentEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aCurrentEngine: *mut *const nsISearchEngine) -> nsresult,
    pub set_currentEngine: unsafe extern "C" fn (this: *const nsIBrowserSearchService, aCurrentEngine: *const nsISearchEngine) -> nsresult,

    /* jsval getDefaultEngineInfo (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDefaultEngineInfo: *const ::libc::c_void,

    /* nsISearchParseSubmissionResult parseSubmissionURL (in AString url); */
    pub parseSubmissionURL: unsafe extern "C" fn (this: *const nsIBrowserSearchService, url: *const nsAString, _retval: *mut *const nsISearchParseSubmissionResult) -> nsresult,

}


impl nsIBrowserSearchService {
    /* void init ([optional] in nsIBrowserSearchInitObserver aObserver); */
    #[inline]
    pub unsafe fn init(&self, aObserver: Option<&nsIBrowserSearchInitObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute bool isInitialized; */
    #[inline]
    pub unsafe fn get_isInitialized(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInitialized)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void resetToOriginalDefaultEngine (); */
    #[inline]
    pub unsafe fn resetToOriginalDefaultEngine(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetToOriginalDefaultEngine)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasEngineWithURL (in AString method, in AString url, in jsval formData); */


    /* void addEngine (in AString engineURL, in long dataType, in AString iconURL, in boolean confirm, [optional] in nsISearchInstallCallback callback); */
    #[inline]
    pub unsafe fn addEngine(&self, engineURL: &[u16], dataType: libc::int32_t, iconURL: &[u16], confirm: bool, callback: Option<&nsISearchInstallCallback>) -> Result<(), nsresult> {
        let engineURL = nsString::from(engineURL);
        let iconURL = nsString::from(iconURL);
        match ((*self.vtable).addEngine)(self as *const _, &*engineURL, dataType, &*iconURL, confirm, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addEngineWithDetails (in AString name, in AString iconURL, in AString alias, in AString description, in AString method, in AString url, [optional] in AString extensionID); */
    #[inline]
    pub unsafe fn addEngineWithDetails(&self, name: &[u16], iconURL: &[u16], alias: &[u16], description: &[u16], method: &[u16], url: &[u16], extensionID: &[u16]) -> Result<(), nsresult> {
        let name = nsString::from(name);
        let iconURL = nsString::from(iconURL);
        let alias = nsString::from(alias);
        let description = nsString::from(description);
        let method = nsString::from(method);
        let url = nsString::from(url);
        let extensionID = nsString::from(extensionID);
        match ((*self.vtable).addEngineWithDetails)(self as *const _, &*name, &*iconURL, &*alias, &*description, &*method, &*url, &*extensionID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restoreDefaultEngines (); */
    #[inline]
    pub unsafe fn restoreDefaultEngines(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restoreDefaultEngines)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISearchEngine getEngineByAlias (in AString alias); */
    #[inline]
    pub unsafe fn getEngineByAlias(&self, alias: &[u16]) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let alias = nsString::from(alias);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEngineByAlias)(self as *const _, &*alias, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISearchEngine getEngineByName (in AString aEngineName); */
    #[inline]
    pub unsafe fn getEngineByName(&self, aEngineName: &[u16]) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let aEngineName = nsString::from(aEngineName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getEngineByName)(self as *const _, &*aEngineName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void getEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */


    /* void getVisibleEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */


    /* void getDefaultEngines ([optional] out unsigned long engineCount, [array, size_is (engineCount), retval] out nsISearchEngine engines); */


    /* void moveEngine (in nsISearchEngine engine, in long newIndex); */
    #[inline]
    pub unsafe fn moveEngine(&self, engine: Option<&nsISearchEngine>, newIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).moveEngine)(self as *const _, engine.map_or(::std::ptr::null(), |x| x as *const _), newIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeEngine (in nsISearchEngine engine); */
    #[inline]
    pub unsafe fn removeEngine(&self, engine: Option<&nsISearchEngine>) -> Result<(), nsresult> {

        match ((*self.vtable).removeEngine)(self as *const _, engine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISearchEngine originalDefaultEngine; */
    #[inline]
    pub unsafe fn get_originalDefaultEngine(&self, ) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalDefaultEngine)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISearchEngine defaultEngine; */
    #[inline]
    pub unsafe fn get_defaultEngine(&self, ) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultEngine)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_defaultEngine(&self, aDefaultEngine: Option<&nsISearchEngine>) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultEngine)(self as *const _, aDefaultEngine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISearchEngine currentEngine; */
    #[inline]
    pub unsafe fn get_currentEngine(&self, ) -> Result<Option<RefPtr<nsISearchEngine>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentEngine)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_currentEngine(&self, aCurrentEngine: Option<&nsISearchEngine>) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentEngine)(self as *const _, aCurrentEngine.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* jsval getDefaultEngineInfo (); */


    /* nsISearchParseSubmissionResult parseSubmissionURL (in AString url); */
    #[inline]
    pub unsafe fn parseSubmissionURL(&self, url: &[u16]) -> Result<Option<RefPtr<nsISearchParseSubmissionResult>>, nsresult> {
        let url = nsString::from(url);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).parseSubmissionURL)(self as *const _, &*url, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


