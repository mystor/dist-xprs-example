//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFrameLoader.idl
//


pub mod nsIFrameLoader_consts {
    pub const EVENT_MODE_NORMAL_DISPATCH: i64 = 0;
    pub const EVENT_MODE_DONT_FORWARD_TO_CHILD: i64 = 1;
}


#[repr(C)]
pub struct nsIFrameLoader {
    vtable: *const nsIFrameLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFrameLoader {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1645af04, 0x1bc7, 0x4363,
            [0x8f, 0x2c, 0xeb, 0x96, 0x79, 0x22, 0x0a, 0xb1])
    }
}

unsafe impl RefCounted for nsIFrameLoader {
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
pub trait nsIFrameLoaderCoerce {
    fn coerce_from(v: &nsIFrameLoader) -> &Self;
}

impl nsIFrameLoaderCoerce for nsIFrameLoader {
    #[inline]
    fn coerce_from(v: &nsIFrameLoader) -> &Self {
        v
    }
}

impl nsIFrameLoader {
    #[inline]
    pub fn coerce<T: nsIFrameLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFrameLoader {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFrameLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFrameLoader) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFrameLoaderVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDocShell docShell; */
    pub get_docShell: unsafe extern "C" fn (this: *const nsIFrameLoader, aDocShell: *mut *const nsIDocShell) -> nsresult,

    /* readonly attribute nsITabParent tabParent; */
    pub get_tabParent: unsafe extern "C" fn (this: *const nsIFrameLoader, aTabParent: *mut *const nsITabParent) -> nsresult,

    /* readonly attribute nsILoadContext loadContext; */
    pub get_loadContext: unsafe extern "C" fn (this: *const nsIFrameLoader, aLoadContext: *mut *const nsILoadContext) -> nsresult,

    /* void loadFrame (); */
    pub loadFrame: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void loadURI (in nsIURI aURI); */
    pub loadURI: unsafe extern "C" fn (this: *const nsIFrameLoader, aURI: *const nsIURI) -> nsresult,

    /* void setIsPrerendered (); */
    pub setIsPrerendered: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void makePrerenderedLoaderActive (); */
    pub makePrerenderedLoaderActive: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* nsISupports appendPartialSHistoryAndSwap (in nsIFrameLoader aOther); */
    pub appendPartialSHistoryAndSwap: unsafe extern "C" fn (this: *const nsIFrameLoader, aOther: *const nsIFrameLoader, _retval: *mut *const nsISupports) -> nsresult,

    /* nsISupports requestGroupedHistoryNavigation (in unsigned long aGlobalIndex); */
    pub requestGroupedHistoryNavigation: unsafe extern "C" fn (this: *const nsIFrameLoader, aGlobalIndex: libc::uint32_t, _retval: *mut *const nsISupports) -> nsresult,

    /* [implicit_jscontext] void addProcessChangeBlockingPromise (in jsval aPromise); */
    /// Unable to call function as its signature contains a non-rust type
    pub addProcessChangeBlockingPromise: *const ::libc::c_void,

    /* void destroy (); */
    pub destroy: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* readonly attribute boolean depthTooGreat; */
    pub get_depthTooGreat: unsafe extern "C" fn (this: *const nsIFrameLoader, aDepthTooGreat: *mut bool) -> nsresult,

    /* [noscript] void updatePositionAndSize (in nsSubDocumentFrame aIFrame); */
    /// Unable to call function as its signature contains a non-rust type
    pub updatePositionAndSize: *const ::libc::c_void,

    /* void activateRemoteFrame (); */
    pub activateRemoteFrame: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void deactivateRemoteFrame (); */
    pub deactivateRemoteFrame: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void sendCrossProcessMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    pub sendCrossProcessMouseEvent: unsafe extern "C" fn (this: *const nsIFrameLoader, aType: *const nsAString, aX: libc::c_float, aY: libc::c_float, aButton: libc::int32_t, aClickCount: libc::int32_t, aModifiers: libc::int32_t, aIgnoreRootScrollFrame: bool) -> nsresult,

    /* void activateFrameEvent (in AString aType, in boolean capture); */
    pub activateFrameEvent: unsafe extern "C" fn (this: *const nsIFrameLoader, aType: *const nsAString, capture: bool) -> nsresult,

    /* readonly attribute nsIMessageSender messageManager; */
    pub get_messageManager: unsafe extern "C" fn (this: *const nsIFrameLoader, aMessageManager: *mut *const nsIMessageSender) -> nsresult,

    /* void sendCrossProcessKeyEvent (in AString aType, in long aKeyCode, in long aCharCode, in long aModifiers, [optional] in boolean aPreventDefault); */
    pub sendCrossProcessKeyEvent: unsafe extern "C" fn (this: *const nsIFrameLoader, aType: *const nsAString, aKeyCode: libc::int32_t, aCharCode: libc::int32_t, aModifiers: libc::int32_t, aPreventDefault: bool) -> nsresult,

    /* void requestNotifyAfterRemotePaint (); */
    pub requestNotifyAfterRemotePaint: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void requestFrameLoaderClose (); */
    pub requestFrameLoaderClose: unsafe extern "C" fn (this: *const nsIFrameLoader) -> nsresult,

    /* void print (in unsigned long long aOuterWindowID, in nsIPrintSettings aPrintSettings, in nsIWebProgressListener aProgressListener); */
    pub print: unsafe extern "C" fn (this: *const nsIFrameLoader, aOuterWindowID: libc::uint64_t, aPrintSettings: *const nsIPrintSettings, aProgressListener: *const nsIWebProgressListener) -> nsresult,

    /* nsIGroupedSHistory ensureGroupedSHistory (); */
    pub ensureGroupedSHistory: unsafe extern "C" fn (this: *const nsIFrameLoader, _retval: *mut *const nsIGroupedSHistory) -> nsresult,

    /* attribute unsigned long eventMode; */
    pub get_eventMode: unsafe extern "C" fn (this: *const nsIFrameLoader, aEventMode: *mut libc::uint32_t) -> nsresult,
    pub set_eventMode: unsafe extern "C" fn (this: *const nsIFrameLoader, aEventMode: libc::uint32_t) -> nsresult,

    /* attribute boolean clipSubdocument; */
    pub get_clipSubdocument: unsafe extern "C" fn (this: *const nsIFrameLoader, aClipSubdocument: *mut bool) -> nsresult,
    pub set_clipSubdocument: unsafe extern "C" fn (this: *const nsIFrameLoader, aClipSubdocument: bool) -> nsresult,

    /* attribute boolean clampScrollPosition; */
    pub get_clampScrollPosition: unsafe extern "C" fn (this: *const nsIFrameLoader, aClampScrollPosition: *mut bool) -> nsresult,
    pub set_clampScrollPosition: unsafe extern "C" fn (this: *const nsIFrameLoader, aClampScrollPosition: bool) -> nsresult,

    /* readonly attribute nsIDOMElement ownerElement; */
    pub get_ownerElement: unsafe extern "C" fn (this: *const nsIFrameLoader, aOwnerElement: *mut *const nsIDOMElement) -> nsresult,

    /* readonly attribute unsigned long long childID; */
    pub get_childID: unsafe extern "C" fn (this: *const nsIFrameLoader, aChildID: *mut libc::uint64_t) -> nsresult,

    /* [infallible] attribute boolean visible; */
    pub get_visible: unsafe extern "C" fn (this: *const nsIFrameLoader, aVisible: *mut bool) -> nsresult,
    pub set_visible: unsafe extern "C" fn (this: *const nsIFrameLoader, aVisible: bool) -> nsresult,

    /* readonly attribute boolean ownerIsMozBrowserFrame; */
    pub get_ownerIsMozBrowserFrame: unsafe extern "C" fn (this: *const nsIFrameLoader, aOwnerIsMozBrowserFrame: *mut bool) -> nsresult,

    /* readonly attribute unsigned long lazyWidth; */
    pub get_lazyWidth: unsafe extern "C" fn (this: *const nsIFrameLoader, aLazyWidth: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long lazyHeight; */
    pub get_lazyHeight: unsafe extern "C" fn (this: *const nsIFrameLoader, aLazyHeight: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute nsIPartialSHistory partialSHistory; */
    pub get_partialSHistory: unsafe extern "C" fn (this: *const nsIFrameLoader, aPartialSHistory: *mut *const nsIPartialSHistory) -> nsresult,

    /* readonly attribute nsIGroupedSHistory groupedSHistory; */
    pub get_groupedSHistory: unsafe extern "C" fn (this: *const nsIFrameLoader, aGroupedSHistory: *mut *const nsIGroupedSHistory) -> nsresult,

    /* [infallible] readonly attribute boolean isDead; */
    pub get_isDead: unsafe extern "C" fn (this: *const nsIFrameLoader, aIsDead: *mut bool) -> nsresult,

}


impl nsIFrameLoader {
    /* readonly attribute nsIDocShell docShell; */
    #[inline]
    pub unsafe fn get_docShell(&self, ) -> Result<Option<RefPtr<nsIDocShell>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_docShell)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsITabParent tabParent; */
    #[inline]
    pub unsafe fn get_tabParent(&self, ) -> Result<Option<RefPtr<nsITabParent>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tabParent)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsILoadContext loadContext; */
    #[inline]
    pub unsafe fn get_loadContext(&self, ) -> Result<Option<RefPtr<nsILoadContext>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_loadContext)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void loadFrame (); */
    #[inline]
    pub unsafe fn loadFrame(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).loadFrame)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void loadURI (in nsIURI aURI); */
    #[inline]
    pub unsafe fn loadURI(&self, aURI: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).loadURI)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setIsPrerendered (); */
    #[inline]
    pub unsafe fn setIsPrerendered(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setIsPrerendered)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void makePrerenderedLoaderActive (); */
    #[inline]
    pub unsafe fn makePrerenderedLoaderActive(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).makePrerenderedLoaderActive)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISupports appendPartialSHistoryAndSwap (in nsIFrameLoader aOther); */
    #[inline]
    pub unsafe fn appendPartialSHistoryAndSwap(&self, aOther: Option<&nsIFrameLoader>) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).appendPartialSHistoryAndSwap)(self as *const _, aOther.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsISupports requestGroupedHistoryNavigation (in unsigned long aGlobalIndex); */
    #[inline]
    pub unsafe fn requestGroupedHistoryNavigation(&self, aGlobalIndex: libc::uint32_t) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).requestGroupedHistoryNavigation)(self as *const _, aGlobalIndex, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] void addProcessChangeBlockingPromise (in jsval aPromise); */


    /* void destroy (); */
    #[inline]
    pub unsafe fn destroy(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroy)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean depthTooGreat; */
    #[inline]
    pub unsafe fn get_depthTooGreat(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_depthTooGreat)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void updatePositionAndSize (in nsSubDocumentFrame aIFrame); */


    /* void activateRemoteFrame (); */
    #[inline]
    pub unsafe fn activateRemoteFrame(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).activateRemoteFrame)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void deactivateRemoteFrame (); */
    #[inline]
    pub unsafe fn deactivateRemoteFrame(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).deactivateRemoteFrame)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendCrossProcessMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    #[inline]
    pub unsafe fn sendCrossProcessMouseEvent(&self, aType: &[u16], aX: libc::c_float, aY: libc::c_float, aButton: libc::int32_t, aClickCount: libc::int32_t, aModifiers: libc::int32_t, aIgnoreRootScrollFrame: bool) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).sendCrossProcessMouseEvent)(self as *const _, &*aType, aX, aY, aButton, aClickCount, aModifiers, aIgnoreRootScrollFrame) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void activateFrameEvent (in AString aType, in boolean capture); */
    #[inline]
    pub unsafe fn activateFrameEvent(&self, aType: &[u16], capture: bool) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).activateFrameEvent)(self as *const _, &*aType, capture) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIMessageSender messageManager; */
    #[inline]
    pub unsafe fn get_messageManager(&self, ) -> Result<Option<RefPtr<nsIMessageSender>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_messageManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void sendCrossProcessKeyEvent (in AString aType, in long aKeyCode, in long aCharCode, in long aModifiers, [optional] in boolean aPreventDefault); */
    #[inline]
    pub unsafe fn sendCrossProcessKeyEvent(&self, aType: &[u16], aKeyCode: libc::int32_t, aCharCode: libc::int32_t, aModifiers: libc::int32_t, aPreventDefault: bool) -> Result<(), nsresult> {
        let aType = nsString::from(aType);
        match ((*self.vtable).sendCrossProcessKeyEvent)(self as *const _, &*aType, aKeyCode, aCharCode, aModifiers, aPreventDefault) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestNotifyAfterRemotePaint (); */
    #[inline]
    pub unsafe fn requestNotifyAfterRemotePaint(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).requestNotifyAfterRemotePaint)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void requestFrameLoaderClose (); */
    #[inline]
    pub unsafe fn requestFrameLoaderClose(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).requestFrameLoaderClose)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void print (in unsigned long long aOuterWindowID, in nsIPrintSettings aPrintSettings, in nsIWebProgressListener aProgressListener); */
    #[inline]
    pub unsafe fn print(&self, aOuterWindowID: libc::uint64_t, aPrintSettings: Option<&nsIPrintSettings>, aProgressListener: Option<&nsIWebProgressListener>) -> Result<(), nsresult> {

        match ((*self.vtable).print)(self as *const _, aOuterWindowID, aPrintSettings.map_or(::std::ptr::null(), |x| x as *const _), aProgressListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIGroupedSHistory ensureGroupedSHistory (); */
    #[inline]
    pub unsafe fn ensureGroupedSHistory(&self, ) -> Result<Option<RefPtr<nsIGroupedSHistory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).ensureGroupedSHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute unsigned long eventMode; */
    #[inline]
    pub unsafe fn get_eventMode(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_eventMode)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_eventMode(&self, aEventMode: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_eventMode)(self as *const _, aEventMode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean clipSubdocument; */
    #[inline]
    pub unsafe fn get_clipSubdocument(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_clipSubdocument)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_clipSubdocument(&self, aClipSubdocument: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_clipSubdocument)(self as *const _, aClipSubdocument) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean clampScrollPosition; */
    #[inline]
    pub unsafe fn get_clampScrollPosition(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_clampScrollPosition)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_clampScrollPosition(&self, aClampScrollPosition: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_clampScrollPosition)(self as *const _, aClampScrollPosition) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement ownerElement; */
    #[inline]
    pub unsafe fn get_ownerElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_ownerElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute unsigned long long childID; */
    #[inline]
    pub unsafe fn get_childID(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_childID)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [infallible] attribute boolean visible; */
    #[inline]
    pub unsafe fn get_visible(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_visible)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_visible(&self, aVisible: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_visible)(self as *const _, aVisible) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute boolean ownerIsMozBrowserFrame; */
    #[inline]
    pub unsafe fn get_ownerIsMozBrowserFrame(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_ownerIsMozBrowserFrame)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long lazyWidth; */
    #[inline]
    pub unsafe fn get_lazyWidth(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lazyWidth)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long lazyHeight; */
    #[inline]
    pub unsafe fn get_lazyHeight(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_lazyHeight)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIPartialSHistory partialSHistory; */
    #[inline]
    pub unsafe fn get_partialSHistory(&self, ) -> Result<Option<RefPtr<nsIPartialSHistory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_partialSHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIGroupedSHistory groupedSHistory; */
    #[inline]
    pub unsafe fn get_groupedSHistory(&self, ) -> Result<Option<RefPtr<nsIGroupedSHistory>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_groupedSHistory)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [infallible] readonly attribute boolean isDead; */
    #[inline]
    pub unsafe fn get_isDead(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isDead)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIFrameLoaderOwner {
    vtable: *const nsIFrameLoaderOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFrameLoaderOwner {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xadc1b3ba, 0x8deb, 0x4943,
            [0x80, 0x45, 0xe6, 0xde, 0x00, 0x44, 0xf2, 0xce])
    }
}

unsafe impl RefCounted for nsIFrameLoaderOwner {
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
pub trait nsIFrameLoaderOwnerCoerce {
    fn coerce_from(v: &nsIFrameLoaderOwner) -> &Self;
}

impl nsIFrameLoaderOwnerCoerce for nsIFrameLoaderOwner {
    #[inline]
    fn coerce_from(v: &nsIFrameLoaderOwner) -> &Self {
        v
    }
}

impl nsIFrameLoaderOwner {
    #[inline]
    pub fn coerce<T: nsIFrameLoaderOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFrameLoaderOwner {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFrameLoaderOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFrameLoaderOwner) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFrameLoaderOwnerVTable {
    pub __base: nsISupportsVTable,

    /* [binaryname(FrameLoaderXPCOM)] readonly attribute nsIFrameLoader frameLoader; */
    pub get_FrameLoaderXPCOM: unsafe extern "C" fn (this: *const nsIFrameLoaderOwner, aFrameLoader: *mut *const nsIFrameLoader) -> nsresult,

    /* [noscript,notxpcom] alreadyAddRefed_nsFrameLoader GetFrameLoader (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetFrameLoader: *const ::libc::c_void,

    /* void setIsPrerendered (); */
    pub setIsPrerendered: unsafe extern "C" fn (this: *const nsIFrameLoaderOwner) -> nsresult,

    /* [noscript,notxpcom] void internalSetFrameLoader (in nsIFrameLoader aNewFrameLoader); */
    pub internalSetFrameLoader: unsafe extern "C" fn (this: *const nsIFrameLoaderOwner, aNewFrameLoader: *const nsIFrameLoader) -> libc::c_void,

}


impl nsIFrameLoaderOwner {
    /* [binaryname(FrameLoaderXPCOM)] readonly attribute nsIFrameLoader frameLoader; */
    #[inline]
    pub unsafe fn get_FrameLoaderXPCOM(&self, ) -> Result<Option<RefPtr<nsIFrameLoader>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_FrameLoaderXPCOM)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript,notxpcom] alreadyAddRefed_nsFrameLoader GetFrameLoader (); */


    /* void setIsPrerendered (); */
    #[inline]
    pub unsafe fn setIsPrerendered(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setIsPrerendered)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript,notxpcom] void internalSetFrameLoader (in nsIFrameLoader aNewFrameLoader); */
    #[inline]
    pub unsafe fn internalSetFrameLoader(&self, aNewFrameLoader: Option<&nsIFrameLoader>) -> () {

        let _retval = ((*self.vtable).internalSetFrameLoader)(self as *const _, aNewFrameLoader.map_or(::std::ptr::null(), |x| x as *const _));
        ()
    }

}


