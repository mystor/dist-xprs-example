//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIExternalHelperAppService.idl
//


#[repr(C)]
pub struct nsIExternalHelperAppService {
    vtable: *const nsIExternalHelperAppServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExternalHelperAppService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e4f3ae1, 0xb737, 0x431f,
            [0xa9, 0x5d, 0x31, 0xfa, 0x8d, 0xa7, 0x01, 0x99])
    }
}

unsafe impl RefCounted for nsIExternalHelperAppService {
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
pub trait nsIExternalHelperAppServiceCoerce {
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self;
}

impl nsIExternalHelperAppServiceCoerce for nsIExternalHelperAppService {
    #[inline]
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self {
        v
    }
}

impl nsIExternalHelperAppService {
    #[inline]
    pub fn coerce<T: nsIExternalHelperAppServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExternalHelperAppService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExternalHelperAppServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalHelperAppService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExternalHelperAppServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
    pub doContent: unsafe extern "C" fn (this: *const nsIExternalHelperAppService, aMimeContentType: *const nsACString, aRequest: *const nsIRequest, aContentContext: *const nsIInterfaceRequestor, aForceSave: bool, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut *const nsIStreamListener) -> nsresult,

    /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
    pub applyDecodingForExtension: unsafe extern "C" fn (this: *const nsIExternalHelperAppService, aExtension: *const nsACString, aEncodingType: *const nsACString, _retval: *mut bool) -> nsresult,

}


impl nsIExternalHelperAppService {
    /* nsIStreamListener doContent (in ACString aMimeContentType, in nsIRequest aRequest, in nsIInterfaceRequestor aContentContext, in boolean aForceSave, [optional] in nsIInterfaceRequestor aWindowContext); */
    #[inline]
    pub unsafe fn doContent(&self, aMimeContentType: &[u8], aRequest: Option<&nsIRequest>, aContentContext: Option<&nsIInterfaceRequestor>, aForceSave: bool, aWindowContext: Option<&nsIInterfaceRequestor>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let aMimeContentType = nsCString::from(aMimeContentType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).doContent)(self as *const _, &*aMimeContentType, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContentContext.map_or(::std::ptr::null(), |x| x as *const _), aForceSave, aWindowContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean applyDecodingForExtension (in AUTF8String aExtension, in ACString aEncodingType); */
    #[inline]
    pub unsafe fn applyDecodingForExtension(&self, aExtension: &[u8], aEncodingType: &[u8]) -> Result<bool, nsresult> {
        let aExtension = nsCString::from(aExtension);
        let aEncodingType = nsCString::from(aEncodingType);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).applyDecodingForExtension)(self as *const _, &*aExtension, &*aEncodingType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsPIExternalAppLauncher {
    vtable: *const nsPIExternalAppLauncherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIExternalAppLauncher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6613e2e7, 0xfeab, 0x4e3a,
            [0xbb, 0x1f, 0xb0, 0x32, 0x00, 0xd5, 0x44, 0xec])
    }
}

unsafe impl RefCounted for nsPIExternalAppLauncher {
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
pub trait nsPIExternalAppLauncherCoerce {
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self;
}

impl nsPIExternalAppLauncherCoerce for nsPIExternalAppLauncher {
    #[inline]
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self {
        v
    }
}

impl nsPIExternalAppLauncher {
    #[inline]
    pub fn coerce<T: nsPIExternalAppLauncherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIExternalAppLauncher {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIExternalAppLauncherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIExternalAppLauncher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIExternalAppLauncherVTable {
    pub __base: nsISupportsVTable,

    /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
    pub deleteTemporaryFileOnExit: unsafe extern "C" fn (this: *const nsPIExternalAppLauncher, aTemporaryFile: *const nsIFile) -> nsresult,

    /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
    pub deleteTemporaryPrivateFileWhenPossible: unsafe extern "C" fn (this: *const nsPIExternalAppLauncher, aTemporaryFile: *const nsIFile) -> nsresult,

}


impl nsPIExternalAppLauncher {
    /* void deleteTemporaryFileOnExit (in nsIFile aTemporaryFile); */
    #[inline]
    pub unsafe fn deleteTemporaryFileOnExit(&self, aTemporaryFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTemporaryFileOnExit)(self as *const _, aTemporaryFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deleteTemporaryPrivateFileWhenPossible (in nsIFile aTemporaryFile); */
    #[inline]
    pub unsafe fn deleteTemporaryPrivateFileWhenPossible(&self, aTemporaryFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).deleteTemporaryPrivateFileWhenPossible)(self as *const _, aTemporaryFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIHelperAppLauncher {
    vtable: *const nsIHelperAppLauncherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHelperAppLauncher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xacf2a516, 0x7d7f, 0x4771,
            [0x8b, 0x22, 0x6c, 0x4a, 0x55, 0x9c, 0x08, 0x8e])
    }
}

unsafe impl RefCounted for nsIHelperAppLauncher {
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
pub trait nsIHelperAppLauncherCoerce {
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self;
}

impl nsIHelperAppLauncherCoerce for nsIHelperAppLauncher {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self {
        v
    }
}

impl nsIHelperAppLauncher {
    #[inline]
    pub fn coerce<T: nsIHelperAppLauncherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHelperAppLauncher {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICancelableCoerce> nsIHelperAppLauncherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHelperAppLauncher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHelperAppLauncherVTable {
    pub __base: nsICancelableVTable,

    /* readonly attribute nsIMIMEInfo MIMEInfo; */
    pub get_MIMEInfo: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aMIMEInfo: *mut *const nsIMIMEInfo) -> nsresult,

    /* readonly attribute nsIURI source; */
    pub get_source: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aSource: *mut *const nsIURI) -> nsresult,

    /* readonly attribute AString suggestedFileName; */
    pub get_suggestedFileName: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aSuggestedFileName: *mut nsAString) -> nsresult,

    /* void saveToDisk (in nsIFile aNewFileLocation, in boolean aRememberThisPreference); */
    pub saveToDisk: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aNewFileLocation: *const nsIFile, aRememberThisPreference: bool) -> nsresult,

    /* void launchWithApplication (in nsIFile aApplication, in boolean aRememberThisPreference); */
    pub launchWithApplication: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aApplication: *const nsIFile, aRememberThisPreference: bool) -> nsresult,

    /* void saveDestinationAvailable (in nsIFile aFile); */
    pub saveDestinationAvailable: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aFile: *const nsIFile) -> nsresult,

    /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
    pub setWebProgressListener: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aWebProgressListener: *const nsIWebProgressListener2) -> nsresult,

    /* readonly attribute nsIFile targetFile; */
    pub get_targetFile: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aTargetFile: *mut *const nsIFile) -> nsresult,

    /* readonly attribute boolean targetFileIsExecutable; */
    pub get_targetFileIsExecutable: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aTargetFileIsExecutable: *mut bool) -> nsresult,

    /* readonly attribute PRTime timeDownloadStarted; */
    pub get_timeDownloadStarted: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aTimeDownloadStarted: *mut PRTime) -> nsresult,

    /* readonly attribute int64_t contentLength; */
    pub get_contentLength: unsafe extern "C" fn (this: *const nsIHelperAppLauncher, aContentLength: *mut int64_t) -> nsresult,

}


impl nsIHelperAppLauncher {
    /* readonly attribute nsIMIMEInfo MIMEInfo; */
    #[inline]
    pub unsafe fn get_MIMEInfo(&self, ) -> Result<Option<RefPtr<nsIMIMEInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_MIMEInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI source; */
    #[inline]
    pub unsafe fn get_source(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_source)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString suggestedFileName; */
    #[inline]
    pub unsafe fn get_suggestedFileName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_suggestedFileName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void saveToDisk (in nsIFile aNewFileLocation, in boolean aRememberThisPreference); */
    #[inline]
    pub unsafe fn saveToDisk(&self, aNewFileLocation: Option<&nsIFile>, aRememberThisPreference: bool) -> Result<(), nsresult> {

        match ((*self.vtable).saveToDisk)(self as *const _, aNewFileLocation.map_or(::std::ptr::null(), |x| x as *const _), aRememberThisPreference) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void launchWithApplication (in nsIFile aApplication, in boolean aRememberThisPreference); */
    #[inline]
    pub unsafe fn launchWithApplication(&self, aApplication: Option<&nsIFile>, aRememberThisPreference: bool) -> Result<(), nsresult> {

        match ((*self.vtable).launchWithApplication)(self as *const _, aApplication.map_or(::std::ptr::null(), |x| x as *const _), aRememberThisPreference) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveDestinationAvailable (in nsIFile aFile); */
    #[inline]
    pub unsafe fn saveDestinationAvailable(&self, aFile: Option<&nsIFile>) -> Result<(), nsresult> {

        match ((*self.vtable).saveDestinationAvailable)(self as *const _, aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setWebProgressListener (in nsIWebProgressListener2 aWebProgressListener); */
    #[inline]
    pub unsafe fn setWebProgressListener(&self, aWebProgressListener: Option<&nsIWebProgressListener2>) -> Result<(), nsresult> {

        match ((*self.vtable).setWebProgressListener)(self as *const _, aWebProgressListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile targetFile; */
    #[inline]
    pub unsafe fn get_targetFile(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_targetFile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean targetFileIsExecutable; */
    #[inline]
    pub unsafe fn get_targetFileIsExecutable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_targetFileIsExecutable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime timeDownloadStarted; */
    #[inline]
    pub unsafe fn get_timeDownloadStarted(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_timeDownloadStarted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t contentLength; */
    #[inline]
    pub unsafe fn get_contentLength(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_contentLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


