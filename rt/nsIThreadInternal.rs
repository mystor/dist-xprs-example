//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadInternal.idl
//


#[repr(C)]
pub struct nsIThreadInternal {
    vtable: *const nsIThreadInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa3a72e5f, 0x71d9, 0x4add,
            [0x8f, 0x30, 0x59, 0xa7, 0x8f, 0xb6, 0xd5, 0xeb])
    }
}

unsafe impl RefCounted for nsIThreadInternal {
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
pub trait nsIThreadInternalCoerce {
    fn coerce_from(v: &nsIThreadInternal) -> &Self;
}

impl nsIThreadInternalCoerce for nsIThreadInternal {
    #[inline]
    fn coerce_from(v: &nsIThreadInternal) -> &Self {
        v
    }
}

impl nsIThreadInternal {
    #[inline]
    pub fn coerce<T: nsIThreadInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadInternal {
    type Target = nsIThread;
    #[inline]
    fn deref(&self) -> &nsIThread {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIThreadCoerce> nsIThreadInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadInternalVTable {
    pub __base: nsIThreadVTable,

    /* attribute nsIThreadObserver observer; */
    pub get_observer: unsafe extern "C" fn (this: *const nsIThreadInternal, aObserver: *mut *const nsIThreadObserver) -> nsresult,
    pub set_observer: unsafe extern "C" fn (this: *const nsIThreadInternal, aObserver: *const nsIThreadObserver) -> nsresult,

    /* void addObserver (in nsIThreadObserver observer); */
    pub addObserver: unsafe extern "C" fn (this: *const nsIThreadInternal, observer: *const nsIThreadObserver) -> nsresult,

    /* void removeObserver (in nsIThreadObserver observer); */
    pub removeObserver: unsafe extern "C" fn (this: *const nsIThreadInternal, observer: *const nsIThreadObserver) -> nsresult,

    /* [noscript] nsIEventTarget pushEventQueue (); */
    pub pushEventQueue: unsafe extern "C" fn (this: *const nsIThreadInternal, _retval: *mut *const nsIEventTarget) -> nsresult,

    /* [noscript] void popEventQueue (in nsIEventTarget aInnermostTarget); */
    pub popEventQueue: unsafe extern "C" fn (this: *const nsIThreadInternal, aInnermostTarget: *const nsIEventTarget) -> nsresult,

}


impl nsIThreadInternal {
    /* attribute nsIThreadObserver observer; */
    #[inline]
    pub unsafe fn get_observer(&self, ) -> Result<Option<RefPtr<nsIThreadObserver>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_observer)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_observer(&self, aObserver: Option<&nsIThreadObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).set_observer)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void addObserver (in nsIThreadObserver observer); */
    #[inline]
    pub unsafe fn addObserver(&self, observer: Option<&nsIThreadObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).addObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeObserver (in nsIThreadObserver observer); */
    #[inline]
    pub unsafe fn removeObserver(&self, observer: Option<&nsIThreadObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).removeObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] nsIEventTarget pushEventQueue (); */
    #[inline]
    pub unsafe fn pushEventQueue(&self, ) -> Result<Option<RefPtr<nsIEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).pushEventQueue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void popEventQueue (in nsIEventTarget aInnermostTarget); */
    #[inline]
    pub unsafe fn popEventQueue(&self, aInnermostTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).popEventQueue)(self as *const _, aInnermostTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIThreadObserver {
    vtable: *const nsIThreadObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcc8da053, 0x1776, 0x44c2,
            [0x91, 0x99, 0xb5, 0xa6, 0x29, 0xd0, 0xa1, 0x9d])
    }
}

unsafe impl RefCounted for nsIThreadObserver {
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
pub trait nsIThreadObserverCoerce {
    fn coerce_from(v: &nsIThreadObserver) -> &Self;
}

impl nsIThreadObserverCoerce for nsIThreadObserver {
    #[inline]
    fn coerce_from(v: &nsIThreadObserver) -> &Self {
        v
    }
}

impl nsIThreadObserver {
    #[inline]
    pub fn coerce<T: nsIThreadObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThreadObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onDispatchedEvent (in nsIThreadInternal thread); */
    pub onDispatchedEvent: unsafe extern "C" fn (this: *const nsIThreadObserver, thread: *const nsIThreadInternal) -> nsresult,

    /* void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait); */
    pub onProcessNextEvent: unsafe extern "C" fn (this: *const nsIThreadObserver, thread: *const nsIThreadInternal, mayWait: bool) -> nsresult,

    /* void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed); */
    pub afterProcessNextEvent: unsafe extern "C" fn (this: *const nsIThreadObserver, thread: *const nsIThreadInternal, eventWasProcessed: bool) -> nsresult,

}


impl nsIThreadObserver {
    /* void onDispatchedEvent (in nsIThreadInternal thread); */
    #[inline]
    pub unsafe fn onDispatchedEvent(&self, thread: Option<&nsIThreadInternal>) -> Result<(), nsresult> {

        match ((*self.vtable).onDispatchedEvent)(self as *const _, thread.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onProcessNextEvent (in nsIThreadInternal thread, in boolean mayWait); */
    #[inline]
    pub unsafe fn onProcessNextEvent(&self, thread: Option<&nsIThreadInternal>, mayWait: bool) -> Result<(), nsresult> {

        match ((*self.vtable).onProcessNextEvent)(self as *const _, thread.map_or(::std::ptr::null(), |x| x as *const _), mayWait) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void afterProcessNextEvent (in nsIThreadInternal thread, in bool eventWasProcessed); */
    #[inline]
    pub unsafe fn afterProcessNextEvent(&self, thread: Option<&nsIThreadInternal>, eventWasProcessed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).afterProcessNextEvent)(self as *const _, thread.map_or(::std::ptr::null(), |x| x as *const _), eventWasProcessed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


