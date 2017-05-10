//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentSecurityPolicy.idl
//


pub type CSPDirective = libc::uint16_t;


pub mod nsIContentSecurityPolicy_consts {
    pub const NO_DIRECTIVE: i64 = 0;
    pub const DEFAULT_SRC_DIRECTIVE: i64 = 1;
    pub const SCRIPT_SRC_DIRECTIVE: i64 = 2;
    pub const OBJECT_SRC_DIRECTIVE: i64 = 3;
    pub const STYLE_SRC_DIRECTIVE: i64 = 4;
    pub const IMG_SRC_DIRECTIVE: i64 = 5;
    pub const MEDIA_SRC_DIRECTIVE: i64 = 6;
    pub const FRAME_SRC_DIRECTIVE: i64 = 7;
    pub const FONT_SRC_DIRECTIVE: i64 = 8;
    pub const CONNECT_SRC_DIRECTIVE: i64 = 9;
    pub const REPORT_URI_DIRECTIVE: i64 = 10;
    pub const FRAME_ANCESTORS_DIRECTIVE: i64 = 11;
    pub const REFLECTED_XSS_DIRECTIVE: i64 = 12;
    pub const BASE_URI_DIRECTIVE: i64 = 13;
    pub const FORM_ACTION_DIRECTIVE: i64 = 14;
    pub const REFERRER_DIRECTIVE: i64 = 15;
    pub const WEB_MANIFEST_SRC_DIRECTIVE: i64 = 16;
    pub const UPGRADE_IF_INSECURE_DIRECTIVE: i64 = 17;
    pub const CHILD_SRC_DIRECTIVE: i64 = 18;
    pub const BLOCK_ALL_MIXED_CONTENT: i64 = 19;
    pub const REQUIRE_SRI_FOR: i64 = 20;
    pub const SANDBOX_DIRECTIVE: i64 = 21;
    pub const VIOLATION_TYPE_INLINE_SCRIPT: i64 = 1;
    pub const VIOLATION_TYPE_EVAL: i64 = 2;
    pub const VIOLATION_TYPE_INLINE_STYLE: i64 = 3;
    pub const VIOLATION_TYPE_NONCE_SCRIPT: i64 = 4;
    pub const VIOLATION_TYPE_NONCE_STYLE: i64 = 5;
    pub const VIOLATION_TYPE_HASH_SCRIPT: i64 = 6;
    pub const VIOLATION_TYPE_HASH_STYLE: i64 = 7;
    pub const VIOLATION_TYPE_REQUIRE_SRI_FOR_STYLE: i64 = 8;
    pub const VIOLATION_TYPE_REQUIRE_SRI_FOR_SCRIPT: i64 = 9;
}


#[repr(C)]
pub struct nsIContentSecurityPolicy {
    vtable: *const nsIContentSecurityPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentSecurityPolicy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb3c4c0ae, 0xbd5e, 0x4cad,
            [0x87, 0xe0, 0x8d, 0x21, 0x0d, 0xbb, 0x3f, 0x9f])
    }
}

unsafe impl RefCounted for nsIContentSecurityPolicy {
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
pub trait nsIContentSecurityPolicyCoerce {
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self;
}

impl nsIContentSecurityPolicyCoerce for nsIContentSecurityPolicy {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self {
        v
    }
}

impl nsIContentSecurityPolicy {
    #[inline]
    pub fn coerce<T: nsIContentSecurityPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentSecurityPolicy {
    type Target = nsISerializable;
    #[inline]
    fn deref(&self) -> &nsISerializable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISerializableCoerce> nsIContentSecurityPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityPolicy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentSecurityPolicyVTable {
    pub __base: nsISerializableVTable,

    /* [binaryname(GetPolicyString)] AString getPolicy (in unsigned long index); */
    pub GetPolicyString: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, index: libc::uint32_t, _retval: *mut nsAString) -> nsresult,

    /* [noscript,nostdcall,notxpcom] CSPPolicyPtr GetPolicy (in unsigned long index); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetPolicy: *const ::libc::c_void,

    /* readonly attribute unsigned long policyCount; */
    pub get_policyCount: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aPolicyCount: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute bool upgradeInsecureRequests; */
    pub get_upgradeInsecureRequests: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aUpgradeInsecureRequests: *mut bool) -> nsresult,

    /* readonly attribute bool blockAllMixedContent; */
    pub get_blockAllMixedContent: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aBlockAllMixedContent: *mut bool) -> nsresult,

    /* bool getReferrerPolicy (out unsigned long policy); */
    pub getReferrerPolicy: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, policy: *mut libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* void appendPolicy (in AString policyString, in boolean reportOnly, in boolean deliveredViaMetaTag); */
    pub appendPolicy: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, policyString: *const nsAString, reportOnly: bool, deliveredViaMetaTag: bool) -> nsresult,

    /* boolean getAllowsInline (in nsContentPolicyType aContentPolicyType, in AString aNonce, in boolean aParserCreated, in AString aContent, in unsigned long aLineNumber); */
    pub getAllowsInline: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aContentPolicyType: nsContentPolicyType, aNonce: *const nsAString, aParserCreated: bool, aContent: *const nsAString, aLineNumber: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* boolean getAllowsEval (out boolean shouldReportViolations); */
    pub getAllowsEval: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, shouldReportViolations: *mut bool, _retval: *mut bool) -> nsresult,

    /* uint32_t getCSPSandboxFlags (); */
    pub getCSPSandboxFlags: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, _retval: *mut uint32_t) -> nsresult,

    /* void logViolationDetails (in unsigned short violationType, in AString sourceFile, in AString scriptSample, in int32_t lineNum, [optional] in AString nonce, [optional] in AString content); */
    pub logViolationDetails: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, violationType: libc::uint16_t, sourceFile: *const nsAString, scriptSample: *const nsAString, lineNum: int32_t, nonce: *const nsAString, content: *const nsAString) -> nsresult,

    /* void setRequestContext (in nsIDOMDocument aDocument, in nsIPrincipal aPrincipal); */
    pub setRequestContext: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aDocument: *const nsIDOMDocument, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* [noscript] void ensureEventTarget (in nsIEventTarget aEventTarget); */
    pub ensureEventTarget: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aEventTarget: *const nsIEventTarget) -> nsresult,

    /* bool requireSRIForType (in nsContentPolicyType aContentType); */
    pub requireSRIForType: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aContentType: nsContentPolicyType, _retval: *mut bool) -> nsresult,

    /* boolean permitsAncestry (in nsIDocShell docShell); */
    pub permitsAncestry: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, docShell: *const nsIDocShell, _retval: *mut bool) -> nsresult,

    /* boolean permits (in nsIURI aURI, in CSPDirective aDir, in boolean aSpecific); */
    pub permits: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aURI: *const nsIURI, aDir: CSPDirective, aSpecific: bool, _retval: *mut bool) -> nsresult,

    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeTypeGuess, in nsISupports aExtra); */
    pub shouldLoad: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, aContentType: nsContentPolicyType, aContentLocation: *const nsIURI, aRequestOrigin: *const nsIURI, aContext: *const nsISupports, aMimeTypeGuess: *const nsACString, aExtra: *const nsISupports, _retval: *mut libc::int16_t) -> nsresult,

    /* AString toJSON (); */
    pub toJSON: unsafe extern "C" fn (this: *const nsIContentSecurityPolicy, _retval: *mut nsAString) -> nsresult,

}


impl nsIContentSecurityPolicy {
    /* [binaryname(GetPolicyString)] AString getPolicy (in unsigned long index); */
    #[inline]
    pub unsafe fn GetPolicyString(&self, index: libc::uint32_t) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).GetPolicyString)(self as *const _, index, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall,notxpcom] CSPPolicyPtr GetPolicy (in unsigned long index); */


    /* readonly attribute unsigned long policyCount; */
    #[inline]
    pub unsafe fn get_policyCount(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_policyCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool upgradeInsecureRequests; */
    #[inline]
    pub unsafe fn get_upgradeInsecureRequests(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_upgradeInsecureRequests)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute bool blockAllMixedContent; */
    #[inline]
    pub unsafe fn get_blockAllMixedContent(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_blockAllMixedContent)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool getReferrerPolicy (out unsigned long policy); */
    #[inline]
    pub unsafe fn getReferrerPolicy(&self, ) -> Result<(libc::uint32_t, bool), nsresult> {
        let mut policy: libc::uint32_t = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getReferrerPolicy)(self as *const _, &mut policy as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((policy, _retval))
    }

    /* void appendPolicy (in AString policyString, in boolean reportOnly, in boolean deliveredViaMetaTag); */
    #[inline]
    pub unsafe fn appendPolicy(&self, policyString: &[u16], reportOnly: bool, deliveredViaMetaTag: bool) -> Result<(), nsresult> {
        let policyString = nsString::from(policyString);
        match ((*self.vtable).appendPolicy)(self as *const _, &*policyString, reportOnly, deliveredViaMetaTag) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getAllowsInline (in nsContentPolicyType aContentPolicyType, in AString aNonce, in boolean aParserCreated, in AString aContent, in unsigned long aLineNumber); */
    #[inline]
    pub unsafe fn getAllowsInline(&self, aContentPolicyType: nsContentPolicyType, aNonce: &[u16], aParserCreated: bool, aContent: &[u16], aLineNumber: libc::uint32_t) -> Result<bool, nsresult> {
        let aNonce = nsString::from(aNonce);
        let aContent = nsString::from(aContent);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getAllowsInline)(self as *const _, aContentPolicyType, &*aNonce, aParserCreated, &*aContent, aLineNumber, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean getAllowsEval (out boolean shouldReportViolations); */
    #[inline]
    pub unsafe fn getAllowsEval(&self, ) -> Result<(bool, bool), nsresult> {
        let mut shouldReportViolations: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getAllowsEval)(self as *const _, &mut shouldReportViolations as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((shouldReportViolations, _retval))
    }

    /* uint32_t getCSPSandboxFlags (); */
    #[inline]
    pub unsafe fn getCSPSandboxFlags(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getCSPSandboxFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void logViolationDetails (in unsigned short violationType, in AString sourceFile, in AString scriptSample, in int32_t lineNum, [optional] in AString nonce, [optional] in AString content); */
    #[inline]
    pub unsafe fn logViolationDetails(&self, violationType: libc::uint16_t, sourceFile: &[u16], scriptSample: &[u16], lineNum: int32_t, nonce: &[u16], content: &[u16]) -> Result<(), nsresult> {
        let sourceFile = nsString::from(sourceFile);
        let scriptSample = nsString::from(scriptSample);
        let nonce = nsString::from(nonce);
        let content = nsString::from(content);
        match ((*self.vtable).logViolationDetails)(self as *const _, violationType, &*sourceFile, &*scriptSample, lineNum, &*nonce, &*content) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRequestContext (in nsIDOMDocument aDocument, in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn setRequestContext(&self, aDocument: Option<&nsIDOMDocument>, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).setRequestContext)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void ensureEventTarget (in nsIEventTarget aEventTarget); */
    #[inline]
    pub unsafe fn ensureEventTarget(&self, aEventTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).ensureEventTarget)(self as *const _, aEventTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool requireSRIForType (in nsContentPolicyType aContentType); */
    #[inline]
    pub unsafe fn requireSRIForType(&self, aContentType: nsContentPolicyType) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).requireSRIForType)(self as *const _, aContentType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean permitsAncestry (in nsIDocShell docShell); */
    #[inline]
    pub unsafe fn permitsAncestry(&self, docShell: Option<&nsIDocShell>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).permitsAncestry)(self as *const _, docShell.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean permits (in nsIURI aURI, in CSPDirective aDir, in boolean aSpecific); */
    #[inline]
    pub unsafe fn permits(&self, aURI: Option<&nsIURI>, aDir: CSPDirective, aSpecific: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).permits)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aDir, aSpecific, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeTypeGuess, in nsISupports aExtra); */
    #[inline]
    pub unsafe fn shouldLoad(&self, aContentType: nsContentPolicyType, aContentLocation: Option<&nsIURI>, aRequestOrigin: Option<&nsIURI>, aContext: Option<&nsISupports>, aMimeTypeGuess: &[u8], aExtra: Option<&nsISupports>) -> Result<libc::int16_t, nsresult> {
        let aMimeTypeGuess = nsCString::from(aMimeTypeGuess);
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).shouldLoad)(self as *const _, aContentType, aContentLocation.map_or(::std::ptr::null(), |x| x as *const _), aRequestOrigin.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeTypeGuess, aExtra.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString toJSON (); */
    #[inline]
    pub unsafe fn toJSON(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).toJSON)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


