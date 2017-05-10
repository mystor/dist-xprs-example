//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDocShell.idl
//


pub mod nsIDocShell_consts {
    pub const INTERNAL_LOAD_FLAGS_NONE: i64 = 0;
    pub const INTERNAL_LOAD_FLAGS_INHERIT_PRINCIPAL: i64 = 1;
    pub const INTERNAL_LOAD_FLAGS_DONT_SEND_REFERRER: i64 = 2;
    pub const INTERNAL_LOAD_FLAGS_ALLOW_THIRD_PARTY_FIXUP: i64 = 4;
    pub const INTERNAL_LOAD_FLAGS_FIRST_LOAD: i64 = 8;
    pub const INTERNAL_LOAD_FLAGS_BYPASS_CLASSIFIER: i64 = 16;
    pub const INTERNAL_LOAD_FLAGS_FORCE_ALLOW_COOKIES: i64 = 32;
    pub const INTERNAL_LOAD_FLAGS_IS_SRCDOC: i64 = 64;
    pub const INTERNAL_LOAD_FLAGS_NO_OPENER: i64 = 256;
    pub const ENUMERATE_FORWARDS: i64 = 0;
    pub const ENUMERATE_BACKWARDS: i64 = 1;
    pub const APP_TYPE_UNKNOWN: i64 = 0;
    pub const APP_TYPE_MAIL: i64 = 1;
    pub const APP_TYPE_EDITOR: i64 = 2;
    pub const BUSY_FLAGS_NONE: i64 = 0;
    pub const BUSY_FLAGS_BUSY: i64 = 1;
    pub const BUSY_FLAGS_BEFORE_PAGE_LOAD: i64 = 2;
    pub const BUSY_FLAGS_PAGE_LOADING: i64 = 4;
    pub const LOAD_CMD_NORMAL: i64 = 1;
    pub const LOAD_CMD_RELOAD: i64 = 2;
    pub const LOAD_CMD_HISTORY: i64 = 4;
    pub const LOAD_CMD_PUSHSTATE: i64 = 8;
    pub const FRAME_TYPE_REGULAR: i64 = 0;
    pub const FRAME_TYPE_BROWSER: i64 = 1;
    pub const TOUCHEVENTS_OVERRIDE_DISABLED: i64 = 0;
    pub const TOUCHEVENTS_OVERRIDE_ENABLED: i64 = 1;
    pub const TOUCHEVENTS_OVERRIDE_NONE: i64 = 2;
}


#[repr(C)]
pub struct nsIDocShell {
    vtable: *const nsIDocShellVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDocShell {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x049234fe, 0xda10, 0x478b,
            [0xbc, 0x5d, 0xbc, 0x6f, 0x9a, 0x1b, 0xa6, 0x3d])
    }
}

unsafe impl RefCounted for nsIDocShell {
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
pub trait nsIDocShellCoerce {
    fn coerce_from(v: &nsIDocShell) -> &Self;
}

impl nsIDocShellCoerce for nsIDocShell {
    #[inline]
    fn coerce_from(v: &nsIDocShell) -> &Self {
        v
    }
}

impl nsIDocShell {
    #[inline]
    pub fn coerce<T: nsIDocShellCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDocShell {
    type Target = nsIDocShellTreeItem;
    #[inline]
    fn deref(&self) -> &nsIDocShellTreeItem {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIDocShellTreeItemCoerce> nsIDocShellCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShell) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDocShellVTable {
    pub __base: nsIDocShellTreeItemVTable,

    /* [noscript] void loadURI (in nsIURI uri, in nsIDocShellLoadInfo loadInfo, in unsigned long aLoadFlags, in boolean firstParty); */
    pub loadURI: unsafe extern "C" fn (this: *const nsIDocShell, uri: *const nsIURI, loadInfo: *const nsIDocShellLoadInfo, aLoadFlags: libc::uint32_t, firstParty: bool) -> nsresult,

    /* [noscript] void loadStream (in nsIInputStream aStream, in nsIURI aURI, in ACString aContentType, in ACString aContentCharset, in nsIDocShellLoadInfo aLoadInfo); */
    pub loadStream: unsafe extern "C" fn (this: *const nsIDocShell, aStream: *const nsIInputStream, aURI: *const nsIURI, aContentType: *const nsACString, aContentCharset: *const nsACString, aLoadInfo: *const nsIDocShellLoadInfo) -> nsresult,

    /* [noscript] void internalLoad (in nsIURI aURI, in nsIURI aOriginalURI, in boolean aLoadReplace, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIPrincipal aTriggeringPrincipal, in nsIPrincipal aPrincipalToInherit, in uint32_t aFlags, in AString aWindowTarget, in string aTypeHint, in AString aFileName, in nsIInputStream aPostDataStream, in nsIInputStream aHeadersStream, in unsigned long aLoadFlags, in nsISHEntry aSHEntry, in boolean aFirstParty, in AString aSrcdoc, in nsIDocShell aSourceDocShell, in nsIURI aBaseURI, in boolean aCheckForPrerender, out nsIDocShell aDocShell, out nsIRequest aRequest); */
    pub internalLoad: unsafe extern "C" fn (this: *const nsIDocShell, aURI: *const nsIURI, aOriginalURI: *const nsIURI, aLoadReplace: bool, aReferrer: *const nsIURI, aReferrerPolicy: libc::uint32_t, aTriggeringPrincipal: *const nsIPrincipal, aPrincipalToInherit: *const nsIPrincipal, aFlags: uint32_t, aWindowTarget: *const nsAString, aTypeHint: *const libc::c_char, aFileName: *const nsAString, aPostDataStream: *const nsIInputStream, aHeadersStream: *const nsIInputStream, aLoadFlags: libc::uint32_t, aSHEntry: *const nsISHEntry, aFirstParty: bool, aSrcdoc: *const nsAString, aSourceDocShell: *const nsIDocShell, aBaseURI: *const nsIURI, aCheckForPrerender: bool, aDocShell: *mut *const nsIDocShell, aRequest: *mut *const nsIRequest) -> nsresult,

    /* [implicit_jscontext] void addState (in jsval aData, in DOMString aTitle, in DOMString aURL, in boolean aReplace); */
    /// Unable to call function as its signature contains a non-rust type
    pub addState: *const ::libc::c_void,

    /* void createLoadInfo (out nsIDocShellLoadInfo loadInfo); */
    pub createLoadInfo: unsafe extern "C" fn (this: *const nsIDocShell, loadInfo: *mut *const nsIDocShellLoadInfo) -> nsresult,

    /* void prepareForNewContentModel (); */
    pub prepareForNewContentModel: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* void setCurrentURI (in nsIURI aURI); */
    pub setCurrentURI: unsafe extern "C" fn (this: *const nsIDocShell, aURI: *const nsIURI) -> nsresult,

    /* [noscript] void firePageHideNotification (in boolean isUnload); */
    pub firePageHideNotification: unsafe extern "C" fn (this: *const nsIDocShell, isUnload: bool) -> nsresult,

    /* [noscript] readonly attribute nsPresContext presContext; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_presContext: *const ::libc::c_void,

    /* [noscript,notxpcom] nsIPresShell GetPresShell (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetPresShell: *const ::libc::c_void,

    /* [noscript] readonly attribute nsIPresShell eldestPresShell; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_eldestPresShell: *const ::libc::c_void,

    /* readonly attribute nsIContentViewer contentViewer; */
    pub get_contentViewer: unsafe extern "C" fn (this: *const nsIDocShell, aContentViewer: *mut *const nsIContentViewer) -> nsresult,

    /* attribute nsIDOMEventTarget chromeEventHandler; */
    pub get_chromeEventHandler: unsafe extern "C" fn (this: *const nsIDocShell, aChromeEventHandler: *mut *const nsIDOMEventTarget) -> nsresult,
    pub set_chromeEventHandler: unsafe extern "C" fn (this: *const nsIDocShell, aChromeEventHandler: *const nsIDOMEventTarget) -> nsresult,

    /* attribute DOMString customUserAgent; */
    pub get_customUserAgent: unsafe extern "C" fn (this: *const nsIDocShell, aCustomUserAgent: *mut nsAString) -> nsresult,
    pub set_customUserAgent: unsafe extern "C" fn (this: *const nsIDocShell, aCustomUserAgent: *const nsAString) -> nsresult,

    /* attribute boolean allowPlugins; */
    pub get_allowPlugins: unsafe extern "C" fn (this: *const nsIDocShell, aAllowPlugins: *mut bool) -> nsresult,
    pub set_allowPlugins: unsafe extern "C" fn (this: *const nsIDocShell, aAllowPlugins: bool) -> nsresult,

    /* attribute boolean allowJavascript; */
    pub get_allowJavascript: unsafe extern "C" fn (this: *const nsIDocShell, aAllowJavascript: *mut bool) -> nsresult,
    pub set_allowJavascript: unsafe extern "C" fn (this: *const nsIDocShell, aAllowJavascript: bool) -> nsresult,

    /* attribute boolean allowMetaRedirects; */
    pub get_allowMetaRedirects: unsafe extern "C" fn (this: *const nsIDocShell, aAllowMetaRedirects: *mut bool) -> nsresult,
    pub set_allowMetaRedirects: unsafe extern "C" fn (this: *const nsIDocShell, aAllowMetaRedirects: bool) -> nsresult,

    /* attribute boolean allowSubframes; */
    pub get_allowSubframes: unsafe extern "C" fn (this: *const nsIDocShell, aAllowSubframes: *mut bool) -> nsresult,
    pub set_allowSubframes: unsafe extern "C" fn (this: *const nsIDocShell, aAllowSubframes: bool) -> nsresult,

    /* attribute boolean allowImages; */
    pub get_allowImages: unsafe extern "C" fn (this: *const nsIDocShell, aAllowImages: *mut bool) -> nsresult,
    pub set_allowImages: unsafe extern "C" fn (this: *const nsIDocShell, aAllowImages: bool) -> nsresult,

    /* [infallible] attribute boolean allowMedia; */
    pub get_allowMedia: unsafe extern "C" fn (this: *const nsIDocShell, aAllowMedia: *mut bool) -> nsresult,
    pub set_allowMedia: unsafe extern "C" fn (this: *const nsIDocShell, aAllowMedia: bool) -> nsresult,

    /* attribute boolean allowDNSPrefetch; */
    pub get_allowDNSPrefetch: unsafe extern "C" fn (this: *const nsIDocShell, aAllowDNSPrefetch: *mut bool) -> nsresult,
    pub set_allowDNSPrefetch: unsafe extern "C" fn (this: *const nsIDocShell, aAllowDNSPrefetch: bool) -> nsresult,

    /* attribute boolean allowWindowControl; */
    pub get_allowWindowControl: unsafe extern "C" fn (this: *const nsIDocShell, aAllowWindowControl: *mut bool) -> nsresult,
    pub set_allowWindowControl: unsafe extern "C" fn (this: *const nsIDocShell, aAllowWindowControl: bool) -> nsresult,

    /* [infallible] attribute boolean allowContentRetargeting; */
    pub get_allowContentRetargeting: unsafe extern "C" fn (this: *const nsIDocShell, aAllowContentRetargeting: *mut bool) -> nsresult,
    pub set_allowContentRetargeting: unsafe extern "C" fn (this: *const nsIDocShell, aAllowContentRetargeting: bool) -> nsresult,

    /* [infallible] attribute boolean allowContentRetargetingOnChildren; */
    pub get_allowContentRetargetingOnChildren: unsafe extern "C" fn (this: *const nsIDocShell, aAllowContentRetargetingOnChildren: *mut bool) -> nsresult,
    pub set_allowContentRetargetingOnChildren: unsafe extern "C" fn (this: *const nsIDocShell, aAllowContentRetargetingOnChildren: bool) -> nsresult,

    /* [infallible] attribute boolean inheritPrivateBrowsingId; */
    pub get_inheritPrivateBrowsingId: unsafe extern "C" fn (this: *const nsIDocShell, aInheritPrivateBrowsingId: *mut bool) -> nsresult,
    pub set_inheritPrivateBrowsingId: unsafe extern "C" fn (this: *const nsIDocShell, aInheritPrivateBrowsingId: bool) -> nsresult,

    /* nsISimpleEnumerator getDocShellEnumerator (in long aItemType, in long aDirection); */
    pub getDocShellEnumerator: unsafe extern "C" fn (this: *const nsIDocShell, aItemType: libc::int32_t, aDirection: libc::int32_t, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* attribute unsigned long appType; */
    pub get_appType: unsafe extern "C" fn (this: *const nsIDocShell, aAppType: *mut libc::uint32_t) -> nsresult,
    pub set_appType: unsafe extern "C" fn (this: *const nsIDocShell, aAppType: libc::uint32_t) -> nsresult,

    /* attribute boolean allowAuth; */
    pub get_allowAuth: unsafe extern "C" fn (this: *const nsIDocShell, aAllowAuth: *mut bool) -> nsresult,
    pub set_allowAuth: unsafe extern "C" fn (this: *const nsIDocShell, aAllowAuth: bool) -> nsresult,

    /* attribute float zoom; */
    pub get_zoom: unsafe extern "C" fn (this: *const nsIDocShell, aZoom: *mut libc::c_float) -> nsresult,
    pub set_zoom: unsafe extern "C" fn (this: *const nsIDocShell, aZoom: libc::c_float) -> nsresult,

    /* attribute long marginWidth; */
    pub get_marginWidth: unsafe extern "C" fn (this: *const nsIDocShell, aMarginWidth: *mut libc::int32_t) -> nsresult,
    pub set_marginWidth: unsafe extern "C" fn (this: *const nsIDocShell, aMarginWidth: libc::int32_t) -> nsresult,

    /* attribute long marginHeight; */
    pub get_marginHeight: unsafe extern "C" fn (this: *const nsIDocShell, aMarginHeight: *mut libc::int32_t) -> nsresult,
    pub set_marginHeight: unsafe extern "C" fn (this: *const nsIDocShell, aMarginHeight: libc::int32_t) -> nsresult,

    /* bool tabToTreeOwner (in boolean forward, in boolean forDocumentNavigation); */
    pub tabToTreeOwner: unsafe extern "C" fn (this: *const nsIDocShell, forward: bool, forDocumentNavigation: bool, _retval: *mut bool) -> nsresult,

    /* readonly attribute unsigned long busyFlags; */
    pub get_busyFlags: unsafe extern "C" fn (this: *const nsIDocShell, aBusyFlags: *mut libc::uint32_t) -> nsresult,

    /* attribute unsigned long loadType; */
    pub get_loadType: unsafe extern "C" fn (this: *const nsIDocShell, aLoadType: *mut libc::uint32_t) -> nsresult,
    pub set_loadType: unsafe extern "C" fn (this: *const nsIDocShell, aLoadType: libc::uint32_t) -> nsresult,

    /* attribute nsLoadFlags defaultLoadFlags; */
    pub get_defaultLoadFlags: unsafe extern "C" fn (this: *const nsIDocShell, aDefaultLoadFlags: *mut nsLoadFlags) -> nsresult,
    pub set_defaultLoadFlags: unsafe extern "C" fn (this: *const nsIDocShell, aDefaultLoadFlags: nsLoadFlags) -> nsresult,

    /* boolean isBeingDestroyed (); */
    pub isBeingDestroyed: unsafe extern "C" fn (this: *const nsIDocShell, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean isExecutingOnLoadHandler; */
    pub get_isExecutingOnLoadHandler: unsafe extern "C" fn (this: *const nsIDocShell, aIsExecutingOnLoadHandler: *mut bool) -> nsresult,

    /* attribute nsILayoutHistoryState layoutHistoryState; */
    pub get_layoutHistoryState: unsafe extern "C" fn (this: *const nsIDocShell, aLayoutHistoryState: *mut *const nsILayoutHistoryState) -> nsresult,
    pub set_layoutHistoryState: unsafe extern "C" fn (this: *const nsIDocShell, aLayoutHistoryState: *const nsILayoutHistoryState) -> nsresult,

    /* readonly attribute boolean shouldSaveLayoutState; */
    pub get_shouldSaveLayoutState: unsafe extern "C" fn (this: *const nsIDocShell, aShouldSaveLayoutState: *mut bool) -> nsresult,

    /* attribute nsISecureBrowserUI securityUI; */
    pub get_securityUI: unsafe extern "C" fn (this: *const nsIDocShell, aSecurityUI: *mut *const nsISecureBrowserUI) -> nsresult,
    pub set_securityUI: unsafe extern "C" fn (this: *const nsIDocShell, aSecurityUI: *const nsISecureBrowserUI) -> nsresult,

    /* void suspendRefreshURIs (); */
    pub suspendRefreshURIs: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* void resumeRefreshURIs (); */
    pub resumeRefreshURIs: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* void beginRestore (in nsIContentViewer viewer, in boolean top); */
    pub beginRestore: unsafe extern "C" fn (this: *const nsIDocShell, viewer: *const nsIContentViewer, top: bool) -> nsresult,

    /* void finishRestore (); */
    pub finishRestore: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* readonly attribute boolean restoringDocument; */
    pub get_restoringDocument: unsafe extern "C" fn (this: *const nsIDocShell, aRestoringDocument: *mut bool) -> nsresult,

    /* attribute boolean useErrorPages; */
    pub get_useErrorPages: unsafe extern "C" fn (this: *const nsIDocShell, aUseErrorPages: *mut bool) -> nsresult,
    pub set_useErrorPages: unsafe extern "C" fn (this: *const nsIDocShell, aUseErrorPages: bool) -> nsresult,

    /* boolean displayLoadError (in nsresult aError, in nsIURI aURI, in wstring aURL, [optional] in nsIChannel aFailedChannel); */
    pub displayLoadError: unsafe extern "C" fn (this: *const nsIDocShell, aError: nsresult, aURI: *const nsIURI, aURL: *const libc::int16_t, aFailedChannel: *const nsIChannel, _retval: *mut bool) -> nsresult,

    /* readonly attribute nsIChannel failedChannel; */
    pub get_failedChannel: unsafe extern "C" fn (this: *const nsIDocShell, aFailedChannel: *mut *const nsIChannel) -> nsresult,

    /* readonly attribute long previousTransIndex; */
    pub get_previousTransIndex: unsafe extern "C" fn (this: *const nsIDocShell, aPreviousTransIndex: *mut libc::int32_t) -> nsresult,

    /* readonly attribute long loadedTransIndex; */
    pub get_loadedTransIndex: unsafe extern "C" fn (this: *const nsIDocShell, aLoadedTransIndex: *mut libc::int32_t) -> nsresult,

    /* void historyPurged (in long numEntries); */
    pub historyPurged: unsafe extern "C" fn (this: *const nsIDocShell, numEntries: libc::int32_t) -> nsresult,

    /* readonly attribute nsIChannel currentDocumentChannel; */
    pub get_currentDocumentChannel: unsafe extern "C" fn (this: *const nsIDocShell, aCurrentDocumentChannel: *mut *const nsIChannel) -> nsresult,

    /* [noscript] void setChildOffset (in unsigned long offset); */
    pub setChildOffset: unsafe extern "C" fn (this: *const nsIDocShell, offset: libc::uint32_t) -> nsresult,

    /* readonly attribute boolean isInUnload; */
    pub get_isInUnload: unsafe extern "C" fn (this: *const nsIDocShell, aIsInUnload: *mut bool) -> nsresult,

    /* readonly attribute boolean channelIsUnsafe; */
    pub get_channelIsUnsafe: unsafe extern "C" fn (this: *const nsIDocShell, aChannelIsUnsafe: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasMixedActiveContentLoaded; */
    pub get_hasMixedActiveContentLoaded: unsafe extern "C" fn (this: *const nsIDocShell, aHasMixedActiveContentLoaded: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasMixedActiveContentBlocked; */
    pub get_hasMixedActiveContentBlocked: unsafe extern "C" fn (this: *const nsIDocShell, aHasMixedActiveContentBlocked: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasMixedDisplayContentLoaded; */
    pub get_hasMixedDisplayContentLoaded: unsafe extern "C" fn (this: *const nsIDocShell, aHasMixedDisplayContentLoaded: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasMixedDisplayContentBlocked; */
    pub get_hasMixedDisplayContentBlocked: unsafe extern "C" fn (this: *const nsIDocShell, aHasMixedDisplayContentBlocked: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasTrackingContentBlocked; */
    pub get_hasTrackingContentBlocked: unsafe extern "C" fn (this: *const nsIDocShell, aHasTrackingContentBlocked: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean hasTrackingContentLoaded; */
    pub get_hasTrackingContentLoaded: unsafe extern "C" fn (this: *const nsIDocShell, aHasTrackingContentLoaded: *mut bool) -> nsresult,

    /* [noscript,notxpcom] void DetachEditorFromWindow (); */
    pub DetachEditorFromWindow: unsafe extern "C" fn (this: *const nsIDocShell) -> libc::c_void,

    /* attribute boolean isOffScreenBrowser; */
    pub get_isOffScreenBrowser: unsafe extern "C" fn (this: *const nsIDocShell, aIsOffScreenBrowser: *mut bool) -> nsresult,
    pub set_isOffScreenBrowser: unsafe extern "C" fn (this: *const nsIDocShell, aIsOffScreenBrowser: bool) -> nsresult,

    /* readonly attribute nsIWebBrowserPrint printPreview; */
    pub get_printPreview: unsafe extern "C" fn (this: *const nsIDocShell, aPrintPreview: *mut *const nsIWebBrowserPrint) -> nsresult,

    /* [infallible] readonly attribute boolean canExecuteScripts; */
    pub get_canExecuteScripts: unsafe extern "C" fn (this: *const nsIDocShell, aCanExecuteScripts: *mut bool) -> nsresult,

    /* attribute boolean isActive; */
    pub get_isActive: unsafe extern "C" fn (this: *const nsIDocShell, aIsActive: *mut bool) -> nsresult,
    pub set_isActive: unsafe extern "C" fn (this: *const nsIDocShell, aIsActive: bool) -> nsresult,

    /* [noscript] void SetIsPrerendered (); */
    pub SetIsPrerendered: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* [infallible] readonly attribute boolean isPrerendered; */
    pub get_isPrerendered: unsafe extern "C" fn (this: *const nsIDocShell, aIsPrerendered: *mut bool) -> nsresult,

    /* readonly attribute nsIDPtr historyID; */
    pub get_historyID: unsafe extern "C" fn (this: *const nsIDocShell, aHistoryID: *mut *const nsID) -> nsresult,

    /* [noscript,notxpcom] nsID HistoryID (); */
    /// Unable to call function as its signature contains a non-rust type
    pub HistoryID: *const ::libc::c_void,

    /* attribute boolean isAppTab; */
    pub get_isAppTab: unsafe extern "C" fn (this: *const nsIDocShell, aIsAppTab: *mut bool) -> nsresult,
    pub set_isAppTab: unsafe extern "C" fn (this: *const nsIDocShell, aIsAppTab: bool) -> nsresult,

    /* void createAboutBlankContentViewer (in nsIPrincipal aPrincipal); */
    pub createAboutBlankContentViewer: unsafe extern "C" fn (this: *const nsIDocShell, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* [noscript] void forceCreateAboutBlankContentViewer (in nsIPrincipal aPrincipal); */
    pub forceCreateAboutBlankContentViewer: unsafe extern "C" fn (this: *const nsIDocShell, aPrincipal: *const nsIPrincipal) -> nsresult,

    /* attribute ACString charset; */
    pub get_charset: unsafe extern "C" fn (this: *const nsIDocShell, aCharset: *mut nsACString) -> nsresult,
    pub set_charset: unsafe extern "C" fn (this: *const nsIDocShell, aCharset: *const nsACString) -> nsresult,

    /* void gatherCharsetMenuTelemetry (); */
    pub gatherCharsetMenuTelemetry: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* attribute ACString forcedCharset; */
    pub get_forcedCharset: unsafe extern "C" fn (this: *const nsIDocShell, aForcedCharset: *mut nsACString) -> nsresult,
    pub set_forcedCharset: unsafe extern "C" fn (this: *const nsIDocShell, aForcedCharset: *const nsACString) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void setParentCharset (in ACString parentCharset, in int32_t parentCharsetSource, in nsIPrincipal parentCharsetPrincipal); */
    pub setParentCharset: unsafe extern "C" fn (this: *const nsIDocShell, parentCharset: *const nsACString, parentCharsetSource: int32_t, parentCharsetPrincipal: *const nsIPrincipal) -> libc::c_void,

    /* [noscript,nostdcall,notxpcom] void getParentCharset (out ACString parentCharset, out int32_t parentCharsetSource, out nsIPrincipal parentCharsetPrincipal); */
    pub getParentCharset: unsafe extern "C" fn (this: *const nsIDocShell, parentCharset: *mut nsACString, parentCharsetSource: *mut int32_t, parentCharsetPrincipal: *mut *const nsIPrincipal) -> libc::c_void,

    /* [infallible] attribute boolean recordProfileTimelineMarkers; */
    pub get_recordProfileTimelineMarkers: unsafe extern "C" fn (this: *const nsIDocShell, aRecordProfileTimelineMarkers: *mut bool) -> nsresult,
    pub set_recordProfileTimelineMarkers: unsafe extern "C" fn (this: *const nsIDocShell, aRecordProfileTimelineMarkers: bool) -> nsresult,

    /* DOMHighResTimeStamp now (); */
    pub now: unsafe extern "C" fn (this: *const nsIDocShell, _retval: *mut DOMHighResTimeStamp) -> nsresult,

    /* [implicit_jscontext] jsval popProfileTimelineMarkers (); */
    /// Unable to call function as its signature contains a non-rust type
    pub popProfileTimelineMarkers: *const ::libc::c_void,

    /* void addWeakPrivacyTransitionObserver (in nsIPrivacyTransitionObserver obs); */
    pub addWeakPrivacyTransitionObserver: unsafe extern "C" fn (this: *const nsIDocShell, obs: *const nsIPrivacyTransitionObserver) -> nsresult,

    /* void addWeakReflowObserver (in nsIReflowObserver obs); */
    pub addWeakReflowObserver: unsafe extern "C" fn (this: *const nsIDocShell, obs: *const nsIReflowObserver) -> nsresult,

    /* void removeWeakReflowObserver (in nsIReflowObserver obs); */
    pub removeWeakReflowObserver: unsafe extern "C" fn (this: *const nsIDocShell, obs: *const nsIReflowObserver) -> nsresult,

    /* [noscript] void notifyReflowObservers (in bool interruptible, in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    pub notifyReflowObservers: unsafe extern "C" fn (this: *const nsIDocShell, interruptible: bool, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> nsresult,

    /* [noscript] void addWeakScrollObserver (in nsIScrollObserver obs); */
    pub addWeakScrollObserver: unsafe extern "C" fn (this: *const nsIDocShell, obs: *const nsIScrollObserver) -> nsresult,

    /* [noscript] void removeWeakScrollObserver (in nsIScrollObserver obs); */
    pub removeWeakScrollObserver: unsafe extern "C" fn (this: *const nsIDocShell, obs: *const nsIScrollObserver) -> nsresult,

    /* [noscript] void notifyScrollObservers (); */
    pub notifyScrollObservers: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* [infallible] attribute unsigned long frameType; */
    pub get_frameType: unsafe extern "C" fn (this: *const nsIDocShell, aFrameType: *mut libc::uint32_t) -> nsresult,
    pub set_frameType: unsafe extern "C" fn (this: *const nsIDocShell, aFrameType: libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean isMozBrowser; */
    pub get_isMozBrowser: unsafe extern "C" fn (this: *const nsIDocShell, aIsMozBrowser: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isIsolatedMozBrowserElement; */
    pub get_isIsolatedMozBrowserElement: unsafe extern "C" fn (this: *const nsIDocShell, aIsIsolatedMozBrowserElement: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isInIsolatedMozBrowserElement; */
    pub get_isInIsolatedMozBrowserElement: unsafe extern "C" fn (this: *const nsIDocShell, aIsInIsolatedMozBrowserElement: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isInMozBrowser; */
    pub get_isInMozBrowser: unsafe extern "C" fn (this: *const nsIDocShell, aIsInMozBrowser: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean isTopLevelContentDocShell; */
    pub get_isTopLevelContentDocShell: unsafe extern "C" fn (this: *const nsIDocShell, aIsTopLevelContentDocShell: *mut bool) -> nsresult,

    /* nsIDocShell getSameTypeParentIgnoreBrowserBoundaries (); */
    pub getSameTypeParentIgnoreBrowserBoundaries: unsafe extern "C" fn (this: *const nsIDocShell, _retval: *mut *const nsIDocShell) -> nsresult,

    /* nsIDocShell getSameTypeRootTreeItemIgnoreBrowserBoundaries (); */
    pub getSameTypeRootTreeItemIgnoreBrowserBoundaries: unsafe extern "C" fn (this: *const nsIDocShell, _retval: *mut *const nsIDocShell) -> nsresult,

    /* readonly attribute bool asyncPanZoomEnabled; */
    pub get_asyncPanZoomEnabled: unsafe extern "C" fn (this: *const nsIDocShell, aAsyncPanZoomEnabled: *mut bool) -> nsresult,

    /* attribute unsigned long sandboxFlags; */
    pub get_sandboxFlags: unsafe extern "C" fn (this: *const nsIDocShell, aSandboxFlags: *mut libc::uint32_t) -> nsresult,
    pub set_sandboxFlags: unsafe extern "C" fn (this: *const nsIDocShell, aSandboxFlags: libc::uint32_t) -> nsresult,

    /* attribute nsIDocShell onePermittedSandboxedNavigator; */
    pub get_onePermittedSandboxedNavigator: unsafe extern "C" fn (this: *const nsIDocShell, aOnePermittedSandboxedNavigator: *mut *const nsIDocShell) -> nsresult,
    pub set_onePermittedSandboxedNavigator: unsafe extern "C" fn (this: *const nsIDocShell, aOnePermittedSandboxedNavigator: *const nsIDocShell) -> nsresult,

    /* [noscript,nostdcall,notxpcom] bool isSandboxedFrom (in nsIDocShell aTargetDocShell); */
    pub isSandboxedFrom: unsafe extern "C" fn (this: *const nsIDocShell, aTargetDocShell: *const nsIDocShell) -> bool,

    /* attribute nsIChannel mixedContentChannel; */
    pub get_mixedContentChannel: unsafe extern "C" fn (this: *const nsIDocShell, aMixedContentChannel: *mut *const nsIChannel) -> nsresult,
    pub set_mixedContentChannel: unsafe extern "C" fn (this: *const nsIDocShell, aMixedContentChannel: *const nsIChannel) -> nsresult,

    /* void GetAllowMixedContentAndConnectionData (out boolean rootHasSecureConnection, out boolean allowMixedContent, out boolean isRootDocShell); */
    pub GetAllowMixedContentAndConnectionData: unsafe extern "C" fn (this: *const nsIDocShell, rootHasSecureConnection: *mut bool, allowMixedContent: *mut bool, isRootDocShell: *mut bool) -> nsresult,

    /* [noscript,notxpcom] bool pluginsAllowedInCurrentDoc (); */
    pub pluginsAllowedInCurrentDoc: unsafe extern "C" fn (this: *const nsIDocShell) -> bool,

    /* [infallible] readonly attribute boolean fullscreenAllowed; */
    pub get_fullscreenAllowed: unsafe extern "C" fn (this: *const nsIDocShell, aFullscreenAllowed: *mut bool) -> nsresult,

    /* void setFullscreenAllowed (in boolean allowed); */
    pub setFullscreenAllowed: unsafe extern "C" fn (this: *const nsIDocShell, allowed: bool) -> nsresult,

    /* [notxpcom] uint32_t orientationLock (); */
    pub orientationLock: unsafe extern "C" fn (this: *const nsIDocShell) -> uint32_t,

    /* [notxpcom] void setOrientationLock (in uint32_t orientationLock); */
    pub setOrientationLock: unsafe extern "C" fn (this: *const nsIDocShell, orientationLock: uint32_t) -> libc::c_void,

    /* [infallible,noscript] attribute boolean affectPrivateSessionLifetime; */
    pub get_affectPrivateSessionLifetime: unsafe extern "C" fn (this: *const nsIDocShell, aAffectPrivateSessionLifetime: *mut bool) -> nsresult,
    pub set_affectPrivateSessionLifetime: unsafe extern "C" fn (this: *const nsIDocShell, aAffectPrivateSessionLifetime: bool) -> nsresult,

    /* [infallible] readonly attribute boolean mayEnableCharacterEncodingMenu; */
    pub get_mayEnableCharacterEncodingMenu: unsafe extern "C" fn (this: *const nsIDocShell, aMayEnableCharacterEncodingMenu: *mut bool) -> nsresult,

    /* attribute nsIEditor editor; */
    pub get_editor: unsafe extern "C" fn (this: *const nsIDocShell, aEditor: *mut *const nsIEditor) -> nsresult,
    pub set_editor: unsafe extern "C" fn (this: *const nsIDocShell, aEditor: *const nsIEditor) -> nsresult,

    /* readonly attribute boolean editable; */
    pub get_editable: unsafe extern "C" fn (this: *const nsIDocShell, aEditable: *mut bool) -> nsresult,

    /* readonly attribute boolean hasEditingSession; */
    pub get_hasEditingSession: unsafe extern "C" fn (this: *const nsIDocShell, aHasEditingSession: *mut bool) -> nsresult,

    /* void makeEditable (in boolean inWaitForUriLoad); */
    pub makeEditable: unsafe extern "C" fn (this: *const nsIDocShell, inWaitForUriLoad: bool) -> nsresult,

    /* nsISHEntry getChildSHEntry (in long aChildOffset); */
    pub getChildSHEntry: unsafe extern "C" fn (this: *const nsIDocShell, aChildOffset: libc::int32_t, _retval: *mut *const nsISHEntry) -> nsresult,

    /* void addChildSHEntry (in nsISHEntry aCloneReference, in nsISHEntry aHistoryEntry, in long aChildOffset, in unsigned long aLoadType, in boolean aCloneChilden); */
    pub addChildSHEntry: unsafe extern "C" fn (this: *const nsIDocShell, aCloneReference: *const nsISHEntry, aHistoryEntry: *const nsISHEntry, aChildOffset: libc::int32_t, aLoadType: libc::uint32_t, aCloneChilden: bool) -> nsresult,

    /* attribute boolean useGlobalHistory; */
    pub get_useGlobalHistory: unsafe extern "C" fn (this: *const nsIDocShell, aUseGlobalHistory: *mut bool) -> nsresult,
    pub set_useGlobalHistory: unsafe extern "C" fn (this: *const nsIDocShell, aUseGlobalHistory: bool) -> nsresult,

    /* void removeFromSessionHistory (); */
    pub removeFromSessionHistory: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

    /* attribute boolean createdDynamically; */
    pub get_createdDynamically: unsafe extern "C" fn (this: *const nsIDocShell, aCreatedDynamically: *mut bool) -> nsresult,
    pub set_createdDynamically: unsafe extern "C" fn (this: *const nsIDocShell, aCreatedDynamically: bool) -> nsresult,

    /* boolean getCurrentSHEntry (out nsISHEntry aEntry); */
    pub getCurrentSHEntry: unsafe extern "C" fn (this: *const nsIDocShell, aEntry: *mut *const nsISHEntry, _retval: *mut bool) -> nsresult,

    /* boolean isCommandEnabled (in string command); */
    pub isCommandEnabled: unsafe extern "C" fn (this: *const nsIDocShell, command: *const libc::c_char, _retval: *mut bool) -> nsresult,

    /* void doCommand (in string command); */
    pub doCommand: unsafe extern "C" fn (this: *const nsIDocShell, command: *const libc::c_char) -> nsresult,

    /* void doCommandWithParams (in string command, in nsICommandParams aParams); */
    pub doCommandWithParams: unsafe extern "C" fn (this: *const nsIDocShell, command: *const libc::c_char, aParams: *const nsICommandParams) -> nsresult,

    /* [noscript,notxpcom] bool IsInvisible (); */
    pub IsInvisible: unsafe extern "C" fn (this: *const nsIDocShell) -> bool,

    /* [noscript,notxpcom] void SetInvisible (in bool aIsInvisibleDochsell); */
    pub SetInvisible: unsafe extern "C" fn (this: *const nsIDocShell, aIsInvisibleDochsell: bool) -> libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsIScriptGlobalObject GetScriptGlobalObject (); */
    pub GetScriptGlobalObject: unsafe extern "C" fn (this: *const nsIDocShell) -> *const nsIScriptGlobalObject,

    /* [infallible] attribute boolean deviceSizeIsPageSize; */
    pub get_deviceSizeIsPageSize: unsafe extern "C" fn (this: *const nsIDocShell, aDeviceSizeIsPageSize: *mut bool) -> nsresult,
    pub set_deviceSizeIsPageSize: unsafe extern "C" fn (this: *const nsIDocShell, aDeviceSizeIsPageSize: bool) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void setOpener (in nsITabParent aOpener); */
    pub setOpener: unsafe extern "C" fn (this: *const nsIDocShell, aOpener: *const nsITabParent) -> libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsITabParent getOpener (); */
    pub getOpener: unsafe extern "C" fn (this: *const nsIDocShell) -> *const nsITabParent,

    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStart (in string aReason, in wstring functionName, in wstring fileName, in unsigned long lineNumber, in jsval asyncStack, in string asyncCause); */
    /// Unable to call function as its signature contains a non-rust type
    pub notifyJSRunToCompletionStart: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStop (); */
    pub notifyJSRunToCompletionStop: unsafe extern "C" fn (this: *const nsIDocShell) -> libc::c_void,

    /* [infallible] readonly attribute boolean hasLoadedNonBlankURI; */
    pub get_hasLoadedNonBlankURI: unsafe extern "C" fn (this: *const nsIDocShell, aHasLoadedNonBlankURI: *mut bool) -> nsresult,

    /* attribute boolean windowDraggingAllowed; */
    pub get_windowDraggingAllowed: unsafe extern "C" fn (this: *const nsIDocShell, aWindowDraggingAllowed: *mut bool) -> nsresult,
    pub set_windowDraggingAllowed: unsafe extern "C" fn (this: *const nsIDocShell, aWindowDraggingAllowed: bool) -> nsresult,

    /* attribute boolean currentScrollRestorationIsManual; */
    pub get_currentScrollRestorationIsManual: unsafe extern "C" fn (this: *const nsIDocShell, aCurrentScrollRestorationIsManual: *mut bool) -> nsresult,
    pub set_currentScrollRestorationIsManual: unsafe extern "C" fn (this: *const nsIDocShell, aCurrentScrollRestorationIsManual: bool) -> nsresult,

    /* [implicit_jscontext] jsval getOriginAttributes (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getOriginAttributes: *const ::libc::c_void,

    /* [implicit_jscontext] void setOriginAttributes (in jsval aAttrs); */
    /// Unable to call function as its signature contains a non-rust type
    pub setOriginAttributes: *const ::libc::c_void,

    /* readonly attribute nsIEditingSession editingSession; */
    pub get_editingSession: unsafe extern "C" fn (this: *const nsIDocShell, aEditingSession: *mut *const nsIEditingSession) -> nsresult,

    /* [binaryname(ScriptableTabChild)] readonly attribute nsITabChild tabChild; */
    pub get_ScriptableTabChild: unsafe extern "C" fn (this: *const nsIDocShell, aTabChild: *mut *const nsITabChild) -> nsresult,

    /* [noscript,nostdcall,notxpcom] TabChildRef GetTabChild (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetTabChild: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsICommandManager GetCommandManager (); */
    pub GetCommandManager: unsafe extern "C" fn (this: *const nsIDocShell) -> *const nsICommandManager,

    /* attribute unsigned long touchEventsOverride; */
    pub get_touchEventsOverride: unsafe extern "C" fn (this: *const nsIDocShell, aTouchEventsOverride: *mut libc::uint32_t) -> nsresult,
    pub set_touchEventsOverride: unsafe extern "C" fn (this: *const nsIDocShell, aTouchEventsOverride: libc::uint32_t) -> nsresult,

    /* [infallible] readonly attribute boolean isOnlyToplevelInTabGroup; */
    pub get_isOnlyToplevelInTabGroup: unsafe extern "C" fn (this: *const nsIDocShell, aIsOnlyToplevelInTabGroup: *mut bool) -> nsresult,

    /* [infallible] readonly attribute boolean awaitingLargeAlloc; */
    pub get_awaitingLargeAlloc: unsafe extern "C" fn (this: *const nsIDocShell, aAwaitingLargeAlloc: *mut bool) -> nsresult,

    /* attribute boolean useTrackingProtection; */
    pub get_useTrackingProtection: unsafe extern "C" fn (this: *const nsIDocShell, aUseTrackingProtection: *mut bool) -> nsresult,
    pub set_useTrackingProtection: unsafe extern "C" fn (this: *const nsIDocShell, aUseTrackingProtection: bool) -> nsresult,

    /* [noscript] void dispatchLocationChangeEvent (); */
    pub dispatchLocationChangeEvent: unsafe extern "C" fn (this: *const nsIDocShell) -> nsresult,

}


impl nsIDocShell {
    /* [noscript] void loadURI (in nsIURI uri, in nsIDocShellLoadInfo loadInfo, in unsigned long aLoadFlags, in boolean firstParty); */
    #[inline]
    pub unsafe fn loadURI(&self, uri: Option<&nsIURI>, loadInfo: Option<&nsIDocShellLoadInfo>, aLoadFlags: libc::uint32_t, firstParty: bool) -> Result<(), nsresult> {

        match ((*self.vtable).loadURI)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), loadInfo.map_or(::std::ptr::null(), |x| x as *const _), aLoadFlags, firstParty) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void loadStream (in nsIInputStream aStream, in nsIURI aURI, in ACString aContentType, in ACString aContentCharset, in nsIDocShellLoadInfo aLoadInfo); */
    #[inline]
    pub unsafe fn loadStream(&self, aStream: Option<&nsIInputStream>, aURI: Option<&nsIURI>, aContentType: &[u8], aContentCharset: &[u8], aLoadInfo: Option<&nsIDocShellLoadInfo>) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        let aContentCharset = nsCString::from(aContentCharset);
        match ((*self.vtable).loadStream)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), &*aContentType, &*aContentCharset, aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void internalLoad (in nsIURI aURI, in nsIURI aOriginalURI, in boolean aLoadReplace, in nsIURI aReferrer, in unsigned long aReferrerPolicy, in nsIPrincipal aTriggeringPrincipal, in nsIPrincipal aPrincipalToInherit, in uint32_t aFlags, in AString aWindowTarget, in string aTypeHint, in AString aFileName, in nsIInputStream aPostDataStream, in nsIInputStream aHeadersStream, in unsigned long aLoadFlags, in nsISHEntry aSHEntry, in boolean aFirstParty, in AString aSrcdoc, in nsIDocShell aSourceDocShell, in nsIURI aBaseURI, in boolean aCheckForPrerender, out nsIDocShell aDocShell, out nsIRequest aRequest); */
    #[inline]
    pub unsafe fn internalLoad(&self, aURI: Option<&nsIURI>, aOriginalURI: Option<&nsIURI>, aLoadReplace: bool, aReferrer: Option<&nsIURI>, aReferrerPolicy: libc::uint32_t, aTriggeringPrincipal: Option<&nsIPrincipal>, aPrincipalToInherit: Option<&nsIPrincipal>, aFlags: uint32_t, aWindowTarget: &[u16], aTypeHint: *const libc::c_char, aFileName: &[u16], aPostDataStream: Option<&nsIInputStream>, aHeadersStream: Option<&nsIInputStream>, aLoadFlags: libc::uint32_t, aSHEntry: Option<&nsISHEntry>, aFirstParty: bool, aSrcdoc: &[u16], aSourceDocShell: Option<&nsIDocShell>, aBaseURI: Option<&nsIURI>, aCheckForPrerender: bool) -> Result<(Option<RefPtr<nsIDocShell>>, Option<RefPtr<nsIRequest>>), nsresult> {
        let aWindowTarget = nsString::from(aWindowTarget);
        let aFileName = nsString::from(aFileName);
        let aSrcdoc = nsString::from(aSrcdoc);
        let mut aDocShell = GetterAddrefs::new();
        let mut aRequest = GetterAddrefs::new();
        match ((*self.vtable).internalLoad)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aOriginalURI.map_or(::std::ptr::null(), |x| x as *const _), aLoadReplace, aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aReferrerPolicy, aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aPrincipalToInherit.map_or(::std::ptr::null(), |x| x as *const _), aFlags, &*aWindowTarget, aTypeHint, &*aFileName, aPostDataStream.map_or(::std::ptr::null(), |x| x as *const _), aHeadersStream.map_or(::std::ptr::null(), |x| x as *const _), aLoadFlags, aSHEntry.map_or(::std::ptr::null(), |x| x as *const _), aFirstParty, &*aSrcdoc, aSourceDocShell.map_or(::std::ptr::null(), |x| x as *const _), aBaseURI.map_or(::std::ptr::null(), |x| x as *const _), aCheckForPrerender, aDocShell.ptr(), aRequest.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aDocShell.refptr(), aRequest.refptr()))
    }

    /* [implicit_jscontext] void addState (in jsval aData, in DOMString aTitle, in DOMString aURL, in boolean aReplace); */


    /* void createLoadInfo (out nsIDocShellLoadInfo loadInfo); */
    #[inline]
    pub unsafe fn createLoadInfo(&self, ) -> Result<Option<RefPtr<nsIDocShellLoadInfo>>, nsresult> {
        let mut loadInfo = GetterAddrefs::new();
        match ((*self.vtable).createLoadInfo)(self as *const _, loadInfo.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(loadInfo.refptr())
    }

    /* void prepareForNewContentModel (); */
    #[inline]
    pub unsafe fn prepareForNewContentModel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).prepareForNewContentModel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCurrentURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn setCurrentURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).setCurrentURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void firePageHideNotification (in boolean isUnload); */
    #[inline]
    pub unsafe fn firePageHideNotification(&self, isUnload: bool) -> Result<(), nsresult> {

        match ((*self.vtable).firePageHideNotification)(self as *const _, isUnload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute nsPresContext presContext; */


    /* [noscript,notxpcom] nsIPresShell GetPresShell (); */


    /* [noscript] readonly attribute nsIPresShell eldestPresShell; */


    /* readonly attribute nsIContentViewer contentViewer; */
    #[inline]
    pub unsafe fn get_contentViewer(&self, ) -> Result<Option<RefPtr<nsIContentViewer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentViewer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIDOMEventTarget chromeEventHandler; */
    #[inline]
    pub unsafe fn get_chromeEventHandler(&self, ) -> Result<Option<RefPtr<nsIDOMEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_chromeEventHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_chromeEventHandler(&self, aChromeEventHandler: Option<&nsIDOMEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).set_chromeEventHandler)(self as *const _, aChromeEventHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute DOMString customUserAgent; */
    #[inline]
    pub unsafe fn get_customUserAgent(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_customUserAgent)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_customUserAgent(&self, aCustomUserAgent: &[u16]) -> Result<(), nsresult> {
        let aCustomUserAgent = nsString::from(aCustomUserAgent);
        match ((*self.vtable).set_customUserAgent)(self as *const _, &*aCustomUserAgent) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowPlugins; */
    #[inline]
    pub unsafe fn get_allowPlugins(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowPlugins)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowPlugins(&self, aAllowPlugins: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowPlugins)(self as *const _, aAllowPlugins) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowJavascript; */
    #[inline]
    pub unsafe fn get_allowJavascript(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowJavascript)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowJavascript(&self, aAllowJavascript: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowJavascript)(self as *const _, aAllowJavascript) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowMetaRedirects; */
    #[inline]
    pub unsafe fn get_allowMetaRedirects(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowMetaRedirects)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowMetaRedirects(&self, aAllowMetaRedirects: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowMetaRedirects)(self as *const _, aAllowMetaRedirects) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowSubframes; */
    #[inline]
    pub unsafe fn get_allowSubframes(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowSubframes)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowSubframes(&self, aAllowSubframes: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowSubframes)(self as *const _, aAllowSubframes) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowImages; */
    #[inline]
    pub unsafe fn get_allowImages(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowImages)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowImages(&self, aAllowImages: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowImages)(self as *const _, aAllowImages) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean allowMedia; */
    #[inline]
    pub unsafe fn get_allowMedia(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowMedia)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowMedia(&self, aAllowMedia: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowMedia)(self as *const _, aAllowMedia) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowDNSPrefetch; */
    #[inline]
    pub unsafe fn get_allowDNSPrefetch(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowDNSPrefetch)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowDNSPrefetch(&self, aAllowDNSPrefetch: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowDNSPrefetch)(self as *const _, aAllowDNSPrefetch) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowWindowControl; */
    #[inline]
    pub unsafe fn get_allowWindowControl(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowWindowControl)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowWindowControl(&self, aAllowWindowControl: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowWindowControl)(self as *const _, aAllowWindowControl) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean allowContentRetargeting; */
    #[inline]
    pub unsafe fn get_allowContentRetargeting(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowContentRetargeting)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowContentRetargeting(&self, aAllowContentRetargeting: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowContentRetargeting)(self as *const _, aAllowContentRetargeting) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean allowContentRetargetingOnChildren; */
    #[inline]
    pub unsafe fn get_allowContentRetargetingOnChildren(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowContentRetargetingOnChildren)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowContentRetargetingOnChildren(&self, aAllowContentRetargetingOnChildren: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowContentRetargetingOnChildren)(self as *const _, aAllowContentRetargetingOnChildren) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute boolean inheritPrivateBrowsingId; */
    #[inline]
    pub unsafe fn get_inheritPrivateBrowsingId(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inheritPrivateBrowsingId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_inheritPrivateBrowsingId(&self, aInheritPrivateBrowsingId: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_inheritPrivateBrowsingId)(self as *const _, aInheritPrivateBrowsingId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getDocShellEnumerator (in long aItemType, in long aDirection); */
    #[inline]
    pub unsafe fn getDocShellEnumerator(&self, aItemType: libc::int32_t, aDirection: libc::int32_t) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getDocShellEnumerator)(self as *const _, aItemType, aDirection, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute unsigned long appType; */
    #[inline]
    pub unsafe fn get_appType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_appType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_appType(&self, aAppType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_appType)(self as *const _, aAppType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean allowAuth; */
    #[inline]
    pub unsafe fn get_allowAuth(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allowAuth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allowAuth(&self, aAllowAuth: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allowAuth)(self as *const _, aAllowAuth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float zoom; */
    #[inline]
    pub unsafe fn get_zoom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_zoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_zoom(&self, aZoom: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_zoom)(self as *const _, aZoom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long marginWidth; */
    #[inline]
    pub unsafe fn get_marginWidth(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_marginWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginWidth(&self, aMarginWidth: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginWidth)(self as *const _, aMarginWidth) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute long marginHeight; */
    #[inline]
    pub unsafe fn get_marginHeight(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_marginHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_marginHeight(&self, aMarginHeight: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_marginHeight)(self as *const _, aMarginHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool tabToTreeOwner (in boolean forward, in boolean forDocumentNavigation); */
    #[inline]
    pub unsafe fn tabToTreeOwner(&self, forward: bool, forDocumentNavigation: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).tabToTreeOwner)(self as *const _, forward, forDocumentNavigation, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long busyFlags; */
    #[inline]
    pub unsafe fn get_busyFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_busyFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute unsigned long loadType; */
    #[inline]
    pub unsafe fn get_loadType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_loadType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadType(&self, aLoadType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadType)(self as *const _, aLoadType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsLoadFlags defaultLoadFlags; */
    #[inline]
    pub unsafe fn get_defaultLoadFlags(&self, ) -> Result<nsLoadFlags, nsresult> {
        let mut _retval: nsLoadFlags = ::std::mem::zeroed();
        match ((*self.vtable).get_defaultLoadFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_defaultLoadFlags(&self, aDefaultLoadFlags: nsLoadFlags) -> Result<(), nsresult> {

        match ((*self.vtable).set_defaultLoadFlags)(self as *const _, aDefaultLoadFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isBeingDestroyed (); */
    #[inline]
    pub unsafe fn isBeingDestroyed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isBeingDestroyed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isExecutingOnLoadHandler; */
    #[inline]
    pub unsafe fn get_isExecutingOnLoadHandler(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isExecutingOnLoadHandler)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsILayoutHistoryState layoutHistoryState; */
    #[inline]
    pub unsafe fn get_layoutHistoryState(&self, ) -> Result<Option<RefPtr<nsILayoutHistoryState>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_layoutHistoryState)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_layoutHistoryState(&self, aLayoutHistoryState: Option<&nsILayoutHistoryState>) -> Result<(), nsresult> {

        match ((*self.vtable).set_layoutHistoryState)(self as *const _, aLayoutHistoryState.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean shouldSaveLayoutState; */
    #[inline]
    pub unsafe fn get_shouldSaveLayoutState(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_shouldSaveLayoutState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsISecureBrowserUI securityUI; */
    #[inline]
    pub unsafe fn get_securityUI(&self, ) -> Result<Option<RefPtr<nsISecureBrowserUI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityUI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_securityUI(&self, aSecurityUI: Option<&nsISecureBrowserUI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_securityUI)(self as *const _, aSecurityUI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void suspendRefreshURIs (); */
    #[inline]
    pub unsafe fn suspendRefreshURIs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).suspendRefreshURIs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumeRefreshURIs (); */
    #[inline]
    pub unsafe fn resumeRefreshURIs(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resumeRefreshURIs)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void beginRestore (in nsIContentViewer viewer, in boolean top); */
    #[inline]
    pub unsafe fn beginRestore(&self, viewer: Option<&nsIContentViewer>, top: bool) -> Result<(), nsresult> {

        match ((*self.vtable).beginRestore)(self as *const _, viewer.map_or(::std::ptr::null(), |x| x as *const _), top) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishRestore (); */
    #[inline]
    pub unsafe fn finishRestore(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finishRestore)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean restoringDocument; */
    #[inline]
    pub unsafe fn get_restoringDocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_restoringDocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean useErrorPages; */
    #[inline]
    pub unsafe fn get_useErrorPages(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useErrorPages)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useErrorPages(&self, aUseErrorPages: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_useErrorPages)(self as *const _, aUseErrorPages) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean displayLoadError (in nsresult aError, in nsIURI aURI, in wstring aURL, [optional] in nsIChannel aFailedChannel); */
    #[inline]
    pub unsafe fn displayLoadError(&self, aError: nsresult, aURI: Option<&nsIURI>, aURL: *const libc::int16_t, aFailedChannel: Option<&nsIChannel>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).displayLoadError)(self as *const _, aError, aURI.map_or(::std::ptr::null(), |x| x as *const _), aURL, aFailedChannel.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIChannel failedChannel; */
    #[inline]
    pub unsafe fn get_failedChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_failedChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long previousTransIndex; */
    #[inline]
    pub unsafe fn get_previousTransIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_previousTransIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long loadedTransIndex; */
    #[inline]
    pub unsafe fn get_loadedTransIndex(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_loadedTransIndex)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void historyPurged (in long numEntries); */
    #[inline]
    pub unsafe fn historyPurged(&self, numEntries: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).historyPurged)(self as *const _, numEntries) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIChannel currentDocumentChannel; */
    #[inline]
    pub unsafe fn get_currentDocumentChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_currentDocumentChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void setChildOffset (in unsigned long offset); */
    #[inline]
    pub unsafe fn setChildOffset(&self, offset: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setChildOffset)(self as *const _, offset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean isInUnload; */
    #[inline]
    pub unsafe fn get_isInUnload(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInUnload)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean channelIsUnsafe; */
    #[inline]
    pub unsafe fn get_channelIsUnsafe(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_channelIsUnsafe)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasMixedActiveContentLoaded; */
    #[inline]
    pub unsafe fn get_hasMixedActiveContentLoaded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasMixedActiveContentLoaded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasMixedActiveContentBlocked; */
    #[inline]
    pub unsafe fn get_hasMixedActiveContentBlocked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasMixedActiveContentBlocked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasMixedDisplayContentLoaded; */
    #[inline]
    pub unsafe fn get_hasMixedDisplayContentLoaded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasMixedDisplayContentLoaded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasMixedDisplayContentBlocked; */
    #[inline]
    pub unsafe fn get_hasMixedDisplayContentBlocked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasMixedDisplayContentBlocked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasTrackingContentBlocked; */
    #[inline]
    pub unsafe fn get_hasTrackingContentBlocked(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasTrackingContentBlocked)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean hasTrackingContentLoaded; */
    #[inline]
    pub unsafe fn get_hasTrackingContentLoaded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasTrackingContentLoaded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] void DetachEditorFromWindow (); */
    #[inline]
    pub unsafe fn DetachEditorFromWindow(&self, ) -> () {

        let _retval = ((*self.vtable).DetachEditorFromWindow)(self as *const _, );
        ()
    }

    /* attribute boolean isOffScreenBrowser; */
    #[inline]
    pub unsafe fn get_isOffScreenBrowser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOffScreenBrowser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isOffScreenBrowser(&self, aIsOffScreenBrowser: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isOffScreenBrowser)(self as *const _, aIsOffScreenBrowser) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIWebBrowserPrint printPreview; */
    #[inline]
    pub unsafe fn get_printPreview(&self, ) -> Result<Option<RefPtr<nsIWebBrowserPrint>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_printPreview)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [infallible] readonly attribute boolean canExecuteScripts; */
    #[inline]
    pub unsafe fn get_canExecuteScripts(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_canExecuteScripts)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean isActive; */
    #[inline]
    pub unsafe fn get_isActive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isActive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isActive(&self, aIsActive: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isActive)(self as *const _, aIsActive) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void SetIsPrerendered (); */
    #[inline]
    pub unsafe fn SetIsPrerendered(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SetIsPrerendered)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isPrerendered; */
    #[inline]
    pub unsafe fn get_isPrerendered(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isPrerendered)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIDPtr historyID; */
    #[inline]
    pub unsafe fn get_historyID(&self, ) -> Result<*const nsID, nsresult> {
        let mut _retval: *const nsID = ::std::ptr::null();
        match ((*self.vtable).get_historyID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] nsID HistoryID (); */


    /* attribute boolean isAppTab; */
    #[inline]
    pub unsafe fn get_isAppTab(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isAppTab)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isAppTab(&self, aIsAppTab: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isAppTab)(self as *const _, aIsAppTab) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void createAboutBlankContentViewer (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn createAboutBlankContentViewer(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).createAboutBlankContentViewer)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void forceCreateAboutBlankContentViewer (in nsIPrincipal aPrincipal); */
    #[inline]
    pub unsafe fn forceCreateAboutBlankContentViewer(&self, aPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).forceCreateAboutBlankContentViewer)(self as *const _, aPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString charset; */
    #[inline]
    pub unsafe fn get_charset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_charset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_charset(&self, aCharset: &[u8]) -> Result<(), nsresult> {
        let aCharset = nsCString::from(aCharset);
        match ((*self.vtable).set_charset)(self as *const _, &*aCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void gatherCharsetMenuTelemetry (); */
    #[inline]
    pub unsafe fn gatherCharsetMenuTelemetry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).gatherCharsetMenuTelemetry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString forcedCharset; */
    #[inline]
    pub unsafe fn get_forcedCharset(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_forcedCharset)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_forcedCharset(&self, aForcedCharset: &[u8]) -> Result<(), nsresult> {
        let aForcedCharset = nsCString::from(aForcedCharset);
        match ((*self.vtable).set_forcedCharset)(self as *const _, &*aForcedCharset) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] void setParentCharset (in ACString parentCharset, in int32_t parentCharsetSource, in nsIPrincipal parentCharsetPrincipal); */
    #[inline]
    pub unsafe fn setParentCharset(&self, parentCharset: &[u8], parentCharsetSource: int32_t, parentCharsetPrincipal: Option<&nsIPrincipal>) -> () {
        let parentCharset = nsCString::from(parentCharset);
        let _retval = ((*self.vtable).setParentCharset)(self as *const _, &*parentCharset, parentCharsetSource, parentCharsetPrincipal.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

    /* [noscript,nostdcall,notxpcom] void getParentCharset (out ACString parentCharset, out int32_t parentCharsetSource, out nsIPrincipal parentCharsetPrincipal); */
    #[inline]
    pub unsafe fn getParentCharset(&self, ) -> (nsCString, int32_t, Option<RefPtr<nsIPrincipal>>) {
        let mut parentCharset = nsCString::new();
        let mut parentCharsetSource: int32_t = ::std::mem::zeroed();
        let mut parentCharsetPrincipal = GetterAddrefs::new();
        let _retval = ((*self.vtable).getParentCharset)(self as *const _, &mut *parentCharset, &mut parentCharsetSource as *mut _, parentCharsetPrincipal.ptr());
        (parentCharset, parentCharsetSource, parentCharsetPrincipal.refptr())
    }

    /* [infallible] attribute boolean recordProfileTimelineMarkers; */
    #[inline]
    pub unsafe fn get_recordProfileTimelineMarkers(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_recordProfileTimelineMarkers)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_recordProfileTimelineMarkers(&self, aRecordProfileTimelineMarkers: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_recordProfileTimelineMarkers)(self as *const _, aRecordProfileTimelineMarkers) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* DOMHighResTimeStamp now (); */
    #[inline]
    pub unsafe fn now(&self, ) -> Result<DOMHighResTimeStamp, nsresult> {
        let mut _retval: DOMHighResTimeStamp = ::std::mem::zeroed();
        match ((*self.vtable).now)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [implicit_jscontext] jsval popProfileTimelineMarkers (); */


    /* void addWeakPrivacyTransitionObserver (in nsIPrivacyTransitionObserver obs); */
    #[inline]
    pub unsafe fn addWeakPrivacyTransitionObserver(&self, obs: Option<&nsIPrivacyTransitionObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addWeakPrivacyTransitionObserver)(self as *const _, obs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addWeakReflowObserver (in nsIReflowObserver obs); */
    #[inline]
    pub unsafe fn addWeakReflowObserver(&self, obs: Option<&nsIReflowObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addWeakReflowObserver)(self as *const _, obs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWeakReflowObserver (in nsIReflowObserver obs); */
    #[inline]
    pub unsafe fn removeWeakReflowObserver(&self, obs: Option<&nsIReflowObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWeakReflowObserver)(self as *const _, obs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void notifyReflowObservers (in bool interruptible, in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
    #[inline]
    pub unsafe fn notifyReflowObservers(&self, interruptible: bool, start: DOMHighResTimeStamp, end: DOMHighResTimeStamp) -> Result<(), nsresult> {

        match ((*self.vtable).notifyReflowObservers)(self as *const _, interruptible, start, end) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void addWeakScrollObserver (in nsIScrollObserver obs); */
    #[inline]
    pub unsafe fn addWeakScrollObserver(&self, obs: Option<&nsIScrollObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addWeakScrollObserver)(self as *const _, obs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void removeWeakScrollObserver (in nsIScrollObserver obs); */
    #[inline]
    pub unsafe fn removeWeakScrollObserver(&self, obs: Option<&nsIScrollObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWeakScrollObserver)(self as *const _, obs.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void notifyScrollObservers (); */
    #[inline]
    pub unsafe fn notifyScrollObservers(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyScrollObservers)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] attribute unsigned long frameType; */
    #[inline]
    pub unsafe fn get_frameType(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_frameType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_frameType(&self, aFrameType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_frameType)(self as *const _, aFrameType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isMozBrowser; */
    #[inline]
    pub unsafe fn get_isMozBrowser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isMozBrowser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isIsolatedMozBrowserElement; */
    #[inline]
    pub unsafe fn get_isIsolatedMozBrowserElement(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isIsolatedMozBrowserElement)(self as *const _, &mut _retval as *mut _) {
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

    /* [infallible] readonly attribute boolean isInMozBrowser; */
    #[inline]
    pub unsafe fn get_isInMozBrowser(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isInMozBrowser)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean isTopLevelContentDocShell; */
    #[inline]
    pub unsafe fn get_isTopLevelContentDocShell(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTopLevelContentDocShell)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIDocShell getSameTypeParentIgnoreBrowserBoundaries (); */
    #[inline]
    pub unsafe fn getSameTypeParentIgnoreBrowserBoundaries(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSameTypeParentIgnoreBrowserBoundaries)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDocShell getSameTypeRootTreeItemIgnoreBrowserBoundaries (); */
    #[inline]
    pub unsafe fn getSameTypeRootTreeItemIgnoreBrowserBoundaries(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSameTypeRootTreeItemIgnoreBrowserBoundaries)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute bool asyncPanZoomEnabled; */
    #[inline]
    pub unsafe fn get_asyncPanZoomEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_asyncPanZoomEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute unsigned long sandboxFlags; */
    #[inline]
    pub unsafe fn get_sandboxFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sandboxFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sandboxFlags(&self, aSandboxFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sandboxFlags)(self as *const _, aSandboxFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDocShell onePermittedSandboxedNavigator; */
    #[inline]
    pub unsafe fn get_onePermittedSandboxedNavigator(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_onePermittedSandboxedNavigator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_onePermittedSandboxedNavigator(&self, aOnePermittedSandboxedNavigator: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).set_onePermittedSandboxedNavigator)(self as *const _, aOnePermittedSandboxedNavigator.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] bool isSandboxedFrom (in nsIDocShell aTargetDocShell); */
    #[inline]
    pub unsafe fn isSandboxedFrom(&self, aTargetDocShell: Option<&nsIDocShell>) -> bool {

        let _retval = ((*self.vtable).isSandboxedFrom)(self as *const _, aTargetDocShell.map_or(::std::ptr::null(), |x| x as *const _));
        _retval
    }

    /* attribute nsIChannel mixedContentChannel; */
    #[inline]
    pub unsafe fn get_mixedContentChannel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_mixedContentChannel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_mixedContentChannel(&self, aMixedContentChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).set_mixedContentChannel)(self as *const _, aMixedContentChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void GetAllowMixedContentAndConnectionData (out boolean rootHasSecureConnection, out boolean allowMixedContent, out boolean isRootDocShell); */
    #[inline]
    pub unsafe fn GetAllowMixedContentAndConnectionData(&self, ) -> Result<(bool, bool, bool), nsresult> {
        let mut rootHasSecureConnection: bool = ::std::mem::zeroed();
        let mut allowMixedContent: bool = ::std::mem::zeroed();
        let mut isRootDocShell: bool = ::std::mem::zeroed();
        match ((*self.vtable).GetAllowMixedContentAndConnectionData)(self as *const _, &mut rootHasSecureConnection as *mut _, &mut allowMixedContent as *mut _, &mut isRootDocShell as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((rootHasSecureConnection, allowMixedContent, isRootDocShell))
    }

    /* [noscript,notxpcom] bool pluginsAllowedInCurrentDoc (); */
    #[inline]
    pub unsafe fn pluginsAllowedInCurrentDoc(&self, ) -> bool {

        let _retval = ((*self.vtable).pluginsAllowedInCurrentDoc)(self as *const _, );
        _retval
    }

    /* [infallible] readonly attribute boolean fullscreenAllowed; */
    #[inline]
    pub unsafe fn get_fullscreenAllowed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_fullscreenAllowed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setFullscreenAllowed (in boolean allowed); */
    #[inline]
    pub unsafe fn setFullscreenAllowed(&self, allowed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setFullscreenAllowed)(self as *const _, allowed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] uint32_t orientationLock (); */
    #[inline]
    pub unsafe fn orientationLock(&self, ) -> uint32_t {

        let _retval = ((*self.vtable).orientationLock)(self as *const _, );
        _retval
    }

    /* [notxpcom] void setOrientationLock (in uint32_t orientationLock); */
    #[inline]
    pub unsafe fn setOrientationLock(&self, orientationLock: uint32_t) -> () {

        let _retval = ((*self.vtable).setOrientationLock)(self as *const _, orientationLock);
        ()
    }

    /* [infallible,noscript] attribute boolean affectPrivateSessionLifetime; */
    #[inline]
    pub unsafe fn get_affectPrivateSessionLifetime(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_affectPrivateSessionLifetime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_affectPrivateSessionLifetime(&self, aAffectPrivateSessionLifetime: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_affectPrivateSessionLifetime)(self as *const _, aAffectPrivateSessionLifetime) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean mayEnableCharacterEncodingMenu; */
    #[inline]
    pub unsafe fn get_mayEnableCharacterEncodingMenu(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_mayEnableCharacterEncodingMenu)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIEditor editor; */
    #[inline]
    pub unsafe fn get_editor(&self, ) -> Result<Option<RefPtr<nsIEditor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_editor)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_editor(&self, aEditor: Option<&nsIEditor>) -> Result<(), nsresult> {

        match ((*self.vtable).set_editor)(self as *const _, aEditor.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean editable; */
    #[inline]
    pub unsafe fn get_editable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_editable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean hasEditingSession; */
    #[inline]
    pub unsafe fn get_hasEditingSession(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasEditingSession)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void makeEditable (in boolean inWaitForUriLoad); */
    #[inline]
    pub unsafe fn makeEditable(&self, inWaitForUriLoad: bool) -> Result<(), nsresult> {

        match ((*self.vtable).makeEditable)(self as *const _, inWaitForUriLoad) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISHEntry getChildSHEntry (in long aChildOffset); */
    #[inline]
    pub unsafe fn getChildSHEntry(&self, aChildOffset: libc::int32_t) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChildSHEntry)(self as *const _, aChildOffset, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addChildSHEntry (in nsISHEntry aCloneReference, in nsISHEntry aHistoryEntry, in long aChildOffset, in unsigned long aLoadType, in boolean aCloneChilden); */
    #[inline]
    pub unsafe fn addChildSHEntry(&self, aCloneReference: Option<&nsISHEntry>, aHistoryEntry: Option<&nsISHEntry>, aChildOffset: libc::int32_t, aLoadType: libc::uint32_t, aCloneChilden: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addChildSHEntry)(self as *const _, aCloneReference.map_or(::std::ptr::null(), |x| x as *const _), aHistoryEntry.map_or(::std::ptr::null(), |x| x as *const _), aChildOffset, aLoadType, aCloneChilden) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean useGlobalHistory; */
    #[inline]
    pub unsafe fn get_useGlobalHistory(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useGlobalHistory)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useGlobalHistory(&self, aUseGlobalHistory: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_useGlobalHistory)(self as *const _, aUseGlobalHistory) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeFromSessionHistory (); */
    #[inline]
    pub unsafe fn removeFromSessionHistory(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).removeFromSessionHistory)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean createdDynamically; */
    #[inline]
    pub unsafe fn get_createdDynamically(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_createdDynamically)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_createdDynamically(&self, aCreatedDynamically: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_createdDynamically)(self as *const _, aCreatedDynamically) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean getCurrentSHEntry (out nsISHEntry aEntry); */
    #[inline]
    pub unsafe fn getCurrentSHEntry(&self, ) -> Result<(Option<RefPtr<nsISHEntry>>, bool), nsresult> {
        let mut aEntry = GetterAddrefs::new();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).getCurrentSHEntry)(self as *const _, aEntry.ptr(), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aEntry.refptr(), _retval))
    }

    /* boolean isCommandEnabled (in string command); */
    #[inline]
    pub unsafe fn isCommandEnabled(&self, command: *const libc::c_char) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandEnabled)(self as *const _, command, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void doCommand (in string command); */
    #[inline]
    pub unsafe fn doCommand(&self, command: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).doCommand)(self as *const _, command) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommandWithParams (in string command, in nsICommandParams aParams); */
    #[inline]
    pub unsafe fn doCommandWithParams(&self, command: *const libc::c_char, aParams: Option<&nsICommandParams>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommandWithParams)(self as *const _, command, aParams.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] bool IsInvisible (); */
    #[inline]
    pub unsafe fn IsInvisible(&self, ) -> bool {

        let _retval = ((*self.vtable).IsInvisible)(self as *const _, );
        _retval
    }

    /* [noscript,notxpcom] void SetInvisible (in bool aIsInvisibleDochsell); */
    #[inline]
    pub unsafe fn SetInvisible(&self, aIsInvisibleDochsell: bool) -> () {

        let _retval = ((*self.vtable).SetInvisible)(self as *const _, aIsInvisibleDochsell);
        ()
    }

    /* [noscript,nostdcall,notxpcom] nsIScriptGlobalObject GetScriptGlobalObject (); */
    #[inline]
    pub unsafe fn GetScriptGlobalObject(&self, ) -> Option<RefPtr<nsIScriptGlobalObject>> {

        let _retval = ((*self.vtable).GetScriptGlobalObject)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* [infallible] attribute boolean deviceSizeIsPageSize; */
    #[inline]
    pub unsafe fn get_deviceSizeIsPageSize(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_deviceSizeIsPageSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_deviceSizeIsPageSize(&self, aDeviceSizeIsPageSize: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_deviceSizeIsPageSize)(self as *const _, aDeviceSizeIsPageSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] void setOpener (in nsITabParent aOpener); */
    #[inline]
    pub unsafe fn setOpener(&self, aOpener: Option<&nsITabParent>) -> () {

        let _retval = ((*self.vtable).setOpener)(self as *const _, aOpener.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

    /* [noscript,nostdcall,notxpcom] nsITabParent getOpener (); */
    #[inline]
    pub unsafe fn getOpener(&self, ) -> Option<RefPtr<nsITabParent>> {

        let _retval = ((*self.vtable).getOpener)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStart (in string aReason, in wstring functionName, in wstring fileName, in unsigned long lineNumber, in jsval asyncStack, in string asyncCause); */


    /* [noscript,nostdcall,notxpcom] void notifyJSRunToCompletionStop (); */
    #[inline]
    pub unsafe fn notifyJSRunToCompletionStop(&self, ) -> () {

        let _retval = ((*self.vtable).notifyJSRunToCompletionStop)(self as *const _, );
        ()
    }

    /* [infallible] readonly attribute boolean hasLoadedNonBlankURI; */
    #[inline]
    pub unsafe fn get_hasLoadedNonBlankURI(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasLoadedNonBlankURI)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean windowDraggingAllowed; */
    #[inline]
    pub unsafe fn get_windowDraggingAllowed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_windowDraggingAllowed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_windowDraggingAllowed(&self, aWindowDraggingAllowed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_windowDraggingAllowed)(self as *const _, aWindowDraggingAllowed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean currentScrollRestorationIsManual; */
    #[inline]
    pub unsafe fn get_currentScrollRestorationIsManual(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_currentScrollRestorationIsManual)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_currentScrollRestorationIsManual(&self, aCurrentScrollRestorationIsManual: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_currentScrollRestorationIsManual)(self as *const _, aCurrentScrollRestorationIsManual) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [implicit_jscontext] jsval getOriginAttributes (); */


    /* [implicit_jscontext] void setOriginAttributes (in jsval aAttrs); */


    /* readonly attribute nsIEditingSession editingSession; */
    #[inline]
    pub unsafe fn get_editingSession(&self, ) -> Result<Option<RefPtr<nsIEditingSession>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_editingSession)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [binaryname(ScriptableTabChild)] readonly attribute nsITabChild tabChild; */
    #[inline]
    pub unsafe fn get_ScriptableTabChild(&self, ) -> Result<Option<RefPtr<nsITabChild>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ScriptableTabChild)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,nostdcall,notxpcom] TabChildRef GetTabChild (); */


    /* [noscript,nostdcall,notxpcom] nsICommandManager GetCommandManager (); */
    #[inline]
    pub unsafe fn GetCommandManager(&self, ) -> Option<RefPtr<nsICommandManager>> {

        let _retval = ((*self.vtable).GetCommandManager)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* attribute unsigned long touchEventsOverride; */
    #[inline]
    pub unsafe fn get_touchEventsOverride(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_touchEventsOverride)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_touchEventsOverride(&self, aTouchEventsOverride: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_touchEventsOverride)(self as *const _, aTouchEventsOverride) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [infallible] readonly attribute boolean isOnlyToplevelInTabGroup; */
    #[inline]
    pub unsafe fn get_isOnlyToplevelInTabGroup(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isOnlyToplevelInTabGroup)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] readonly attribute boolean awaitingLargeAlloc; */
    #[inline]
    pub unsafe fn get_awaitingLargeAlloc(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_awaitingLargeAlloc)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean useTrackingProtection; */
    #[inline]
    pub unsafe fn get_useTrackingProtection(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_useTrackingProtection)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_useTrackingProtection(&self, aUseTrackingProtection: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_useTrackingProtection)(self as *const _, aUseTrackingProtection) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void dispatchLocationChangeEvent (); */
    #[inline]
    pub unsafe fn dispatchLocationChangeEvent(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).dispatchLocationChangeEvent)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


