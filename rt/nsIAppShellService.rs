//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAppShellService.idl
//


pub mod nsIAppShellService_consts {
    pub const SIZE_TO_CONTENT: i64 = -1;
}


#[repr(C)]
pub struct nsIAppShellService {
    vtable: *const nsIAppShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAppShellService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x19266025, 0x354c, 0x4bb9,
            [0x98, 0x6b, 0x34, 0x83, 0xb2, 0xb1, 0xcd, 0xef])
    }
}

unsafe impl RefCounted for nsIAppShellService {
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
pub trait nsIAppShellServiceCoerce {
    fn coerce_from(v: &nsIAppShellService) -> &Self;
}

impl nsIAppShellServiceCoerce for nsIAppShellService {
    #[inline]
    fn coerce_from(v: &nsIAppShellService) -> &Self {
        v
    }
}

impl nsIAppShellService {
    #[inline]
    pub fn coerce<T: nsIAppShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAppShellService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIAppShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppShellService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAppShellServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsIXULWindow createTopLevelWindow (in nsIXULWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpenerWindow); */
    pub createTopLevelWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aParent: *const nsIXULWindow, aUrl: *const nsIURI, aChromeMask: uint32_t, aInitialWidth: libc::int32_t, aInitialHeight: libc::int32_t, aOpeningTab: *const nsITabParent, aOpenerWindow: *const mozIDOMWindowProxy, _retval: *mut *const nsIXULWindow) -> nsresult,

    /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome); */
    pub createWindowlessBrowser: unsafe extern "C" fn (this: *const nsIAppShellService, aIsChrome: bool, _retval: *mut *const nsIWindowlessBrowser) -> nsresult,

    /* [noscript] void createHiddenWindow (); */
    pub createHiddenWindow: unsafe extern "C" fn (this: *const nsIAppShellService) -> nsresult,

    /* void destroyHiddenWindow (); */
    pub destroyHiddenWindow: unsafe extern "C" fn (this: *const nsIAppShellService) -> nsresult,

    /* [noscript] void setScreenId (in uint32_t aScreenId); */
    pub setScreenId: unsafe extern "C" fn (this: *const nsIAppShellService, aScreenId: uint32_t) -> nsresult,

    /* readonly attribute nsIXULWindow hiddenWindow; */
    pub get_hiddenWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aHiddenWindow: *mut *const nsIXULWindow) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
    pub get_hiddenDOMWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aHiddenDOMWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute nsIXULWindow hiddenPrivateWindow; */
    pub get_hiddenPrivateWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aHiddenPrivateWindow: *mut *const nsIXULWindow) -> nsresult,

    /* readonly attribute mozIDOMWindowProxy hiddenPrivateDOMWindow; */
    pub get_hiddenPrivateDOMWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aHiddenPrivateDOMWindow: *mut *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute boolean applicationProvidedHiddenWindow; */
    pub get_applicationProvidedHiddenWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aApplicationProvidedHiddenWindow: *mut bool) -> nsresult,

    /* void registerTopLevelWindow (in nsIXULWindow aWindow); */
    pub registerTopLevelWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aWindow: *const nsIXULWindow) -> nsresult,

    /* void unregisterTopLevelWindow (in nsIXULWindow aWindow); */
    pub unregisterTopLevelWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aWindow: *const nsIXULWindow) -> nsresult,

    /* [noscript] readonly attribute boolean hasHiddenPrivateWindow; */
    pub get_hasHiddenPrivateWindow: unsafe extern "C" fn (this: *const nsIAppShellService, aHasHiddenPrivateWindow: *mut bool) -> nsresult,

    /* bool startEventLoopLagTracking (); */
    pub startEventLoopLagTracking: unsafe extern "C" fn (this: *const nsIAppShellService, _retval: *mut bool) -> nsresult,

    /* void stopEventLoopLagTracking (); */
    pub stopEventLoopLagTracking: unsafe extern "C" fn (this: *const nsIAppShellService) -> nsresult,

}


impl nsIAppShellService {
    /* nsIXULWindow createTopLevelWindow (in nsIXULWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight, in nsITabParent aOpeningTab, in mozIDOMWindowProxy aOpenerWindow); */
    #[inline]
    pub unsafe fn createTopLevelWindow(&self, aParent: Option<&nsIXULWindow>, aUrl: Option<&nsIURI>, aChromeMask: uint32_t, aInitialWidth: libc::int32_t, aInitialHeight: libc::int32_t, aOpeningTab: Option<&nsITabParent>, aOpenerWindow: Option<&mozIDOMWindowProxy>) -> Result<Option<RefPtr<nsIXULWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createTopLevelWindow)(self as *const _, aParent.map_or(::std::ptr::null(), |x| x as *const _), aUrl.map_or(::std::ptr::null(), |x| x as *const _), aChromeMask, aInitialWidth, aInitialHeight, aOpeningTab.map_or(::std::ptr::null(), |x| x as *const _), aOpenerWindow.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome); */
    #[inline]
    pub unsafe fn createWindowlessBrowser(&self, aIsChrome: bool) -> Result<Option<RefPtr<nsIWindowlessBrowser>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createWindowlessBrowser)(self as *const _, aIsChrome, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void createHiddenWindow (); */
    #[inline]
    pub unsafe fn createHiddenWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).createHiddenWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void destroyHiddenWindow (); */
    #[inline]
    pub unsafe fn destroyHiddenWindow(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).destroyHiddenWindow)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setScreenId (in uint32_t aScreenId); */
    #[inline]
    pub unsafe fn setScreenId(&self, aScreenId: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setScreenId)(self as *const _, aScreenId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIXULWindow hiddenWindow; */
    #[inline]
    pub unsafe fn get_hiddenWindow(&self, ) -> Result<Option<RefPtr<nsIXULWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_hiddenWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
    #[inline]
    pub unsafe fn get_hiddenDOMWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_hiddenDOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIXULWindow hiddenPrivateWindow; */
    #[inline]
    pub unsafe fn get_hiddenPrivateWindow(&self, ) -> Result<Option<RefPtr<nsIXULWindow>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_hiddenPrivateWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute mozIDOMWindowProxy hiddenPrivateDOMWindow; */
    #[inline]
    pub unsafe fn get_hiddenPrivateDOMWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_hiddenPrivateDOMWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean applicationProvidedHiddenWindow; */
    #[inline]
    pub unsafe fn get_applicationProvidedHiddenWindow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_applicationProvidedHiddenWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void registerTopLevelWindow (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn registerTopLevelWindow(&self, aWindow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).registerTopLevelWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unregisterTopLevelWindow (in nsIXULWindow aWindow); */
    #[inline]
    pub unsafe fn unregisterTopLevelWindow(&self, aWindow: Option<&nsIXULWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).unregisterTopLevelWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute boolean hasHiddenPrivateWindow; */
    #[inline]
    pub unsafe fn get_hasHiddenPrivateWindow(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasHiddenPrivateWindow)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* bool startEventLoopLagTracking (); */
    #[inline]
    pub unsafe fn startEventLoopLagTracking(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).startEventLoopLagTracking)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void stopEventLoopLagTracking (); */
    #[inline]
    pub unsafe fn stopEventLoopLagTracking(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).stopEventLoopLagTracking)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


