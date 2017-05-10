//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWindowWatcher.idl
//


#[repr(C)]
pub struct nsIWindowWatcher {
    vtable: *const nsIWindowWatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWindowWatcher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x641fe945, 0x6902, 0x4b3f,
            [0x87, 0xc2, 0x0d, 0xae, 0xf3, 0x24, 0x99, 0xb3])
    }
}

unsafe impl RefCounted for nsIWindowWatcher {
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
pub trait nsIWindowWatcherCoerce {
    fn coerce_from(v: &nsIWindowWatcher) -> &Self;
}

impl nsIWindowWatcherCoerce for nsIWindowWatcher {
    #[inline]
    fn coerce_from(v: &nsIWindowWatcher) -> &Self {
        v
    }
}

impl nsIWindowWatcher {
    #[inline]
    pub fn coerce<T: nsIWindowWatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWindowWatcher {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWindowWatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowWatcher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWindowWatcherVTable {
    pub __base: nsISupportsVTable,

    /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in nsISupports aArguments); */
    pub openWindow: unsafe extern "C" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, aUrl: *const libc::c_char, aName: *const libc::c_char, aFeatures: *const libc::c_char, aArguments: *const nsISupports, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* void registerNotification (in nsIObserver aObserver); */
    pub registerNotification: unsafe extern "C" fn (this: *const nsIWindowWatcher, aObserver: *const nsIObserver) -> nsresult,

    /* void unregisterNotification (in nsIObserver aObserver); */
    pub unregisterNotification: unsafe extern "C" fn (this: *const nsIWindowWatcher, aObserver: *const nsIObserver) -> nsresult,

    /* nsISimpleEnumerator getWindowEnumerator (); */
    pub getWindowEnumerator: unsafe extern "C" fn (this: *const nsIWindowWatcher, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
    pub getNewPrompter: unsafe extern "C" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, _retval: *mut *const nsIPrompt) -> nsresult,

    /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
    pub getNewAuthPrompter: unsafe extern "C" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, _retval: *mut *const nsIAuthPrompt) -> nsresult,

    /* void setWindowCreator (in nsIWindowCreator creator); */
    pub setWindowCreator: unsafe extern "C" fn (this: *const nsIWindowWatcher, creator: *const nsIWindowCreator) -> nsresult,

    /* boolean hasWindowCreator (); */
    pub hasWindowCreator: unsafe extern "C" fn (this: *const nsIWindowWatcher, _retval: *mut bool) -> nsresult,

    /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
    pub getChromeForWindow: unsafe extern "C" fn (this: *const nsIWindowWatcher, aWindow: *const mozIDOMWindowProxy, _retval: *mut *const nsIWebBrowserChrome) -> nsresult,

    /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
    pub getWindowByName: unsafe extern "C" fn (this: *const nsIWindowWatcher, aTargetName: *const nsAString, aCurrentWindow: *const mozIDOMWindowProxy, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* attribute mozIDOMWindowProxy activeWindow; */
    pub get_activeWindow: unsafe extern "C" fn (this: *const nsIWindowWatcher, aActiveWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_activeWindow: unsafe extern "C" fn (this: *const nsIWindowWatcher, aActiveWindow: *const mozIDOMWindowProxy) -> nsresult,

}


impl nsIWindowWatcher {
    /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in string aUrl, in string aName, in string aFeatures, in nsISupports aArguments); */
    #[inline]
    pub unsafe fn openWindow(&self, aParent: Option<&mozIDOMWindowProxy>, aUrl: *const libc::c_char, aName: *const libc::c_char, aFeatures: *const libc::c_char, aArguments: Option<&nsISupports>) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).openWindow)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aUrl, aName, aFeatures, aArguments.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void registerNotification (in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn registerNotification(&self, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).registerNotification)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterNotification (in nsIObserver aObserver); */
    #[inline]
    pub unsafe fn unregisterNotification(&self, aObserver: Option<&nsIObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterNotification)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsISimpleEnumerator getWindowEnumerator (); */
    #[inline]
    pub unsafe fn getWindowEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWindowEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
    #[inline]
    pub unsafe fn getNewPrompter(&self, aParent: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIPrompt>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNewPrompter)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
    #[inline]
    pub unsafe fn getNewAuthPrompter(&self, aParent: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIAuthPrompt>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getNewAuthPrompter)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void setWindowCreator (in nsIWindowCreator creator); */
    #[inline]
    pub unsafe fn setWindowCreator(&self, creator: Option<&nsIWindowCreator>) -> Result<(), nsresult> {

        match ((*self.vtable).setWindowCreator)(self as *const _, creator.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasWindowCreator (); */
    #[inline]
    pub unsafe fn hasWindowCreator(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasWindowCreator)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn getChromeForWindow(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIWebBrowserChrome>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getChromeForWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
    #[inline]
    pub unsafe fn getWindowByName(&self, aTargetName: &[u16], aCurrentWindow: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let aTargetName = nsString::from(aTargetName);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWindowByName)(self as *const _, &*aTargetName, aCurrentWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute mozIDOMWindowProxy activeWindow; */
    #[inline]
    pub unsafe fn get_activeWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activeWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_activeWindow(&self, aActiveWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_activeWindow)(self as *const _, aActiveWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


