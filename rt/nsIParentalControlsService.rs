//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParentalControlsService.idl
//


pub mod nsIParentalControlsService_consts {
    pub const DOWNLOAD: i64 = 1;
    pub const INSTALL_EXTENSION: i64 = 2;
    pub const INSTALL_APP: i64 = 3;
    pub const BROWSE: i64 = 4;
    pub const SHARE: i64 = 5;
    pub const BOOKMARK: i64 = 6;
    pub const ADD_CONTACT: i64 = 7;
    pub const SET_IMAGE: i64 = 8;
    pub const MODIFY_ACCOUNTS: i64 = 9;
    pub const REMOTE_DEBUGGING: i64 = 10;
    pub const IMPORT_SETTINGS: i64 = 11;
    pub const PRIVATE_BROWSING: i64 = 12;
    pub const DATA_CHOICES: i64 = 13;
    pub const CLEAR_HISTORY: i64 = 14;
    pub const MASTER_PASSWORD: i64 = 15;
    pub const GUEST_BROWSING: i64 = 16;
    pub const ADVANCED_SETTINGS: i64 = 17;
    pub const CAMERA_MICROPHONE: i64 = 18;
    pub const BLOCK_LIST: i64 = 19;
    pub const TELEMETRY: i64 = 20;
    pub const HEALTH_REPORT: i64 = 21;
    pub const DEFAULT_THEME: i64 = 22;
    pub const ePCLog_URIVisit: i64 = 1;
    pub const ePCLog_FileDownload: i64 = 2;
}


#[repr(C)]
pub struct nsIParentalControlsService {
    vtable: *const nsIParentalControlsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIParentalControlsService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2e97e5dd, 0x467b, 0x4aea,
            [0xa1, 0xbb, 0x67, 0x73, 0xc0, 0xf2, 0xbe, 0xb0])
    }
}

unsafe impl RefCounted for nsIParentalControlsService {
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
pub trait nsIParentalControlsServiceCoerce {
    fn coerce_from(v: &nsIParentalControlsService) -> &Self;
}

impl nsIParentalControlsServiceCoerce for nsIParentalControlsService {
    #[inline]
    fn coerce_from(v: &nsIParentalControlsService) -> &Self {
        v
    }
}

impl nsIParentalControlsService {
    #[inline]
    pub fn coerce<T: nsIParentalControlsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIParentalControlsService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIParentalControlsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentalControlsService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIParentalControlsServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean parentalControlsEnabled; */
    pub get_parentalControlsEnabled: unsafe extern "C" fn (this: *const nsIParentalControlsService, aParentalControlsEnabled: *mut bool) -> nsresult,

    /* readonly attribute boolean blockFileDownloadsEnabled; */
    pub get_blockFileDownloadsEnabled: unsafe extern "C" fn (this: *const nsIParentalControlsService, aBlockFileDownloadsEnabled: *mut bool) -> nsresult,

    /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
    pub isAllowed: unsafe extern "C" fn (this: *const nsIParentalControlsService, aAction: libc::int16_t, aUri: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean requestURIOverride (in nsIURI aTarget, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub requestURIOverride: unsafe extern "C" fn (this: *const nsIParentalControlsService, aTarget: *const nsIURI, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut bool) -> nsresult,

    /* boolean requestURIOverrides (in nsIArray aTargets, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub requestURIOverrides: unsafe extern "C" fn (this: *const nsIParentalControlsService, aTargets: *const nsIArray, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean loggingEnabled; */
    pub get_loggingEnabled: unsafe extern "C" fn (this: *const nsIParentalControlsService, aLoggingEnabled: *mut bool) -> nsresult,

    /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
    pub log: unsafe extern "C" fn (this: *const nsIParentalControlsService, aEntryType: libc::int16_t, aFlag: bool, aSource: *const nsIURI, aTarget: *const nsIFile) -> nsresult,

}


impl nsIParentalControlsService {
    /* readonly attribute boolean parentalControlsEnabled; */
    #[inline]
    pub unsafe fn get_parentalControlsEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_parentalControlsEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean blockFileDownloadsEnabled; */
    #[inline]
    pub unsafe fn get_blockFileDownloadsEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_blockFileDownloadsEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
    #[inline]
    pub unsafe fn isAllowed(&self, aAction: libc::int16_t, aUri: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAllowed)(self as *const _, aAction, aUri.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean requestURIOverride (in nsIURI aTarget, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn requestURIOverride(&self, aTarget: Option<&nsIURI>, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).requestURIOverride)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean requestURIOverrides (in nsIArray aTargets, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn requestURIOverrides(&self, aTargets: Option<&nsIArray>, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).requestURIOverrides)(self as *const _, aTargets.map_or(::std::ptr::null(), |x| x as *const _), aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean loggingEnabled; */
    #[inline]
    pub unsafe fn get_loggingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loggingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
    #[inline]
    pub unsafe fn log(&self, aEntryType: libc::int16_t, aFlag: bool, aSource: Option<&nsIURI>, aTarget: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).log)(self as *const _, aEntryType, aFlag, aSource.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


