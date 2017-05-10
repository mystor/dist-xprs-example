//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThread.idl
//


#[repr(C)]
pub struct nsIThread {
    vtable: *const nsIThreadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThread {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5801d193, 0x29d1, 0x4964,
            [0xa6, 0xb7, 0x70, 0xeb, 0x69, 0x7d, 0xdf, 0x2b])
    }
}

unsafe impl RefCounted for nsIThread {
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
pub trait nsIThreadCoerce {
    fn coerce_from(v: &nsIThread) -> &Self;
}

impl nsIThreadCoerce for nsIThread {
    #[inline]
    fn coerce_from(v: &nsIThread) -> &Self {
        v
    }
}

impl nsIThread {
    #[inline]
    pub fn coerce<T: nsIThreadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThread {
    type Target = nsIEventTarget;
    #[inline]
    fn deref(&self) -> &nsIEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIEventTargetCoerce> nsIThreadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThread) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadVTable {
    pub __base: nsIEventTargetVTable,

    /* [noscript] readonly attribute PRThread PRThread; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_PRThread: *const ::libc::c_void,

    /* [noscript] attribute boolean CanInvokeJS; */
    pub get_CanInvokeJS: unsafe extern "C" fn (this: *const nsIThread, aCanInvokeJS: *mut bool) -> nsresult,
    pub set_CanInvokeJS: unsafe extern "C" fn (this: *const nsIThread, aCanInvokeJS: bool) -> nsresult,

    /* void shutdown (); */
    pub shutdown: unsafe extern "C" fn (this: *const nsIThread) -> nsresult,

    /* boolean hasPendingEvents (); */
    pub hasPendingEvents: unsafe extern "C" fn (this: *const nsIThread, _retval: *mut bool) -> nsresult,

    /* boolean processNextEvent (in boolean mayWait); */
    pub processNextEvent: unsafe extern "C" fn (this: *const nsIThread, mayWait: bool, _retval: *mut bool) -> nsresult,

    /* void asyncShutdown (); */
    pub asyncShutdown: unsafe extern "C" fn (this: *const nsIThread) -> nsresult,

    /* [noscript] void registerIdlePeriod (in alreadyAddRefed_nsIIdlePeriod aIdlePeriod); */
    /// Unable to call function as its signature contains a non-rust type
    pub registerIdlePeriod: *const ::libc::c_void,

    /* [noscript] void idleDispatch (in alreadyAddRefed_nsIRunnable event); */
    /// Unable to call function as its signature contains a non-rust type
    pub idleDispatch: *const ::libc::c_void,

}


impl nsIThread {
    /* [noscript] readonly attribute PRThread PRThread; */


    /* [noscript] attribute boolean CanInvokeJS; */
    #[inline]
    pub unsafe fn get_CanInvokeJS(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_CanInvokeJS)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_CanInvokeJS(&self, aCanInvokeJS: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_CanInvokeJS)(self as *const _, aCanInvokeJS) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void shutdown (); */
    #[inline]
    pub unsafe fn shutdown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).shutdown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean hasPendingEvents (); */
    #[inline]
    pub unsafe fn hasPendingEvents(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).hasPendingEvents)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* boolean processNextEvent (in boolean mayWait); */
    #[inline]
    pub unsafe fn processNextEvent(&self, mayWait: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).processNextEvent)(self as *const _, mayWait, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void asyncShutdown (); */
    #[inline]
    pub unsafe fn asyncShutdown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).asyncShutdown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void registerIdlePeriod (in alreadyAddRefed_nsIIdlePeriod aIdlePeriod); */


    /* [noscript] void idleDispatch (in alreadyAddRefed_nsIRunnable event); */


}


