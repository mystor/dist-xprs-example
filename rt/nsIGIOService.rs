//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIGIOService.idl
//


pub mod nsIGIOMimeApp_consts {
    pub const EXPECTS_URIS: i64 = 0;
    pub const EXPECTS_PATHS: i64 = 1;
    pub const EXPECTS_URIS_FOR_NON_FILES: i64 = 2;
}


#[repr(C)]
pub struct nsIGIOMimeApp {
    vtable: *const nsIGIOMimeAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGIOMimeApp {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xca6bad0c, 0x8a48, 0x48ac,
            [0x82, 0xc7, 0x27, 0xbb, 0x8f, 0x51, 0x0f, 0xbe])
    }
}

unsafe impl RefCounted for nsIGIOMimeApp {
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
pub trait nsIGIOMimeAppCoerce {
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self;
}

impl nsIGIOMimeAppCoerce for nsIGIOMimeApp {
    #[inline]
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self {
        v
    }
}

impl nsIGIOMimeApp {
    #[inline]
    pub fn coerce<T: nsIGIOMimeAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGIOMimeApp {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGIOMimeAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGIOMimeApp) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGIOMimeAppVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String id; */
    pub get_id: unsafe extern "C" fn (this: *const nsIGIOMimeApp, aId: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String name; */
    pub get_name: unsafe extern "C" fn (this: *const nsIGIOMimeApp, aName: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String command; */
    pub get_command: unsafe extern "C" fn (this: *const nsIGIOMimeApp, aCommand: *mut nsACString) -> nsresult,

    /* readonly attribute long expectsURIs; */
    pub get_expectsURIs: unsafe extern "C" fn (this: *const nsIGIOMimeApp, aExpectsURIs: *mut libc::int32_t) -> nsresult,

    /* readonly attribute nsIUTF8StringEnumerator supportedURISchemes; */
    pub get_supportedURISchemes: unsafe extern "C" fn (this: *const nsIGIOMimeApp, aSupportedURISchemes: *mut *const nsIUTF8StringEnumerator) -> nsresult,

    /* void launch (in AUTF8String uri); */
    pub launch: unsafe extern "C" fn (this: *const nsIGIOMimeApp, uri: *const nsACString) -> nsresult,

    /* void setAsDefaultForMimeType (in AUTF8String mimeType); */
    pub setAsDefaultForMimeType: unsafe extern "C" fn (this: *const nsIGIOMimeApp, mimeType: *const nsACString) -> nsresult,

    /* void setAsDefaultForFileExtensions (in AUTF8String extensions); */
    pub setAsDefaultForFileExtensions: unsafe extern "C" fn (this: *const nsIGIOMimeApp, extensions: *const nsACString) -> nsresult,

    /* void setAsDefaultForURIScheme (in AUTF8String uriScheme); */
    pub setAsDefaultForURIScheme: unsafe extern "C" fn (this: *const nsIGIOMimeApp, uriScheme: *const nsACString) -> nsresult,

}


impl nsIGIOMimeApp {
    /* readonly attribute AUTF8String id; */
    #[inline]
    pub unsafe fn get_id(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_id)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String name; */
    #[inline]
    pub unsafe fn get_name(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_name)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String command; */
    #[inline]
    pub unsafe fn get_command(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_command)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long expectsURIs; */
    #[inline]
    pub unsafe fn get_expectsURIs(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_expectsURIs)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIUTF8StringEnumerator supportedURISchemes; */
    #[inline]
    pub unsafe fn get_supportedURISchemes(&self, ) -> Result<Option<RefPtr<nsIUTF8StringEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_supportedURISchemes)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void launch (in AUTF8String uri); */
    #[inline]
    pub unsafe fn launch(&self, uri: &[u8]) -> Result<(), nsresult> {
        let uri = nsCString::from(uri);
        match ((*self.vtable).launch)(self as *const _, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsDefaultForMimeType (in AUTF8String mimeType); */
    #[inline]
    pub unsafe fn setAsDefaultForMimeType(&self, mimeType: &[u8]) -> Result<(), nsresult> {
        let mimeType = nsCString::from(mimeType);
        match ((*self.vtable).setAsDefaultForMimeType)(self as *const _, &*mimeType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsDefaultForFileExtensions (in AUTF8String extensions); */
    #[inline]
    pub unsafe fn setAsDefaultForFileExtensions(&self, extensions: &[u8]) -> Result<(), nsresult> {
        let extensions = nsCString::from(extensions);
        match ((*self.vtable).setAsDefaultForFileExtensions)(self as *const _, &*extensions) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setAsDefaultForURIScheme (in AUTF8String uriScheme); */
    #[inline]
    pub unsafe fn setAsDefaultForURIScheme(&self, uriScheme: &[u8]) -> Result<(), nsresult> {
        let uriScheme = nsCString::from(uriScheme);
        match ((*self.vtable).setAsDefaultForURIScheme)(self as *const _, &*uriScheme) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIGIOService {
    vtable: *const nsIGIOServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGIOService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xeda22a30, 0x84e1, 0x4e16,
            [0x9c, 0xa0, 0xcd, 0x15, 0x53, 0xc2, 0xb3, 0x4a])
    }
}

unsafe impl RefCounted for nsIGIOService {
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
pub trait nsIGIOServiceCoerce {
    fn coerce_from(v: &nsIGIOService) -> &Self;
}

impl nsIGIOServiceCoerce for nsIGIOService {
    #[inline]
    fn coerce_from(v: &nsIGIOService) -> &Self {
        v
    }
}

impl nsIGIOService {
    #[inline]
    pub fn coerce<T: nsIGIOServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGIOService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGIOServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGIOService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGIOServiceVTable {
    pub __base: nsISupportsVTable,

    /* AUTF8String getMimeTypeFromExtension (in AUTF8String extension); */
    pub getMimeTypeFromExtension: unsafe extern "C" fn (this: *const nsIGIOService, extension: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* nsIGIOMimeApp getAppForURIScheme (in AUTF8String aURIScheme); */
    pub getAppForURIScheme: unsafe extern "C" fn (this: *const nsIGIOService, aURIScheme: *const nsACString, _retval: *mut *const nsIGIOMimeApp) -> nsresult,

    /* nsIGIOMimeApp getAppForMimeType (in AUTF8String mimeType); */
    pub getAppForMimeType: unsafe extern "C" fn (this: *const nsIGIOService, mimeType: *const nsACString, _retval: *mut *const nsIGIOMimeApp) -> nsresult,

    /* nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName); */
    pub createAppFromCommand: unsafe extern "C" fn (this: *const nsIGIOService, cmd: *const nsACString, appName: *const nsACString, _retval: *mut *const nsIGIOMimeApp) -> nsresult,

    /* AUTF8String getDescriptionForMimeType (in AUTF8String mimeType); */
    pub getDescriptionForMimeType: unsafe extern "C" fn (this: *const nsIGIOService, mimeType: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* void showURI (in nsIURI uri); */
    pub showURI: unsafe extern "C" fn (this: *const nsIGIOService, uri: *const nsIURI) -> nsresult,

    /* [noscript] void showURIForInput (in ACString uri); */
    pub showURIForInput: unsafe extern "C" fn (this: *const nsIGIOService, uri: *const nsACString) -> nsresult,

    /* [noscript] void orgFreedesktopFileManager1ShowItems (in ACString path); */
    pub orgFreedesktopFileManager1ShowItems: unsafe extern "C" fn (this: *const nsIGIOService, path: *const nsACString) -> nsresult,

}


impl nsIGIOService {
    /* AUTF8String getMimeTypeFromExtension (in AUTF8String extension); */
    #[inline]
    pub unsafe fn getMimeTypeFromExtension(&self, extension: &[u8]) -> Result<nsCString, nsresult> {
        let extension = nsCString::from(extension);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getMimeTypeFromExtension)(self as *const _, &*extension, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIGIOMimeApp getAppForURIScheme (in AUTF8String aURIScheme); */
    #[inline]
    pub unsafe fn getAppForURIScheme(&self, aURIScheme: &[u8]) -> Result<Option<RefPtr<nsIGIOMimeApp>>, nsresult> {
        let aURIScheme = nsCString::from(aURIScheme);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAppForURIScheme)(self as *const _, &*aURIScheme, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIGIOMimeApp getAppForMimeType (in AUTF8String mimeType); */
    #[inline]
    pub unsafe fn getAppForMimeType(&self, mimeType: &[u8]) -> Result<Option<RefPtr<nsIGIOMimeApp>>, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAppForMimeType)(self as *const _, &*mimeType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIGIOMimeApp createAppFromCommand (in AUTF8String cmd, in AUTF8String appName); */
    #[inline]
    pub unsafe fn createAppFromCommand(&self, cmd: &[u8], appName: &[u8]) -> Result<Option<RefPtr<nsIGIOMimeApp>>, nsresult> {
        let cmd = nsCString::from(cmd);
        let appName = nsCString::from(appName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createAppFromCommand)(self as *const _, &*cmd, &*appName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* AUTF8String getDescriptionForMimeType (in AUTF8String mimeType); */
    #[inline]
    pub unsafe fn getDescriptionForMimeType(&self, mimeType: &[u8]) -> Result<nsCString, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getDescriptionForMimeType)(self as *const _, &*mimeType, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void showURI (in nsIURI uri); */
    #[inline]
    pub unsafe fn showURI(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).showURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void showURIForInput (in ACString uri); */
    #[inline]
    pub unsafe fn showURIForInput(&self, uri: &[u8]) -> Result<(), nsresult> {
        let uri = nsCString::from(uri);
        match ((*self.vtable).showURIForInput)(self as *const _, &*uri) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void orgFreedesktopFileManager1ShowItems (in ACString path); */
    #[inline]
    pub unsafe fn orgFreedesktopFileManager1ShowItems(&self, path: &[u8]) -> Result<(), nsresult> {
        let path = nsCString::from(path);
        match ((*self.vtable).orgFreedesktopFileManager1ShowItems)(self as *const _, &*path) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


