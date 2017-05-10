//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPrincipal.idl
//


pub mod nsIPrincipal_consts {
    pub const APP_STATUS_NOT_INSTALLED: i64 = 0;
    pub const APP_STATUS_INSTALLED: i64 = 1;
    pub const APP_STATUS_PRIVILEGED: i64 = 2;
    pub const APP_STATUS_CERTIFIED: i64 = 3;
}


#[repr(C)]
pub struct nsIPrincipal {
    vtable: *const nsIPrincipalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPrincipal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf75f502d, 0x79fd, 0x48be,
            [0xa0, 0x79, 0xe5, 0xa7, 0xb8, 0xf8, 0x0c, 0x8b])
    }
}

unsafe impl RefCounted for nsIPrincipal {
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
pub trait nsIPrincipalCoerce {
    fn coerce_from(v: &nsIPrincipal) -> &Self;
}

impl nsIPrincipalCoerce for nsIPrincipal {
    #[inline]
    fn coerce_from(v: &nsIPrincipal) -> &Self {
        v
    }
}

impl nsIPrincipal {
    #[inline]
    pub fn coerce<T: nsIPrincipalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPrincipal {
    type Target = nsISerializable;
    #[inline]
    fn deref(&self) -> &nsISerializable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISerializableCoerce> nsIPrincipalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrincipal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPrincipalVTable {
    pub __base: nsISerializableVTable,

    /* boolean equals (in nsIPrincipal other); */
    pub equals: unsafe extern "C" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* boolean equalsConsideringDomain (in nsIPrincipal other); */
    pub equalsConsideringDomain: unsafe extern "C" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* [noscript] readonly attribute unsigned long hashValue; */
    pub get_hashValue: unsafe extern "C" fn (this: *const nsIPrincipal, aHashValue: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsIPrincipal, aURI: *mut *const nsIURI) -> nsresult,

    /* [noscript] attribute nsIURI domain; */
    pub get_domain: unsafe extern "C" fn (this: *const nsIPrincipal, aDomain: *mut *const nsIURI) -> nsresult,
    pub set_domain: unsafe extern "C" fn (this: *const nsIPrincipal, aDomain: *const nsIURI) -> nsresult,

    /* boolean subsumes (in nsIPrincipal other); */
    pub subsumes: unsafe extern "C" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* boolean subsumesConsideringDomain (in nsIPrincipal other); */
    pub subsumesConsideringDomain: unsafe extern "C" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* boolean subsumesConsideringDomainIgnoringFPD (in nsIPrincipal other); */
    pub subsumesConsideringDomainIgnoringFPD: unsafe extern "C" fn (this: *const nsIPrincipal, other: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* void checkMayLoad (in nsIURI uri, in boolean report, in boolean allowIfInheritsPrincipal); */
    pub checkMayLoad: unsafe extern "C" fn (this: *const nsIPrincipal, uri: *const nsIURI, report: bool, allowIfInheritsPrincipal: bool) -> nsresult,

    /* [noscript] attribute nsIContentSecurityPolicy csp; */
    pub get_csp: unsafe extern "C" fn (this: *const nsIPrincipal, aCsp: *mut *const nsIContentSecurityPolicy) -> nsresult,
    pub set_csp: unsafe extern "C" fn (this: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy) -> nsresult,

    /* [noscript] nsIContentSecurityPolicy ensureCSP (in nsIDOMDocument aDocument); */
    pub ensureCSP: unsafe extern "C" fn (this: *const nsIPrincipal, aDocument: *const nsIDOMDocument, _retval: *mut *const nsIContentSecurityPolicy) -> nsresult,

    /* [noscript] readonly attribute nsIContentSecurityPolicy preloadCsp; */
    pub get_preloadCsp: unsafe extern "C" fn (this: *const nsIPrincipal, aPreloadCsp: *mut *const nsIContentSecurityPolicy) -> nsresult,

    /* [noscript] nsIContentSecurityPolicy ensurePreloadCSP (in nsIDOMDocument aDocument); */
    pub ensurePreloadCSP: unsafe extern "C" fn (this: *const nsIPrincipal, aDocument: *const nsIDOMDocument, _retval: *mut *const nsIContentSecurityPolicy) -> nsresult,

    /* readonly attribute AString cspJSON; */
    pub get_cspJSON: unsafe extern "C" fn (this: *const nsIPrincipal, aCspJSON: *mut nsAString) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_originAttributes: *const ::libc::c_void,

    /* [binaryname(OriginAttributesRef),noscript,nostdcall,notxpcom] const_OriginAttributes OriginAttributesRef (); */
    /// Unable to call function as its signature contains a non-rust type
    pub OriginAttributesRef: *const ::libc::c_void,

    /* readonly attribute ACString origin; */
    pub get_origin: unsafe extern "C" fn (this: *const nsIPrincipal, aOrigin: *mut nsACString) -> nsresult,

    /* readonly attribute ACString originNoSuffix; */
    pub get_originNoSuffix: unsafe extern "C" fn (this: *const nsIPrincipal, aOriginNoSuffix: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String originSuffix; */
    pub get_originSuffix: unsafe extern "C" fn (this: *const nsIPrincipal, aOriginSuffix: *mut nsACString) -> nsresult,

    /* readonly attribute ACString baseDomain; */
    pub get_baseDomain: unsafe extern "C" fn (this: *const nsIPrincipal, aBaseDomain: *mut nsACString) -> nsresult,

    /* [infallible] readonly attribute unsigned short appStatus; */
    pub get_appStatus: unsafe extern "C" fn (this: *const nsIPrincipal, aAppStatus: *mut libc::uint16_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long appId; */
    pub get_appId: unsafe extern "C" fn (this: *const nsIPrincipal, aAppId: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute AString addonId; */
    pub get_addonId: unsafe extern "C" fn (this: *const nsIPrincipal, aAddonId: *mut nsAString) -> nsresult,

    /* [infallible] readonly attribute unsigned long userContextId; */
    pub get_userContextId: unsafe extern "C" fn (this: *const nsIPrincipal, aUserContextId: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute unsigned long privateBrowsingId; */
    pub get_privateBrowsingId: unsafe extern "C" fn (this: *const nsIPrincipal, aPrivateBrowsingId: *mut libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean isInIsolatedMozBrowserElement; */
    pub get_isInIsolatedMozBrowserElement: unsafe extern "C" fn (this: *const nsIPrincipal, aIsInIsolatedMozBrowserElement: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean unknownAppId; */
    pub get_unknownAppId: unsafe extern "C" fn (this: *const nsIPrincipal, aUnknownAppId: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isNullPrincipal; */
    pub get_isNullPrincipal: unsafe extern "C" fn (this: *const nsIPrincipal, aIsNullPrincipal: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isCodebasePrincipal; */
    pub get_isCodebasePrincipal: unsafe extern "C" fn (this: *const nsIPrincipal, aIsCodebasePrincipal: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isExpandedPrincipal; */
    pub get_isExpandedPrincipal: unsafe extern "C" fn (this: *const nsIPrincipal, aIsExpandedPrincipal: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isSystemPrincipal; */
    pub get_isSystemPrincipal: unsafe extern "C" fn (this: *const nsIPrincipal, aIsSystemPrincipal: *mut bool) -> nsresult,

}


impl nsIPrincipal {
    /* boolean equals (in nsIPrincipal other); */
    #[inline]
    pub unsafe fn equals(&self, other: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equals)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean equalsConsideringDomain (in nsIPrincipal other); */
    #[inline]
    pub unsafe fn equalsConsideringDomain(&self, other: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).equalsConsideringDomain)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute unsigned long hashValue; */
    #[inline]
    pub unsafe fn get_hashValue(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hashValue)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIURI URI; */
    #[inline]
    pub unsafe fn get_URI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_URI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] attribute nsIURI domain; */
    #[inline]
    pub unsafe fn get_domain(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_domain)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_domain(&self, aDomain: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_domain)(self as *const _, aDomain.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean subsumes (in nsIPrincipal other); */
    #[inline]
    pub unsafe fn subsumes(&self, other: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).subsumes)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean subsumesConsideringDomain (in nsIPrincipal other); */
    #[inline]
    pub unsafe fn subsumesConsideringDomain(&self, other: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).subsumesConsideringDomain)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean subsumesConsideringDomainIgnoringFPD (in nsIPrincipal other); */
    #[inline]
    pub unsafe fn subsumesConsideringDomainIgnoringFPD(&self, other: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).subsumesConsideringDomainIgnoringFPD)(self as *const _, other.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void checkMayLoad (in nsIURI uri, in boolean report, in boolean allowIfInheritsPrincipal); */
    #[inline]
    pub unsafe fn checkMayLoad(&self, uri: Option<&nsIURI>, report: bool, allowIfInheritsPrincipal: bool) -> Result<(), nsresult> {

        match ((*self.vtable).checkMayLoad)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), report, allowIfInheritsPrincipal) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute nsIContentSecurityPolicy csp; */
    #[inline]
    pub unsafe fn get_csp(&self, ) -> Result<Option<RefPtr<nsIContentSecurityPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_csp)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_csp(&self, aCsp: Option<&nsIContentSecurityPolicy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_csp)(self as *const _, aCsp.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] nsIContentSecurityPolicy ensureCSP (in nsIDOMDocument aDocument); */
    #[inline]
    pub unsafe fn ensureCSP(&self, aDocument: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIContentSecurityPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).ensureCSP)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] readonly attribute nsIContentSecurityPolicy preloadCsp; */
    #[inline]
    pub unsafe fn get_preloadCsp(&self, ) -> Result<Option<RefPtr<nsIContentSecurityPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_preloadCsp)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] nsIContentSecurityPolicy ensurePreloadCSP (in nsIDOMDocument aDocument); */
    #[inline]
    pub unsafe fn ensurePreloadCSP(&self, aDocument: Option<&nsIDOMDocument>) -> Result<Option<RefPtr<nsIContentSecurityPolicy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).ensurePreloadCSP)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute AString cspJSON; */
    #[inline]
    pub unsafe fn get_cspJSON(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_cspJSON)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */


    /* [binaryname(OriginAttributesRef),noscript,nostdcall,notxpcom] const_OriginAttributes OriginAttributesRef (); */


    /* readonly attribute ACString origin; */
    #[inline]
    pub unsafe fn get_origin(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_origin)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString originNoSuffix; */
    #[inline]
    pub unsafe fn get_originNoSuffix(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_originNoSuffix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String originSuffix; */
    #[inline]
    pub unsafe fn get_originSuffix(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_originSuffix)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString baseDomain; */
    #[inline]
    pub unsafe fn get_baseDomain(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_baseDomain)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned short appStatus; */
    #[inline]
    pub unsafe fn get_appStatus(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_appStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long appId; */
    #[inline]
    pub unsafe fn get_appId(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_appId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString addonId; */
    #[inline]
    pub unsafe fn get_addonId(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_addonId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long userContextId; */
    #[inline]
    pub unsafe fn get_userContextId(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_userContextId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute unsigned long privateBrowsingId; */
    #[inline]
    pub unsafe fn get_privateBrowsingId(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_privateBrowsingId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isInIsolatedMozBrowserElement; */
    #[inline]
    pub unsafe fn get_isInIsolatedMozBrowserElement(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInIsolatedMozBrowserElement)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean unknownAppId; */
    #[inline]
    pub unsafe fn get_unknownAppId(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_unknownAppId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isNullPrincipal; */
    #[inline]
    pub unsafe fn get_isNullPrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isNullPrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isCodebasePrincipal; */
    #[inline]
    pub unsafe fn get_isCodebasePrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isCodebasePrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isExpandedPrincipal; */
    #[inline]
    pub unsafe fn get_isExpandedPrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isExpandedPrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isSystemPrincipal; */
    #[inline]
    pub unsafe fn get_isSystemPrincipal(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSystemPrincipal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIExpandedPrincipal {
    vtable: *const nsIExpandedPrincipalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIExpandedPrincipal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf3e177df, 0x6a5e, 0x489f,
            [0x80, 0xa7, 0x2d, 0xd1, 0x48, 0x14, 0x71, 0xd8])
    }
}

unsafe impl RefCounted for nsIExpandedPrincipal {
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
pub trait nsIExpandedPrincipalCoerce {
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self;
}

impl nsIExpandedPrincipalCoerce for nsIExpandedPrincipal {
    #[inline]
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self {
        v
    }
}

impl nsIExpandedPrincipal {
    #[inline]
    pub fn coerce<T: nsIExpandedPrincipalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIExpandedPrincipal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIExpandedPrincipalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExpandedPrincipal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIExpandedPrincipalVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] readonly attribute PrincipalArray whiteList; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_whiteList: *const ::libc::c_void,

}


impl nsIExpandedPrincipal {
    /* [noscript] readonly attribute PrincipalArray whiteList; */


}


