//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIShellService.idl
//


pub mod nsIShellService_consts {
    pub const BACKGROUND_TILE: i64 = 1;
    pub const BACKGROUND_STRETCH: i64 = 2;
    pub const BACKGROUND_CENTER: i64 = 3;
    pub const BACKGROUND_FILL: i64 = 4;
    pub const BACKGROUND_FIT: i64 = 5;
    pub const APPLICATION_MAIL: i64 = 0;
    pub const APPLICATION_NEWS: i64 = 1;
}


#[repr(C)]
pub struct nsIShellService {
    vtable: *const nsIShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIShellService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2d1a95e4, 0x5bd8, 0x4eeb,
            [0xb0, 0xa8, 0xc1, 0x45, 0x5f, 0xd2, 0xa3, 0x57])
    }
}

unsafe impl RefCounted for nsIShellService {
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
pub trait nsIShellServiceCoerce {
    fn coerce_from(v: &nsIShellService) -> &Self;
}

impl nsIShellServiceCoerce for nsIShellService {
    #[inline]
    fn coerce_from(v: &nsIShellService) -> &Self {
        v
    }
}

impl nsIShellService {
    #[inline]
    pub fn coerce<T: nsIShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIShellService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIShellService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIShellServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean isDefaultBrowser (in boolean aStartupCheck, [optional] in boolean aForAllTypes); */
    pub isDefaultBrowser: unsafe extern "C" fn (this: *const nsIShellService, aStartupCheck: bool, aForAllTypes: bool, _retval: *mut bool) -> nsresult,

    /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
    pub setDefaultBrowser: unsafe extern "C" fn (this: *const nsIShellService, aClaimAllTypes: bool, aForAllUsers: bool) -> nsresult,

    /* void setDesktopBackground (in nsIDOMElement aElement, in long aPosition); */
    pub setDesktopBackground: unsafe extern "C" fn (this: *const nsIShellService, aElement: *const nsIDOMElement, aPosition: libc::int32_t) -> nsresult,

    /* void openApplication (in long aApplication); */
    pub openApplication: unsafe extern "C" fn (this: *const nsIShellService, aApplication: libc::int32_t) -> nsresult,

    /* attribute unsigned long desktopBackgroundColor; */
    pub get_desktopBackgroundColor: unsafe extern "C" fn (this: *const nsIShellService, aDesktopBackgroundColor: *mut libc::uint32_t) -> nsresult,
    pub set_desktopBackgroundColor: unsafe extern "C" fn (this: *const nsIShellService, aDesktopBackgroundColor: libc::uint32_t) -> nsresult,

    /* void openApplicationWithURI (in nsIFile aApplication, in ACString aURI); */
    pub openApplicationWithURI: unsafe extern "C" fn (this: *const nsIShellService, aApplication: *const nsIFile, aURI: *const nsACString) -> nsresult,

    /* readonly attribute nsIFile defaultFeedReader; */
    pub get_defaultFeedReader: unsafe extern "C" fn (this: *const nsIShellService, aDefaultFeedReader: *mut *const nsIFile) -> nsresult,

}


impl nsIShellService {
    /* boolean isDefaultBrowser (in boolean aStartupCheck, [optional] in boolean aForAllTypes); */
    #[inline]
    pub unsafe fn isDefaultBrowser(&self, aStartupCheck: bool, aForAllTypes: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDefaultBrowser)(self as *const _, aStartupCheck, aForAllTypes, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
    #[inline]
    pub unsafe fn setDefaultBrowser(&self, aClaimAllTypes: bool, aForAllUsers: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setDefaultBrowser)(self as *const _, aClaimAllTypes, aForAllUsers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setDesktopBackground (in nsIDOMElement aElement, in long aPosition); */
    #[inline]
    pub unsafe fn setDesktopBackground(&self, aElement: Option<&nsIDOMElement>, aPosition: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setDesktopBackground)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openApplication (in long aApplication); */
    #[inline]
    pub unsafe fn openApplication(&self, aApplication: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).openApplication)(self as *const _, aApplication) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long desktopBackgroundColor; */
    #[inline]
    pub unsafe fn get_desktopBackgroundColor(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_desktopBackgroundColor)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_desktopBackgroundColor(&self, aDesktopBackgroundColor: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_desktopBackgroundColor)(self as *const _, aDesktopBackgroundColor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void openApplicationWithURI (in nsIFile aApplication, in ACString aURI); */
    #[inline]
    pub unsafe fn openApplicationWithURI(&self, aApplication: Option<&nsIFile>, aURI: &[u8]) -> Result<(), nsresult> {
        let aURI = nsCString::from(aURI);
        match ((*self.vtable).openApplicationWithURI)(self as *const _, aApplication.map_or(::std::ptr::null(), |x| x as *const _), &*aURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIFile defaultFeedReader; */
    #[inline]
    pub unsafe fn get_defaultFeedReader(&self, ) -> Result<Option<RefPtr<nsIFile>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_defaultFeedReader)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


