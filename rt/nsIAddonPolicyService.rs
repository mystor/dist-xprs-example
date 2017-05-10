//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAddonPolicyService.idl
//


#[repr(C)]
pub struct nsIAddonPolicyService {
    vtable: *const nsIAddonPolicyServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAddonPolicyService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a034ef9, 0x9d14, 0x4c5d,
            [0x83, 0x19, 0x06, 0xc1, 0xab, 0x57, 0x4b, 0xaa])
    }
}

unsafe impl RefCounted for nsIAddonPolicyService {
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
pub trait nsIAddonPolicyServiceCoerce {
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self;
}

impl nsIAddonPolicyServiceCoerce for nsIAddonPolicyService {
    #[inline]
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self {
        v
    }
}

impl nsIAddonPolicyService {
    #[inline]
    pub fn coerce<T: nsIAddonPolicyServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAddonPolicyService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAddonPolicyServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAddonPolicyServiceVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute AString baseCSP; */
    pub get_baseCSP: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aBaseCSP: *mut nsAString) -> nsresult,

    /* readonly attribute AString defaultCSP; */
    pub get_defaultCSP: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aDefaultCSP: *mut nsAString) -> nsresult,

    /* AString getAddonCSP (in AString aAddonId); */
    pub getAddonCSP: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aAddonId: *const nsAString, _retval: *mut nsAString) -> nsresult,

    /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
    pub getGeneratedBackgroundPageUrl: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aAddonId: *const nsACString, _retval: *mut nsACString) -> nsresult,

    /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
    pub addonHasPermission: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aAddonId: *const nsAString, aPerm: *const nsAString, _retval: *mut bool) -> nsresult,

    /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
    pub addonMayLoadURI: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aAddonId: *const nsAString, aURI: *const nsIURI, aExplicit: bool, _retval: *mut bool) -> nsresult,

    /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
    pub extensionURILoadableByAnyone: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* AString extensionURIToAddonId (in nsIURI aURI); */
    pub extensionURIToAddonId: unsafe extern "C" fn (this: *const nsIAddonPolicyService, aURI: *const nsIURI, _retval: *mut nsAString) -> nsresult,

}


impl nsIAddonPolicyService {
    /* readonly attribute AString baseCSP; */
    #[inline]
    pub unsafe fn get_baseCSP(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_baseCSP)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString defaultCSP; */
    #[inline]
    pub unsafe fn get_defaultCSP(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_defaultCSP)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getAddonCSP (in AString aAddonId); */
    #[inline]
    pub unsafe fn getAddonCSP(&self, aAddonId: &[u16]) -> Result<nsString, nsresult> {
        let aAddonId = nsString::from(aAddonId);
        let mut _retval = nsString::new();
        match ((*self.vtable).getAddonCSP)(self as *const _, &*aAddonId, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
    #[inline]
    pub unsafe fn getGeneratedBackgroundPageUrl(&self, aAddonId: &[u8]) -> Result<nsCString, nsresult> {
        let aAddonId = nsCString::from(aAddonId);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getGeneratedBackgroundPageUrl)(self as *const _, &*aAddonId, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
    #[inline]
    pub unsafe fn addonHasPermission(&self, aAddonId: &[u16], aPerm: &[u16]) -> Result<bool, nsresult> {
        let aAddonId = nsString::from(aAddonId);
        let aPerm = nsString::from(aPerm);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).addonHasPermission)(self as *const _, &*aAddonId, &*aPerm, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
    #[inline]
    pub unsafe fn addonMayLoadURI(&self, aAddonId: &[u16], aURI: Option<&nsIURI>, aExplicit: bool) -> Result<bool, nsresult> {
        let aAddonId = nsString::from(aAddonId);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).addonMayLoadURI)(self as *const _, &*aAddonId, aURI.map_or(::std::ptr::null(), |x| x as *const _), aExplicit, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
    #[inline]
    pub unsafe fn extensionURILoadableByAnyone(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).extensionURILoadableByAnyone)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString extensionURIToAddonId (in nsIURI aURI); */
    #[inline]
    pub unsafe fn extensionURIToAddonId(&self, aURI: Option<&nsIURI>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).extensionURIToAddonId)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIAddonContentPolicy {
    vtable: *const nsIAddonContentPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAddonContentPolicy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a4fe60b, 0x9131, 0x45f5,
            [0x83, 0xf3, 0xdc, 0x63, 0xb5, 0xd7, 0x1a, 0x5d])
    }
}

unsafe impl RefCounted for nsIAddonContentPolicy {
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
pub trait nsIAddonContentPolicyCoerce {
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self;
}

impl nsIAddonContentPolicyCoerce for nsIAddonContentPolicy {
    #[inline]
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self {
        v
    }
}

impl nsIAddonContentPolicy {
    #[inline]
    pub fn coerce<T: nsIAddonContentPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAddonContentPolicy {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAddonContentPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAddonContentPolicyVTable {
    pub __base: nsISupportsVTable,

    /* AString validateAddonCSP (in AString aPolicyString); */
    pub validateAddonCSP: unsafe extern "C" fn (this: *const nsIAddonContentPolicy, aPolicyString: *const nsAString, _retval: *mut nsAString) -> nsresult,

}


impl nsIAddonContentPolicy {
    /* AString validateAddonCSP (in AString aPolicyString); */
    #[inline]
    pub unsafe fn validateAddonCSP(&self, aPolicyString: &[u16]) -> Result<nsString, nsresult> {
        let aPolicyString = nsString::from(aPolicyString);
        let mut _retval = nsString::new();
        match ((*self.vtable).validateAddonCSP)(self as *const _, &*aPolicyString, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


