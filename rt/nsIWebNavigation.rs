//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebNavigation.idl
//


pub mod nsIWebNavigation_consts {
    pub const LOAD_FLAGS_MASK: i64 = 65535;
    pub const LOAD_FLAGS_NONE: i64 = 0;
    pub const LOAD_FLAGS_IS_REFRESH: i64 = 16;
    pub const LOAD_FLAGS_IS_LINK: i64 = 32;
    pub const LOAD_FLAGS_BYPASS_HISTORY: i64 = 64;
    pub const LOAD_FLAGS_REPLACE_HISTORY: i64 = 128;
    pub const LOAD_FLAGS_BYPASS_CACHE: i64 = 256;
    pub const LOAD_FLAGS_BYPASS_PROXY: i64 = 512;
    pub const LOAD_FLAGS_CHARSET_CHANGE: i64 = 1024;
    pub const LOAD_FLAGS_STOP_CONTENT: i64 = 2048;
    pub const LOAD_FLAGS_FROM_EXTERNAL: i64 = 4096;
    pub const LOAD_FLAGS_ALLOW_MIXED_CONTENT: i64 = 8192;
    pub const LOAD_FLAGS_FIRST_LOAD: i64 = 16384;
    pub const LOAD_FLAGS_ALLOW_POPUPS: i64 = 32768;
    pub const LOAD_FLAGS_BYPASS_CLASSIFIER: i64 = 65536;
    pub const LOAD_FLAGS_FORCE_ALLOW_COOKIES: i64 = 131072;
    pub const LOAD_FLAGS_DISALLOW_INHERIT_PRINCIPAL: i64 = 262144;
    pub const LOAD_FLAGS_DISALLOW_INHERIT_OWNER: i64 = 262144;
    pub const LOAD_FLAGS_ERROR_LOAD_CHANGES_RV: i64 = 524288;
    pub const LOAD_FLAGS_ALLOW_THIRD_PARTY_FIXUP: i64 = 1048576;
    pub const LOAD_FLAGS_FIXUP_SCHEME_TYPOS: i64 = 2097152;
    pub const STOP_NETWORK: i64 = 1;
    pub const STOP_CONTENT: i64 = 2;
    pub const STOP_ALL: i64 = 3;
}


#[repr(C)]
pub struct nsIWebNavigation {
    vtable: *const nsIWebNavigationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebNavigation {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ade79d4, 0x8cb9, 0x4952,
            [0xb1, 0x8d, 0x4f, 0x9b, 0x63, 0xca, 0x0d, 0x31])
    }
}

unsafe impl RefCounted for nsIWebNavigation {
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
pub trait nsIWebNavigationCoerce {
    fn coerce_from(v: &nsIWebNavigation) -> &Self;
}

impl nsIWebNavigationCoerce for nsIWebNavigation {
    #[inline]
    fn coerce_from(v: &nsIWebNavigation) -> &Self {
        v
    }
}

impl nsIWebNavigation {
    #[inline]
    pub fn coerce<T: nsIWebNavigationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebNavigation {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebNavigationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebNavigation) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebNavigationVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean canGoBack; */
    pub get_canGoBack: unsafe extern "C" fn (this: *const nsIWebNavigation, aCanGoBack: *mut bool) -> nsresult,

    /* readonly attribute boolean canGoForward; */
    pub get_canGoForward: unsafe extern "C" fn (this: *const nsIWebNavigation, aCanGoForward: *mut bool) -> nsresult,

    /* void goBack (); */
    pub goBack: unsafe extern "C" fn (this: *const nsIWebNavigation) -> nsresult,

    /* void goForward (); */
    pub goForward: unsafe extern "C" fn (this: *const nsIWebNavigation) -> nsresult,

    /* void gotoIndex (in long index); */
    pub gotoIndex: unsafe extern "C" fn (this: *const nsIWebNavigation, index: libc::int32_t) -> nsresult,

    /* void loadURI (in wstring aURI, in unsigned long aLoadFlags, in nsIURI aReferrer, in nsIInputStream aPostData, in nsIInputStream aHeaders, [optional] in nsIPrincipal aTriggeringPrincipal); */
    pub loadURI: unsafe extern "C" fn (this: *const nsIWebNavigation, aURI: *const libc::int16_t, aLoadFlags: libc::uint32_t, aReferrer: *const nsIURI, aPostData: *const nsIInputStream, aHeaders: *const nsIInputStream, aTriggeringPrincipal: *const nsIPrincipal) -> nsresult,

    /* void loadURIWithOptions (in wstring aURI, in unsigned long aLoadFlags, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in nsIInputStream aHeaders, in nsIURI aBaseURI, [optional] in nsIPrincipal aTriggeringPrincipal); */
    pub loadURIWithOptions: unsafe extern "C" fn (this: *const nsIWebNavigation, aURI: *const libc::int16_t, aLoadFlags: libc::uint32_t, aReferrer: *const nsIURI, aReferrerPolicy: libc::uint32_t, aPostData: *const nsIInputStream, aHeaders: *const nsIInputStream, aBaseURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal) -> nsresult,

    /* void reload (in unsigned long aReloadFlags); */
    pub reload: unsafe extern "C" fn (this: *const nsIWebNavigation, aReloadFlags: libc::uint32_t) -> nsresult,

    /* void stop (in unsigned long aStopFlags); */
    pub stop: unsafe extern "C" fn (this: *const nsIWebNavigation, aStopFlags: libc::uint32_t) -> nsresult,

    /* readonly attribute nsIDOMDocument document; */
    pub get_document: unsafe extern "C" fn (this: *const nsIWebNavigation, aDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* readonly attribute nsIURI currentURI; */
    pub get_currentURI: unsafe extern "C" fn (this: *const nsIWebNavigation, aCurrentURI: *mut *const nsIURI) -> nsresult,

    /* readonly attribute nsIURI referringURI; */
    pub get_referringURI: unsafe extern "C" fn (this: *const nsIWebNavigation, aReferringURI: *mut *const nsIURI) -> nsresult,

    /* attribute nsISHistory sessionHistory; */
    pub get_sessionHistory: unsafe extern "C" fn (this: *const nsIWebNavigation, aSessionHistory: *mut *const nsISHistory) -> nsresult,
    pub set_sessionHistory: unsafe extern "C" fn (this: *const nsIWebNavigation, aSessionHistory: *const nsISHistory) -> nsresult,

    /* void setOriginAttributesBeforeLoading (in jsval originAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub setOriginAttributesBeforeLoading: *const ::libc::c_void,

}


impl nsIWebNavigation {
    /* readonly attribute boolean canGoBack; */
    #[inline]
    pub unsafe fn get_canGoBack(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canGoBack)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean canGoForward; */
    #[inline]
    pub unsafe fn get_canGoForward(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canGoForward)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void goBack (); */
    #[inline]
    pub unsafe fn goBack(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).goBack)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void goForward (); */
    #[inline]
    pub unsafe fn goForward(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).goForward)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void gotoIndex (in long index); */
    #[inline]
    pub unsafe fn gotoIndex(&self, index: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).gotoIndex)(self as *const _, index) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadURI (in wstring aURI, in unsigned long aLoadFlags, in nsIURI aReferrer, in nsIInputStream aPostData, in nsIInputStream aHeaders, [optional] in nsIPrincipal aTriggeringPrincipal); */
    #[inline]
    pub unsafe fn loadURI(&self, aURI: *const libc::int16_t, aLoadFlags: libc::uint32_t, aReferrer: Option<&nsIURI>, aPostData: Option<&nsIInputStream>, aHeaders: Option<&nsIInputStream>, aTriggeringPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).loadURI)(self as *const _, aURI, aLoadFlags, aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aPostData.map_or(::std::ptr::null(), |x| x as *const _), aHeaders.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadURIWithOptions (in wstring aURI, in unsigned long aLoadFlags, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in nsIInputStream aHeaders, in nsIURI aBaseURI, [optional] in nsIPrincipal aTriggeringPrincipal); */
    #[inline]
    pub unsafe fn loadURIWithOptions(&self, aURI: *const libc::int16_t, aLoadFlags: libc::uint32_t, aReferrer: Option<&nsIURI>, aReferrerPolicy: libc::uint32_t, aPostData: Option<&nsIInputStream>, aHeaders: Option<&nsIInputStream>, aBaseURI: Option<&nsIURI>, aTriggeringPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).loadURIWithOptions)(self as *const _, aURI, aLoadFlags, aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aReferrerPolicy, aPostData.map_or(::std::ptr::null(), |x| x as *const _), aHeaders.map_or(::std::ptr::null(), |x| x as *const _), aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reload (in unsigned long aReloadFlags); */
    #[inline]
    pub unsafe fn reload(&self, aReloadFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).reload)(self as *const _, aReloadFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stop (in unsigned long aStopFlags); */
    #[inline]
    pub unsafe fn stop(&self, aStopFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, aStopFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMDocument document; */
    #[inline]
    pub unsafe fn get_document(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_document)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI currentURI; */
    #[inline]
    pub unsafe fn get_currentURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI referringURI; */
    #[inline]
    pub unsafe fn get_referringURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referringURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISHistory sessionHistory; */
    #[inline]
    pub unsafe fn get_sessionHistory(&self, ) -> Result<Option<RefPtr<nsISHistory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sessionHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_sessionHistory(&self, aSessionHistory: Option<&nsISHistory>) -> Result<(), nsresult> {

        match ((*self.vtable).set_sessionHistory)(self as *const _, aSessionHistory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setOriginAttributesBeforeLoading (in jsval originAttributes); */


}


