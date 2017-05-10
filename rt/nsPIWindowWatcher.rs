//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPIWindowWatcher.idl
//


#[repr(C)]
pub struct nsPIWindowWatcher {
    vtable: *const nsPIWindowWatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPIWindowWatcher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd162f9c4, 0x19d5, 0x4723,
            [0x93, 0x1f, 0xf1, 0xe5, 0x1b, 0xfa, 0x9f, 0x68])
    }
}

unsafe impl RefCounted for nsPIWindowWatcher {
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
pub trait nsPIWindowWatcherCoerce {
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self;
}

impl nsPIWindowWatcherCoerce for nsPIWindowWatcher {
    #[inline]
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self {
        v
    }
}

impl nsPIWindowWatcher {
    #[inline]
    pub fn coerce<T: nsPIWindowWatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPIWindowWatcher {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPIWindowWatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIWindowWatcher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPIWindowWatcherVTable {
    pub __base: nsISupportsVTable,

    /* void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome); */
    pub addWindow: unsafe extern "C" fn (this: *const nsPIWindowWatcher, aWindow: *const mozIDOMWindowProxy, aChrome: *const nsIWebBrowserChrome) -> nsresult,

    /* void removeWindow (in mozIDOMWindowProxy aWindow); */
    pub removeWindow: unsafe extern "C" fn (this: *const nsPIWindowWatcher, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* mozIDOMWindowProxy openWindow2 (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in nsIDocShellLoadInfo aLoadInfo); */
    pub openWindow2: unsafe extern "C" fn (this: *const nsPIWindowWatcher, aParent: *const mozIDOMWindowProxy, aUrl: *const libc::c_char, aName: *const libc::c_char, aFeatures: *const libc::c_char, aCalledFromScript: bool, aDialog: bool, aNavigate: bool, aArgs: *const nsISupports, aIsPopupSpam: bool, aForceNoOpener: bool, aLoadInfo: *const nsIDocShellLoadInfo, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* nsITabParent openWindowWithTabParent (in nsITabParent aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in unsigned long long aNextTabParentId); */
    pub openWindowWithTabParent: unsafe extern "C" fn (this: *const nsPIWindowWatcher, aOpeningTab: *const nsITabParent, aFeatures: *const nsACString, aCalledFromJS: bool, aOpenerFullZoom: libc::c_float, aNextTabParentId: libc::uint64_t, _retval: *mut *const nsITabParent) -> nsresult,

    /* nsIDocShellTreeItem findItemWithName (in AString aName, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
    pub findItemWithName: unsafe extern "C" fn (this: *const nsPIWindowWatcher, aName: *const nsAString, aRequestor: *const nsIDocShellTreeItem, aOriginalRequestor: *const nsIDocShellTreeItem, _retval: *mut *const nsIDocShellTreeItem) -> nsresult,

}


impl nsPIWindowWatcher {
    /* void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome); */
    #[inline]
    pub unsafe fn addWindow(&self, aWindow: Option<&mozIDOMWindowProxy>, aChrome: Option<&nsIWebBrowserChrome>) -> Result<(), nsresult> {

        match ((*self.vtable).addWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aChrome.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeWindow (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn removeWindow(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).removeWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIDOMWindowProxy openWindow2 (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in nsIDocShellLoadInfo aLoadInfo); */
    #[inline]
    pub unsafe fn openWindow2(&self, aParent: Option<&mozIDOMWindowProxy>, aUrl: *const libc::c_char, aName: *const libc::c_char, aFeatures: *const libc::c_char, aCalledFromScript: bool, aDialog: bool, aNavigate: bool, aArgs: Option<&nsISupports>, aIsPopupSpam: bool, aForceNoOpener: bool, aLoadInfo: Option<&nsIDocShellLoadInfo>) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openWindow2)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aUrl, aName, aFeatures, aCalledFromScript, aDialog, aNavigate, aArgs.map_or(::std::ptr::null(), |x| x as *const _), aIsPopupSpam, aForceNoOpener, aLoadInfo.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsITabParent openWindowWithTabParent (in nsITabParent aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in unsigned long long aNextTabParentId); */
    #[inline]
    pub unsafe fn openWindowWithTabParent(&self, aOpeningTab: Option<&nsITabParent>, aFeatures: &[u8], aCalledFromJS: bool, aOpenerFullZoom: libc::c_float, aNextTabParentId: libc::uint64_t) -> Result<Option<RefPtr<nsITabParent>>, nsresult> {
        let aFeatures = nsCString::from(aFeatures);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openWindowWithTabParent)(self as *const _, aOpeningTab.map_or(::std::ptr::null(), |x| x as *const _), &*aFeatures, aCalledFromJS, aOpenerFullZoom, aNextTabParentId, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIDocShellTreeItem findItemWithName (in AString aName, in nsIDocShellTreeItem aRequestor, in nsIDocShellTreeItem aOriginalRequestor); */
    #[inline]
    pub unsafe fn findItemWithName(&self, aName: &[u16], aRequestor: Option<&nsIDocShellTreeItem>, aOriginalRequestor: Option<&nsIDocShellTreeItem>) -> Result<Option<RefPtr<nsIDocShellTreeItem>>, nsresult> {
        let aName = nsString::from(aName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).findItemWithName)(self as *const _, &*aName, aRequestor.map_or(::std::ptr::null(), |x| x as *const _), aOriginalRequestor.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


