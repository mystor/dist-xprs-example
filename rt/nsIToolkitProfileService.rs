//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIToolkitProfileService.idl
//


#[repr(C)]
pub struct nsIToolkitProfileService {
    vtable: *const nsIToolkitProfileServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIToolkitProfileService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1947899b, 0xf369, 0x48fa,
            [0x89, 0xda, 0xf7, 0xc3, 0x7b, 0xb1, 0xe6, 0xbc])
    }
}

unsafe impl RefCounted for nsIToolkitProfileService {
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
pub trait nsIToolkitProfileServiceCoerce {
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self;
}

impl nsIToolkitProfileServiceCoerce for nsIToolkitProfileService {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self {
        v
    }
}

impl nsIToolkitProfileService {
    #[inline]
    pub fn coerce<T: nsIToolkitProfileServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIToolkitProfileService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIToolkitProfileServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitProfileService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIToolkitProfileServiceVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean startWithLastProfile; */
    pub get_startWithLastProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aStartWithLastProfile: *mut bool) -> nsresult,
    pub set_startWithLastProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aStartWithLastProfile: bool) -> nsresult,

    /* attribute boolean startOffline; */
    pub get_startOffline: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aStartOffline: *mut bool) -> nsresult,
    pub set_startOffline: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aStartOffline: bool) -> nsresult,

    /* readonly attribute nsISimpleEnumerator profiles; */
    pub get_profiles: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aProfiles: *mut *const nsISimpleEnumerator) -> nsresult,

    /* attribute nsIToolkitProfile selectedProfile; */
    pub get_selectedProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aSelectedProfile: *mut *const nsIToolkitProfile) -> nsresult,
    pub set_selectedProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aSelectedProfile: *const nsIToolkitProfile) -> nsresult,

    /* attribute nsIToolkitProfile defaultProfile; */
    pub get_defaultProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aDefaultProfile: *mut *const nsIToolkitProfile) -> nsresult,
    pub set_defaultProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aDefaultProfile: *const nsIToolkitProfile) -> nsresult,

    /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
    pub getProfileByName: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aName: *const nsACString, _retval: *mut *const nsIToolkitProfile) -> nsresult,

    /* nsIProfileLock lockProfilePath (in nsIFile aDirectory, in nsIFile aTempDirectory); */
    pub lockProfilePath: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aDirectory: *const nsIFile, aTempDirectory: *const nsIFile, _retval: *mut *const nsIProfileLock) -> nsresult,

    /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
    pub createProfile: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aRootDir: *const nsIFile, aName: *const nsACString, _retval: *mut *const nsIToolkitProfile) -> nsresult,

    /* nsIToolkitProfile createDefaultProfileForApp (in AUTF8String aProfileName, in AUTF8String aAppName, in AUTF8String aVendorName); */
    pub createDefaultProfileForApp: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aProfileName: *const nsACString, aAppName: *const nsACString, aVendorName: *const nsACString, _retval: *mut *const nsIToolkitProfile) -> nsresult,

    /* readonly attribute unsigned long profileCount; */
    pub get_profileCount: unsafe extern "C" fn (this: *const nsIToolkitProfileService, aProfileCount: *mut libc::uint32_t) -> nsresult,

    /* void flush (); */
    pub flush: unsafe extern "C" fn (this: *const nsIToolkitProfileService) -> nsresult,

}


impl nsIToolkitProfileService {
    /* attribute boolean startWithLastProfile; */
    #[inline]
    pub unsafe fn get_startWithLastProfile(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_startWithLastProfile)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_startWithLastProfile(&self, aStartWithLastProfile: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_startWithLastProfile)(self as *const _, aStartWithLastProfile) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean startOffline; */
    #[inline]
    pub unsafe fn get_startOffline(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_startOffline)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_startOffline(&self, aStartOffline: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_startOffline)(self as *const _, aStartOffline) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISimpleEnumerator profiles; */
    #[inline]
    pub unsafe fn get_profiles(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_profiles)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIToolkitProfile selectedProfile; */
    #[inline]
    pub unsafe fn get_selectedProfile(&self, ) -> Result<Option<RefPtr<nsIToolkitProfile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_selectedProfile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_selectedProfile(&self, aSelectedProfile: Option<&nsIToolkitProfile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_selectedProfile)(self as *const _, aSelectedProfile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIToolkitProfile defaultProfile; */
    #[inline]
    pub unsafe fn get_defaultProfile(&self, ) -> Result<Option<RefPtr<nsIToolkitProfile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultProfile)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_defaultProfile(&self, aDefaultProfile: Option<&nsIToolkitProfile>) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultProfile)(self as *const _, aDefaultProfile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIToolkitProfile getProfileByName (in AUTF8String aName); */
    #[inline]
    pub unsafe fn getProfileByName(&self, aName: &[u8]) -> Result<Option<RefPtr<nsIToolkitProfile>>, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getProfileByName)(self as *const _, &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIProfileLock lockProfilePath (in nsIFile aDirectory, in nsIFile aTempDirectory); */
    #[inline]
    pub unsafe fn lockProfilePath(&self, aDirectory: Option<&nsIFile>, aTempDirectory: Option<&nsIFile>) -> Result<Option<RefPtr<nsIProfileLock>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).lockProfilePath)(self as *const _, aDirectory.map_or(::std::ptr::null(), |x| x as *const _), aTempDirectory.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIToolkitProfile createProfile (in nsIFile aRootDir, in AUTF8String aName); */
    #[inline]
    pub unsafe fn createProfile(&self, aRootDir: Option<&nsIFile>, aName: &[u8]) -> Result<Option<RefPtr<nsIToolkitProfile>>, nsresult> {
        let aName = nsCString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createProfile)(self as *const _, aRootDir.map_or(::std::ptr::null(), |x| x as *const _), &*aName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIToolkitProfile createDefaultProfileForApp (in AUTF8String aProfileName, in AUTF8String aAppName, in AUTF8String aVendorName); */
    #[inline]
    pub unsafe fn createDefaultProfileForApp(&self, aProfileName: &[u8], aAppName: &[u8], aVendorName: &[u8]) -> Result<Option<RefPtr<nsIToolkitProfile>>, nsresult> {
        let aProfileName = nsCString::from(aProfileName);
        let aAppName = nsCString::from(aAppName);
        let aVendorName = nsCString::from(aVendorName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createDefaultProfileForApp)(self as *const _, &*aProfileName, &*aAppName, &*aVendorName, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long profileCount; */
    #[inline]
    pub unsafe fn get_profileCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_profileCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void flush (); */
    #[inline]
    pub unsafe fn flush(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).flush)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


