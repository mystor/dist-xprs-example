//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWorkerDebuggerManager.idl
//


#[repr(C)]
pub struct nsIWorkerDebuggerManagerListener {
    vtable: *const nsIWorkerDebuggerManagerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerDebuggerManagerListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd2aa74ee, 0x6b98, 0x4d5d,
            [0x81, 0x73, 0x4e, 0x23, 0x42, 0x2d, 0xaf, 0x1e])
    }
}

unsafe impl RefCounted for nsIWorkerDebuggerManagerListener {
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
pub trait nsIWorkerDebuggerManagerListenerCoerce {
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self;
}

impl nsIWorkerDebuggerManagerListenerCoerce for nsIWorkerDebuggerManagerListener {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerManagerListener {
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerManagerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerDebuggerManagerListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerDebuggerManagerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerDebuggerManagerListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onRegister (in nsIWorkerDebugger debugger); */
    pub onRegister: unsafe extern "C" fn (this: *const nsIWorkerDebuggerManagerListener, debugger: *const nsIWorkerDebugger) -> nsresult,

    /* void onUnregister (in nsIWorkerDebugger debugger); */
    pub onUnregister: unsafe extern "C" fn (this: *const nsIWorkerDebuggerManagerListener, debugger: *const nsIWorkerDebugger) -> nsresult,

}


impl nsIWorkerDebuggerManagerListener {
    /* void onRegister (in nsIWorkerDebugger debugger); */
    #[inline]
    pub unsafe fn onRegister(&self, debugger: Option<&nsIWorkerDebugger>) -> Result<(), nsresult> {

        match ((*self.vtable).onRegister)(self as *const _, debugger.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onUnregister (in nsIWorkerDebugger debugger); */
    #[inline]
    pub unsafe fn onUnregister(&self, debugger: Option<&nsIWorkerDebugger>) -> Result<(), nsresult> {

        match ((*self.vtable).onUnregister)(self as *const _, debugger.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIWorkerDebuggerManager {
    vtable: *const nsIWorkerDebuggerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWorkerDebuggerManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x056d7918, 0xdc86, 0x452a,
            [0xb4, 0xe6, 0x86, 0xda, 0x34, 0x05, 0xf0, 0x15])
    }
}

unsafe impl RefCounted for nsIWorkerDebuggerManager {
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
pub trait nsIWorkerDebuggerManagerCoerce {
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self;
}

impl nsIWorkerDebuggerManagerCoerce for nsIWorkerDebuggerManager {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerManager {
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWorkerDebuggerManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWorkerDebuggerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWorkerDebuggerManagerVTable {
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
    pub getWorkerDebuggerEnumerator: unsafe extern "C" fn (this: *const nsIWorkerDebuggerManager, _retval: *mut *const nsISimpleEnumerator) -> nsresult,

    /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
    pub addListener: unsafe extern "C" fn (this: *const nsIWorkerDebuggerManager, listener: *const nsIWorkerDebuggerManagerListener) -> nsresult,

    /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
    pub removeListener: unsafe extern "C" fn (this: *const nsIWorkerDebuggerManager, listener: *const nsIWorkerDebuggerManagerListener) -> nsresult,

}


impl nsIWorkerDebuggerManager {
    /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
    #[inline]
    pub unsafe fn getWorkerDebuggerEnumerator(&self, ) -> Result<Option<RefPtr<nsISimpleEnumerator>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getWorkerDebuggerEnumerator)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
    #[inline]
    pub unsafe fn addListener(&self, listener: Option<&nsIWorkerDebuggerManagerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).addListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
    #[inline]
    pub unsafe fn removeListener(&self, listener: Option<&nsIWorkerDebuggerManagerListener>) -> Result<(), nsresult> {

        match ((*self.vtable).removeListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


