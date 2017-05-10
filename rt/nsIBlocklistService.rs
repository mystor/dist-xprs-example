//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIBlocklistService.idl
//


pub mod nsIBlocklistService_consts {
    pub const STATE_NOT_BLOCKED: i64 = 0;
    pub const STATE_SOFTBLOCKED: i64 = 1;
    pub const STATE_BLOCKED: i64 = 2;
    pub const STATE_OUTDATED: i64 = 3;
    pub const STATE_VULNERABLE_UPDATE_AVAILABLE: i64 = 4;
    pub const STATE_VULNERABLE_NO_UPDATE: i64 = 5;
}


#[repr(C)]
pub struct nsIBlocklistService {
    vtable: *const nsIBlocklistServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBlocklistService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa6dcc76e, 0x9f62, 0x4cc1,
            [0xa4, 0x70, 0xb4, 0x83, 0xa1, 0xa6, 0xf0, 0x96])
    }
}

unsafe impl RefCounted for nsIBlocklistService {
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
pub trait nsIBlocklistServiceCoerce {
    fn coerce_from(v: &nsIBlocklistService) -> &Self;
}

impl nsIBlocklistServiceCoerce for nsIBlocklistService {
    #[inline]
    fn coerce_from(v: &nsIBlocklistService) -> &Self {
        v
    }
}

impl nsIBlocklistService {
    #[inline]
    pub fn coerce<T: nsIBlocklistServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBlocklistService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBlocklistServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBlocklistService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBlocklistServiceVTable {
    pub __base: nsISupportsVTable,

    /* boolean isAddonBlocklisted (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    /// Unable to call function as its signature contains a non-rust type
    pub isAddonBlocklisted: *const ::libc::c_void,

    /* unsigned long getAddonBlocklistState (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAddonBlocklistState: *const ::libc::c_void,

    /* unsigned long getPluginBlocklistState (in nsIPluginTag plugin, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    pub getPluginBlocklistState: unsafe extern "C" fn (this: *const nsIBlocklistService, plugin: *const nsIPluginTag, appVersion: *const nsAString, toolkitVersion: *const nsAString, _retval: *mut libc::uint32_t) -> nsresult,

    /* AString getAddonBlocklistURL (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAddonBlocklistURL: *const ::libc::c_void,

    /* AString getPluginBlocklistURL (in nsIPluginTag plugin); */
    pub getPluginBlocklistURL: unsafe extern "C" fn (this: *const nsIBlocklistService, plugin: *const nsIPluginTag, _retval: *mut nsAString) -> nsresult,

    /* AString getPluginInfoURL (in nsIPluginTag plugin); */
    pub getPluginInfoURL: unsafe extern "C" fn (this: *const nsIBlocklistService, plugin: *const nsIPluginTag, _retval: *mut nsAString) -> nsresult,

}


impl nsIBlocklistService {
    /* boolean isAddonBlocklisted (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */


    /* unsigned long getAddonBlocklistState (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */


    /* unsigned long getPluginBlocklistState (in nsIPluginTag plugin, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    #[inline]
    pub unsafe fn getPluginBlocklistState(&self, plugin: Option<&nsIPluginTag>, appVersion: &[u16], toolkitVersion: &[u16]) -> Result<libc::uint32_t, nsresult> {
        let appVersion = nsString::from(appVersion);
        let toolkitVersion = nsString::from(toolkitVersion);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getPluginBlocklistState)(self as *const _, plugin.map_or(::std::ptr::null(), |x| x as *const _), &*appVersion, &*toolkitVersion, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getAddonBlocklistURL (in jsval addon, [optional] in AString appVersion, [optional] in AString toolkitVersion); */


    /* AString getPluginBlocklistURL (in nsIPluginTag plugin); */
    #[inline]
    pub unsafe fn getPluginBlocklistURL(&self, plugin: Option<&nsIPluginTag>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getPluginBlocklistURL)(self as *const _, plugin.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* AString getPluginInfoURL (in nsIPluginTag plugin); */
    #[inline]
    pub unsafe fn getPluginInfoURL(&self, plugin: Option<&nsIPluginTag>) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).getPluginInfoURL)(self as *const _, plugin.map_or(::std::ptr::null(), |x| x as *const _), &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIBlocklistPrompt {
    vtable: *const nsIBlocklistPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIBlocklistPrompt {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xba915921, 0xb9c0, 0x400d,
            [0x8e, 0x4f, 0xca, 0x1b, 0x80, 0xc5, 0x69, 0x9a])
    }
}

unsafe impl RefCounted for nsIBlocklistPrompt {
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
pub trait nsIBlocklistPromptCoerce {
    fn coerce_from(v: &nsIBlocklistPrompt) -> &Self;
}

impl nsIBlocklistPromptCoerce for nsIBlocklistPrompt {
    #[inline]
    fn coerce_from(v: &nsIBlocklistPrompt) -> &Self {
        v
    }
}

impl nsIBlocklistPrompt {
    #[inline]
    pub fn coerce<T: nsIBlocklistPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIBlocklistPrompt {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIBlocklistPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBlocklistPrompt) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIBlocklistPromptVTable {
    pub __base: nsISupportsVTable,

    /* void prompt ([array, size_is (aCount)] in nsIVariant aAddons, [optional] in uint32_t aCount); */
    /// Unable to call function as its signature contains a non-rust type
    pub prompt: *const ::libc::c_void,

}


impl nsIBlocklistPrompt {
    /* void prompt ([array, size_is (aCount)] in nsIVariant aAddons, [optional] in uint32_t aCount); */


}


