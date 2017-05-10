//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISlowScriptDebug.idl
//


#[repr(C)]
pub struct nsISlowScriptDebugCallback {
    vtable: *const nsISlowScriptDebugCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISlowScriptDebugCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf7dbb80c, 0x5d1e, 0x4fd9,
            [0xb5, 0x5c, 0xa9, 0xff, 0xda, 0x4a, 0x75, 0xb1])
    }
}

unsafe impl RefCounted for nsISlowScriptDebugCallback {
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
pub trait nsISlowScriptDebugCallbackCoerce {
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self;
}

impl nsISlowScriptDebugCallbackCoerce for nsISlowScriptDebugCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebugCallback {
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISlowScriptDebugCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISlowScriptDebugCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISlowScriptDebugCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
    pub handleSlowScriptDebug: unsafe extern "C" fn (this: *const nsISlowScriptDebugCallback, aWindow: *const nsIDOMWindow) -> nsresult,

}


impl nsISlowScriptDebugCallback {
    /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
    #[inline]
    pub unsafe fn handleSlowScriptDebug(&self, aWindow: Option<&nsIDOMWindow>) -> Result<(), nsresult> {

        match ((*self.vtable).handleSlowScriptDebug)(self as *const _, aWindow.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISlowScriptDebuggerStartupCallback {
    vtable: *const nsISlowScriptDebuggerStartupCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISlowScriptDebuggerStartupCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb1c6ecd0, 0x8fa4, 0x11e4,
            [0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsISlowScriptDebuggerStartupCallback {
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
pub trait nsISlowScriptDebuggerStartupCallbackCoerce {
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self;
}

impl nsISlowScriptDebuggerStartupCallbackCoerce for nsISlowScriptDebuggerStartupCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebuggerStartupCallback {
    #[inline]
    pub fn coerce<T: nsISlowScriptDebuggerStartupCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISlowScriptDebuggerStartupCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISlowScriptDebuggerStartupCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISlowScriptDebuggerStartupCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void finishDebuggerStartup (); */
    pub finishDebuggerStartup: unsafe extern "C" fn (this: *const nsISlowScriptDebuggerStartupCallback) -> nsresult,

}


impl nsISlowScriptDebuggerStartupCallback {
    /* void finishDebuggerStartup (); */
    #[inline]
    pub unsafe fn finishDebuggerStartup(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).finishDebuggerStartup)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISlowScriptDebugRemoteCallback {
    vtable: *const nsISlowScriptDebugRemoteCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISlowScriptDebugRemoteCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdbee14b0, 0x8fa0, 0x11e4,
            [0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsISlowScriptDebugRemoteCallback {
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
pub trait nsISlowScriptDebugRemoteCallbackCoerce {
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self;
}

impl nsISlowScriptDebugRemoteCallbackCoerce for nsISlowScriptDebugRemoteCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebugRemoteCallback {
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugRemoteCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISlowScriptDebugRemoteCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISlowScriptDebugRemoteCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISlowScriptDebugRemoteCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void handleSlowScriptDebug (in nsIDOMEventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
    pub handleSlowScriptDebug: unsafe extern "C" fn (this: *const nsISlowScriptDebugRemoteCallback, aBrowser: *const nsIDOMEventTarget, aCallback: *const nsISlowScriptDebuggerStartupCallback) -> nsresult,

}


impl nsISlowScriptDebugRemoteCallback {
    /* void handleSlowScriptDebug (in nsIDOMEventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
    #[inline]
    pub unsafe fn handleSlowScriptDebug(&self, aBrowser: Option<&nsIDOMEventTarget>, aCallback: Option<&nsISlowScriptDebuggerStartupCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).handleSlowScriptDebug)(self as *const _, aBrowser.map_or(::std::ptr::null(), |x| x as *const _), aCallback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsISlowScriptDebug {
    vtable: *const nsISlowScriptDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISlowScriptDebug {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf75d4164, 0x3aa7, 0x4395,
            [0xba, 0x44, 0xa5, 0xf9, 0x5b, 0x2e, 0x84, 0x27])
    }
}

unsafe impl RefCounted for nsISlowScriptDebug {
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
pub trait nsISlowScriptDebugCoerce {
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self;
}

impl nsISlowScriptDebugCoerce for nsISlowScriptDebug {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self {
        v
    }
}

impl nsISlowScriptDebug {
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISlowScriptDebug {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISlowScriptDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISlowScriptDebugVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsISlowScriptDebugCallback activationHandler; */
    pub get_activationHandler: unsafe extern "C" fn (this: *const nsISlowScriptDebug, aActivationHandler: *mut *const nsISlowScriptDebugCallback) -> nsresult,
    pub set_activationHandler: unsafe extern "C" fn (this: *const nsISlowScriptDebug, aActivationHandler: *const nsISlowScriptDebugCallback) -> nsresult,

    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
    pub get_remoteActivationHandler: unsafe extern "C" fn (this: *const nsISlowScriptDebug, aRemoteActivationHandler: *mut *const nsISlowScriptDebugRemoteCallback) -> nsresult,
    pub set_remoteActivationHandler: unsafe extern "C" fn (this: *const nsISlowScriptDebug, aRemoteActivationHandler: *const nsISlowScriptDebugRemoteCallback) -> nsresult,

}


impl nsISlowScriptDebug {
    /* attribute nsISlowScriptDebugCallback activationHandler; */
    #[inline]
    pub unsafe fn get_activationHandler(&self, ) -> Result<Option<RefPtr<nsISlowScriptDebugCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_activationHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_activationHandler(&self, aActivationHandler: Option<&nsISlowScriptDebugCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_activationHandler)(self as *const _, aActivationHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
    #[inline]
    pub unsafe fn get_remoteActivationHandler(&self, ) -> Result<Option<RefPtr<nsISlowScriptDebugRemoteCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_remoteActivationHandler)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_remoteActivationHandler(&self, aRemoteActivationHandler: Option<&nsISlowScriptDebugRemoteCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).set_remoteActivationHandler)(self as *const _, aRemoteActivationHandler.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


