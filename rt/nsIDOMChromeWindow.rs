//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMChromeWindow.idl
//


pub mod nsIDOMChromeWindow_consts {
    pub const STATE_MAXIMIZED: i64 = 1;
    pub const STATE_MINIMIZED: i64 = 2;
    pub const STATE_NORMAL: i64 = 3;
    pub const STATE_FULLSCREEN: i64 = 4;
}


#[repr(C)]
pub struct nsIDOMChromeWindow {
    vtable: *const nsIDOMChromeWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMChromeWindow {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x78bdcb41, 0x1efa, 0x409f,
            [0xaa, 0xba, 0x70, 0x84, 0x22, 0x13, 0xf8, 0x0f])
    }
}

unsafe impl RefCounted for nsIDOMChromeWindow {
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
pub trait nsIDOMChromeWindowCoerce {
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self;
}

impl nsIDOMChromeWindowCoerce for nsIDOMChromeWindow {
    #[inline]
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self {
        v
    }
}

impl nsIDOMChromeWindow {
    #[inline]
    pub fn coerce<T: nsIDOMChromeWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMChromeWindow {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMChromeWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMChromeWindow) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMChromeWindowVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short windowState; */
    pub get_windowState: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aWindowState: *mut libc::uint16_t) -> nsresult,

    /* attribute nsIBrowserDOMWindow browserDOMWindow; */
    pub get_browserDOMWindow: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aBrowserDOMWindow: *mut *const nsIBrowserDOMWindow) -> nsresult,
    pub set_browserDOMWindow: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aBrowserDOMWindow: *const nsIBrowserDOMWindow) -> nsresult,

    /* void getAttention (); */
    pub getAttention: unsafe extern "C" fn (this: *const nsIDOMChromeWindow) -> nsresult,

    /* void getAttentionWithCycleCount (in long aCycleCount); */
    pub getAttentionWithCycleCount: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aCycleCount: libc::int32_t) -> nsresult,

    /* void setCursor (in DOMString cursor); */
    pub setCursor: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, cursor: *const nsAString) -> nsresult,

    /* void maximize (); */
    pub maximize: unsafe extern "C" fn (this: *const nsIDOMChromeWindow) -> nsresult,

    /* void minimize (); */
    pub minimize: unsafe extern "C" fn (this: *const nsIDOMChromeWindow) -> nsresult,

    /* void restore (); */
    pub restore: unsafe extern "C" fn (this: *const nsIDOMChromeWindow) -> nsresult,

    /* void notifyDefaultButtonLoaded (in nsIDOMElement defaultButton); */
    pub notifyDefaultButtonLoaded: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, defaultButton: *const nsIDOMElement) -> nsresult,

    /* readonly attribute nsIMessageBroadcaster messageManager; */
    pub get_messageManager: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aMessageManager: *mut *const nsIMessageBroadcaster) -> nsresult,

    /* nsIMessageBroadcaster getGroupMessageManager (in AString group); */
    pub getGroupMessageManager: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, group: *const nsAString, _retval: *mut *const nsIMessageBroadcaster) -> nsresult,

    /* void beginWindowMove (in nsIDOMEvent mouseDownEvent, [optional] in nsIDOMElement panel); */
    pub beginWindowMove: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, mouseDownEvent: *const nsIDOMEvent, panel: *const nsIDOMElement) -> nsresult,

    /* void setOpenerForInitialContentBrowser (in mozIDOMWindowProxy aOpener); */
    pub setOpenerForInitialContentBrowser: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, aOpener: *const mozIDOMWindowProxy) -> nsresult,

    /* mozIDOMWindowProxy takeOpenerForInitialContentBrowser (); */
    pub takeOpenerForInitialContentBrowser: unsafe extern "C" fn (this: *const nsIDOMChromeWindow, _retval: *mut *const mozIDOMWindowProxy) -> nsresult,

}


impl nsIDOMChromeWindow {
    /* readonly attribute unsigned short windowState; */
    #[inline]
    pub unsafe fn get_windowState(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_windowState)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute nsIBrowserDOMWindow browserDOMWindow; */
    #[inline]
    pub unsafe fn get_browserDOMWindow(&self, ) -> Result<Option<RefPtr<nsIBrowserDOMWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_browserDOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_browserDOMWindow(&self, aBrowserDOMWindow: Option<&nsIBrowserDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).set_browserDOMWindow)(self as *const _, aBrowserDOMWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getAttention (); */
    #[inline]
    pub unsafe fn getAttention(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).getAttention)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getAttentionWithCycleCount (in long aCycleCount); */
    #[inline]
    pub unsafe fn getAttentionWithCycleCount(&self, aCycleCount: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).getAttentionWithCycleCount)(self as *const _, aCycleCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCursor (in DOMString cursor); */
    #[inline]
    pub unsafe fn setCursor(&self, cursor: &[u16]) -> Result<(), nsresult> {
        let cursor = nsString::from(cursor);
        match ((*self.vtable).setCursor)(self as *const _, &*cursor) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void maximize (); */
    #[inline]
    pub unsafe fn maximize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).maximize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void minimize (); */
    #[inline]
    pub unsafe fn minimize(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).minimize)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void restore (); */
    #[inline]
    pub unsafe fn restore(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).restore)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyDefaultButtonLoaded (in nsIDOMElement defaultButton); */
    #[inline]
    pub unsafe fn notifyDefaultButtonLoaded(&self, defaultButton: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).notifyDefaultButtonLoaded)(self as *const _, defaultButton.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIMessageBroadcaster messageManager; */
    #[inline]
    pub unsafe fn get_messageManager(&self, ) -> Result<Option<RefPtr<nsIMessageBroadcaster>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_messageManager)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIMessageBroadcaster getGroupMessageManager (in AString group); */
    #[inline]
    pub unsafe fn getGroupMessageManager(&self, group: &[u16]) -> Result<Option<RefPtr<nsIMessageBroadcaster>>, nsresult> {
        let group = nsString::from(group);
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getGroupMessageManager)(self as *const _, &*group, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void beginWindowMove (in nsIDOMEvent mouseDownEvent, [optional] in nsIDOMElement panel); */
    #[inline]
    pub unsafe fn beginWindowMove(&self, mouseDownEvent: Option<&nsIDOMEvent>, panel: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).beginWindowMove)(self as *const _, mouseDownEvent.map_or(::std::ptr::null(), |x| x as *const _), panel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setOpenerForInitialContentBrowser (in mozIDOMWindowProxy aOpener); */
    #[inline]
    pub unsafe fn setOpenerForInitialContentBrowser(&self, aOpener: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).setOpenerForInitialContentBrowser)(self as *const _, aOpener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* mozIDOMWindowProxy takeOpenerForInitialContentBrowser (); */
    #[inline]
    pub unsafe fn takeOpenerForInitialContentBrowser(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).takeOpenerForInitialContentBrowser)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


