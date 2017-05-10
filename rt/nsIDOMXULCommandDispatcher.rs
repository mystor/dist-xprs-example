//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDOMXULCommandDispatcher.idl
//


#[repr(C)]
pub struct nsIDOMXULCommandDispatcher {
    vtable: *const nsIDOMXULCommandDispatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDOMXULCommandDispatcher {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa9fa9fd3, 0x8d62, 0x4f94,
            [0x9e, 0xd8, 0x3e, 0xa9, 0xc3, 0xcf, 0x07, 0x73])
    }
}

unsafe impl RefCounted for nsIDOMXULCommandDispatcher {
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
pub trait nsIDOMXULCommandDispatcherCoerce {
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self;
}

impl nsIDOMXULCommandDispatcherCoerce for nsIDOMXULCommandDispatcher {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self {
        v
    }
}

impl nsIDOMXULCommandDispatcher {
    #[inline]
    pub fn coerce<T: nsIDOMXULCommandDispatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDOMXULCommandDispatcher {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDOMXULCommandDispatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMXULCommandDispatcher) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDOMXULCommandDispatcherVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIDOMElement focusedElement; */
    pub get_focusedElement: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedElement: *mut *const nsIDOMElement) -> nsresult,
    pub set_focusedElement: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedElement: *const nsIDOMElement) -> nsresult,

    /* attribute mozIDOMWindowProxy focusedWindow; */
    pub get_focusedWindow: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedWindow: *mut *const mozIDOMWindowProxy) -> nsresult,
    pub set_focusedWindow: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aFocusedWindow: *const mozIDOMWindowProxy) -> nsresult,

    /* void addCommandUpdater (in nsIDOMElement updater, in DOMString events, in DOMString targets); */
    pub addCommandUpdater: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, updater: *const nsIDOMElement, events: *const nsAString, targets: *const nsAString) -> nsresult,

    /* void removeCommandUpdater (in nsIDOMElement updater); */
    pub removeCommandUpdater: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, updater: *const nsIDOMElement) -> nsresult,

    /* void updateCommands (in DOMString eventName); */
    pub updateCommands: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, eventName: *const nsAString) -> nsresult,

    /* nsIController getControllerForCommand (in string command); */
    pub getControllerForCommand: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, command: *const libc::c_char, _retval: *mut *const nsIController) -> nsresult,

    /* nsIControllers getControllers (); */
    pub getControllers: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, _retval: *mut *const nsIControllers) -> nsresult,

    /* void advanceFocus (); */
    pub advanceFocus: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher) -> nsresult,

    /* void rewindFocus (); */
    pub rewindFocus: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher) -> nsresult,

    /* void advanceFocusIntoSubtree (in nsIDOMElement elt); */
    pub advanceFocusIntoSubtree: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, elt: *const nsIDOMElement) -> nsresult,

    /* attribute boolean suppressFocusScroll; */
    pub get_suppressFocusScroll: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aSuppressFocusScroll: *mut bool) -> nsresult,
    pub set_suppressFocusScroll: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher, aSuppressFocusScroll: bool) -> nsresult,

    /* void lock (); */
    pub lock: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher) -> nsresult,

    /* void unlock (); */
    pub unlock: unsafe extern "C" fn (this: *const nsIDOMXULCommandDispatcher) -> nsresult,

}


impl nsIDOMXULCommandDispatcher {
    /* attribute nsIDOMElement focusedElement; */
    #[inline]
    pub unsafe fn get_focusedElement(&self, ) -> Result<Option<RefPtr<nsIDOMElement>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_focusedElement)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_focusedElement(&self, aFocusedElement: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).set_focusedElement)(self as *const _, aFocusedElement.map_or(::std::ptr::null(), |x| x as *const _)) {
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

    /* void addCommandUpdater (in nsIDOMElement updater, in DOMString events, in DOMString targets); */
    #[inline]
    pub unsafe fn addCommandUpdater(&self, updater: Option<&nsIDOMElement>, events: &[u16], targets: &[u16]) -> Result<(), nsresult> {
        let events = nsString::from(events);
        let targets = nsString::from(targets);
        match ((*self.vtable).addCommandUpdater)(self as *const _, updater.map_or(::std::ptr::null(), |x| x as *const _), &*events, &*targets) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeCommandUpdater (in nsIDOMElement updater); */
    #[inline]
    pub unsafe fn removeCommandUpdater(&self, updater: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).removeCommandUpdater)(self as *const _, updater.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateCommands (in DOMString eventName); */
    #[inline]
    pub unsafe fn updateCommands(&self, eventName: &[u16]) -> Result<(), nsresult> {
        let eventName = nsString::from(eventName);
        match ((*self.vtable).updateCommands)(self as *const _, &*eventName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIController getControllerForCommand (in string command); */
    #[inline]
    pub unsafe fn getControllerForCommand(&self, command: *const libc::c_char) -> Result<Option<RefPtr<nsIController>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getControllerForCommand)(self as *const _, command, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsIControllers getControllers (); */
    #[inline]
    pub unsafe fn getControllers(&self, ) -> Result<Option<RefPtr<nsIControllers>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getControllers)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void advanceFocus (); */
    #[inline]
    pub unsafe fn advanceFocus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).advanceFocus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void rewindFocus (); */
    #[inline]
    pub unsafe fn rewindFocus(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).rewindFocus)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void advanceFocusIntoSubtree (in nsIDOMElement elt); */
    #[inline]
    pub unsafe fn advanceFocusIntoSubtree(&self, elt: Option<&nsIDOMElement>) -> Result<(), nsresult> {

        match ((*self.vtable).advanceFocusIntoSubtree)(self as *const _, elt.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean suppressFocusScroll; */
    #[inline]
    pub unsafe fn get_suppressFocusScroll(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_suppressFocusScroll)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_suppressFocusScroll(&self, aSuppressFocusScroll: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_suppressFocusScroll)(self as *const _, aSuppressFocusScroll) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void lock (); */
    #[inline]
    pub unsafe fn lock(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).lock)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void unlock (); */
    #[inline]
    pub unsafe fn unlock(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).unlock)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


