//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIXULBrowserWindow.idl
//


#[repr(C)]
pub struct nsIXULBrowserWindow {
    vtable: *const nsIXULBrowserWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIXULBrowserWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa8675fa9, 0xc8b4, 0x4350,
            [0x98, 0x03, 0xc3, 0x8f, 0x34, 0x4a, 0x9e, 0x38])
    }
}

unsafe impl RefCounted for nsIXULBrowserWindow {
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
pub trait nsIXULBrowserWindowCoerce {
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self;
}

impl nsIXULBrowserWindowCoerce for nsIXULBrowserWindow {
    #[inline]
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self {
        v
    }
}

impl nsIXULBrowserWindow {
    #[inline]
    pub fn coerce<T: nsIXULBrowserWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIXULBrowserWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIXULBrowserWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULBrowserWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIXULBrowserWindowVTable {
    pub __base: nsISupportsVTable,

    /* void setJSStatus (in AString status); */
    pub setJSStatus: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, status: *const nsAString) -> nsresult,

    /* void setOverLink (in AString link, in nsIDOMElement element); */
    pub setOverLink: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, link: *const nsAString, element: *const nsIDOMElement) -> nsresult,

    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in nsIDOMNode linkNode, in boolean isAppTab); */
    pub onBeforeLinkTraversal: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, originalTarget: *const nsAString, linkURI: *const nsIURI, linkNode: *const nsIDOMNode, isAppTab: bool, _retval: *mut nsAString) -> nsresult,

    /* void forceInitialBrowserRemote (in AString aRemoteType); */
    pub forceInitialBrowserRemote: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, aRemoteType: *const nsAString) -> nsresult,

    /* void forceInitialBrowserNonRemote (in mozIDOMWindowProxy openerWindow); */
    pub forceInitialBrowserNonRemote: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, openerWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal); */
    pub shouldLoadURI: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, aDocShell: *const nsIDocShell, aURI: *const nsIURI, aReferrer: *const nsIURI, aHasPostData: bool, aTriggeringPrincipal: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction); */
    pub showTooltip: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, x: libc::int32_t, y: libc::int32_t, tooltip: *const nsAString, direction: *const nsAString) -> nsresult,

    /* void hideTooltip (); */
    pub hideTooltip: unsafe extern "C" fn (this: *const nsIXULBrowserWindow) -> nsresult,

    /* uint32_t getTabCount (); */
    pub getTabCount: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, _retval: *mut uint32_t) -> nsresult,

    /* void navigateAndRestoreByIndex (in nsIBrowser aBrowser, in long aIndex); */
    pub navigateAndRestoreByIndex: unsafe extern "C" fn (this: *const nsIXULBrowserWindow, aBrowser: *const nsIBrowser, aIndex: libc::int32_t) -> nsresult,

}


impl nsIXULBrowserWindow {
    /* void setJSStatus (in AString status); */
    #[inline]
    pub unsafe fn setJSStatus(&self, status: &[u16]) -> Result<(), nsresult> {
        let status = nsString::from(status);
        match ((*self.vtable).setJSStatus)(self as *const _, &*status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setOverLink (in AString link, in nsIDOMElement element); */
    #[inline]
    pub unsafe fn setOverLink(&self, link: &[u16], element: Option<&nsIDOMElement>) -> Result<(), nsresult> {
        let link = nsString::from(link);
        match ((*self.vtable).setOverLink)(self as *const _, &*link, element.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in nsIDOMNode linkNode, in boolean isAppTab); */
    #[inline]
    pub unsafe fn onBeforeLinkTraversal(&self, originalTarget: &[u16], linkURI: Option<&nsIURI>, linkNode: Option<&nsIDOMNode>, isAppTab: bool) -> Result<nsString, nsresult> {
        let originalTarget = nsString::from(originalTarget);
        let mut _retval = nsString::new();
        match ((*self.vtable).onBeforeLinkTraversal)(self as *const _, &*originalTarget, linkURI.map_or(::std::ptr::null(), |x| x as *const _), linkNode.map_or(::std::ptr::null(), |x| x as *const _), isAppTab, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void forceInitialBrowserRemote (in AString aRemoteType); */
    #[inline]
    pub unsafe fn forceInitialBrowserRemote(&self, aRemoteType: &[u16]) -> Result<(), nsresult> {
        let aRemoteType = nsString::from(aRemoteType);
        match ((*self.vtable).forceInitialBrowserRemote)(self as *const _, &*aRemoteType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceInitialBrowserNonRemote (in mozIDOMWindowProxy openerWindow); */
    #[inline]
    pub unsafe fn forceInitialBrowserNonRemote(&self, openerWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).forceInitialBrowserNonRemote)(self as *const _, openerWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal); */
    #[inline]
    pub unsafe fn shouldLoadURI(&self, aDocShell: Option<&nsIDocShell>, aURI: Option<&nsIURI>, aReferrer: Option<&nsIURI>, aHasPostData: bool, aTriggeringPrincipal: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldLoadURI)(self as *const _, aDocShell.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aHasPostData, aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction); */
    #[inline]
    pub unsafe fn showTooltip(&self, x: libc::int32_t, y: libc::int32_t, tooltip: &[u16], direction: &[u16]) -> Result<(), nsresult> {
        let tooltip = nsString::from(tooltip);
        let direction = nsString::from(direction);
        match ((*self.vtable).showTooltip)(self as *const _, x, y, &*tooltip, &*direction) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void hideTooltip (); */
    #[inline]
    pub unsafe fn hideTooltip(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).hideTooltip)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* uint32_t getTabCount (); */
    #[inline]
    pub unsafe fn getTabCount(&self, ) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTabCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void navigateAndRestoreByIndex (in nsIBrowser aBrowser, in long aIndex); */
    #[inline]
    pub unsafe fn navigateAndRestoreByIndex(&self, aBrowser: Option<&nsIBrowser>, aIndex: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).navigateAndRestoreByIndex)(self as *const _, aBrowser.map_or(::std::ptr::null(), |x| x as *const _), aIndex) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


