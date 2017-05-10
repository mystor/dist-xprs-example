//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITabChild.idl
//


#[repr(C)]
pub struct nsITabChild {
    vtable: *const nsITabChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITabChild {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1fb79c27, 0xe760, 0x4088,
            [0xb1, 0x9c, 0x1c, 0xe3, 0x67, 0x3e, 0xc2, 0x4e])
    }
}

unsafe impl RefCounted for nsITabChild {
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
pub trait nsITabChildCoerce {
    fn coerce_from(v: &nsITabChild) -> &Self;
}

impl nsITabChildCoerce for nsITabChild {
    #[inline]
    fn coerce_from(v: &nsITabChild) -> &Self {
        v
    }
}

impl nsITabChild {
    #[inline]
    pub fn coerce<T: nsITabChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITabChild {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITabChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITabChild) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITabChildVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIContentFrameMessageManager messageManager; */
    pub get_messageManager: unsafe extern "C" fn (this: *const nsITabChild, aMessageManager: *mut *const nsIContentFrameMessageManager) -> nsresult,

    /* attribute nsIWebBrowserChrome3 webBrowserChrome; */
    pub get_webBrowserChrome: unsafe extern "C" fn (this: *const nsITabChild, aWebBrowserChrome: *mut *const nsIWebBrowserChrome3) -> nsresult,
    pub set_webBrowserChrome: unsafe extern "C" fn (this: *const nsITabChild, aWebBrowserChrome: *const nsIWebBrowserChrome3) -> nsresult,

    /* [notxpcom] void sendRequestFocus (in boolean canFocus); */
    pub sendRequestFocus: unsafe extern "C" fn (this: *const nsITabChild, canFocus: bool) -> libc::c_void,

    /* [notxpcom] void sendGetTabCount (out uint32_t tabCount); */
    pub sendGetTabCount: unsafe extern "C" fn (this: *const nsITabChild, tabCount: *mut uint32_t) -> libc::c_void,

    /* [noscript,notxpcom] void enableDisableCommands (in AString action, in CommandsArrayRef enabledCommands, in CommandsArrayRef disabledCommands); */
    /// Unable to call function as its signature contains a non-rust type
    pub enableDisableCommands: *const ::libc::c_void,

    /* [noscript] void remoteSizeShellTo (in int32_t width, in int32_t height, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    pub remoteSizeShellTo: unsafe extern "C" fn (this: *const nsITabChild, width: int32_t, height: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> nsresult,

    /* [noscript] void remoteDropLinks (in unsigned long linksCount, [array, size_is (linksCount)] in nsIDroppedLinkItem links); */
    /// Unable to call function as its signature contains a non-rust type
    pub remoteDropLinks: *const ::libc::c_void,

    /* readonly attribute uint64_t tabId; */
    pub get_tabId: unsafe extern "C" fn (this: *const nsITabChild, aTabId: *mut uint64_t) -> nsresult,

    /* [noscript,notxpcom] void beforeUnloadAdded (); */
    pub beforeUnloadAdded: unsafe extern "C" fn (this: *const nsITabChild) -> libc::c_void,

    /* [noscript,notxpcom] void beforeUnloadRemoved (); */
    pub beforeUnloadRemoved: unsafe extern "C" fn (this: *const nsITabChild) -> libc::c_void,

}


impl nsITabChild {
    /* readonly attribute nsIContentFrameMessageManager messageManager; */
    #[inline]
    pub unsafe fn get_messageManager(&self, ) -> Result<Option<RefPtr<nsIContentFrameMessageManager>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_messageManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIWebBrowserChrome3 webBrowserChrome; */
    #[inline]
    pub unsafe fn get_webBrowserChrome(&self, ) -> Result<Option<RefPtr<nsIWebBrowserChrome3>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_webBrowserChrome)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_webBrowserChrome(&self, aWebBrowserChrome: Option<&nsIWebBrowserChrome3>) -> Result<(), nsresult> {

        match ((*self.vtable).set_webBrowserChrome)(self as *const _, aWebBrowserChrome.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [notxpcom] void sendRequestFocus (in boolean canFocus); */
    #[inline]
    pub unsafe fn sendRequestFocus(&self, canFocus: bool) -> () {

        let _retval = ((*self.vtable).sendRequestFocus)(self as *const _, canFocus);
        ()
    }

    /* [notxpcom] void sendGetTabCount (out uint32_t tabCount); */
    #[inline]
    pub unsafe fn sendGetTabCount(&self, ) -> uint32_t {
        let mut tabCount: uint32_t = ::std::mem::zeroed();
        let _retval = ((*self.vtable).sendGetTabCount)(self as *const _, &mut tabCount as *mut _);
        tabCount
    }

    /* [noscript,notxpcom] void enableDisableCommands (in AString action, in CommandsArrayRef enabledCommands, in CommandsArrayRef disabledCommands); */


    /* [noscript] void remoteSizeShellTo (in int32_t width, in int32_t height, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    #[inline]
    pub unsafe fn remoteSizeShellTo(&self, width: int32_t, height: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).remoteSizeShellTo)(self as *const _, width, height, shellItemWidth, shellItemHeight) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void remoteDropLinks (in unsigned long linksCount, [array, size_is (linksCount)] in nsIDroppedLinkItem links); */


    /* readonly attribute uint64_t tabId; */
    #[inline]
    pub unsafe fn get_tabId(&self, ) -> Result<uint64_t, nsresult> {
        let mut _retval: uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tabId)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript,notxpcom] void beforeUnloadAdded (); */
    #[inline]
    pub unsafe fn beforeUnloadAdded(&self, ) -> () {

        let _retval = ((*self.vtable).beforeUnloadAdded)(self as *const _, );
        ()
    }

    /* [noscript,notxpcom] void beforeUnloadRemoved (); */
    #[inline]
    pub unsafe fn beforeUnloadRemoved(&self, ) -> () {

        let _retval = ((*self.vtable).beforeUnloadRemoved)(self as *const _, );
        ()
    }

}


