//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadPool.idl
//


#[repr(C)]
pub struct nsIThreadPoolListener {
    vtable: *const nsIThreadPoolListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadPoolListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xef194cab, 0x3f86, 0x4b61,
            [0xb1, 0x32, 0xe5, 0xe9, 0x6a, 0x79, 0xe5, 0xd1])
    }
}

unsafe impl RefCounted for nsIThreadPoolListener {
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
pub trait nsIThreadPoolListenerCoerce {
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self;
}

impl nsIThreadPoolListenerCoerce for nsIThreadPoolListener {
    #[inline]
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self {
        v
    }
}

impl nsIThreadPoolListener {
    #[inline]
    pub fn coerce<T: nsIThreadPoolListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadPoolListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThreadPoolListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadPoolListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadPoolListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onThreadCreated (); */
    pub onThreadCreated: unsafe extern "C" fn (this: *const nsIThreadPoolListener) -> nsresult,

    /* void onThreadShuttingDown (); */
    pub onThreadShuttingDown: unsafe extern "C" fn (this: *const nsIThreadPoolListener) -> nsresult,

}


impl nsIThreadPoolListener {
    /* void onThreadCreated (); */
    #[inline]
    pub unsafe fn onThreadCreated(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onThreadCreated)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onThreadShuttingDown (); */
    #[inline]
    pub unsafe fn onThreadShuttingDown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onThreadShuttingDown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIThreadPool {
    vtable: *const nsIThreadPoolVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadPool {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x76ce99c9, 0x8e43, 0x489a,
            [0x97, 0x89, 0xf2, 0x7c, 0xc4, 0x42, 0x49, 0x65])
    }
}

unsafe impl RefCounted for nsIThreadPool {
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
pub trait nsIThreadPoolCoerce {
    fn coerce_from(v: &nsIThreadPool) -> &Self;
}

impl nsIThreadPoolCoerce for nsIThreadPool {
    #[inline]
    fn coerce_from(v: &nsIThreadPool) -> &Self {
        v
    }
}

impl nsIThreadPool {
    #[inline]
    pub fn coerce<T: nsIThreadPoolCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadPool {
    type Target = nsIEventTarget;
    #[inline]
    fn deref(&self) -> &nsIEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIEventTargetCoerce> nsIThreadPoolCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadPool) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadPoolVTable {
    pub __base: nsIEventTargetVTable,

    /* void shutdown (); */
    pub shutdown: unsafe extern "C" fn (this: *const nsIThreadPool) -> nsresult,

    /* attribute unsigned long threadLimit; */
    pub get_threadLimit: unsafe extern "C" fn (this: *const nsIThreadPool, aThreadLimit: *mut libc::uint32_t) -> nsresult,
    pub set_threadLimit: unsafe extern "C" fn (this: *const nsIThreadPool, aThreadLimit: libc::uint32_t) -> nsresult,

    /* attribute unsigned long idleThreadLimit; */
    pub get_idleThreadLimit: unsafe extern "C" fn (this: *const nsIThreadPool, aIdleThreadLimit: *mut libc::uint32_t) -> nsresult,
    pub set_idleThreadLimit: unsafe extern "C" fn (this: *const nsIThreadPool, aIdleThreadLimit: libc::uint32_t) -> nsresult,

    /* attribute unsigned long idleThreadTimeout; */
    pub get_idleThreadTimeout: unsafe extern "C" fn (this: *const nsIThreadPool, aIdleThreadTimeout: *mut libc::uint32_t) -> nsresult,
    pub set_idleThreadTimeout: unsafe extern "C" fn (this: *const nsIThreadPool, aIdleThreadTimeout: libc::uint32_t) -> nsresult,

    /* attribute unsigned long threadStackSize; */
    pub get_threadStackSize: unsafe extern "C" fn (this: *const nsIThreadPool, aThreadStackSize: *mut libc::uint32_t) -> nsresult,
    pub set_threadStackSize: unsafe extern "C" fn (this: *const nsIThreadPool, aThreadStackSize: libc::uint32_t) -> nsresult,

    /* attribute nsIThreadPoolListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIThreadPool, aListener: *mut *const nsIThreadPoolListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIThreadPool, aListener: *const nsIThreadPoolListener) -> nsresult,

    /* void setName (in ACString aName); */
    pub setName: unsafe extern "C" fn (this: *const nsIThreadPool, aName: *const nsACString) -> nsresult,

}


impl nsIThreadPool {
    /* void shutdown (); */
    #[inline]
    pub unsafe fn shutdown(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).shutdown)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long threadLimit; */
    #[inline]
    pub unsafe fn get_threadLimit(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_threadLimit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_threadLimit(&self, aThreadLimit: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_threadLimit)(self as *const _, aThreadLimit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long idleThreadLimit; */
    #[inline]
    pub unsafe fn get_idleThreadLimit(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_idleThreadLimit)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_idleThreadLimit(&self, aIdleThreadLimit: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_idleThreadLimit)(self as *const _, aIdleThreadLimit) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long idleThreadTimeout; */
    #[inline]
    pub unsafe fn get_idleThreadTimeout(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_idleThreadTimeout)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_idleThreadTimeout(&self, aIdleThreadTimeout: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_idleThreadTimeout)(self as *const _, aIdleThreadTimeout) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long threadStackSize; */
    #[inline]
    pub unsafe fn get_threadStackSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_threadStackSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_threadStackSize(&self, aThreadStackSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_threadStackSize)(self as *const _, aThreadStackSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute nsIThreadPoolListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIThreadPoolListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIThreadPoolListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setName (in ACString aName); */
    #[inline]
    pub unsafe fn setName(&self, aName: &[u8]) -> Result<(), nsresult> {
        let aName = nsCString::from(aName);
        match ((*self.vtable).setName)(self as *const _, &*aName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


