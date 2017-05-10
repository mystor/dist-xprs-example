//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserChrome3.idl
//


#[repr(C)]
pub struct nsIWebBrowserChrome3 {
    vtable: *const nsIWebBrowserChrome3VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserChrome3 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x542b6625, 0x35a9, 0x426a,
            [0x82, 0x57, 0xc1, 0x2a, 0x34, 0x53, 0x83, 0xb0])
    }
}

unsafe impl RefCounted for nsIWebBrowserChrome3 {
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
pub trait nsIWebBrowserChrome3Coerce {
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self;
}

impl nsIWebBrowserChrome3Coerce for nsIWebBrowserChrome3 {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self {
        v
    }
}

impl nsIWebBrowserChrome3 {
    #[inline]
    pub fn coerce<T: nsIWebBrowserChrome3Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserChrome3 {
    type Target = nsIWebBrowserChrome2;
    #[inline]
    fn deref(&self) -> &nsIWebBrowserChrome2 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebBrowserChrome2Coerce> nsIWebBrowserChrome3Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserChrome3VTable {
    pub __base: nsIWebBrowserChrome2VTable,

    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in nsIDOMNode linkNode, in boolean isAppTab); */
    pub onBeforeLinkTraversal: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, originalTarget: *const nsAString, linkURI: *const nsIURI, linkNode: *const nsIDOMNode, isAppTab: bool, _retval: *mut nsAString) -> nsresult,

    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal); */
    pub shouldLoadURI: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, aDocShell: *const nsIDocShell, aURI: *const nsIURI, aReferrer: *const nsIURI, aHasPostData: bool, aTriggeringPrincipal: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
    pub shouldLoadURIInThisProcess: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, aURI: *const nsIURI, _retval: *mut bool) -> nsresult,

    /* bool reloadInFreshProcess (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in nsIPrincipal aTriggeringPrincipal, in uint32_t aLoadFlags); */
    pub reloadInFreshProcess: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, aDocShell: *const nsIDocShell, aURI: *const nsIURI, aReferrer: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aLoadFlags: uint32_t, _retval: *mut bool) -> nsresult,

    /* void startPrerenderingDocument (in nsIURI aHref, in nsIURI aReferrer); */
    pub startPrerenderingDocument: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, aHref: *const nsIURI, aReferrer: *const nsIURI) -> nsresult,

    /* bool shouldSwitchToPrerenderedDocument (in nsIURI aHref, in nsIURI aReferrer, in nsIRunnable aSuccess, in nsIRunnable aFailure); */
    pub shouldSwitchToPrerenderedDocument: unsafe extern "C" fn (this: *const nsIWebBrowserChrome3, aHref: *const nsIURI, aReferrer: *const nsIURI, aSuccess: *const nsIRunnable, aFailure: *const nsIRunnable, _retval: *mut bool) -> nsresult,

}


impl nsIWebBrowserChrome3 {
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

    /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
    #[inline]
    pub unsafe fn shouldLoadURIInThisProcess(&self, aURI: Option<&nsIURI>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldLoadURIInThisProcess)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool reloadInFreshProcess (in nsIDocShell aDocShell, in nsIURI aURI, in nsIURI aReferrer, in nsIPrincipal aTriggeringPrincipal, in uint32_t aLoadFlags); */
    #[inline]
    pub unsafe fn reloadInFreshProcess(&self, aDocShell: Option<&nsIDocShell>, aURI: Option<&nsIURI>, aReferrer: Option<&nsIURI>, aTriggeringPrincipal: Option<&nsIPrincipal>, aLoadFlags: uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).reloadInFreshProcess)(self as *const _, aDocShell.map_or(::std::ptr::null(), |x| x as *const _), aURI.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aTriggeringPrincipal.map_or(::std::ptr::null(), |x| x as *const _), aLoadFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void startPrerenderingDocument (in nsIURI aHref, in nsIURI aReferrer); */
    #[inline]
    pub unsafe fn startPrerenderingDocument(&self, aHref: Option<&nsIURI>, aReferrer: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).startPrerenderingDocument)(self as *const _, aHref.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* bool shouldSwitchToPrerenderedDocument (in nsIURI aHref, in nsIURI aReferrer, in nsIRunnable aSuccess, in nsIRunnable aFailure); */
    #[inline]
    pub unsafe fn shouldSwitchToPrerenderedDocument(&self, aHref: Option<&nsIURI>, aReferrer: Option<&nsIURI>, aSuccess: Option<&nsIRunnable>, aFailure: Option<&nsIRunnable>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldSwitchToPrerenderedDocument)(self as *const _, aHref.map_or(::std::ptr::null(), |x| x as *const _), aReferrer.map_or(::std::ptr::null(), |x| x as *const _), aSuccess.map_or(::std::ptr::null(), |x| x as *const _), aFailure.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


