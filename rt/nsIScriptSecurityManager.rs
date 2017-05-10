//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIScriptSecurityManager.idl
//


pub mod nsIScriptSecurityManager_consts {
    pub const STANDARD: i64 = 0;
    pub const LOAD_IS_AUTOMATIC_DOCUMENT_REPLACEMENT: i64 = 1;
    pub const ALLOW_CHROME: i64 = 2;
    pub const DISALLOW_INHERIT_PRINCIPAL: i64 = 4;
    pub const DISALLOW_SCRIPT_OR_DATA: i64 = 4;
    pub const DISALLOW_SCRIPT: i64 = 8;
    pub const DONT_REPORT_ERRORS: i64 = 16;
    pub const NO_APP_ID: i64 = 0;
    pub const UNKNOWN_APP_ID: i64 = 4294967295;
    pub const DEFAULT_USER_CONTEXT_ID: i64 = 0;
}


#[repr(C)]
pub struct nsIScriptSecurityManager {
    vtable: *const nsIScriptSecurityManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIScriptSecurityManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x51daad87, 0x3a0c, 0x44cc,
            [0xb6, 0x20, 0x73, 0x56, 0x80, 0x1c, 0x90, 0x22])
    }
}

unsafe impl RefCounted for nsIScriptSecurityManager {
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
pub trait nsIScriptSecurityManagerCoerce {
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self;
}

impl nsIScriptSecurityManagerCoerce for nsIScriptSecurityManager {
    #[inline]
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self {
        v
    }
}

impl nsIScriptSecurityManager {
    #[inline]
    pub fn coerce<T: nsIScriptSecurityManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIScriptSecurityManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIScriptSecurityManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptSecurityManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIScriptSecurityManagerVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void canCreateWrapper (in JSContextPtr aJSContext, in nsIIDRef aIID, in nsISupports aObj, in nsIClassInfo aClassInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub canCreateWrapper: *const ::libc::c_void,

    /* [noscript] void canCreateInstance (in JSContextPtr aJSContext, in nsCIDRef aCID); */
    /// Unable to call function as its signature contains a non-rust type
    pub canCreateInstance: *const ::libc::c_void,

    /* [noscript] void canGetService (in JSContextPtr aJSContext, in nsCIDRef aCID); */
    /// Unable to call function as its signature contains a non-rust type
    pub canGetService: *const ::libc::c_void,

    /* [noscript] void checkLoadURIFromScript (in JSContextPtr cx, in nsIURI uri); */
    /// Unable to call function as its signature contains a non-rust type
    pub checkLoadURIFromScript: *const ::libc::c_void,

    /* void checkLoadURIWithPrincipal (in nsIPrincipal aPrincipal, in nsIURI uri, in unsigned long flags); */
    pub checkLoadURIWithPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aPrincipal: *const nsIPrincipal, uri: *const nsIURI, flags: libc::uint32_t) -> nsresult,

    /* void checkLoadURIStrWithPrincipal (in nsIPrincipal aPrincipal, in AUTF8String uri, in unsigned long flags); */
    pub checkLoadURIStrWithPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aPrincipal: *const nsIPrincipal, uri: *const nsACString, flags: libc::uint32_t) -> nsresult,

    /* nsIPrincipal getSystemPrincipal (); */
    pub getSystemPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [deprecated] nsIPrincipal getAppCodebasePrincipal (in nsIURI uri, in unsigned long appId, in boolean inMozBrowser); */
    pub getAppCodebasePrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, appId: libc::uint32_t, inMozBrowser: bool, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* nsIPrincipal getLoadContextCodebasePrincipal (in nsIURI uri, in nsILoadContext loadContext); */
    pub getLoadContextCodebasePrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, loadContext: *const nsILoadContext, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* nsIPrincipal getDocShellCodebasePrincipal (in nsIURI uri, in nsIDocShell docShell); */
    pub getDocShellCodebasePrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, docShell: *const nsIDocShell, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [deprecated] nsIPrincipal getNoAppCodebasePrincipal (in nsIURI uri); */
    pub getNoAppCodebasePrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [deprecated] nsIPrincipal getCodebasePrincipal (in nsIURI uri); */
    pub getCodebasePrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, uri: *const nsIURI, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [implicit_jscontext] nsIPrincipal createCodebasePrincipal (in nsIURI uri, in jsval originAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub createCodebasePrincipal: *const ::libc::c_void,

    /* nsIPrincipal createCodebasePrincipalFromOrigin (in ACString origin); */
    pub createCodebasePrincipalFromOrigin: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, origin: *const nsACString, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [implicit_jscontext] nsIPrincipal createNullPrincipal (in jsval originAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub createNullPrincipal: *const ::libc::c_void,

    /* void checkSameOriginURI (in nsIURI aSourceURI, in nsIURI aTargetURI, in boolean reportError); */
    pub checkSameOriginURI: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aSourceURI: *const nsIURI, aTargetURI: *const nsIURI, reportError: bool) -> nsresult,

    /* nsIPrincipal getChannelResultPrincipal (in nsIChannel aChannel); */
    pub getChannelResultPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* [noscript,nostdcall] nsIPrincipal getChannelResultPrincipalIfNotSandboxed (in nsIChannel aChannel); */
    pub getChannelResultPrincipalIfNotSandboxed: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* nsIPrincipal getChannelURIPrincipal (in nsIChannel aChannel); */
    pub getChannelURIPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aChannel: *const nsIChannel, _retval: *mut *const nsIPrincipal) -> nsresult,

    /* boolean isSystemPrincipal (in nsIPrincipal aPrincipal); */
    pub isSystemPrincipal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aPrincipal: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* nsIDomainPolicy activateDomainPolicy (); */
    pub activateDomainPolicy: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, _retval: *mut *const nsIDomainPolicy) -> nsresult,

    /* readonly attribute boolean domainPolicyActive; */
    pub get_domainPolicyActive: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aDomainPolicyActive: *mut bool) -> nsresult,

    /* [noscript] nsIDomainPolicy activateDomainPolicyInternal (); */
    pub activateDomainPolicyInternal: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, _retval: *mut *const nsIDomainPolicy) -> nsresult,

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
    /// Unable to call function as its signature contains a non-rust type
    pub cloneDomainPolicy: *const ::libc::c_void,

    /* bool policyAllowsScript (in nsIURI aDomain); */
    pub policyAllowsScript: unsafe extern "C" fn (this: *const nsIScriptSecurityManager, aDomain: *const nsIURI, _retval: *mut bool) -> nsresult,

}


impl nsIScriptSecurityManager {
    /* [noscript] void canCreateWrapper (in JSContextPtr aJSContext, in nsIIDRef aIID, in nsISupports aObj, in nsIClassInfo aClassInfo); */


    /* [noscript] void canCreateInstance (in JSContextPtr aJSContext, in nsCIDRef aCID); */


    /* [noscript] void canGetService (in JSContextPtr aJSContext, in nsCIDRef aCID); */


    /* [noscript] void checkLoadURIFromScript (in JSContextPtr cx, in nsIURI uri); */


    /* void checkLoadURIWithPrincipal (in nsIPrincipal aPrincipal, in nsIURI uri, in unsigned long flags); */
    #[inline]
    pub unsafe fn checkLoadURIWithPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, uri: Option<&nsIURI>, flags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).checkLoadURIWithPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), uri.map_or(::std::ptr::null(), |x| x as *const _), flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void checkLoadURIStrWithPrincipal (in nsIPrincipal aPrincipal, in AUTF8String uri, in unsigned long flags); */
    #[inline]
    pub unsafe fn checkLoadURIStrWithPrincipal(&self, aPrincipal: Option<&nsIPrincipal>, uri: &[u8], flags: libc::uint32_t) -> Result<(), nsresult> {
        let uri = nsCString::from(uri);
        match ((*self.vtable).checkLoadURIStrWithPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &*uri, flags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPrincipal getSystemPrincipal (); */
    #[inline]
    pub unsafe fn getSystemPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSystemPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [deprecated] nsIPrincipal getAppCodebasePrincipal (in nsIURI uri, in unsigned long appId, in boolean inMozBrowser); */
    #[inline]
    pub unsafe fn getAppCodebasePrincipal(&self, uri: Option<&nsIURI>, appId: libc::uint32_t, inMozBrowser: bool) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAppCodebasePrincipal)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), appId, inMozBrowser, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPrincipal getLoadContextCodebasePrincipal (in nsIURI uri, in nsILoadContext loadContext); */
    #[inline]
    pub unsafe fn getLoadContextCodebasePrincipal(&self, uri: Option<&nsIURI>, loadContext: Option<&nsILoadContext>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getLoadContextCodebasePrincipal)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), loadContext.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPrincipal getDocShellCodebasePrincipal (in nsIURI uri, in nsIDocShell docShell); */
    #[inline]
    pub unsafe fn getDocShellCodebasePrincipal(&self, uri: Option<&nsIURI>, docShell: Option<&nsIDocShell>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDocShellCodebasePrincipal)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), docShell.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [deprecated] nsIPrincipal getNoAppCodebasePrincipal (in nsIURI uri); */
    #[inline]
    pub unsafe fn getNoAppCodebasePrincipal(&self, uri: Option<&nsIURI>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNoAppCodebasePrincipal)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [deprecated] nsIPrincipal getCodebasePrincipal (in nsIURI uri); */
    #[inline]
    pub unsafe fn getCodebasePrincipal(&self, uri: Option<&nsIURI>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getCodebasePrincipal)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] nsIPrincipal createCodebasePrincipal (in nsIURI uri, in jsval originAttributes); */


    /* nsIPrincipal createCodebasePrincipalFromOrigin (in ACString origin); */
    #[inline]
    pub unsafe fn createCodebasePrincipalFromOrigin(&self, origin: &[u8]) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let origin = nsCString::from(origin);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createCodebasePrincipalFromOrigin)(self as *const _, &*origin, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] nsIPrincipal createNullPrincipal (in jsval originAttributes); */


    /* void checkSameOriginURI (in nsIURI aSourceURI, in nsIURI aTargetURI, in boolean reportError); */
    #[inline]
    pub unsafe fn checkSameOriginURI(&self, aSourceURI: Option<&nsIURI>, aTargetURI: Option<&nsIURI>, reportError: bool) -> Result<(), nsresult> {

        match ((*self.vtable).checkSameOriginURI)(self as *const _, aSourceURI.map_or(::std::ptr::null(), |x| x as *const _), aTargetURI.map_or(::std::ptr::null(), |x| x as *const _), reportError) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIPrincipal getChannelResultPrincipal (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getChannelResultPrincipal(&self, aChannel: Option<&nsIChannel>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChannelResultPrincipal)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,nostdcall] nsIPrincipal getChannelResultPrincipalIfNotSandboxed (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getChannelResultPrincipalIfNotSandboxed(&self, aChannel: Option<&nsIChannel>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChannelResultPrincipalIfNotSandboxed)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPrincipal getChannelURIPrincipal (in nsIChannel aChannel); */
    #[inline]
    pub unsafe fn getChannelURIPrincipal(&self, aChannel: Option<&nsIChannel>) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChannelURIPrincipal)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* boolean isSystemPrincipal (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn isSystemPrincipal(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isSystemPrincipal)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDomainPolicy activateDomainPolicy (); */
    #[inline]
    pub unsafe fn activateDomainPolicy(&self, ) -> Result<Option<RefPtr<nsIDomainPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).activateDomainPolicy)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean domainPolicyActive; */
    #[inline]
    pub unsafe fn get_domainPolicyActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_domainPolicyActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] nsIDomainPolicy activateDomainPolicyInternal (); */
    #[inline]
    pub unsafe fn activateDomainPolicyInternal(&self, ) -> Result<Option<RefPtr<nsIDomainPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).activateDomainPolicyInternal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */


    /* bool policyAllowsScript (in nsIURI aDomain); */
    #[inline]
    pub unsafe fn policyAllowsScript(&self, aDomain: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).policyAllowsScript)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


