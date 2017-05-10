//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsILoadInfo.idl
//


pub type nsSecurityFlags = libc::uint32_t;


pub mod nsILoadInfo_consts {
    pub const SEC_NORMAL: i64 = 0;
    pub const SEC_REQUIRE_SAME_ORIGIN_DATA_INHERITS: i64 = 1;
    pub const SEC_REQUIRE_SAME_ORIGIN_DATA_IS_BLOCKED: i64 = 2;
    pub const SEC_ALLOW_CROSS_ORIGIN_DATA_INHERITS: i64 = 4;
    pub const SEC_ALLOW_CROSS_ORIGIN_DATA_IS_NULL: i64 = 8;
    pub const SEC_REQUIRE_CORS_DATA_INHERITS: i64 = 16;
    pub const SEC_COOKIES_DEFAULT: i64 = 0;
    pub const SEC_COOKIES_INCLUDE: i64 = 32;
    pub const SEC_COOKIES_SAME_ORIGIN: i64 = 64;
    pub const SEC_COOKIES_OMIT: i64 = 96;
    pub const SEC_FORCE_INHERIT_PRINCIPAL: i64 = 128;
    pub const SEC_SANDBOXED: i64 = 256;
    pub const SEC_ABOUT_BLANK_INHERITS: i64 = 512;
    pub const SEC_ALLOW_CHROME: i64 = 1024;
    pub const SEC_DISALLOW_SCRIPT: i64 = 2048;
    pub const SEC_DONT_FOLLOW_REDIRECTS: i64 = 4096;
    pub const SEC_LOAD_ERROR_PAGE: i64 = 8192;
    pub const SEC_FORCE_INHERIT_PRINCIPAL_OVERRULE_OWNER: i64 = 16384;
    pub const TAINTING_BASIC: i64 = 0;
    pub const TAINTING_CORS: i64 = 1;
    pub const TAINTING_OPAQUE: i64 = 2;
}


#[repr(C)]
pub struct nsILoadInfo {
    vtable: *const nsILoadInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsILoadInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xddc65bf9, 0x2f60, 0x41ab,
            [0xb2, 0x2a, 0x4f, 0x1a, 0xe9, 0xef, 0xcd, 0x36])
    }
}

unsafe impl RefCounted for nsILoadInfo {
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
pub trait nsILoadInfoCoerce {
    fn coerce_from(v: &nsILoadInfo) -> &Self;
}

impl nsILoadInfoCoerce for nsILoadInfo {
    #[inline]
    fn coerce_from(v: &nsILoadInfo) -> &Self {
        v
    }
}

impl nsILoadInfo {
    #[inline]
    pub fn coerce<T: nsILoadInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsILoadInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsILoadInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsILoadInfoVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal loadingPrincipal; */
    pub get_loadingPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo, aLoadingPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* [binaryname(LoadingPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryLoadingPrincipal (); */
    pub LoadingPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo) -> *const nsIPrincipal,

    /* readonly attribute nsIPrincipal triggeringPrincipal; */
    pub get_triggeringPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo, aTriggeringPrincipal: *mut *const nsIPrincipal) -> nsresult,

    /* [binaryname(TriggeringPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryTriggeringPrincipal (); */
    pub TriggeringPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo) -> *const nsIPrincipal,

    /* attribute nsIPrincipal principalToInherit; */
    pub get_principalToInherit: unsafe extern "C" fn (this: *const nsILoadInfo, aPrincipalToInherit: *mut *const nsIPrincipal) -> nsresult,
    pub set_principalToInherit: unsafe extern "C" fn (this: *const nsILoadInfo, aPrincipalToInherit: *const nsIPrincipal) -> nsresult,

    /* [binaryname(PrincipalToInherit),noscript,nostdcall,notxpcom] nsIPrincipal binaryPrincipalToInherit (); */
    pub PrincipalToInherit: unsafe extern "C" fn (this: *const nsILoadInfo) -> *const nsIPrincipal,

    /* readonly attribute nsIDOMDocument loadingDocument; */
    pub get_loadingDocument: unsafe extern "C" fn (this: *const nsILoadInfo, aLoadingDocument: *mut *const nsIDOMDocument) -> nsresult,

    /* [binaryname(LoadingNode),noscript,nostdcall,notxpcom] nsINode binaryLoadingNode (); */
    pub LoadingNode: unsafe extern "C" fn (this: *const nsILoadInfo) -> *const nsINode,

    /* readonly attribute nsSecurityFlags securityFlags; */
    pub get_securityFlags: unsafe extern "C" fn (this: *const nsILoadInfo, aSecurityFlags: *mut nsSecurityFlags) -> nsresult,

    /* [infallible] readonly attribute unsigned long securityMode; */
    pub get_securityMode: unsafe extern "C" fn (this: *const nsILoadInfo, aSecurityMode: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean isInThirdPartyContext; */
    pub get_isInThirdPartyContext: unsafe extern "C" fn (this: *const nsILoadInfo, aIsInThirdPartyContext: *mut bool) -> nsresult,

    /* [infallible] readonly attribute unsigned long cookiePolicy; */
    pub get_cookiePolicy: unsafe extern "C" fn (this: *const nsILoadInfo, aCookiePolicy: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipal; */
    pub get_forceInheritPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo, aForceInheritPrincipal: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipalOverruleOwner; */
    pub get_forceInheritPrincipalOverruleOwner: unsafe extern "C" fn (this: *const nsILoadInfo, aForceInheritPrincipalOverruleOwner: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean loadingSandboxed; */
    pub get_loadingSandboxed: unsafe extern "C" fn (this: *const nsILoadInfo, aLoadingSandboxed: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean aboutBlankInherits; */
    pub get_aboutBlankInherits: unsafe extern "C" fn (this: *const nsILoadInfo, aAboutBlankInherits: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean allowChrome; */
    pub get_allowChrome: unsafe extern "C" fn (this: *const nsILoadInfo, aAllowChrome: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean disallowScript; */
    pub get_disallowScript: unsafe extern "C" fn (this: *const nsILoadInfo, aDisallowScript: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean dontFollowRedirects; */
    pub get_dontFollowRedirects: unsafe extern "C" fn (this: *const nsILoadInfo, aDontFollowRedirects: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean loadErrorPage; */
    pub get_loadErrorPage: unsafe extern "C" fn (this: *const nsILoadInfo, aLoadErrorPage: *mut bool) -> nsresult,

    /* readonly attribute nsContentPolicyType externalContentPolicyType; */
    pub get_externalContentPolicyType: unsafe extern "C" fn (this: *const nsILoadInfo, aExternalContentPolicyType: *mut nsContentPolicyType) -> nsresult,

    /* [noscript,notxpcom] nsContentPolicyType internalContentPolicyType (); */
    pub internalContentPolicyType: unsafe extern "C" fn (this: *const nsILoadInfo) -> nsContentPolicyType,

    /* [infallible] readonly attribute boolean upgradeInsecureRequests; */
    pub get_upgradeInsecureRequests: unsafe extern "C" fn (this: *const nsILoadInfo, aUpgradeInsecureRequests: *mut bool) -> nsresult,

    /* [infallible] attribute boolean verifySignedContent; */
    pub get_verifySignedContent: unsafe extern "C" fn (this: *const nsILoadInfo, aVerifySignedContent: *mut bool) -> nsresult,
    pub set_verifySignedContent: unsafe extern "C" fn (this: *const nsILoadInfo, aVerifySignedContent: bool) -> nsresult,

    /* [infallible] attribute boolean enforceSRI; */
    pub get_enforceSRI: unsafe extern "C" fn (this: *const nsILoadInfo, aEnforceSRI: *mut bool) -> nsresult,
    pub set_enforceSRI: unsafe extern "C" fn (this: *const nsILoadInfo, aEnforceSRI: bool) -> nsresult,

    /* [infallible] readonly attribute boolean forceInheritPrincipalDropped; */
    pub get_forceInheritPrincipalDropped: unsafe extern "C" fn (this: *const nsILoadInfo, aForceInheritPrincipalDropped: *mut bool) -> nsresult,

    /* [infallible] readonly attribute unsigned long long innerWindowID; */
    pub get_innerWindowID: unsafe extern "C" fn (this: *const nsILoadInfo, aInnerWindowID: *mut libc::uint64_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long long outerWindowID; */
    pub get_outerWindowID: unsafe extern "C" fn (this: *const nsILoadInfo, aOuterWindowID: *mut libc::uint64_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long long parentOuterWindowID; */
    pub get_parentOuterWindowID: unsafe extern "C" fn (this: *const nsILoadInfo, aParentOuterWindowID: *mut libc::uint64_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long long frameOuterWindowID; */
    pub get_frameOuterWindowID: unsafe extern "C" fn (this: *const nsILoadInfo, aFrameOuterWindowID: *mut libc::uint64_t) -> nsresult,

    /* void resetPrincipalToInheritToNullPrincipal (); */
    pub resetPrincipalToInheritToNullPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo) -> nsresult,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_ScriptableOriginAttributes: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_ScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetOriginAttributes: *const ::libc::c_void,

    /* [infallible] attribute boolean enforceSecurity; */
    pub get_enforceSecurity: unsafe extern "C" fn (this: *const nsILoadInfo, aEnforceSecurity: *mut bool) -> nsresult,
    pub set_enforceSecurity: unsafe extern "C" fn (this: *const nsILoadInfo, aEnforceSecurity: bool) -> nsresult,

    /* [infallible] attribute boolean initialSecurityCheckDone; */
    pub get_initialSecurityCheckDone: unsafe extern "C" fn (this: *const nsILoadInfo, aInitialSecurityCheckDone: *mut bool) -> nsresult,
    pub set_initialSecurityCheckDone: unsafe extern "C" fn (this: *const nsILoadInfo, aInitialSecurityCheckDone: bool) -> nsresult,

    /* void appendRedirectedPrincipal (in nsIPrincipal principal, in boolean isInternalRedirect); */
    pub appendRedirectedPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo, principal: *const nsIPrincipal, isInternalRedirect: bool) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval redirectChainIncludingInternalRedirects; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_redirectChainIncludingInternalRedirects: *const ::libc::c_void,

    /* [binaryname(RedirectChainIncludingInternalRedirects),noscript,nostdcall,notxpcom] const_nsIPrincipalArray binaryRedirectChainIncludingInternalRedirects (); */
    /// Unable to call function as its signature contains a non-rust type
    pub RedirectChainIncludingInternalRedirects: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval redirectChain; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_redirectChain: *const ::libc::c_void,

    /* [binaryname(RedirectChain),noscript,nostdcall,notxpcom] const_nsIPrincipalArray binaryRedirectChain (); */
    /// Unable to call function as its signature contains a non-rust type
    pub RedirectChain: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setCorsPreflightInfo (in StringArrayRef unsafeHeaders, in boolean forcePreflight); */
    /// Unable to call function as its signature contains a non-rust type
    pub setCorsPreflightInfo: *const ::libc::c_void,

    /* [binaryname(CorsUnsafeHeaders),noscript,nostdcall,notxpcom] StringArrayRef corsUnsafeHeaders (); */
    /// Unable to call function as its signature contains a non-rust type
    pub CorsUnsafeHeaders: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean forcePreflight; */
    pub get_forcePreflight: unsafe extern "C" fn (this: *const nsILoadInfo, aForcePreflight: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isPreflight; */
    pub get_isPreflight: unsafe extern "C" fn (this: *const nsILoadInfo, aIsPreflight: *mut bool) -> nsresult,

    /* [infallible,noscript] readonly attribute boolean forceHSTSPriming; */
    pub get_forceHSTSPriming: unsafe extern "C" fn (this: *const nsILoadInfo, aForceHSTSPriming: *mut bool) -> nsresult,

    /* [infallible,noscript] readonly attribute boolean mixedContentWouldBlock; */
    pub get_mixedContentWouldBlock: unsafe extern "C" fn (this: *const nsILoadInfo, aMixedContentWouldBlock: *mut bool) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void setHSTSPriming (in boolean mixeContentWouldBlock); */
    pub setHSTSPriming: unsafe extern "C" fn (this: *const nsILoadInfo, mixeContentWouldBlock: bool) -> libc::c_void,

    /* [noscript,nostdcall,notxpcom] void clearHSTSPriming (); */
    pub clearHSTSPriming: unsafe extern "C" fn (this: *const nsILoadInfo) -> libc::c_void,

    /* readonly attribute unsigned long tainting; */
    pub get_tainting: unsafe extern "C" fn (this: *const nsILoadInfo, aTainting: *mut libc::uint32_t) -> nsresult,

    /* void maybeIncreaseTainting (in unsigned long aTainting); */
    pub maybeIncreaseTainting: unsafe extern "C" fn (this: *const nsILoadInfo, aTainting: libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean isTopLevelLoad; */
    pub get_isTopLevelLoad: unsafe extern "C" fn (this: *const nsILoadInfo, aIsTopLevelLoad: *mut bool) -> nsresult,

    /* attribute nsIURI resultPrincipalURI; */
    pub get_resultPrincipalURI: unsafe extern "C" fn (this: *const nsILoadInfo, aResultPrincipalURI: *mut *const nsIURI) -> nsresult,
    pub set_resultPrincipalURI: unsafe extern "C" fn (this: *const nsILoadInfo, aResultPrincipalURI: *const nsIURI) -> nsresult,

    /* [noscript] readonly attribute nsIPrincipal sandboxedLoadingPrincipal; */
    pub get_sandboxedLoadingPrincipal: unsafe extern "C" fn (this: *const nsILoadInfo, aSandboxedLoadingPrincipal: *mut *const nsIPrincipal) -> nsresult,

}


impl nsILoadInfo {
    /* readonly attribute nsIPrincipal loadingPrincipal; */
    #[inline]
    pub unsafe fn get_loadingPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadingPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(LoadingPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryLoadingPrincipal (); */
    #[inline]
    pub unsafe fn LoadingPrincipal(&self, ) -> Option<RefPtr<nsIPrincipal>> {

        let _retval = ((*self.vtable).LoadingPrincipal)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* readonly attribute nsIPrincipal triggeringPrincipal; */
    #[inline]
    pub unsafe fn get_triggeringPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_triggeringPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(TriggeringPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryTriggeringPrincipal (); */
    #[inline]
    pub unsafe fn TriggeringPrincipal(&self, ) -> Option<RefPtr<nsIPrincipal>> {

        let _retval = ((*self.vtable).TriggeringPrincipal)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* attribute nsIPrincipal principalToInherit; */
    #[inline]
    pub unsafe fn get_principalToInherit(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principalToInherit)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_principalToInherit(&self, aPrincipalToInherit: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).set_principalToInherit)(self as *const _, aPrincipalToInherit.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(PrincipalToInherit),noscript,nostdcall,notxpcom] nsIPrincipal binaryPrincipalToInherit (); */
    #[inline]
    pub unsafe fn PrincipalToInherit(&self, ) -> Option<RefPtr<nsIPrincipal>> {

        let _retval = ((*self.vtable).PrincipalToInherit)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* readonly attribute nsIDOMDocument loadingDocument; */
    #[inline]
    pub unsafe fn get_loadingDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadingDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(LoadingNode),noscript,nostdcall,notxpcom] nsINode binaryLoadingNode (); */
    #[inline]
    pub unsafe fn LoadingNode(&self, ) -> Option<RefPtr<nsINode>> {

        let _retval = ((*self.vtable).LoadingNode)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* readonly attribute nsSecurityFlags securityFlags; */
    #[inline]
    pub unsafe fn get_securityFlags(&self, ) -> Result<nsSecurityFlags, nsresult> {
        let mut _retval: nsSecurityFlags = ::std::mem::zeroed();
        match ((*self.vtable).get_securityFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long securityMode; */
    #[inline]
    pub unsafe fn get_securityMode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_securityMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isInThirdPartyContext; */
    #[inline]
    pub unsafe fn get_isInThirdPartyContext(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInThirdPartyContext)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long cookiePolicy; */
    #[inline]
    pub unsafe fn get_cookiePolicy(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_cookiePolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean forceInheritPrincipal; */
    #[inline]
    pub unsafe fn get_forceInheritPrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceInheritPrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean forceInheritPrincipalOverruleOwner; */
    #[inline]
    pub unsafe fn get_forceInheritPrincipalOverruleOwner(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceInheritPrincipalOverruleOwner)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean loadingSandboxed; */
    #[inline]
    pub unsafe fn get_loadingSandboxed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadingSandboxed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean aboutBlankInherits; */
    #[inline]
    pub unsafe fn get_aboutBlankInherits(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_aboutBlankInherits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean allowChrome; */
    #[inline]
    pub unsafe fn get_allowChrome(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowChrome)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean disallowScript; */
    #[inline]
    pub unsafe fn get_disallowScript(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_disallowScript)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean dontFollowRedirects; */
    #[inline]
    pub unsafe fn get_dontFollowRedirects(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_dontFollowRedirects)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean loadErrorPage; */
    #[inline]
    pub unsafe fn get_loadErrorPage(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadErrorPage)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsContentPolicyType externalContentPolicyType; */
    #[inline]
    pub unsafe fn get_externalContentPolicyType(&self, ) -> Result<nsContentPolicyType, nsresult> {
        let mut _retval: nsContentPolicyType = ::std::mem::zeroed();
        match ((*self.vtable).get_externalContentPolicyType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] nsContentPolicyType internalContentPolicyType (); */
    #[inline]
    pub unsafe fn internalContentPolicyType(&self, ) -> nsContentPolicyType {

        let _retval = ((*self.vtable).internalContentPolicyType)(self as *const _, );
        _retval
    }

    /* [infallible] readonly attribute boolean upgradeInsecureRequests; */
    #[inline]
    pub unsafe fn get_upgradeInsecureRequests(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_upgradeInsecureRequests)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] attribute boolean verifySignedContent; */
    #[inline]
    pub unsafe fn get_verifySignedContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_verifySignedContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_verifySignedContent(&self, aVerifySignedContent: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_verifySignedContent)(self as *const _, aVerifySignedContent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean enforceSRI; */
    #[inline]
    pub unsafe fn get_enforceSRI(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enforceSRI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enforceSRI(&self, aEnforceSRI: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_enforceSRI)(self as *const _, aEnforceSRI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean forceInheritPrincipalDropped; */
    #[inline]
    pub unsafe fn get_forceInheritPrincipalDropped(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceInheritPrincipalDropped)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long long innerWindowID; */
    #[inline]
    pub unsafe fn get_innerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_innerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long long outerWindowID; */
    #[inline]
    pub unsafe fn get_outerWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_outerWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long long parentOuterWindowID; */
    #[inline]
    pub unsafe fn get_parentOuterWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_parentOuterWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long long frameOuterWindowID; */
    #[inline]
    pub unsafe fn get_frameOuterWindowID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_frameOuterWindowID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void resetPrincipalToInheritToNullPrincipal (); */
    #[inline]
    pub unsafe fn resetPrincipalToInheritToNullPrincipal(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetPrincipalToInheritToNullPrincipal)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */



    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */


    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */


    /* [infallible] attribute boolean enforceSecurity; */
    #[inline]
    pub unsafe fn get_enforceSecurity(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_enforceSecurity)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_enforceSecurity(&self, aEnforceSecurity: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_enforceSecurity)(self as *const _, aEnforceSecurity) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean initialSecurityCheckDone; */
    #[inline]
    pub unsafe fn get_initialSecurityCheckDone(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_initialSecurityCheckDone)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_initialSecurityCheckDone(&self, aInitialSecurityCheckDone: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_initialSecurityCheckDone)(self as *const _, aInitialSecurityCheckDone) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void appendRedirectedPrincipal (in nsIPrincipal principal, in boolean isInternalRedirect); */
    #[inline]
    pub unsafe fn appendRedirectedPrincipal(&self, principal: Option<&nsIPrincipal>, isInternalRedirect: bool) -> Result<(), nsresult> {

        match ((*self.vtable).appendRedirectedPrincipal)(self as *const _, principal.map_or(::std::ptr::null(), |x| x as *const _), isInternalRedirect) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] readonly attribute jsval redirectChainIncludingInternalRedirects; */


    /* [binaryname(RedirectChainIncludingInternalRedirects),noscript,nostdcall,notxpcom] const_nsIPrincipalArray binaryRedirectChainIncludingInternalRedirects (); */


    /* [implicit_jscontext] readonly attribute jsval redirectChain; */


    /* [binaryname(RedirectChain),noscript,nostdcall,notxpcom] const_nsIPrincipalArray binaryRedirectChain (); */


    /* [noscript,nostdcall,notxpcom] void setCorsPreflightInfo (in StringArrayRef unsafeHeaders, in boolean forcePreflight); */


    /* [binaryname(CorsUnsafeHeaders),noscript,nostdcall,notxpcom] StringArrayRef corsUnsafeHeaders (); */


    /* [infallible] readonly attribute boolean forcePreflight; */
    #[inline]
    pub unsafe fn get_forcePreflight(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forcePreflight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isPreflight; */
    #[inline]
    pub unsafe fn get_isPreflight(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPreflight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible,noscript] readonly attribute boolean forceHSTSPriming; */
    #[inline]
    pub unsafe fn get_forceHSTSPriming(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_forceHSTSPriming)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible,noscript] readonly attribute boolean mixedContentWouldBlock; */
    #[inline]
    pub unsafe fn get_mixedContentWouldBlock(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mixedContentWouldBlock)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall,notxpcom] void setHSTSPriming (in boolean mixeContentWouldBlock); */
    #[inline]
    pub unsafe fn setHSTSPriming(&self, mixeContentWouldBlock: bool) -> () {

        let _retval = ((*self.vtable).setHSTSPriming)(self as *const _, mixeContentWouldBlock);
        ()
    }

    /* [noscript,nostdcall,notxpcom] void clearHSTSPriming (); */
    #[inline]
    pub unsafe fn clearHSTSPriming(&self, ) -> () {

        let _retval = ((*self.vtable).clearHSTSPriming)(self as *const _, );
        ()
    }

    /* readonly attribute unsigned long tainting; */
    #[inline]
    pub unsafe fn get_tainting(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tainting)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void maybeIncreaseTainting (in unsigned long aTainting); */
    #[inline]
    pub unsafe fn maybeIncreaseTainting(&self, aTainting: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).maybeIncreaseTainting)(self as *const _, aTainting) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isTopLevelLoad; */
    #[inline]
    pub unsafe fn get_isTopLevelLoad(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTopLevelLoad)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIURI resultPrincipalURI; */
    #[inline]
    pub unsafe fn get_resultPrincipalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_resultPrincipalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_resultPrincipalURI(&self, aResultPrincipalURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_resultPrincipalURI)(self as *const _, aResultPrincipalURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute nsIPrincipal sandboxedLoadingPrincipal; */
    #[inline]
    pub unsafe fn get_sandboxedLoadingPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_sandboxedLoadingPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


