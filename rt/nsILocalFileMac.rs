//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILocalFileMac.idl
//


#[repr(C)]
pub struct nsILocalFileMac {
    vtable: *const nsILocalFileMacVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILocalFileMac {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x623eca5b, 0xc25d, 0x4e27,
            [0xbe, 0x5a, 0x78, 0x9a, 0x66, 0xc4, 0xb2, 0xf7])
    }
}

unsafe impl RefCounted for nsILocalFileMac {
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
pub trait nsILocalFileMacCoerce {
    fn coerce_from(v: &nsILocalFileMac) -> &Self;
}

impl nsILocalFileMacCoerce for nsILocalFileMac {
    #[inline]
    fn coerce_from(v: &nsILocalFileMac) -> &Self {
        v
    }
}

impl nsILocalFileMac {
    #[inline]
    pub fn coerce<T: nsILocalFileMacCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILocalFileMac {
    type Target = nsILocalFile;
    #[inline]
    fn deref(&self) -> &nsILocalFile {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsILocalFileCoerce> nsILocalFileMacCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalFileMac) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILocalFileMacVTable {
    pub __base: nsILocalFileVTable,

    /* [noscript] void initWithCFURL (in CFURLRef aCFURL); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithCFURL: *const ::libc::c_void,

    /* [noscript] void initWithFSRef ([const] in FSRefPtr aFSRef); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithFSRef: *const ::libc::c_void,

    /* [noscript] CFURLRef getCFURL (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getCFURL: *const ::libc::c_void,

    /* [noscript] FSRef getFSRef (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFSRef: *const ::libc::c_void,

    /* [noscript] FSSpec getFSSpec (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getFSSpec: *const ::libc::c_void,

    /* readonly attribute int64_t fileSizeWithResFork; */
    pub get_fileSizeWithResFork: unsafe extern "C" fn (this: *const nsILocalFileMac, aFileSizeWithResFork: *mut int64_t) -> nsresult,

    /* [noscript] attribute OSType fileType; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_fileType: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_fileType: *const ::libc::c_void,

    /* [noscript] attribute OSType fileCreator; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_fileCreator: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_fileCreator: *const ::libc::c_void,

    /* void launchWithDoc (in nsIFile aDocToLoad, in boolean aLaunchInBackground); */
    pub launchWithDoc: unsafe extern "C" fn (this: *const nsILocalFileMac, aDocToLoad: *const nsIFile, aLaunchInBackground: bool) -> nsresult,

    /* void openDocWithApp (in nsIFile aAppToOpenWith, in boolean aLaunchInBackground); */
    pub openDocWithApp: unsafe extern "C" fn (this: *const nsILocalFileMac, aAppToOpenWith: *const nsIFile, aLaunchInBackground: bool) -> nsresult,

    /* boolean isPackage (); */
    pub isPackage: unsafe extern "C" fn (this: *const nsILocalFileMac, _retval: *mut bool) -> nsresult,

    /* readonly attribute AString bundleDisplayName; */
    pub get_bundleDisplayName: unsafe extern "C" fn (this: *const nsILocalFileMac, aBundleDisplayName: *mut nsAString) -> nsresult,

    /* readonly attribute AUTF8String bundleIdentifier; */
    pub get_bundleIdentifier: unsafe extern "C" fn (this: *const nsILocalFileMac, aBundleIdentifier: *mut nsACString) -> nsresult,

    /* readonly attribute int64_t bundleContentsLastModifiedTime; */
    pub get_bundleContentsLastModifiedTime: unsafe extern "C" fn (this: *const nsILocalFileMac, aBundleContentsLastModifiedTime: *mut int64_t) -> nsresult,

}


impl nsILocalFileMac {
    /* [noscript] void initWithCFURL (in CFURLRef aCFURL); */


    /* [noscript] void initWithFSRef ([const] in FSRefPtr aFSRef); */


    /* [noscript] CFURLRef getCFURL (); */


    /* [noscript] FSRef getFSRef (); */


    /* [noscript] FSSpec getFSSpec (); */


    /* readonly attribute int64_t fileSizeWithResFork; */
    #[inline]
    pub unsafe fn get_fileSizeWithResFork(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_fileSizeWithResFork)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] attribute OSType fileType; */



    /* [noscript] attribute OSType fileCreator; */



    /* void launchWithDoc (in nsIFile aDocToLoad, in boolean aLaunchInBackground); */
    #[inline]
    pub unsafe fn launchWithDoc(&self, aDocToLoad: Option<&nsIFile>, aLaunchInBackground: bool) -> Result<(), nsresult> {

        match ((*self.vtable).launchWithDoc)(self as *const _, aDocToLoad.map_or(::std::ptr::null(), |x| x as *const _), aLaunchInBackground) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openDocWithApp (in nsIFile aAppToOpenWith, in boolean aLaunchInBackground); */
    #[inline]
    pub unsafe fn openDocWithApp(&self, aAppToOpenWith: Option<&nsIFile>, aLaunchInBackground: bool) -> Result<(), nsresult> {

        match ((*self.vtable).openDocWithApp)(self as *const _, aAppToOpenWith.map_or(::std::ptr::null(), |x| x as *const _), aLaunchInBackground) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isPackage (); */
    #[inline]
    pub unsafe fn isPackage(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isPackage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString bundleDisplayName; */
    #[inline]
    pub unsafe fn get_bundleDisplayName(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_bundleDisplayName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String bundleIdentifier; */
    #[inline]
    pub unsafe fn get_bundleIdentifier(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_bundleIdentifier)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute int64_t bundleContentsLastModifiedTime; */
    #[inline]
    pub unsafe fn get_bundleContentsLastModifiedTime(&self, ) -> Result<int64_t, nsresult> {
        let mut _retval: int64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_bundleContentsLastModifiedTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


