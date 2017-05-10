//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentViewer.idl
//


pub mod nsIContentViewer_consts {
    pub const eDelayResize: i64 = 1;
}


#[repr(C)]
pub struct nsIContentViewer {
    vtable: *const nsIContentViewerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentViewer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2da17016, 0x7851, 0x4a45,
            [0xa7, 0xa8, 0x00, 0xb3, 0x60, 0xe0, 0x15, 0x95])
    }
}

unsafe impl RefCounted for nsIContentViewer {
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
pub trait nsIContentViewerCoerce {
    fn coerce_from(v: &nsIContentViewer) -> &Self;
}

impl nsIContentViewerCoerce for nsIContentViewer {
    #[inline]
    fn coerce_from(v: &nsIContentViewer) -> &Self {
        v
    }
}

impl nsIContentViewer {
    #[inline]
    pub fn coerce<T: nsIContentViewerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentViewer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIContentViewerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentViewerVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void init (in nsIWidgetPtr aParentWidget, [const] in nsIntRectRef aBounds); */
    /// Unable to call function as its signature contains a non-rust type
    pub init: *const ::libc::c_void,

    /* attribute nsIDocShell container; */
    pub get_container: unsafe extern "C" fn (this: *const nsIContentViewer, aContainer: *mut *const nsIDocShell) -> nsresult,
    pub set_container: unsafe extern "C" fn (this: *const nsIContentViewer, aContainer: *const nsIDocShell) -> nsresult,

    /* [noscript,nostdcall,notxpcom] void loadStart (in nsIDocument aDoc); */
    pub loadStart: unsafe extern "C" fn (this: *const nsIContentViewer, aDoc: *const nsIDocument) -> libc::c_void,

    /* void loadComplete (in nsresult aStatus); */
    pub loadComplete: unsafe extern "C" fn (this: *const nsIContentViewer, aStatus: nsresult) -> nsresult,

    /* [noscript] readonly attribute boolean loadCompleted; */
    pub get_loadCompleted: unsafe extern "C" fn (this: *const nsIContentViewer, aLoadCompleted: *mut bool) -> nsresult,

    /* [noscript] readonly attribute boolean isStopped; */
    pub get_isStopped: unsafe extern "C" fn (this: *const nsIContentViewer, aIsStopped: *mut bool) -> nsresult,

    /* boolean permitUnload (); */
    pub permitUnload: unsafe extern "C" fn (this: *const nsIContentViewer, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean inPermitUnload; */
    pub get_inPermitUnload: unsafe extern "C" fn (this: *const nsIContentViewer, aInPermitUnload: *mut bool) -> nsresult,

    /* [noscript,nostdcall] boolean permitUnloadInternal (inout boolean aShouldPrompt); */
    pub permitUnloadInternal: unsafe extern "C" fn (this: *const nsIContentViewer, aShouldPrompt: *mut bool, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean beforeUnloadFiring; */
    pub get_beforeUnloadFiring: unsafe extern "C" fn (this: *const nsIContentViewer, aBeforeUnloadFiring: *mut bool) -> nsresult,

    /* void pageHide (in boolean isUnload); */
    pub pageHide: unsafe extern "C" fn (this: *const nsIContentViewer, isUnload: bool) -> nsresult,

    /* void close (in nsISHEntry historyEntry); */
    pub close: unsafe extern "C" fn (this: *const nsIContentViewer, historyEntry: *const nsISHEntry) -> nsresult,

    /* void destroy (); */
    pub destroy: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* void stop (); */
    pub stop: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* attribute nsIDOMDocument DOMDocument; */
    pub get_DOMDocument: unsafe extern "C" fn (this: *const nsIContentViewer, aDOMDocument: *mut *const nsIDOMDocument) -> nsresult,
    pub set_DOMDocument: unsafe extern "C" fn (this: *const nsIContentViewer, aDOMDocument: *const nsIDOMDocument) -> nsresult,

    /* [noscript,notxpcom] nsIDocument getDocument (); */
    pub getDocument: unsafe extern "C" fn (this: *const nsIContentViewer) -> *const nsIDocument,

    /* [noscript] void getBounds (in nsIntRectRef aBounds); */
    /// Unable to call function as its signature contains a non-rust type
    pub getBounds: *const ::libc::c_void,

    /* [noscript] void setBounds ([const] in nsIntRectRef aBounds); */
    /// Unable to call function as its signature contains a non-rust type
    pub setBounds: *const ::libc::c_void,

    /* [noscript] void setBoundsWithFlags ([const] in nsIntRectRef aBounds, in unsigned long aFlags); */
    /// Unable to call function as its signature contains a non-rust type
    pub setBoundsWithFlags: *const ::libc::c_void,

    /* [noscript] attribute nsIContentViewer previousViewer; */
    pub get_previousViewer: unsafe extern "C" fn (this: *const nsIContentViewer, aPreviousViewer: *mut *const nsIContentViewer) -> nsresult,
    pub set_previousViewer: unsafe extern "C" fn (this: *const nsIContentViewer, aPreviousViewer: *const nsIContentViewer) -> nsresult,

    /* void move (in long aX, in long aY); */
    pub move_: unsafe extern "C" fn (this: *const nsIContentViewer, aX: libc::int32_t, aY: libc::int32_t) -> nsresult,

    /* void show (); */
    pub show: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* void hide (); */
    pub hide: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* attribute boolean sticky; */
    pub get_sticky: unsafe extern "C" fn (this: *const nsIContentViewer, aSticky: *mut bool) -> nsresult,
    pub set_sticky: unsafe extern "C" fn (this: *const nsIContentViewer, aSticky: bool) -> nsresult,

    /* boolean requestWindowClose (); */
    pub requestWindowClose: unsafe extern "C" fn (this: *const nsIContentViewer, _retval: *mut bool) -> nsresult,

    /* void open (in nsISupports aState, in nsISHEntry aSHEntry); */
    pub open: unsafe extern "C" fn (this: *const nsIContentViewer, aState: *const nsISupports, aSHEntry: *const nsISHEntry) -> nsresult,

    /* void clearHistoryEntry (); */
    pub clearHistoryEntry: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* void setPageMode (in boolean aPageMode, in nsIPrintSettings aPrintSettings); */
    pub setPageMode: unsafe extern "C" fn (this: *const nsIContentViewer, aPageMode: bool, aPrintSettings: *const nsIPrintSettings) -> nsresult,

    /* readonly attribute nsISHEntry historyEntry; */
    pub get_historyEntry: unsafe extern "C" fn (this: *const nsIContentViewer, aHistoryEntry: *mut *const nsISHEntry) -> nsresult,

    /* readonly attribute boolean isTabModalPromptAllowed; */
    pub get_isTabModalPromptAllowed: unsafe extern "C" fn (this: *const nsIContentViewer, aIsTabModalPromptAllowed: *mut bool) -> nsresult,

    /* attribute boolean isHidden; */
    pub get_isHidden: unsafe extern "C" fn (this: *const nsIContentViewer, aIsHidden: *mut bool) -> nsresult,
    pub set_isHidden: unsafe extern "C" fn (this: *const nsIContentViewer, aIsHidden: bool) -> nsresult,

    /* [noscript] readonly attribute nsIPresShellPtr presShell; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_presShell: *const ::libc::c_void,

    /* [noscript] readonly attribute nsPresContextPtr presContext; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_presContext: *const ::libc::c_void,

    /* [noscript] void setDocumentInternal (in nsIDocument aDocument, in boolean aForceReuseInnerWindow); */
    pub setDocumentInternal: unsafe extern "C" fn (this: *const nsIContentViewer, aDocument: *const nsIDocument, aForceReuseInnerWindow: bool) -> nsresult,

    /* [noscript,nostdcall,notxpcom] nsViewPtr findContainerView (); */
    /// Unable to call function as its signature contains a non-rust type
    pub findContainerView: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setNavigationTiming (in nsDOMNavigationTimingPtr aTiming); */
    /// Unable to call function as its signature contains a non-rust type
    pub setNavigationTiming: *const ::libc::c_void,

    /* void scrollToNode (in nsIDOMNode node); */
    pub scrollToNode: unsafe extern "C" fn (this: *const nsIContentViewer, node: *const nsIDOMNode) -> nsresult,

    /* attribute float textZoom; */
    pub get_textZoom: unsafe extern "C" fn (this: *const nsIContentViewer, aTextZoom: *mut libc::c_float) -> nsresult,
    pub set_textZoom: unsafe extern "C" fn (this: *const nsIContentViewer, aTextZoom: libc::c_float) -> nsresult,

    /* readonly attribute float effectiveTextZoom; */
    pub get_effectiveTextZoom: unsafe extern "C" fn (this: *const nsIContentViewer, aEffectiveTextZoom: *mut libc::c_float) -> nsresult,

    /* attribute float fullZoom; */
    pub get_fullZoom: unsafe extern "C" fn (this: *const nsIContentViewer, aFullZoom: *mut libc::c_float) -> nsresult,
    pub set_fullZoom: unsafe extern "C" fn (this: *const nsIContentViewer, aFullZoom: libc::c_float) -> nsresult,

    /* attribute float overrideDPPX; */
    pub get_overrideDPPX: unsafe extern "C" fn (this: *const nsIContentViewer, aOverrideDPPX: *mut libc::c_float) -> nsresult,
    pub set_overrideDPPX: unsafe extern "C" fn (this: *const nsIContentViewer, aOverrideDPPX: libc::c_float) -> nsresult,

    /* attribute boolean authorStyleDisabled; */
    pub get_authorStyleDisabled: unsafe extern "C" fn (this: *const nsIContentViewer, aAuthorStyleDisabled: *mut bool) -> nsresult,
    pub set_authorStyleDisabled: unsafe extern "C" fn (this: *const nsIContentViewer, aAuthorStyleDisabled: bool) -> nsresult,

    /* attribute ACString forceCharacterSet; */
    pub get_forceCharacterSet: unsafe extern "C" fn (this: *const nsIContentViewer, aForceCharacterSet: *mut nsACString) -> nsresult,
    pub set_forceCharacterSet: unsafe extern "C" fn (this: *const nsIContentViewer, aForceCharacterSet: *const nsACString) -> nsresult,

    /* attribute ACString hintCharacterSet; */
    pub get_hintCharacterSet: unsafe extern "C" fn (this: *const nsIContentViewer, aHintCharacterSet: *mut nsACString) -> nsresult,
    pub set_hintCharacterSet: unsafe extern "C" fn (this: *const nsIContentViewer, aHintCharacterSet: *const nsACString) -> nsresult,

    /* attribute int32_t hintCharacterSetSource; */
    pub get_hintCharacterSetSource: unsafe extern "C" fn (this: *const nsIContentViewer, aHintCharacterSetSource: *mut int32_t) -> nsresult,
    pub set_hintCharacterSetSource: unsafe extern "C" fn (this: *const nsIContentViewer, aHintCharacterSetSource: int32_t) -> nsresult,

    /* void getContentSize (out long width, out long height); */
    pub getContentSize: unsafe extern "C" fn (this: *const nsIContentViewer, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* void getContentSizeConstrained (in long maxWidth, in long maxHeight, out long width, out long height); */
    pub getContentSizeConstrained: unsafe extern "C" fn (this: *const nsIContentViewer, maxWidth: libc::int32_t, maxHeight: libc::int32_t, width: *mut libc::int32_t, height: *mut libc::int32_t) -> nsresult,

    /* attribute long minFontSize; */
    pub get_minFontSize: unsafe extern "C" fn (this: *const nsIContentViewer, aMinFontSize: *mut libc::int32_t) -> nsresult,
    pub set_minFontSize: unsafe extern "C" fn (this: *const nsIContentViewer, aMinFontSize: libc::int32_t) -> nsresult,

    /* [noscript] void appendSubtree (in nsIContentViewerTArray array); */
    /// Unable to call function as its signature contains a non-rust type
    pub appendSubtree: *const ::libc::c_void,

    /* void pausePainting (); */
    pub pausePainting: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* void resumePainting (); */
    pub resumePainting: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

    /* void emulateMedium (in AString aMediaType); */
    pub emulateMedium: unsafe extern "C" fn (this: *const nsIContentViewer, aMediaType: *const nsAString) -> nsresult,

    /* void stopEmulatingMedium (); */
    pub stopEmulatingMedium: unsafe extern "C" fn (this: *const nsIContentViewer) -> nsresult,

}


impl nsIContentViewer {
    /* [noscript] void init (in nsIWidgetPtr aParentWidget, [const] in nsIntRectRef aBounds); */


    /* attribute nsIDocShell container; */
    #[inline]
    pub unsafe fn get_container(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_container)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_container(&self, aContainer: Option<&nsIDocShell>) -> Result<(), nsresult> {

        match ((*self.vtable).set_container)(self as *const _, aContainer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] void loadStart (in nsIDocument aDoc); */
    #[inline]
    pub unsafe fn loadStart(&self, aDoc: Option<&nsIDocument>) -> () {

        let _retval = ((*self.vtable).loadStart)(self as *const _, aDoc.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

    /* void loadComplete (in nsresult aStatus); */
    #[inline]
    pub unsafe fn loadComplete(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).loadComplete)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute boolean loadCompleted; */
    #[inline]
    pub unsafe fn get_loadCompleted(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadCompleted)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute boolean isStopped; */
    #[inline]
    pub unsafe fn get_isStopped(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isStopped)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean permitUnload (); */
    #[inline]
    pub unsafe fn permitUnload(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).permitUnload)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean inPermitUnload; */
    #[inline]
    pub unsafe fn get_inPermitUnload(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_inPermitUnload)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,nostdcall] boolean permitUnloadInternal (inout boolean aShouldPrompt); */
    #[inline]
    pub unsafe fn permitUnloadInternal(&self, ) -> Result<(bool, bool), nsresult> {
        let mut aShouldPrompt: bool = ::std::mem::zeroed();
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).permitUnloadInternal)(self as *const _, &mut aShouldPrompt as *mut _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aShouldPrompt, _retval))
    }

    /* readonly attribute boolean beforeUnloadFiring; */
    #[inline]
    pub unsafe fn get_beforeUnloadFiring(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_beforeUnloadFiring)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void pageHide (in boolean isUnload); */
    #[inline]
    pub unsafe fn pageHide(&self, isUnload: bool) -> Result<(), nsresult> {

        match ((*self.vtable).pageHide)(self as *const _, isUnload) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (in nsISHEntry historyEntry); */
    #[inline]
    pub unsafe fn close(&self, historyEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, historyEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void destroy (); */
    #[inline]
    pub unsafe fn destroy(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroy)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stop (); */
    #[inline]
    pub unsafe fn stop(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stop)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIDOMDocument DOMDocument; */
    #[inline]
    pub unsafe fn get_DOMDocument(&self, ) -> Result<Option<RefPtr<nsIDOMDocument>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_DOMDocument)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_DOMDocument(&self, aDOMDocument: Option<&nsIDOMDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).set_DOMDocument)(self as *const _, aDOMDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] nsIDocument getDocument (); */
    #[inline]
    pub unsafe fn getDocument(&self, ) -> Option<RefPtr<nsIDocument>> {

        let _retval = ((*self.vtable).getDocument)(self as *const _, );
        RefPtr::from_raw(_retval)
    }

    /* [noscript] void getBounds (in nsIntRectRef aBounds); */


    /* [noscript] void setBounds ([const] in nsIntRectRef aBounds); */


    /* [noscript] void setBoundsWithFlags ([const] in nsIntRectRef aBounds, in unsigned long aFlags); */


    /* [noscript] attribute nsIContentViewer previousViewer; */
    #[inline]
    pub unsafe fn get_previousViewer(&self, ) -> Result<Option<RefPtr<nsIContentViewer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_previousViewer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_previousViewer(&self, aPreviousViewer: Option<&nsIContentViewer>) -> Result<(), nsresult> {

        match ((*self.vtable).set_previousViewer)(self as *const _, aPreviousViewer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void move (in long aX, in long aY); */
    #[inline]
    pub unsafe fn move_(&self, aX: libc::int32_t, aY: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).move_)(self as *const _, aX, aY) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void show (); */
    #[inline]
    pub unsafe fn show(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).show)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hide (); */
    #[inline]
    pub unsafe fn hide(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hide)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean sticky; */
    #[inline]
    pub unsafe fn get_sticky(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_sticky)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sticky(&self, aSticky: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_sticky)(self as *const _, aSticky) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean requestWindowClose (); */
    #[inline]
    pub unsafe fn requestWindowClose(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).requestWindowClose)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void open (in nsISupports aState, in nsISHEntry aSHEntry); */
    #[inline]
    pub unsafe fn open(&self, aState: Option<&nsISupports>, aSHEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).open)(self as *const _, aState.map_or(::std::ptr::null(), |x| x as *const _), aSHEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void clearHistoryEntry (); */
    #[inline]
    pub unsafe fn clearHistoryEntry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearHistoryEntry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setPageMode (in boolean aPageMode, in nsIPrintSettings aPrintSettings); */
    #[inline]
    pub unsafe fn setPageMode(&self, aPageMode: bool, aPrintSettings: Option<&nsIPrintSettings>) -> Result<(), nsresult> {

        match ((*self.vtable).setPageMode)(self as *const _, aPageMode, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsISHEntry historyEntry; */
    #[inline]
    pub unsafe fn get_historyEntry(&self, ) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_historyEntry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean isTabModalPromptAllowed; */
    #[inline]
    pub unsafe fn get_isTabModalPromptAllowed(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isTabModalPromptAllowed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute boolean isHidden; */
    #[inline]
    pub unsafe fn get_isHidden(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isHidden)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_isHidden(&self, aIsHidden: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_isHidden)(self as *const _, aIsHidden) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute nsIPresShellPtr presShell; */


    /* [noscript] readonly attribute nsPresContextPtr presContext; */


    /* [noscript] void setDocumentInternal (in nsIDocument aDocument, in boolean aForceReuseInnerWindow); */
    #[inline]
    pub unsafe fn setDocumentInternal(&self, aDocument: Option<&nsIDocument>, aForceReuseInnerWindow: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setDocumentInternal)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _), aForceReuseInnerWindow) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,nostdcall,notxpcom] nsViewPtr findContainerView (); */


    /* [noscript,nostdcall,notxpcom] void setNavigationTiming (in nsDOMNavigationTimingPtr aTiming); */


    /* void scrollToNode (in nsIDOMNode node); */
    #[inline]
    pub unsafe fn scrollToNode(&self, node: Option<&nsIDOMNode>) -> Result<(), nsresult> {

        match ((*self.vtable).scrollToNode)(self as *const _, node.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float textZoom; */
    #[inline]
    pub unsafe fn get_textZoom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_textZoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_textZoom(&self, aTextZoom: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_textZoom)(self as *const _, aTextZoom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute float effectiveTextZoom; */
    #[inline]
    pub unsafe fn get_effectiveTextZoom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_effectiveTextZoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute float fullZoom; */
    #[inline]
    pub unsafe fn get_fullZoom(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_fullZoom)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_fullZoom(&self, aFullZoom: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_fullZoom)(self as *const _, aFullZoom) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute float overrideDPPX; */
    #[inline]
    pub unsafe fn get_overrideDPPX(&self, ) -> Result<libc::c_float, nsresult> {
        let mut _retval: libc::c_float = ::std::mem::zeroed();
        match ((*self.vtable).get_overrideDPPX)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_overrideDPPX(&self, aOverrideDPPX: libc::c_float) -> Result<(), nsresult> {

        match ((*self.vtable).set_overrideDPPX)(self as *const _, aOverrideDPPX) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean authorStyleDisabled; */
    #[inline]
    pub unsafe fn get_authorStyleDisabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_authorStyleDisabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_authorStyleDisabled(&self, aAuthorStyleDisabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_authorStyleDisabled)(self as *const _, aAuthorStyleDisabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString forceCharacterSet; */
    #[inline]
    pub unsafe fn get_forceCharacterSet(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_forceCharacterSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_forceCharacterSet(&self, aForceCharacterSet: &[u8]) -> Result<(), nsresult> {
        let aForceCharacterSet = nsCString::from(aForceCharacterSet);
        match ((*self.vtable).set_forceCharacterSet)(self as *const _, &*aForceCharacterSet) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString hintCharacterSet; */
    #[inline]
    pub unsafe fn get_hintCharacterSet(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_hintCharacterSet)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hintCharacterSet(&self, aHintCharacterSet: &[u8]) -> Result<(), nsresult> {
        let aHintCharacterSet = nsCString::from(aHintCharacterSet);
        match ((*self.vtable).set_hintCharacterSet)(self as *const _, &*aHintCharacterSet) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute int32_t hintCharacterSetSource; */
    #[inline]
    pub unsafe fn get_hintCharacterSetSource(&self, ) -> Result<int32_t, nsresult> {
        let mut _retval: int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_hintCharacterSetSource)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_hintCharacterSetSource(&self, aHintCharacterSetSource: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_hintCharacterSetSource)(self as *const _, aHintCharacterSetSource) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getContentSize (out long width, out long height); */
    #[inline]
    pub unsafe fn getContentSize(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getContentSize)(self as *const _, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((width, height))
    }

    /* void getContentSizeConstrained (in long maxWidth, in long maxHeight, out long width, out long height); */
    #[inline]
    pub unsafe fn getContentSizeConstrained(&self, maxWidth: libc::int32_t, maxHeight: libc::int32_t) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut width: libc::int32_t = ::std::mem::zeroed();
        let mut height: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getContentSizeConstrained)(self as *const _, maxWidth, maxHeight, &mut width as *mut _, &mut height as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((width, height))
    }

    /* attribute long minFontSize; */
    #[inline]
    pub unsafe fn get_minFontSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_minFontSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_minFontSize(&self, aMinFontSize: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_minFontSize)(self as *const _, aMinFontSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void appendSubtree (in nsIContentViewerTArray array); */


    /* void pausePainting (); */
    #[inline]
    pub unsafe fn pausePainting(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).pausePainting)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void resumePainting (); */
    #[inline]
    pub unsafe fn resumePainting(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resumePainting)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void emulateMedium (in AString aMediaType); */
    #[inline]
    pub unsafe fn emulateMedium(&self, aMediaType: &[u16]) -> Result<(), nsresult> {
        let aMediaType = nsString::from(aMediaType);
        match ((*self.vtable).emulateMedium)(self as *const _, &*aMediaType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopEmulatingMedium (); */
    #[inline]
    pub unsafe fn stopEmulatingMedium(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopEmulatingMedium)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


