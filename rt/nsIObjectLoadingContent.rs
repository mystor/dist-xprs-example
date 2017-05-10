//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIObjectLoadingContent.idl
//


pub mod nsIObjectLoadingContent_consts {
    pub const TYPE_LOADING: i64 = 0;
    pub const TYPE_IMAGE: i64 = 1;
    pub const TYPE_PLUGIN: i64 = 2;
    pub const TYPE_DOCUMENT: i64 = 3;
    pub const TYPE_NULL: i64 = 4;
    pub const PLUGIN_ACTIVE: i64 = 255;
    pub const PLUGIN_UNSUPPORTED: i64 = 0;
    pub const PLUGIN_ALTERNATE: i64 = 1;
    pub const PLUGIN_DISABLED: i64 = 2;
    pub const PLUGIN_BLOCKLISTED: i64 = 3;
    pub const PLUGIN_OUTDATED: i64 = 4;
    pub const PLUGIN_CRASHED: i64 = 5;
    pub const PLUGIN_SUPPRESSED: i64 = 6;
    pub const PLUGIN_USER_DISABLED: i64 = 7;
    pub const PLUGIN_CLICK_TO_PLAY: i64 = 8;
    pub const PLUGIN_VULNERABLE_UPDATABLE: i64 = 9;
    pub const PLUGIN_VULNERABLE_NO_UPDATE: i64 = 10;
}


#[repr(C)]
pub struct nsIObjectLoadingContent {
    vtable: *const nsIObjectLoadingContentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIObjectLoadingContent {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2eb3195e, 0x3eea, 0x4083,
            [0xbb, 0x1d, 0xd2, 0xd7, 0x0f, 0xa3, 0x5c, 0xcb])
    }
}

unsafe impl RefCounted for nsIObjectLoadingContent {
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
pub trait nsIObjectLoadingContentCoerce {
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self;
}

impl nsIObjectLoadingContentCoerce for nsIObjectLoadingContent {
    #[inline]
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self {
        v
    }
}

impl nsIObjectLoadingContent {
    #[inline]
    pub fn coerce<T: nsIObjectLoadingContentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIObjectLoadingContent {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIObjectLoadingContentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIObjectLoadingContentVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString actualType; */
    pub get_actualType: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aActualType: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long displayedType; */
    pub get_displayedType: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aDisplayedType: *mut libc::uint32_t) -> nsresult,

    /* unsigned long getContentTypeForMIMEType (in AUTF8String aMimeType); */
    pub getContentTypeForMIMEType: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aMimeType: *const nsACString, _retval: *mut libc::uint32_t) -> nsresult,

    /* [noscript] readonly attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aBaseURI: *mut *const nsIURI) -> nsresult,

    /* [noscript] readonly attribute nsNPAPIPluginInstancePtr pluginInstance; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_pluginInstance: *const ::libc::c_void,

    /* [noscript] void hasNewFrame (in nsIObjectFrame aFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub hasNewFrame: *const ::libc::c_void,

    /* [noscript] nsIFrame getPrintFrame (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPrintFrame: *const ::libc::c_void,

    /* [noscript] void pluginDestroyed (); */
    pub pluginDestroyed: unsafe extern "C" fn (this: *const nsIObjectLoadingContent) -> nsresult,

    /* [noscript] void pluginCrashed (in nsIPluginTag pluginTag, in AString pluginDumpID, in AString browserDumpID, in boolean submittedCrashReport); */
    pub pluginCrashed: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, pluginTag: *const nsIPluginTag, pluginDumpID: *const nsAString, browserDumpID: *const nsAString, submittedCrashReport: bool) -> nsresult,

    /* void reload (in boolean aClearActivation); */
    pub reload: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aClearActivation: bool) -> nsresult,

    /* readonly attribute boolean activated; */
    pub get_activated: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aActivated: *mut bool) -> nsresult,

    /* [noscript] void stopPluginInstance (); */
    pub stopPluginInstance: unsafe extern "C" fn (this: *const nsIObjectLoadingContent) -> nsresult,

    /* [noscript] void syncStartPluginInstance (); */
    pub syncStartPluginInstance: unsafe extern "C" fn (this: *const nsIObjectLoadingContent) -> nsresult,

    /* [noscript] void asyncStartPluginInstance (); */
    pub asyncStartPluginInstance: unsafe extern "C" fn (this: *const nsIObjectLoadingContent) -> nsresult,

    /* [noscript] void initializeFromChannel (in nsIRequest request); */
    pub initializeFromChannel: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, request: *const nsIRequest) -> nsresult,

    /* readonly attribute nsIURI srcURI; */
    pub get_srcURI: unsafe extern "C" fn (this: *const nsIObjectLoadingContent, aSrcURI: *mut *const nsIURI) -> nsresult,

}


impl nsIObjectLoadingContent {
    /* readonly attribute ACString actualType; */
    #[inline]
    pub unsafe fn get_actualType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_actualType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long displayedType; */
    #[inline]
    pub unsafe fn get_displayedType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_displayedType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getContentTypeForMIMEType (in AUTF8String aMimeType); */
    #[inline]
    pub unsafe fn getContentTypeForMIMEType(&self, aMimeType: &[u8]) -> Result<libc::uint32_t, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getContentTypeForMIMEType)(self as *const _, &*aMimeType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] readonly attribute nsNPAPIPluginInstancePtr pluginInstance; */


    /* [noscript] void hasNewFrame (in nsIObjectFrame aFrame); */


    /* [noscript] nsIFrame getPrintFrame (); */


    /* [noscript] void pluginDestroyed (); */
    #[inline]
    pub unsafe fn pluginDestroyed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).pluginDestroyed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void pluginCrashed (in nsIPluginTag pluginTag, in AString pluginDumpID, in AString browserDumpID, in boolean submittedCrashReport); */
    #[inline]
    pub unsafe fn pluginCrashed(&self, pluginTag: Option<&nsIPluginTag>, pluginDumpID: &[u16], browserDumpID: &[u16], submittedCrashReport: bool) -> Result<(), nsresult> {
        let pluginDumpID = nsString::from(pluginDumpID);
        let browserDumpID = nsString::from(browserDumpID);
        match ((*self.vtable).pluginCrashed)(self as *const _, pluginTag.map_or(::std::ptr::null(), |x| x as *const _), &*pluginDumpID, &*browserDumpID, submittedCrashReport) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reload (in boolean aClearActivation); */
    #[inline]
    pub unsafe fn reload(&self, aClearActivation: bool) -> Result<(), nsresult> {

        match ((*self.vtable).reload)(self as *const _, aClearActivation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean activated; */
    #[inline]
    pub unsafe fn get_activated(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_activated)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void stopPluginInstance (); */
    #[inline]
    pub unsafe fn stopPluginInstance(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopPluginInstance)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void syncStartPluginInstance (); */
    #[inline]
    pub unsafe fn syncStartPluginInstance(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).syncStartPluginInstance)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void asyncStartPluginInstance (); */
    #[inline]
    pub unsafe fn asyncStartPluginInstance(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).asyncStartPluginInstance)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initializeFromChannel (in nsIRequest request); */
    #[inline]
    pub unsafe fn initializeFromChannel(&self, request: Option<&nsIRequest>) -> Result<(), nsresult> {

        match ((*self.vtable).initializeFromChannel)(self as *const _, request.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIURI srcURI; */
    #[inline]
    pub unsafe fn get_srcURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_srcURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


