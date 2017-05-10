//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPluginHost.idl
//


#[repr(C)]
pub struct nsIClearSiteDataCallback {
    vtable: *const nsIClearSiteDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIClearSiteDataCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9c311778, 0x7c2c, 0x4ad8,
            [0xb4, 0x39, 0xb8, 0xa2, 0x78, 0x6a, 0x20, 0xdd])
    }
}

unsafe impl RefCounted for nsIClearSiteDataCallback {
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
pub trait nsIClearSiteDataCallbackCoerce {
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self;
}

impl nsIClearSiteDataCallbackCoerce for nsIClearSiteDataCallback {
    #[inline]
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self {
        v
    }
}

impl nsIClearSiteDataCallback {
    #[inline]
    pub fn coerce<T: nsIClearSiteDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIClearSiteDataCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIClearSiteDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIClearSiteDataCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void callback (in nsresult rv); */
    pub callback: unsafe extern "C" fn (this: *const nsIClearSiteDataCallback, rv: nsresult) -> nsresult,

}


impl nsIClearSiteDataCallback {
    /* void callback (in nsresult rv); */
    #[inline]
    pub unsafe fn callback(&self, rv: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).callback)(self as *const _, rv) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsIPluginHost_consts {
    pub const FLAG_CLEAR_ALL: i64 = 0;
    pub const FLAG_CLEAR_CACHE: i64 = 1;
    pub const EXCLUDE_NONE: i64 = 0;
    pub const EXCLUDE_DISABLED: i64 = 1;
    pub const EXCLUDE_FAKE: i64 = 2;
}


#[repr(C)]
pub struct nsIPluginHost {
    vtable: *const nsIPluginHostVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPluginHost {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf938f5ba, 0x7093, 0x42cd,
            [0xa5, 0x59, 0xaf, 0x80, 0x39, 0xd9, 0x92, 0x04])
    }
}

unsafe impl RefCounted for nsIPluginHost {
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
pub trait nsIPluginHostCoerce {
    fn coerce_from(v: &nsIPluginHost) -> &Self;
}

impl nsIPluginHostCoerce for nsIPluginHost {
    #[inline]
    fn coerce_from(v: &nsIPluginHost) -> &Self {
        v
    }
}

impl nsIPluginHost {
    #[inline]
    pub fn coerce<T: nsIPluginHostCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPluginHost {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPluginHostCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginHost) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPluginHostVTable {
    pub __base: nsISupportsVTable,

    /* void reloadPlugins (); */
    pub reloadPlugins: unsafe extern "C" fn (this: *const nsIPluginHost) -> nsresult,

    /* void getPluginTags ([optional] out unsigned long aPluginCount, [array, size_is (aPluginCount), retval] out nsIPluginTag aResults); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPluginTags: *const ::libc::c_void,

    /* void clearSiteData (in nsIPluginTag plugin, in AUTF8String domain, in uint64_t flags, in int64_t maxAge, in nsIClearSiteDataCallback callback); */
    pub clearSiteData: unsafe extern "C" fn (this: *const nsIPluginHost, plugin: *const nsIPluginTag, domain: *const nsACString, flags: uint64_t, maxAge: int64_t, callback: *const nsIClearSiteDataCallback) -> nsresult,

    /* boolean siteHasData (in nsIPluginTag plugin, in AUTF8String domain); */
    pub siteHasData: unsafe extern "C" fn (this: *const nsIPluginHost, plugin: *const nsIPluginTag, domain: *const nsACString, _retval: *mut bool) -> nsresult,

    /* ACString getPermissionStringForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub getPermissionStringForType: unsafe extern "C" fn (this: *const nsIPluginHost, mimeType: *const nsACString, excludeFlags: uint32_t, _retval: *mut nsACString) -> nsresult,

    /* ACString getPermissionStringForTag (in nsIPluginTag tag, [optional] in uint32_t excludeFlags); */
    pub getPermissionStringForTag: unsafe extern "C" fn (this: *const nsIPluginHost, tag: *const nsIPluginTag, excludeFlags: uint32_t, _retval: *mut nsACString) -> nsresult,

    /* nsIPluginTag getPluginTagForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub getPluginTagForType: unsafe extern "C" fn (this: *const nsIPluginHost, mimeType: *const nsACString, excludeFlags: uint32_t, _retval: *mut *const nsIPluginTag) -> nsresult,

    /* unsigned long getStateForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub getStateForType: unsafe extern "C" fn (this: *const nsIPluginHost, mimeType: *const nsACString, excludeFlags: uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* uint32_t getBlocklistStateForType (in AUTF8String aMimeType, [optional] in uint32_t excludeFlags); */
    pub getBlocklistStateForType: unsafe extern "C" fn (this: *const nsIPluginHost, aMimeType: *const nsACString, excludeFlags: uint32_t, _retval: *mut uint32_t) -> nsresult,

    /* [implicit_jscontext] nsIFakePluginTag registerFakePlugin (in jsval initDictionary); */
    /// Unable to call function as its signature contains a non-rust type
    pub registerFakePlugin: *const ::libc::c_void,

    /* nsIFakePluginTag getFakePlugin (in AUTF8String mimeType); */
    pub getFakePlugin: unsafe extern "C" fn (this: *const nsIPluginHost, mimeType: *const nsACString, _retval: *mut *const nsIFakePluginTag) -> nsresult,

    /* void unregisterFakePlugin (in AUTF8String handlerURI); */
    pub unregisterFakePlugin: unsafe extern "C" fn (this: *const nsIPluginHost, handlerURI: *const nsACString) -> nsresult,

}


impl nsIPluginHost {
    /* void reloadPlugins (); */
    #[inline]
    pub unsafe fn reloadPlugins(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reloadPlugins)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getPluginTags ([optional] out unsigned long aPluginCount, [array, size_is (aPluginCount), retval] out nsIPluginTag aResults); */


    /* void clearSiteData (in nsIPluginTag plugin, in AUTF8String domain, in uint64_t flags, in int64_t maxAge, in nsIClearSiteDataCallback callback); */
    #[inline]
    pub unsafe fn clearSiteData(&self, plugin: Option<&nsIPluginTag>, domain: &[u8], flags: uint64_t, maxAge: int64_t, callback: Option<&nsIClearSiteDataCallback>) -> Result<(), nsresult> {
        let domain = nsCString::from(domain);
        match ((*self.vtable).clearSiteData)(self as *const _, plugin.map_or(::std::ptr::null(), |x| x as *const _), &*domain, flags, maxAge, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean siteHasData (in nsIPluginTag plugin, in AUTF8String domain); */
    #[inline]
    pub unsafe fn siteHasData(&self, plugin: Option<&nsIPluginTag>, domain: &[u8]) -> Result<bool, nsresult> {
        let domain = nsCString::from(domain);
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).siteHasData)(self as *const _, plugin.map_or(::std::ptr::null(), |x| x as *const _), &*domain, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getPermissionStringForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    #[inline]
    pub unsafe fn getPermissionStringForType(&self, mimeType: &[u8], excludeFlags: uint32_t) -> Result<nsCString, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPermissionStringForType)(self as *const _, &*mimeType, excludeFlags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* ACString getPermissionStringForTag (in nsIPluginTag tag, [optional] in uint32_t excludeFlags); */
    #[inline]
    pub unsafe fn getPermissionStringForTag(&self, tag: Option<&nsIPluginTag>, excludeFlags: uint32_t) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).getPermissionStringForTag)(self as *const _, tag.map_or(::std::ptr::null(), |x| x as *const _), excludeFlags, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIPluginTag getPluginTagForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    #[inline]
    pub unsafe fn getPluginTagForType(&self, mimeType: &[u8], excludeFlags: uint32_t) -> Result<Option<RefPtr<nsIPluginTag>>, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getPluginTagForType)(self as *const _, &*mimeType, excludeFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* unsigned long getStateForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    #[inline]
    pub unsafe fn getStateForType(&self, mimeType: &[u8], excludeFlags: uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getStateForType)(self as *const _, &*mimeType, excludeFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* uint32_t getBlocklistStateForType (in AUTF8String aMimeType, [optional] in uint32_t excludeFlags); */
    #[inline]
    pub unsafe fn getBlocklistStateForType(&self, aMimeType: &[u8], excludeFlags: uint32_t) -> Result<uint32_t, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getBlocklistStateForType)(self as *const _, &*aMimeType, excludeFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] nsIFakePluginTag registerFakePlugin (in jsval initDictionary); */


    /* nsIFakePluginTag getFakePlugin (in AUTF8String mimeType); */
    #[inline]
    pub unsafe fn getFakePlugin(&self, mimeType: &[u8]) -> Result<Option<RefPtr<nsIFakePluginTag>>, nsresult> {
        let mimeType = nsCString::from(mimeType);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFakePlugin)(self as *const _, &*mimeType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void unregisterFakePlugin (in AUTF8String handlerURI); */
    #[inline]
    pub unsafe fn unregisterFakePlugin(&self, handlerURI: &[u8]) -> Result<(), nsresult> {
        let handlerURI = nsCString::from(handlerURI);
        match ((*self.vtable).unregisterFakePlugin)(self as *const _, &*handlerURI) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


