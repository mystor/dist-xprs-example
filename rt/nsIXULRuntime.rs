//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULRuntime.idl
//


pub mod nsIXULRuntime_consts {
    pub const PROCESS_TYPE_DEFAULT: i64 = 0;
    pub const PROCESS_TYPE_PLUGIN: i64 = 1;
    pub const PROCESS_TYPE_CONTENT: i64 = 2;
    pub const PROCESS_TYPE_IPDLUNITTEST: i64 = 3;
    pub const PROCESS_TYPE_GMPLUGIN: i64 = 4;
    pub const PROCESS_TYPE_GPU: i64 = 5;
    pub const E10S_MULTI_EXPERIMENT: i64 = 1;
}


#[repr(C)]
pub struct nsIXULRuntime {
    vtable: *const nsIXULRuntimeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULRuntime {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa1b2e167, 0xb748, 0x42bf,
            [0xba, 0x85, 0x99, 0x6e, 0xc3, 0x90, 0x62, 0xb9])
    }
}

unsafe impl RefCounted for nsIXULRuntime {
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
pub trait nsIXULRuntimeCoerce {
    fn coerce_from(v: &nsIXULRuntime) -> &Self;
}

impl nsIXULRuntimeCoerce for nsIXULRuntime {
    #[inline]
    fn coerce_from(v: &nsIXULRuntime) -> &Self {
        v
    }
}

impl nsIXULRuntime {
    #[inline]
    pub fn coerce<T: nsIXULRuntimeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULRuntime {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULRuntimeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULRuntime) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULRuntimeVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean inSafeMode; */
    pub get_inSafeMode: unsafe extern "C" fn (this: *const nsIXULRuntime, aInSafeMode: *mut bool) -> nsresult,

    /* attribute boolean logConsoleErrors; */
    pub get_logConsoleErrors: unsafe extern "C" fn (this: *const nsIXULRuntime, aLogConsoleErrors: *mut bool) -> nsresult,
    pub set_logConsoleErrors: unsafe extern "C" fn (this: *const nsIXULRuntime, aLogConsoleErrors: bool) -> nsresult,

    /* readonly attribute AUTF8String OS; */
    pub get_OS: unsafe extern "C" fn (this: *const nsIXULRuntime, aOS: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String XPCOMABI; */
    pub get_XPCOMABI: unsafe extern "C" fn (this: *const nsIXULRuntime, aXPCOMABI: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String widgetToolkit; */
    pub get_widgetToolkit: unsafe extern "C" fn (this: *const nsIXULRuntime, aWidgetToolkit: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long processType; */
    pub get_processType: unsafe extern "C" fn (this: *const nsIXULRuntime, aProcessType: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long processID; */
    pub get_processID: unsafe extern "C" fn (this: *const nsIXULRuntime, aProcessID: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute uint64_t uniqueProcessID; */
    pub get_uniqueProcessID: unsafe extern "C" fn (this: *const nsIXULRuntime, aUniqueProcessID: *mut uint64_t) -> nsresult,

    /* readonly attribute DOMString remoteType; */
    pub get_remoteType: unsafe extern "C" fn (this: *const nsIXULRuntime, aRemoteType: *mut nsAString) -> nsresult,

    /* readonly attribute boolean browserTabsRemoteAutostart; */
    pub get_browserTabsRemoteAutostart: unsafe extern "C" fn (this: *const nsIXULRuntime, aBrowserTabsRemoteAutostart: *mut bool) -> nsresult,

    /* readonly attribute uint32_t maxWebProcessCount; */
    pub get_maxWebProcessCount: unsafe extern "C" fn (this: *const nsIXULRuntime, aMaxWebProcessCount: *mut uint32_t) -> nsresult,

    /* readonly attribute unsigned long multiprocessBlockPolicy; */
    pub get_multiprocessBlockPolicy: unsafe extern "C" fn (this: *const nsIXULRuntime, aMultiprocessBlockPolicy: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute boolean accessibilityEnabled; */
    pub get_accessibilityEnabled: unsafe extern "C" fn (this: *const nsIXULRuntime, aAccessibilityEnabled: *mut bool) -> nsresult,

    /* readonly attribute boolean is64Bit; */
    pub get_is64Bit: unsafe extern "C" fn (this: *const nsIXULRuntime, aIs64Bit: *mut bool) -> nsresult,

    /* void invalidateCachesOnRestart (); */
    pub invalidateCachesOnRestart: unsafe extern "C" fn (this: *const nsIXULRuntime) -> nsresult,

    /* void ensureContentProcess (); */
    pub ensureContentProcess: unsafe extern "C" fn (this: *const nsIXULRuntime) -> nsresult,

    /* readonly attribute PRTime replacedLockTime; */
    pub get_replacedLockTime: unsafe extern "C" fn (this: *const nsIXULRuntime, aReplacedLockTime: *mut PRTime) -> nsresult,

    /* readonly attribute DOMString lastRunCrashID; */
    pub get_lastRunCrashID: unsafe extern "C" fn (this: *const nsIXULRuntime, aLastRunCrashID: *mut nsAString) -> nsresult,

    /* readonly attribute boolean isReleaseOrBeta; */
    pub get_isReleaseOrBeta: unsafe extern "C" fn (this: *const nsIXULRuntime, aIsReleaseOrBeta: *mut bool) -> nsresult,

    /* readonly attribute boolean isOfficialBranding; */
    pub get_isOfficialBranding: unsafe extern "C" fn (this: *const nsIXULRuntime, aIsOfficialBranding: *mut bool) -> nsresult,

    /* readonly attribute AUTF8String defaultUpdateChannel; */
    pub get_defaultUpdateChannel: unsafe extern "C" fn (this: *const nsIXULRuntime, aDefaultUpdateChannel: *mut nsACString) -> nsresult,

    /* readonly attribute AUTF8String distributionID; */
    pub get_distributionID: unsafe extern "C" fn (this: *const nsIXULRuntime, aDistributionID: *mut nsACString) -> nsresult,

    /* readonly attribute boolean isOfficial; */
    pub get_isOfficial: unsafe extern "C" fn (this: *const nsIXULRuntime, aIsOfficial: *mut bool) -> nsresult,

    /* readonly attribute boolean windowsDLLBlocklistStatus; */
    pub get_windowsDLLBlocklistStatus: unsafe extern "C" fn (this: *const nsIXULRuntime, aWindowsDLLBlocklistStatus: *mut bool) -> nsresult,

}


impl nsIXULRuntime {
    /* readonly attribute boolean inSafeMode; */
    #[inline]
    pub unsafe fn get_inSafeMode(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inSafeMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean logConsoleErrors; */
    #[inline]
    pub unsafe fn get_logConsoleErrors(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_logConsoleErrors)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_logConsoleErrors(&self, aLogConsoleErrors: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_logConsoleErrors)(self as *const _, aLogConsoleErrors) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute AUTF8String OS; */
    #[inline]
    pub unsafe fn get_OS(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_OS)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String XPCOMABI; */
    #[inline]
    pub unsafe fn get_XPCOMABI(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_XPCOMABI)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String widgetToolkit; */
    #[inline]
    pub unsafe fn get_widgetToolkit(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_widgetToolkit)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long processType; */
    #[inline]
    pub unsafe fn get_processType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_processType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long processID; */
    #[inline]
    pub unsafe fn get_processID(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_processID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint64_t uniqueProcessID; */
    #[inline]
    pub unsafe fn get_uniqueProcessID(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_uniqueProcessID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString remoteType; */
    #[inline]
    pub unsafe fn get_remoteType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_remoteType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean browserTabsRemoteAutostart; */
    #[inline]
    pub unsafe fn get_browserTabsRemoteAutostart(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_browserTabsRemoteAutostart)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute uint32_t maxWebProcessCount; */
    #[inline]
    pub unsafe fn get_maxWebProcessCount(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_maxWebProcessCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long multiprocessBlockPolicy; */
    #[inline]
    pub unsafe fn get_multiprocessBlockPolicy(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_multiprocessBlockPolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean accessibilityEnabled; */
    #[inline]
    pub unsafe fn get_accessibilityEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_accessibilityEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean is64Bit; */
    #[inline]
    pub unsafe fn get_is64Bit(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_is64Bit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void invalidateCachesOnRestart (); */
    #[inline]
    pub unsafe fn invalidateCachesOnRestart(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).invalidateCachesOnRestart)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void ensureContentProcess (); */
    #[inline]
    pub unsafe fn ensureContentProcess(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).ensureContentProcess)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute PRTime replacedLockTime; */
    #[inline]
    pub unsafe fn get_replacedLockTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_replacedLockTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString lastRunCrashID; */
    #[inline]
    pub unsafe fn get_lastRunCrashID(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_lastRunCrashID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isReleaseOrBeta; */
    #[inline]
    pub unsafe fn get_isReleaseOrBeta(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isReleaseOrBeta)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isOfficialBranding; */
    #[inline]
    pub unsafe fn get_isOfficialBranding(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOfficialBranding)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String defaultUpdateChannel; */
    #[inline]
    pub unsafe fn get_defaultUpdateChannel(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_defaultUpdateChannel)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String distributionID; */
    #[inline]
    pub unsafe fn get_distributionID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_distributionID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isOfficial; */
    #[inline]
    pub unsafe fn get_isOfficial(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOfficial)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean windowsDLLBlocklistStatus; */
    #[inline]
    pub unsafe fn get_windowsDLLBlocklistStatus(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_windowsDLLBlocklistStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


