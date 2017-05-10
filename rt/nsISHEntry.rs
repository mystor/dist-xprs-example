//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISHEntry.idl
//


#[repr(C)]
pub struct nsISHEntry {
    vtable: *const nsISHEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHEntry {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0dad26b8, 0xa259, 0x42c7,
            [0x93, 0xf1, 0x2f, 0xa7, 0xfc, 0x07, 0x6e, 0x45])
    }
}

unsafe impl RefCounted for nsISHEntry {
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
pub trait nsISHEntryCoerce {
    fn coerce_from(v: &nsISHEntry) -> &Self;
}

impl nsISHEntryCoerce for nsISHEntry {
    #[inline]
    fn coerce_from(v: &nsISHEntry) -> &Self {
        v
    }
}

impl nsISHEntry {
    #[inline]
    pub fn coerce<T: nsISHEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHEntry {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHEntry) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHEntryVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI URI; */
    pub get_URI: unsafe extern "C" fn (this: *const nsISHEntry, aURI: *mut *const nsIURI) -> nsresult,

    /* attribute nsIURI originalURI; */
    pub get_originalURI: unsafe extern "C" fn (this: *const nsISHEntry, aOriginalURI: *mut *const nsIURI) -> nsresult,
    pub set_originalURI: unsafe extern "C" fn (this: *const nsISHEntry, aOriginalURI: *const nsIURI) -> nsresult,

    /* attribute boolean loadReplace; */
    pub get_loadReplace: unsafe extern "C" fn (this: *const nsISHEntry, aLoadReplace: *mut bool) -> nsresult,
    pub set_loadReplace: unsafe extern "C" fn (this: *const nsISHEntry, aLoadReplace: bool) -> nsresult,

    /* readonly attribute wstring title; */
    pub get_title: unsafe extern "C" fn (this: *const nsISHEntry, aTitle: *mut *const libc::int16_t) -> nsresult,

    /* readonly attribute boolean isSubFrame; */
    pub get_isSubFrame: unsafe extern "C" fn (this: *const nsISHEntry, aIsSubFrame: *mut bool) -> nsresult,

    /* void setURI (in nsIURI aURI); */
    pub setURI: unsafe extern "C" fn (this: *const nsISHEntry, aURI: *const nsIURI) -> nsresult,

    /* attribute nsIURI referrerURI; */
    pub get_referrerURI: unsafe extern "C" fn (this: *const nsISHEntry, aReferrerURI: *mut *const nsIURI) -> nsresult,
    pub set_referrerURI: unsafe extern "C" fn (this: *const nsISHEntry, aReferrerURI: *const nsIURI) -> nsresult,

    /* attribute unsigned long referrerPolicy; */
    pub get_referrerPolicy: unsafe extern "C" fn (this: *const nsISHEntry, aReferrerPolicy: *mut libc::uint32_t) -> nsresult,
    pub set_referrerPolicy: unsafe extern "C" fn (this: *const nsISHEntry, aReferrerPolicy: libc::uint32_t) -> nsresult,

    /* attribute nsIContentViewer contentViewer; */
    pub get_contentViewer: unsafe extern "C" fn (this: *const nsISHEntry, aContentViewer: *mut *const nsIContentViewer) -> nsresult,
    pub set_contentViewer: unsafe extern "C" fn (this: *const nsISHEntry, aContentViewer: *const nsIContentViewer) -> nsresult,

    /* attribute boolean sticky; */
    pub get_sticky: unsafe extern "C" fn (this: *const nsISHEntry, aSticky: *mut bool) -> nsresult,
    pub set_sticky: unsafe extern "C" fn (this: *const nsISHEntry, aSticky: bool) -> nsresult,

    /* attribute nsISupports windowState; */
    pub get_windowState: unsafe extern "C" fn (this: *const nsISHEntry, aWindowState: *mut *const nsISupports) -> nsresult,
    pub set_windowState: unsafe extern "C" fn (this: *const nsISHEntry, aWindowState: *const nsISupports) -> nsresult,

    /* [noscript] void getViewerBounds (in nsIntRect bounds); */
    /// Unable to call function as its signature contains a non-rust type
    pub getViewerBounds: *const ::libc::c_void,

    /* [noscript] void setViewerBounds ([const] in nsIntRect bounds); */
    /// Unable to call function as its signature contains a non-rust type
    pub setViewerBounds: *const ::libc::c_void,

    /* void addChildShell (in nsIDocShellTreeItem shell); */
    pub addChildShell: unsafe extern "C" fn (this: *const nsISHEntry, shell: *const nsIDocShellTreeItem) -> nsresult,

    /* nsIDocShellTreeItem childShellAt (in long index); */
    pub childShellAt: unsafe extern "C" fn (this: *const nsISHEntry, index: libc::int32_t, _retval: *mut *const nsIDocShellTreeItem) -> nsresult,

    /* void clearChildShells (); */
    pub clearChildShells: unsafe extern "C" fn (this: *const nsISHEntry) -> nsresult,

    /* attribute nsIMutableArray refreshURIList; */
    pub get_refreshURIList: unsafe extern "C" fn (this: *const nsISHEntry, aRefreshURIList: *mut *const nsIMutableArray) -> nsresult,
    pub set_refreshURIList: unsafe extern "C" fn (this: *const nsISHEntry, aRefreshURIList: *const nsIMutableArray) -> nsresult,

    /* void syncPresentationState (); */
    pub syncPresentationState: unsafe extern "C" fn (this: *const nsISHEntry) -> nsresult,

    /* void setTitle (in AString aTitle); */
    pub setTitle: unsafe extern "C" fn (this: *const nsISHEntry, aTitle: *const nsAString) -> nsresult,

    /* attribute nsIInputStream postData; */
    pub get_postData: unsafe extern "C" fn (this: *const nsISHEntry, aPostData: *mut *const nsIInputStream) -> nsresult,
    pub set_postData: unsafe extern "C" fn (this: *const nsISHEntry, aPostData: *const nsIInputStream) -> nsresult,

    /* attribute nsILayoutHistoryState layoutHistoryState; */
    pub get_layoutHistoryState: unsafe extern "C" fn (this: *const nsISHEntry, aLayoutHistoryState: *mut *const nsILayoutHistoryState) -> nsresult,
    pub set_layoutHistoryState: unsafe extern "C" fn (this: *const nsISHEntry, aLayoutHistoryState: *const nsILayoutHistoryState) -> nsresult,

    /* nsILayoutHistoryState initLayoutHistoryState (); */
    pub initLayoutHistoryState: unsafe extern "C" fn (this: *const nsISHEntry, _retval: *mut *const nsILayoutHistoryState) -> nsresult,

    /* attribute nsISHEntry parent; */
    pub get_parent: unsafe extern "C" fn (this: *const nsISHEntry, aParent: *mut *const nsISHEntry) -> nsresult,
    pub set_parent: unsafe extern "C" fn (this: *const nsISHEntry, aParent: *const nsISHEntry) -> nsresult,

    /* attribute unsigned long loadType; */
    pub get_loadType: unsafe extern "C" fn (this: *const nsISHEntry, aLoadType: *mut libc::uint32_t) -> nsresult,
    pub set_loadType: unsafe extern "C" fn (this: *const nsISHEntry, aLoadType: libc::uint32_t) -> nsresult,

    /* attribute unsigned long ID; */
    pub get_ID: unsafe extern "C" fn (this: *const nsISHEntry, aID: *mut libc::uint32_t) -> nsresult,
    pub set_ID: unsafe extern "C" fn (this: *const nsISHEntry, aID: libc::uint32_t) -> nsresult,

    /* attribute nsISupports cacheKey; */
    pub get_cacheKey: unsafe extern "C" fn (this: *const nsISHEntry, aCacheKey: *mut *const nsISupports) -> nsresult,
    pub set_cacheKey: unsafe extern "C" fn (this: *const nsISHEntry, aCacheKey: *const nsISupports) -> nsresult,

    /* attribute boolean saveLayoutStateFlag; */
    pub get_saveLayoutStateFlag: unsafe extern "C" fn (this: *const nsISHEntry, aSaveLayoutStateFlag: *mut bool) -> nsresult,
    pub set_saveLayoutStateFlag: unsafe extern "C" fn (this: *const nsISHEntry, aSaveLayoutStateFlag: bool) -> nsresult,

    /* attribute boolean expirationStatus; */
    pub get_expirationStatus: unsafe extern "C" fn (this: *const nsISHEntry, aExpirationStatus: *mut bool) -> nsresult,
    pub set_expirationStatus: unsafe extern "C" fn (this: *const nsISHEntry, aExpirationStatus: bool) -> nsresult,

    /* attribute ACString contentType; */
    pub get_contentType: unsafe extern "C" fn (this: *const nsISHEntry, aContentType: *mut nsACString) -> nsresult,
    pub set_contentType: unsafe extern "C" fn (this: *const nsISHEntry, aContentType: *const nsACString) -> nsresult,

    /* attribute boolean URIWasModified; */
    pub get_URIWasModified: unsafe extern "C" fn (this: *const nsISHEntry, aURIWasModified: *mut bool) -> nsresult,
    pub set_URIWasModified: unsafe extern "C" fn (this: *const nsISHEntry, aURIWasModified: bool) -> nsresult,

    /* void setScrollPosition (in long x, in long y); */
    pub setScrollPosition: unsafe extern "C" fn (this: *const nsISHEntry, x: libc::int32_t, y: libc::int32_t) -> nsresult,

    /* void getScrollPosition (out long x, out long y); */
    pub getScrollPosition: unsafe extern "C" fn (this: *const nsISHEntry, x: *mut libc::int32_t, y: *mut libc::int32_t) -> nsresult,

    /* [noscript] void create (in nsIURI URI, in AString title, in nsIInputStream inputStream, in nsILayoutHistoryState layoutHistoryState, in nsISupports cacheKey, in ACString contentType, in nsIPrincipal triggeringPrincipal, in nsIPrincipal principalToInherit, in nsIDRef docshellID, in boolean dynamicCreation); */
    pub create: unsafe extern "C" fn (this: *const nsISHEntry, URI: *const nsIURI, title: *const nsAString, inputStream: *const nsIInputStream, layoutHistoryState: *const nsILayoutHistoryState, cacheKey: *const nsISupports, contentType: *const nsACString, triggeringPrincipal: *const nsIPrincipal, principalToInherit: *const nsIPrincipal, docshellID: *const nsID, dynamicCreation: bool) -> nsresult,

    /* nsISHEntry clone (); */
    pub clone: unsafe extern "C" fn (this: *const nsISHEntry, _retval: *mut *const nsISHEntry) -> nsresult,

    /* void setIsSubFrame (in boolean aFlag); */
    pub setIsSubFrame: unsafe extern "C" fn (this: *const nsISHEntry, aFlag: bool) -> nsresult,

    /* nsIContentViewer getAnyContentViewer (out nsISHEntry ownerEntry); */
    pub getAnyContentViewer: unsafe extern "C" fn (this: *const nsISHEntry, ownerEntry: *mut *const nsISHEntry, _retval: *mut *const nsIContentViewer) -> nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub get_triggeringPrincipal: unsafe extern "C" fn (this: *const nsISHEntry, aTriggeringPrincipal: *mut *const nsIPrincipal) -> nsresult,
    pub set_triggeringPrincipal: unsafe extern "C" fn (this: *const nsISHEntry, aTriggeringPrincipal: *const nsIPrincipal) -> nsresult,

    /* attribute nsIPrincipal principalToInherit; */
    pub get_principalToInherit: unsafe extern "C" fn (this: *const nsISHEntry, aPrincipalToInherit: *mut *const nsIPrincipal) -> nsresult,
    pub set_principalToInherit: unsafe extern "C" fn (this: *const nsISHEntry, aPrincipalToInherit: *const nsIPrincipal) -> nsresult,

    /* attribute nsIStructuredCloneContainer stateData; */
    pub get_stateData: unsafe extern "C" fn (this: *const nsISHEntry, aStateData: *mut *const nsIStructuredCloneContainer) -> nsresult,
    pub set_stateData: unsafe extern "C" fn (this: *const nsISHEntry, aStateData: *const nsIStructuredCloneContainer) -> nsresult,

    /* [noscript,notxpcom] nsDocShellEditorDataPtr forgetEditorData (); */
    /// Unable to call function as its signature contains a non-rust type
    pub forgetEditorData: *const ::libc::c_void,

    /* [noscript,notxpcom] void setEditorData (in nsDocShellEditorDataPtr aData); */
    /// Unable to call function as its signature contains a non-rust type
    pub setEditorData: *const ::libc::c_void,

    /* [noscript,notxpcom] boolean hasDetachedEditor (); */
    pub hasDetachedEditor: unsafe extern "C" fn (this: *const nsISHEntry) -> bool,

    /* boolean isDynamicallyAdded (); */
    pub isDynamicallyAdded: unsafe extern "C" fn (this: *const nsISHEntry, _retval: *mut bool) -> nsresult,

    /* boolean hasDynamicallyAddedChild (); */
    pub hasDynamicallyAddedChild: unsafe extern "C" fn (this: *const nsISHEntry, _retval: *mut bool) -> nsresult,

    /* attribute nsIDPtr docshellID; */
    pub get_docshellID: unsafe extern "C" fn (this: *const nsISHEntry, aDocshellID: *mut *const nsID) -> nsresult,
    pub set_docshellID: unsafe extern "C" fn (this: *const nsISHEntry, aDocshellID: *const nsID) -> nsresult,

    /* [noscript,notxpcom] nsID DocshellID (); */
    /// Unable to call function as its signature contains a non-rust type
    pub DocshellID: *const ::libc::c_void,

    /* readonly attribute nsIBFCacheEntry BFCacheEntry; */
    pub get_BFCacheEntry: unsafe extern "C" fn (this: *const nsISHEntry, aBFCacheEntry: *mut *const nsIBFCacheEntry) -> nsresult,

    /* [noscript,notxpcom] boolean hasBFCacheEntry (in nsIBFCacheEntry aEntry); */
    pub hasBFCacheEntry: unsafe extern "C" fn (this: *const nsISHEntry, aEntry: *const nsIBFCacheEntry) -> bool,

    /* void adoptBFCacheEntry (in nsISHEntry aEntry); */
    pub adoptBFCacheEntry: unsafe extern "C" fn (this: *const nsISHEntry, aEntry: *const nsISHEntry) -> nsresult,

    /* void abandonBFCacheEntry (); */
    pub abandonBFCacheEntry: unsafe extern "C" fn (this: *const nsISHEntry) -> nsresult,

    /* boolean sharesDocumentWith (in nsISHEntry aEntry); */
    pub sharesDocumentWith: unsafe extern "C" fn (this: *const nsISHEntry, aEntry: *const nsISHEntry, _retval: *mut bool) -> nsresult,

    /* readonly attribute boolean isSrcdocEntry; */
    pub get_isSrcdocEntry: unsafe extern "C" fn (this: *const nsISHEntry, aIsSrcdocEntry: *mut bool) -> nsresult,

    /* attribute AString srcdocData; */
    pub get_srcdocData: unsafe extern "C" fn (this: *const nsISHEntry, aSrcdocData: *mut nsAString) -> nsresult,
    pub set_srcdocData: unsafe extern "C" fn (this: *const nsISHEntry, aSrcdocData: *const nsAString) -> nsresult,

    /* attribute nsIURI baseURI; */
    pub get_baseURI: unsafe extern "C" fn (this: *const nsISHEntry, aBaseURI: *mut *const nsIURI) -> nsresult,
    pub set_baseURI: unsafe extern "C" fn (this: *const nsISHEntry, aBaseURI: *const nsIURI) -> nsresult,

    /* attribute boolean scrollRestorationIsManual; */
    pub get_scrollRestorationIsManual: unsafe extern "C" fn (this: *const nsISHEntry, aScrollRestorationIsManual: *mut bool) -> nsresult,
    pub set_scrollRestorationIsManual: unsafe extern "C" fn (this: *const nsISHEntry, aScrollRestorationIsManual: bool) -> nsresult,

    /* [noscript] void setSHistory (in nsISHistory aSHistory); */
    pub setSHistory: unsafe extern "C" fn (this: *const nsISHEntry, aSHistory: *const nsISHistory) -> nsresult,

}


impl nsISHEntry {
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

    /* attribute nsIURI originalURI; */
    #[inline]
    pub unsafe fn get_originalURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_originalURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_originalURI(&self, aOriginalURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_originalURI)(self as *const _, aOriginalURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean loadReplace; */
    #[inline]
    pub unsafe fn get_loadReplace(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_loadReplace)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_loadReplace(&self, aLoadReplace: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_loadReplace)(self as *const _, aLoadReplace) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute wstring title; */
    #[inline]
    pub unsafe fn get_title(&self, ) -> Result<*const libc::int16_t, nsresult> {
        let mut _retval: *const libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_title)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSubFrame; */
    #[inline]
    pub unsafe fn get_isSubFrame(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSubFrame)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn setURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).setURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI referrerURI; */
    #[inline]
    pub unsafe fn get_referrerURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_referrerURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_referrerURI(&self, aReferrerURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_referrerURI)(self as *const _, aReferrerURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long referrerPolicy; */
    #[inline]
    pub unsafe fn get_referrerPolicy(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_referrerPolicy)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_referrerPolicy(&self, aReferrerPolicy: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_referrerPolicy)(self as *const _, aReferrerPolicy) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIContentViewer contentViewer; */
    #[inline]
    pub unsafe fn get_contentViewer(&self, ) -> Result<Option<RefPtr<nsIContentViewer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_contentViewer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_contentViewer(&self, aContentViewer: Option<&nsIContentViewer>) -> Result<(), nsresult> {

        match ((*self.vtable).set_contentViewer)(self as *const _, aContentViewer.map_or(::std::ptr::null(), |x| x as *const _)) {
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

    /* attribute nsISupports windowState; */
    #[inline]
    pub unsafe fn get_windowState(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_windowState)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_windowState(&self, aWindowState: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_windowState)(self as *const _, aWindowState.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void getViewerBounds (in nsIntRect bounds); */


    /* [noscript] void setViewerBounds ([const] in nsIntRect bounds); */


    /* void addChildShell (in nsIDocShellTreeItem shell); */
    #[inline]
    pub unsafe fn addChildShell(&self, shell: Option<&nsIDocShellTreeItem>) -> Result<(), nsresult> {

        match ((*self.vtable).addChildShell)(self as *const _, shell.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDocShellTreeItem childShellAt (in long index); */
    #[inline]
    pub unsafe fn childShellAt(&self, index: libc::int32_t) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).childShellAt)(self as *const _, index, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void clearChildShells (); */
    #[inline]
    pub unsafe fn clearChildShells(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).clearChildShells)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIMutableArray refreshURIList; */
    #[inline]
    pub unsafe fn get_refreshURIList(&self, ) -> Result<Option<RefPtr<nsIMutableArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_refreshURIList)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_refreshURIList(&self, aRefreshURIList: Option<&nsIMutableArray>) -> Result<(), nsresult> {

        match ((*self.vtable).set_refreshURIList)(self as *const _, aRefreshURIList.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void syncPresentationState (); */
    #[inline]
    pub unsafe fn syncPresentationState(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).syncPresentationState)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setTitle (in AString aTitle); */
    #[inline]
    pub unsafe fn setTitle(&self, aTitle: &[u16]) -> Result<(), nsresult> {
        let aTitle = nsString::from(aTitle);
        match ((*self.vtable).setTitle)(self as *const _, &*aTitle) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIInputStream postData; */
    #[inline]
    pub unsafe fn get_postData(&self, ) -> Result<Option<RefPtr<nsIInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_postData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_postData(&self, aPostData: Option<&nsIInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).set_postData)(self as *const _, aPostData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* nsILayoutHistoryState initLayoutHistoryState (); */
    #[inline]
    pub unsafe fn initLayoutHistoryState(&self, ) -> Result<Option<RefPtr<nsILayoutHistoryState>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).initLayoutHistoryState)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsISHEntry parent; */
    #[inline]
    pub unsafe fn get_parent(&self, ) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_parent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_parent(&self, aParent: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).set_parent)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* attribute unsigned long ID; */
    #[inline]
    pub unsafe fn get_ID(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_ID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_ID(&self, aID: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_ID)(self as *const _, aID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISupports cacheKey; */
    #[inline]
    pub unsafe fn get_cacheKey(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_cacheKey)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_cacheKey(&self, aCacheKey: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).set_cacheKey)(self as *const _, aCacheKey.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean saveLayoutStateFlag; */
    #[inline]
    pub unsafe fn get_saveLayoutStateFlag(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_saveLayoutStateFlag)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_saveLayoutStateFlag(&self, aSaveLayoutStateFlag: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_saveLayoutStateFlag)(self as *const _, aSaveLayoutStateFlag) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean expirationStatus; */
    #[inline]
    pub unsafe fn get_expirationStatus(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_expirationStatus)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_expirationStatus(&self, aExpirationStatus: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_expirationStatus)(self as *const _, aExpirationStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute ACString contentType; */
    #[inline]
    pub unsafe fn get_contentType(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_contentType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_contentType(&self, aContentType: &[u8]) -> Result<(), nsresult> {
        let aContentType = nsCString::from(aContentType);
        match ((*self.vtable).set_contentType)(self as *const _, &*aContentType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean URIWasModified; */
    #[inline]
    pub unsafe fn get_URIWasModified(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_URIWasModified)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_URIWasModified(&self, aURIWasModified: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_URIWasModified)(self as *const _, aURIWasModified) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setScrollPosition (in long x, in long y); */
    #[inline]
    pub unsafe fn setScrollPosition(&self, x: libc::int32_t, y: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setScrollPosition)(self as *const _, x, y) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getScrollPosition (out long x, out long y); */
    #[inline]
    pub unsafe fn getScrollPosition(&self, ) -> Result<(libc::int32_t, libc::int32_t), nsresult> {
        let mut x: libc::int32_t = ::std::mem::zeroed();
        let mut y: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).getScrollPosition)(self as *const _, &mut x as *mut _, &mut y as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((x, y))
    }

    /* [noscript] void create (in nsIURI URI, in AString title, in nsIInputStream inputStream, in nsILayoutHistoryState layoutHistoryState, in nsISupports cacheKey, in ACString contentType, in nsIPrincipal triggeringPrincipal, in nsIPrincipal principalToInherit, in nsIDRef docshellID, in boolean dynamicCreation); */
    #[inline]
    pub unsafe fn create(&self, URI: Option<&nsIURI>, title: &[u16], inputStream: Option<&nsIInputStream>, layoutHistoryState: Option<&nsILayoutHistoryState>, cacheKey: Option<&nsISupports>, contentType: &[u8], triggeringPrincipal: Option<&nsIPrincipal>, principalToInherit: Option<&nsIPrincipal>, docshellID: *const nsID, dynamicCreation: bool) -> Result<(), nsresult> {
        let title = nsString::from(title);
        let contentType = nsCString::from(contentType);
        match ((*self.vtable).create)(self as *const _, URI.map_or(::std::ptr::null(), |x| x as *const _), &*title, inputStream.map_or(::std::ptr::null(), |x| x as *const _), layoutHistoryState.map_or(::std::ptr::null(), |x| x as *const _), cacheKey.map_or(::std::ptr::null(), |x| x as *const _), &*contentType, triggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), principalToInherit.map_or(::std::ptr::null(), |x| x as *const _), docshellID, dynamicCreation) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISHEntry clone (); */
    #[inline]
    pub unsafe fn clone(&self, ) -> Result<Option<RefPtr<nsISHEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).clone)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setIsSubFrame (in boolean aFlag); */
    #[inline]
    pub unsafe fn setIsSubFrame(&self, aFlag: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setIsSubFrame)(self as *const _, aFlag) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIContentViewer getAnyContentViewer (out nsISHEntry ownerEntry); */
    #[inline]
    pub unsafe fn getAnyContentViewer(&self, ) -> Result<(Option<RefPtr<nsISHEntry>>, Option<RefPtr<nsIContentViewer>>), nsresult> {
        let mut ownerEntry = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAnyContentViewer)(self as *const _, ownerEntry.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((ownerEntry.refptr(), _retval.refptr()))
    }

    /* attribute nsIPrincipal triggeringPrincipal; */
    #[inline]
    pub unsafe fn get_triggeringPrincipal(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_triggeringPrincipal)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_triggeringPrincipal(&self, aTriggeringPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).set_triggeringPrincipal)(self as *const _, aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIPrincipal principalToInherit; */
    #[inline]
    pub unsafe fn get_principalToInherit(&self, ) -> Result<Option<RefPtr<nsIPrincipal>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_principalToInherit)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_principalToInherit(&self, aPrincipalToInherit: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).set_principalToInherit)(self as *const _, aPrincipalToInherit.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIStructuredCloneContainer stateData; */
    #[inline]
    pub unsafe fn get_stateData(&self, ) -> Result<Option<RefPtr<nsIStructuredCloneContainer>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_stateData)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_stateData(&self, aStateData: Option<&nsIStructuredCloneContainer>) -> Result<(), nsresult> {

        match ((*self.vtable).set_stateData)(self as *const _, aStateData.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] nsDocShellEditorDataPtr forgetEditorData (); */


    /* [noscript,notxpcom] void setEditorData (in nsDocShellEditorDataPtr aData); */


    /* [noscript,notxpcom] boolean hasDetachedEditor (); */
    #[inline]
    pub unsafe fn hasDetachedEditor(&self, ) -> bool {

        let _retval = ((*self.vtable).hasDetachedEditor)(self as *const _, );
        _retval
    }

    /* boolean isDynamicallyAdded (); */
    #[inline]
    pub unsafe fn isDynamicallyAdded(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isDynamicallyAdded)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean hasDynamicallyAddedChild (); */
    #[inline]
    pub unsafe fn hasDynamicallyAddedChild(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasDynamicallyAddedChild)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIDPtr docshellID; */
    #[inline]
    pub unsafe fn get_docshellID(&self, ) -> Result<*const nsID, nsresult> {
        let mut _retval: *const nsID = ::std::ptr::null();
        match ((*self.vtable).get_docshellID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_docshellID(&self, aDocshellID: *const nsID) -> Result<(), nsresult> {

        match ((*self.vtable).set_docshellID)(self as *const _, aDocshellID) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] nsID DocshellID (); */


    /* readonly attribute nsIBFCacheEntry BFCacheEntry; */
    #[inline]
    pub unsafe fn get_BFCacheEntry(&self, ) -> Result<Option<RefPtr<nsIBFCacheEntry>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_BFCacheEntry)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,notxpcom] boolean hasBFCacheEntry (in nsIBFCacheEntry aEntry); */
    #[inline]
    pub unsafe fn hasBFCacheEntry(&self, aEntry: Option<&nsIBFCacheEntry>) -> bool {

        let _retval = ((*self.vtable).hasBFCacheEntry)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _));
        _retval
    }

    /* void adoptBFCacheEntry (in nsISHEntry aEntry); */
    #[inline]
    pub unsafe fn adoptBFCacheEntry(&self, aEntry: Option<&nsISHEntry>) -> Result<(), nsresult> {

        match ((*self.vtable).adoptBFCacheEntry)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void abandonBFCacheEntry (); */
    #[inline]
    pub unsafe fn abandonBFCacheEntry(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).abandonBFCacheEntry)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean sharesDocumentWith (in nsISHEntry aEntry); */
    #[inline]
    pub unsafe fn sharesDocumentWith(&self, aEntry: Option<&nsISHEntry>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).sharesDocumentWith)(self as *const _, aEntry.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute boolean isSrcdocEntry; */
    #[inline]
    pub unsafe fn get_isSrcdocEntry(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isSrcdocEntry)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AString srcdocData; */
    #[inline]
    pub unsafe fn get_srcdocData(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_srcdocData)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_srcdocData(&self, aSrcdocData: &[u16]) -> Result<(), nsresult> {
        let aSrcdocData = nsString::from(aSrcdocData);
        match ((*self.vtable).set_srcdocData)(self as *const _, &*aSrcdocData) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIURI baseURI; */
    #[inline]
    pub unsafe fn get_baseURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_baseURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_baseURI(&self, aBaseURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).set_baseURI)(self as *const _, aBaseURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean scrollRestorationIsManual; */
    #[inline]
    pub unsafe fn get_scrollRestorationIsManual(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_scrollRestorationIsManual)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_scrollRestorationIsManual(&self, aScrollRestorationIsManual: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_scrollRestorationIsManual)(self as *const _, aScrollRestorationIsManual) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setSHistory (in nsISHistory aSHistory); */
    #[inline]
    pub unsafe fn setSHistory(&self, aSHistory: Option<&nsISHistory>) -> Result<(), nsresult> {

        match ((*self.vtable).setSHistory)(self as *const _, aSHistory.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISHEntryInternal {
    vtable: *const nsISHEntryInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISHEntryInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbb66ac35, 0x253b, 0x471f,
            [0xa3, 0x17, 0x3e, 0xce, 0x94, 0x0f, 0x04, 0xc5])
    }
}

unsafe impl RefCounted for nsISHEntryInternal {
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
pub trait nsISHEntryInternalCoerce {
    fn coerce_from(v: &nsISHEntryInternal) -> &Self;
}

impl nsISHEntryInternalCoerce for nsISHEntryInternal {
    #[inline]
    fn coerce_from(v: &nsISHEntryInternal) -> &Self {
        v
    }
}

impl nsISHEntryInternal {
    #[inline]
    pub fn coerce<T: nsISHEntryInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISHEntryInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISHEntryInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHEntryInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISHEntryInternalVTable {
    pub __base: nsISupportsVTable,

    /* [notxpcom] void RemoveFromBFCacheAsync (); */
    pub RemoveFromBFCacheAsync: unsafe extern "C" fn (this: *const nsISHEntryInternal) -> libc::c_void,

    /* [notxpcom] void RemoveFromBFCacheSync (); */
    pub RemoveFromBFCacheSync: unsafe extern "C" fn (this: *const nsISHEntryInternal) -> libc::c_void,

    /* attribute unsigned long lastTouched; */
    pub get_lastTouched: unsafe extern "C" fn (this: *const nsISHEntryInternal, aLastTouched: *mut libc::uint32_t) -> nsresult,
    pub set_lastTouched: unsafe extern "C" fn (this: *const nsISHEntryInternal, aLastTouched: libc::uint32_t) -> nsresult,

    /* [noscript,notxpcom] nsSHEntryShared getSharedState (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSharedState: *const ::libc::c_void,

}


impl nsISHEntryInternal {
    /* [notxpcom] void RemoveFromBFCacheAsync (); */
    #[inline]
    pub unsafe fn RemoveFromBFCacheAsync(&self, ) -> () {

        let _retval = ((*self.vtable).RemoveFromBFCacheAsync)(self as *const _, );
        ()
    }

    /* [notxpcom] void RemoveFromBFCacheSync (); */
    #[inline]
    pub unsafe fn RemoveFromBFCacheSync(&self, ) -> () {

        let _retval = ((*self.vtable).RemoveFromBFCacheSync)(self as *const _, );
        ()
    }

    /* attribute unsigned long lastTouched; */
    #[inline]
    pub unsafe fn get_lastTouched(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lastTouched)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_lastTouched(&self, aLastTouched: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_lastTouched)(self as *const _, aLastTouched) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] nsSHEntryShared getSharedState (); */


}


