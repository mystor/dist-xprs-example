//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChromeRegistry.idl
//


pub mod nsIChromeRegistry_consts {
    pub const NONE: i64 = 0;
    pub const PARTIAL: i64 = 1;
    pub const FULL: i64 = 2;
}


#[repr(C)]
pub struct nsIChromeRegistry {
    vtable: *const nsIChromeRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIChromeRegistry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x249fb5ad, 0xae29, 0x4e2c,
            [0xa7, 0x28, 0xba, 0x5c, 0xf4, 0x64, 0xd1, 0x88])
    }
}

unsafe impl RefCounted for nsIChromeRegistry {
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
pub trait nsIChromeRegistryCoerce {
    fn coerce_from(v: &nsIChromeRegistry) -> &Self;
}

impl nsIChromeRegistryCoerce for nsIChromeRegistry {
    #[inline]
    fn coerce_from(v: &nsIChromeRegistry) -> &Self {
        v
    }
}

impl nsIChromeRegistry {
    #[inline]
    pub fn coerce<T: nsIChromeRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIChromeRegistry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIChromeRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChromeRegistry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIChromeRegistryVTable {
    pub __base: nsISupportsVTable,

    /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
    pub convertChromeURL: unsafe extern "C" fn (this: *const nsIChromeRegistry, aChromeURL: *const nsIURI, _retval: *mut *const nsIURI) -> nsresult,

    /* void checkForNewChrome (); */
    pub checkForNewChrome: unsafe extern "C" fn (this: *const nsIChromeRegistry) -> nsresult,

    /* [notxpcom] boolean wrappersEnabled (in nsIURI aURI); */
    pub wrappersEnabled: unsafe extern "C" fn (this: *const nsIChromeRegistry, aURI: *const nsIURI) -> bool,

}


impl nsIChromeRegistry {
    /* nsIURI convertChromeURL (in nsIURI aChromeURL); */
    #[inline]
    pub unsafe fn convertChromeURL(&self, aChromeURL: Option<&nsIURI>) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).convertChromeURL)(self as *const _, aChromeURL.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void checkForNewChrome (); */
    #[inline]
    pub unsafe fn checkForNewChrome(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkForNewChrome)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] boolean wrappersEnabled (in nsIURI aURI); */
    #[inline]
    pub unsafe fn wrappersEnabled(&self, aURI: Option<&nsIURI>) -> bool {

        let _retval = ((*self.vtable).wrappersEnabled)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _));
        _retval
    }

}


#[repr(C)]
pub struct nsIXULChromeRegistry {
    vtable: *const nsIXULChromeRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULChromeRegistry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x93251ddf, 0x5e85, 0x4172,
            [0xac, 0x2a, 0x31, 0x78, 0x05, 0x62, 0x97, 0x4f])
    }
}

unsafe impl RefCounted for nsIXULChromeRegistry {
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
pub trait nsIXULChromeRegistryCoerce {
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self;
}

impl nsIXULChromeRegistryCoerce for nsIXULChromeRegistry {
    #[inline]
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self {
        v
    }
}

impl nsIXULChromeRegistry {
    #[inline]
    pub fn coerce<T: nsIXULChromeRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULChromeRegistry {
    type Target = nsIChromeRegistry;
    #[inline]
    fn deref(&self) -> &nsIChromeRegistry {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIChromeRegistryCoerce> nsIXULChromeRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULChromeRegistry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULChromeRegistryVTable {
    pub __base: nsIChromeRegistryVTable,

    /* void reloadChrome (); */
    pub reloadChrome: unsafe extern "C" fn (this: *const nsIXULChromeRegistry) -> nsresult,

    /* ACString getSelectedLocale (in ACString packageName, [optional] in boolean asBCP47); */
    pub getSelectedLocale: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, packageName: *const nsACString, asBCP47: bool, _retval: *mut nsACString) -> nsresult,

    /* boolean isLocaleRTL (in ACString package); */
    pub isLocaleRTL: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, package: *const nsACString, _retval: *mut bool) -> nsresult,

    /* void refreshSkins (); */
    pub refreshSkins: unsafe extern "C" fn (this: *const nsIXULChromeRegistry) -> nsresult,

    /* boolean allowScriptsForPackage (in nsIURI url); */
    pub allowScriptsForPackage: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean allowContentToAccess (in nsIURI url); */
    pub allowContentToAccess: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean canLoadURLRemotely (in nsIURI url); */
    pub canLoadURLRemotely: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* boolean mustLoadURLRemotely (in nsIURI url); */
    pub mustLoadURLRemotely: unsafe extern "C" fn (this: *const nsIXULChromeRegistry, url: *const nsIURI, _retval: *mut bool) -> nsresult,

}


impl nsIXULChromeRegistry {
    /* void reloadChrome (); */
    #[inline]
    pub unsafe fn reloadChrome(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reloadChrome)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* ACString getSelectedLocale (in ACString packageName, [optional] in boolean asBCP47); */
    #[inline]
    pub unsafe fn getSelectedLocale(&self, packageName: &[u8], asBCP47: bool) -> Result<nsCString, nsresult> {
        let packageName = nsCString::from(packageName);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getSelectedLocale)(self as *const _, &*packageName, asBCP47, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isLocaleRTL (in ACString package); */
    #[inline]
    pub unsafe fn isLocaleRTL(&self, package: &[u8]) -> Result<bool, nsresult> {
        let package = nsCString::from(package);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isLocaleRTL)(self as *const _, &*package, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void refreshSkins (); */
    #[inline]
    pub unsafe fn refreshSkins(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).refreshSkins)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean allowScriptsForPackage (in nsIURI url); */
    #[inline]
    pub unsafe fn allowScriptsForPackage(&self, url: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowScriptsForPackage)(self as *const _, url.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean allowContentToAccess (in nsIURI url); */
    #[inline]
    pub unsafe fn allowContentToAccess(&self, url: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).allowContentToAccess)(self as *const _, url.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean canLoadURLRemotely (in nsIURI url); */
    #[inline]
    pub unsafe fn canLoadURLRemotely(&self, url: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).canLoadURLRemotely)(self as *const _, url.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean mustLoadURLRemotely (in nsIURI url); */
    #[inline]
    pub unsafe fn mustLoadURLRemotely(&self, url: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).mustLoadURLRemotely)(self as *const _, url.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


