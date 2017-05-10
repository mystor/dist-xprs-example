//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICommandManager.idl
//


#[repr(C)]
pub struct nsICommandManager {
    vtable: *const nsICommandManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICommandManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbb5a1730, 0xd83b, 0x4fa2,
            [0x83, 0x1b, 0x35, 0xb9, 0xd5, 0x84, 0x2e, 0x84])
    }
}

unsafe impl RefCounted for nsICommandManager {
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
pub trait nsICommandManagerCoerce {
    fn coerce_from(v: &nsICommandManager) -> &Self;
}

impl nsICommandManagerCoerce for nsICommandManager {
    #[inline]
    fn coerce_from(v: &nsICommandManager) -> &Self {
        v
    }
}

impl nsICommandManager {
    #[inline]
    pub fn coerce<T: nsICommandManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICommandManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICommandManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICommandManagerVTable {
    pub __base: nsISupportsVTable,

    /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
    pub addCommandObserver: unsafe extern "C" fn (this: *const nsICommandManager, aCommandObserver: *const nsIObserver, aCommandToObserve: *const libc::c_char) -> nsresult,

    /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
    pub removeCommandObserver: unsafe extern "C" fn (this: *const nsICommandManager, aCommandObserver: *const nsIObserver, aCommandObserved: *const libc::c_char) -> nsresult,

    /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    pub isCommandSupported: unsafe extern "C" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> nsresult,

    /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    pub isCommandEnabled: unsafe extern "C" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, _retval: *mut bool) -> nsresult,

    /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
    pub getCommandState: unsafe extern "C" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aTargetWindow: *const mozIDOMWindowProxy, aCommandParams: *const nsICommandParams) -> nsresult,

    /* void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
    pub doCommand: unsafe extern "C" fn (this: *const nsICommandManager, aCommandName: *const libc::c_char, aCommandParams: *const nsICommandParams, aTargetWindow: *const mozIDOMWindowProxy) -> nsresult,

}


impl nsICommandManager {
    /* void addCommandObserver (in nsIObserver aCommandObserver, in string aCommandToObserve); */
    #[inline]
    pub unsafe fn addCommandObserver(&self, aCommandObserver: Option<&nsIObserver>, aCommandToObserve: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).addCommandObserver)(self as *const _, aCommandObserver.map_or(::std::ptr::null(), |x| x as *const _), aCommandToObserve) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeCommandObserver (in nsIObserver aCommandObserver, in string aCommandObserved); */
    #[inline]
    pub unsafe fn removeCommandObserver(&self, aCommandObserver: Option<&nsIObserver>, aCommandObserved: *const libc::c_char) -> Result<(), nsresult> {

        match ((*self.vtable).removeCommandObserver)(self as *const _, aCommandObserver.map_or(::std::ptr::null(), |x| x as *const _), aCommandObserved) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isCommandSupported (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    #[inline]
    pub unsafe fn isCommandSupported(&self, aCommandName: *const libc::c_char, aTargetWindow: Option<&mozIDOMWindowProxy>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandSupported)(self as *const _, aCommandName, aTargetWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean isCommandEnabled (in string aCommandName, in mozIDOMWindowProxy aTargetWindow); */
    #[inline]
    pub unsafe fn isCommandEnabled(&self, aCommandName: *const libc::c_char, aTargetWindow: Option<&mozIDOMWindowProxy>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isCommandEnabled)(self as *const _, aCommandName, aTargetWindow.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void getCommandState (in string aCommandName, in mozIDOMWindowProxy aTargetWindow, in nsICommandParams aCommandParams); */
    #[inline]
    pub unsafe fn getCommandState(&self, aCommandName: *const libc::c_char, aTargetWindow: Option<&mozIDOMWindowProxy>, aCommandParams: Option<&nsICommandParams>) -> Result<(), nsresult> {

        match ((*self.vtable).getCommandState)(self as *const _, aCommandName, aTargetWindow.map_or(::std::ptr::null(), |x| x as *const _), aCommandParams.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void doCommand (in string aCommandName, in nsICommandParams aCommandParams, in mozIDOMWindowProxy aTargetWindow); */
    #[inline]
    pub unsafe fn doCommand(&self, aCommandName: *const libc::c_char, aCommandParams: Option<&nsICommandParams>, aTargetWindow: Option<&mozIDOMWindowProxy>) -> Result<(), nsresult> {

        match ((*self.vtable).doCommand)(self as *const _, aCommandName, aCommandParams.map_or(::std::ptr::null(), |x| x as *const _), aTargetWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


