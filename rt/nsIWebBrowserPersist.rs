//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersist.idl
//


pub mod nsIWebBrowserPersist_consts {
    pub const PERSIST_FLAGS_NONE: i64 = 0;
    pub const PERSIST_FLAGS_FROM_CACHE: i64 = 1;
    pub const PERSIST_FLAGS_BYPASS_CACHE: i64 = 2;
    pub const PERSIST_FLAGS_IGNORE_REDIRECTED_DATA: i64 = 4;
    pub const PERSIST_FLAGS_IGNORE_IFRAMES: i64 = 8;
    pub const PERSIST_FLAGS_NO_CONVERSION: i64 = 16;
    pub const PERSIST_FLAGS_REPLACE_EXISTING_FILES: i64 = 32;
    pub const PERSIST_FLAGS_NO_BASE_TAG_MODIFICATIONS: i64 = 64;
    pub const PERSIST_FLAGS_FIXUP_ORIGINAL_DOM: i64 = 128;
    pub const PERSIST_FLAGS_FIXUP_LINKS_TO_DESTINATION: i64 = 256;
    pub const PERSIST_FLAGS_DONT_FIXUP_LINKS: i64 = 512;
    pub const PERSIST_FLAGS_SERIALIZE_OUTPUT: i64 = 1024;
    pub const PERSIST_FLAGS_DONT_CHANGE_FILENAMES: i64 = 2048;
    pub const PERSIST_FLAGS_FAIL_ON_BROKEN_LINKS: i64 = 4096;
    pub const PERSIST_FLAGS_CLEANUP_ON_FAILURE: i64 = 8192;
    pub const PERSIST_FLAGS_AUTODETECT_APPLY_CONVERSION: i64 = 16384;
    pub const PERSIST_FLAGS_APPEND_TO_FILE: i64 = 32768;
    pub const PERSIST_FLAGS_FORCE_ALLOW_COOKIES: i64 = 65536;
    pub const PERSIST_STATE_READY: i64 = 1;
    pub const PERSIST_STATE_SAVING: i64 = 2;
    pub const PERSIST_STATE_FINISHED: i64 = 3;
    pub const ENCODE_FLAGS_SELECTION_ONLY: i64 = 1;
    pub const ENCODE_FLAGS_FORMATTED: i64 = 2;
    pub const ENCODE_FLAGS_RAW: i64 = 4;
    pub const ENCODE_FLAGS_BODY_ONLY: i64 = 8;
    pub const ENCODE_FLAGS_PREFORMATTED: i64 = 16;
    pub const ENCODE_FLAGS_WRAP: i64 = 32;
    pub const ENCODE_FLAGS_FORMAT_FLOWED: i64 = 64;
    pub const ENCODE_FLAGS_ABSOLUTE_LINKS: i64 = 128;
    pub const ENCODE_FLAGS_ENCODE_W3C_ENTITIES: i64 = 256;
    pub const ENCODE_FLAGS_CR_LINEBREAKS: i64 = 512;
    pub const ENCODE_FLAGS_LF_LINEBREAKS: i64 = 1024;
    pub const ENCODE_FLAGS_NOSCRIPT_CONTENT: i64 = 2048;
    pub const ENCODE_FLAGS_NOFRAMES_CONTENT: i64 = 4096;
    pub const ENCODE_FLAGS_ENCODE_BASIC_ENTITIES: i64 = 8192;
    pub const ENCODE_FLAGS_ENCODE_LATIN1_ENTITIES: i64 = 16384;
    pub const ENCODE_FLAGS_ENCODE_HTML_ENTITIES: i64 = 32768;
}


#[repr(C)]
pub struct nsIWebBrowserPersist {
    vtable: *const nsIWebBrowserPersistVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersist {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8cd752a4, 0x60b1, 0x42c3,
            [0xa8, 0x19, 0x65, 0xc7, 0xa1, 0x13, 0x8a, 0x28])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersist {
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
pub trait nsIWebBrowserPersistCoerce {
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self;
}

impl nsIWebBrowserPersistCoerce for nsIWebBrowserPersist {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self {
        v
    }
}

impl nsIWebBrowserPersist {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersist {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICancelableCoerce> nsIWebBrowserPersistCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersist) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistVTable {
    pub __base: nsICancelableVTable,

    /* attribute unsigned long persistFlags; */
    pub get_persistFlags: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aPersistFlags: *mut libc::uint32_t) -> nsresult,
    pub set_persistFlags: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aPersistFlags: libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long currentState; */
    pub get_currentState: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aCurrentState: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsresult result; */
    pub get_result: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aResult: *mut nsresult) -> nsresult,

    /* attribute nsIWebProgressListener progressListener; */
    pub get_progressListener: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aProgressListener: *mut *const nsIWebProgressListener) -> nsresult,
    pub set_progressListener: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aProgressListener: *const nsIWebProgressListener) -> nsresult,

    /* void saveURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsILoadContext aPrivacyContext); */
    pub saveURI: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aURI: *const nsIURI, aCacheKey: *const nsISupports, aReferrer: *const nsIURI, aReferrerPolicy: libc::uint32_t, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aPrivacyContext: *const nsILoadContext) -> nsresult,

    /* void savePrivacyAwareURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in boolean aIsPrivate); */
    pub savePrivacyAwareURI: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aURI: *const nsIURI, aCacheKey: *const nsISupports, aReferrer: *const nsIURI, aReferrerPolicy: libc::uint32_t, aPostData: *const nsIInputStream, aExtraHeaders: *const libc::c_char, aFile: *const nsISupports, aIsPrivate: bool) -> nsresult,

    /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
    pub saveChannel: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aChannel: *const nsIChannel, aFile: *const nsISupports) -> nsresult,

    /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
    pub saveDocument: unsafe extern "C" fn (this: *const nsIWebBrowserPersist, aDocument: *const nsISupports, aFile: *const nsISupports, aDataPath: *const nsISupports, aOutputContentType: *const libc::c_char, aEncodingFlags: libc::uint32_t, aWrapColumn: libc::uint32_t) -> nsresult,

    /* void cancelSave (); */
    pub cancelSave: unsafe extern "C" fn (this: *const nsIWebBrowserPersist) -> nsresult,

}


impl nsIWebBrowserPersist {
    /* attribute unsigned long persistFlags; */
    #[inline]
    pub unsafe fn get_persistFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_persistFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_persistFlags(&self, aPersistFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_persistFlags)(self as *const _, aPersistFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute unsigned long currentState; */
    #[inline]
    pub unsafe fn get_currentState(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_currentState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsresult result; */
    #[inline]
    pub unsafe fn get_result(&self, ) -> Result<nsresult, nsresult> {
        let mut _retval: nsresult = ::std::mem::zeroed();
        match ((*self.vtable).get_result)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIWebProgressListener progressListener; */
    #[inline]
    pub unsafe fn get_progressListener(&self, ) -> Result<Option<RefPtr<nsIWebProgressListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_progressListener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_progressListener(&self, aProgressListener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_progressListener)(self as *const _, aProgressListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsILoadContext aPrivacyContext); */
    #[inline]
    pub unsafe fn saveURI(&self, aURI: Option<&nsIURI>, aCacheKey: Option<&nsISupports>, aReferrer: Option<&nsIURI>, aReferrerPolicy: libc::uint32_t, aPostData: Option<&nsIInputStream>, aExtraHeaders: *const libc::c_char, aFile: Option<&nsISupports>, aPrivacyContext: Option<&nsILoadContext>) -> Result<(), nsresult> {

        match ((*self.vtable).saveURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aCacheKey.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aReferrerPolicy, aPostData.map_or(::std::ptr::null(), |x| x as *const _), aExtraHeaders, aFile.map_or(::std::ptr::null(), |x| x as *const _), aPrivacyContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void savePrivacyAwareURI (in nsIURI aURI, in nsISupports aCacheKey, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in boolean aIsPrivate); */
    #[inline]
    pub unsafe fn savePrivacyAwareURI(&self, aURI: Option<&nsIURI>, aCacheKey: Option<&nsISupports>, aReferrer: Option<&nsIURI>, aReferrerPolicy: libc::uint32_t, aPostData: Option<&nsIInputStream>, aExtraHeaders: *const libc::c_char, aFile: Option<&nsISupports>, aIsPrivate: bool) -> Result<(), nsresult> {

        match ((*self.vtable).savePrivacyAwareURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aCacheKey.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aReferrerPolicy, aPostData.map_or(::std::ptr::null(), |x| x as *const _), aExtraHeaders, aFile.map_or(::std::ptr::null(), |x| x as *const _), aIsPrivate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
    #[inline]
    pub unsafe fn saveChannel(&self, aChannel: Option<&nsIChannel>, aFile: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).saveChannel)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aFile.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
    #[inline]
    pub unsafe fn saveDocument(&self, aDocument: Option<&nsISupports>, aFile: Option<&nsISupports>, aDataPath: Option<&nsISupports>, aOutputContentType: *const libc::c_char, aEncodingFlags: libc::uint32_t, aWrapColumn: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).saveDocument)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aFile.map_or(::std::ptr::null(), |x| x as *const _), aDataPath.map_or(::std::ptr::null(), |x| x as *const _), aOutputContentType, aEncodingFlags, aWrapColumn) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancelSave (); */
    #[inline]
    pub unsafe fn cancelSave(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancelSave)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


