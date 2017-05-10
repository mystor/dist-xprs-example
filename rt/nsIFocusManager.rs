//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIFocusManager.idl
//


pub mod nsIFocusManager_consts {
    pub const FLAG_RAISE: i64 = 1;
    pub const FLAG_NOSCROLL: i64 = 2;
    pub const FLAG_NOSWITCHFRAME: i64 = 4;
    pub const FLAG_NOPARENTFRAME: i64 = 8;
    pub const FLAG_BYMOUSE: i64 = 4096;
    pub const FLAG_BYKEY: i64 = 8192;
    pub const FLAG_BYMOVEFOCUS: i64 = 16384;
    pub const FLAG_SHOWRING: i64 = 1048576;
    pub const FLAG_BYTOUCH: i64 = 2097152;
    pub const MOVEFOCUS_FORWARD: i64 = 1;
    pub const MOVEFOCUS_BACKWARD: i64 = 2;
    pub const MOVEFOCUS_FORWARDDOC: i64 = 3;
    pub const MOVEFOCUS_BACKWARDDOC: i64 = 4;
    pub const MOVEFOCUS_FIRST: i64 = 5;
    pub const MOVEFOCUS_LAST: i64 = 6;
    pub const MOVEFOCUS_ROOT: i64 = 7;
    pub const MOVEFOCUS_CARET: i64 = 8;
    pub const MOVEFOCUS_FIRSTDOC: i64 = 9;
    pub const MOVEFOCUS_LASTDOC: i64 = 10;
}


#[repr(C)]
pub struct nsIFocusManager {
    vtable: *const nsIFocusManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIFocusManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x86e1f1e1, 0x365d, 0x493b,
            [0xb5, 0x2a, 0xa6, 0x49, 0xf3, 0xf3, 0x11, 0xdc])
    }
}

unsafe impl RefCounted for nsIFocusManager {
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
pub trait nsIFocusManagerCoerce {
    fn coerce_from(v: &nsIFocusManager) -> &Self;
}

impl nsIFocusManagerCoerce for nsIFocusManager {
    #[inline]
    fn coerce_from(v: &nsIFocusManager) -> &Self {
        v
    }
}

impl nsIFocusManager {
    #[inline]
    pub fn coerce<T: nsIFocusManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIFocusManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIFocusManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFocusManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIFocusManagerVTable {
    pub __base: nsISupportsVTable,

    /* attribute mozIDOMWindowProxy activeWindow; */
    pub get_activeWindow: unsafe extern "C" fn (this: *const nsIFocusManager, aActiveWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_activeWindow: unsafe extern "C" fn (this: *const nsIFocusManager, aActiveWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* attribute mozIDOMWindowProxy focusedWindow; */
    pub get_focusedWindow: unsafe extern "C" fn (this: *const nsIFocusManager, aFocusedWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_focusedWindow: unsafe extern "C" fn (this: *const nsIFocusManager, aFocusedWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* readonly attribute nsIDOMElement focusedElement; */
    pub get_focusedElement: unsafe extern "C" fn (this: *const nsIFocusManager, aFocusedElement: *mut *const nsIDOMElement) -> nsresult,

    /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
    pub getLastFocusMethod: unsafe extern "C" fn (this: *const nsIFocusManager, window: *const mozIDOMWindowProxy, _retval: *mut uint32_t) -> nsresult,

    /* void setFocus (in nsIDOMElement aElement, in unsigned long aFlags); */
    pub setFocus: unsafe extern "C" fn (this: *const nsIFocusManager, aElement: *const nsIDOMElement, aFlags: libc::uint32_t) -> nsresult,

    /* nsIDOMElement moveFocus (in mozIDOMWindowProxy aWindow, in nsIDOMElement aStartElement, in unsigned long aType, in unsigned long aFlags); */
    pub moveFocus: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, aStartElement: *const nsIDOMElement, aType: libc::uint32_t, aFlags: libc::uint32_t, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void clearFocus (in mozIDOMWindowProxy aWindow); */
    pub clearFocus: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* nsIDOMElement getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
    pub getFocusedElementForWindow: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, aDeep: bool, aFocusedWindow: *mut *const mozIDOMWindowProxy, _retval: *mut *const nsIDOMElement) -> nsresult,

    /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
    pub moveCaretToFocus: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* boolean elementIsFocusable (in nsIDOMElement aElement, in unsigned long aFlags); */
    pub elementIsFocusable: unsafe extern "C" fn (this: *const nsIFocusManager, aElement: *const nsIDOMElement, aFlags: libc::uint32_t, _retval: *mut bool) -> nsresult,

    /* [noscript] void windowRaised (in mozIDOMWindowProxy aWindow); */
    pub windowRaised: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* [noscript] void windowLowered (in mozIDOMWindowProxy aWindow); */
    pub windowLowered: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* [noscript] void windowShown (in mozIDOMWindowProxy aWindow, in boolean aNeedsFocus); */
    pub windowShown: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, aNeedsFocus: bool) -> nsresult,

    /* [noscript] void windowHidden (in mozIDOMWindowProxy aWindow); */
    pub windowHidden: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* [noscript] void fireDelayedEvents (in nsIDocument aDocument); */
    pub fireDelayedEvents: unsafe extern "C" fn (this: *const nsIFocusManager, aDocument: *const nsIDocument) -> nsresult,

    /* [noscript] void focusPlugin (in nsIContent aPlugin); */
    pub focusPlugin: unsafe extern "C" fn (this: *const nsIFocusManager, aPlugin: *const nsIContent) -> nsresult,

    /* [noscript] void parentActivated (in mozIDOMWindowProxy aWindow, in bool active); */
    pub parentActivated: unsafe extern "C" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, active: bool) -> nsresult,

}


impl nsIFocusManager {
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

    /* attribute mozIDOMWindowProxy focusedWindow; */
    #[inline]
    pub unsafe fn get_focusedWindow(&self, ) -> Result<Option<RefPtr<mozIDOMWindowProxy>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedWindow)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_focusedWindow(&self, aFocusedWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).set_focusedWindow)(self as *const _, aFocusedWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIDOMElement focusedElement; */
    #[inline]
    pub unsafe fn get_focusedElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
    #[inline]
    pub unsafe fn getLastFocusMethod(&self, window: Option<&mozIDOMWindowProxy>) -> Result<uint32_t, nsresult> {
        let mut _retval: uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getLastFocusMethod)(self as *const _, window.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setFocus (in nsIDOMElement aElement, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn setFocus(&self, aElement: Option<&nsIDOMElement>, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setFocus)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement moveFocus (in mozIDOMWindowProxy aWindow, in nsIDOMElement aStartElement, in unsigned long aType, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn moveFocus(&self, aWindow: Option<&mozIDOMWindowProxy>, aStartElement: Option<&nsIDOMElement>, aType: libc::uint32_t, aFlags: libc::uint32_t) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).moveFocus)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aStartElement.map_or(::std::ptr::null(), |x| x as *const _), aType, aFlags, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void clearFocus (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn clearFocus(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).clearFocus)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIDOMElement getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
    #[inline]
    pub unsafe fn getFocusedElementForWindow(&self, aWindow: Option<&mozIDOMWindowProxy>, aDeep: bool) -> Result<(Option<RefPtr<mozIDOMWindowProxy>>, Option<RefPtr<nsIDOMElement>>), nsresult> {
        let mut aFocusedWindow = GetterAddrefs::new();
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getFocusedElementForWindow)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aDeep, aFocusedWindow.ptr(), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok((aFocusedWindow.refptr(), _retval.refptr()))
    }

    /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn moveCaretToFocus(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).moveCaretToFocus)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean elementIsFocusable (in nsIDOMElement aElement, in unsigned long aFlags); */
    #[inline]
    pub unsafe fn elementIsFocusable(&self, aElement: Option<&nsIDOMElement>, aFlags: libc::uint32_t) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).elementIsFocusable)(self as *const _, aElement.map_or(::std::ptr::null(), |x| x as *const _), aFlags, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] void windowRaised (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn windowRaised(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).windowRaised)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void windowLowered (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn windowLowered(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).windowLowered)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void windowShown (in mozIDOMWindowProxy aWindow, in boolean aNeedsFocus); */
    #[inline]
    pub unsafe fn windowShown(&self, aWindow: Option<&mozIDOMWindowProxy>, aNeedsFocus: bool) -> Result<(), nsresult> {

        match ((*self.vtable).windowShown)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aNeedsFocus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void windowHidden (in mozIDOMWindowProxy aWindow); */
    #[inline]
    pub unsafe fn windowHidden(&self, aWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).windowHidden)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void fireDelayedEvents (in nsIDocument aDocument); */
    #[inline]
    pub unsafe fn fireDelayedEvents(&self, aDocument: Option<&nsIDocument>) -> Result<(), nsresult> {

        match ((*self.vtable).fireDelayedEvents)(self as *const _, aDocument.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void focusPlugin (in nsIContent aPlugin); */
    #[inline]
    pub unsafe fn focusPlugin(&self, aPlugin: Option<&nsIContent>) -> Result<(), nsresult> {

        match ((*self.vtable).focusPlugin)(self as *const _, aPlugin.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void parentActivated (in mozIDOMWindowProxy aWindow, in bool active); */
    #[inline]
    pub unsafe fn parentActivated(&self, aWindow: Option<&mozIDOMWindowProxy>, active: bool) -> Result<(), nsresult> {

        match ((*self.vtable).parentActivated)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _), active) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


