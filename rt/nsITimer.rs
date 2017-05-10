//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITimer.idl
//


#[repr(C)]
pub struct nsITimerCallback {
    vtable: *const nsITimerCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITimerCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa796816d, 0x7d47, 0x4348,
            [0x9a, 0xb8, 0xc7, 0xae, 0xb3, 0x21, 0x6a, 0x7d])
    }
}

unsafe impl RefCounted for nsITimerCallback {
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
pub trait nsITimerCallbackCoerce {
    fn coerce_from(v: &nsITimerCallback) -> &Self;
}

impl nsITimerCallbackCoerce for nsITimerCallback {
    #[inline]
    fn coerce_from(v: &nsITimerCallback) -> &Self {
        v
    }
}

impl nsITimerCallback {
    #[inline]
    pub fn coerce<T: nsITimerCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITimerCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITimerCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimerCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITimerCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void notify (in nsITimer timer); */
    pub notify: unsafe extern "C" fn (this: *const nsITimerCallback, timer: *const nsITimer) -> nsresult,

}


impl nsITimerCallback {
    /* void notify (in nsITimer timer); */
    #[inline]
    pub unsafe fn notify(&self, timer: Option<&nsITimer>) -> Result<(), nsresult> {

        match ((*self.vtable).notify)(self as *const _, timer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsITimer_consts {
    pub const TYPE_ONE_SHOT: i64 = 0;
    pub const TYPE_REPEATING_SLACK: i64 = 1;
    pub const TYPE_REPEATING_PRECISE: i64 = 2;
    pub const TYPE_REPEATING_PRECISE_CAN_SKIP: i64 = 3;
}


#[repr(C)]
pub struct nsITimer {
    vtable: *const nsITimerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITimer {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3de4b105, 0x363c, 0x482c,
            [0xa4, 0x09, 0xba, 0xac, 0x83, 0xa0, 0x1b, 0xfc])
    }
}

unsafe impl RefCounted for nsITimer {
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
pub trait nsITimerCoerce {
    fn coerce_from(v: &nsITimer) -> &Self;
}

impl nsITimerCoerce for nsITimer {
    #[inline]
    fn coerce_from(v: &nsITimer) -> &Self {
        v
    }
}

impl nsITimer {
    #[inline]
    pub fn coerce<T: nsITimerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITimer {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITimerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimer) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITimerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIObserver aObserver, in unsigned long aDelay, in unsigned long aType); */
    pub init: unsafe extern "C" fn (this: *const nsITimer, aObserver: *const nsIObserver, aDelay: libc::uint32_t, aType: libc::uint32_t) -> nsresult,

    /* [noscript] void initWithFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithFuncCallback: *const ::libc::c_void,

    /* void initWithCallback (in nsITimerCallback aCallback, in unsigned long aDelay, in unsigned long aType); */
    pub initWithCallback: unsafe extern "C" fn (this: *const nsITimer, aCallback: *const nsITimerCallback, aDelay: libc::uint32_t, aType: libc::uint32_t) -> nsresult,

    /* void cancel (); */
    pub cancel: unsafe extern "C" fn (this: *const nsITimer) -> nsresult,

    /* [noscript] void initWithNamedFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in string aName); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithNamedFuncCallback: *const ::libc::c_void,

    /* [noscript] void initWithNameableFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in nsTimerNameCallbackFunc aNameCallback); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithNameableFuncCallback: *const ::libc::c_void,

    /* attribute unsigned long delay; */
    pub get_delay: unsafe extern "C" fn (this: *const nsITimer, aDelay: *mut libc::uint32_t) -> nsresult,
    pub set_delay: unsafe extern "C" fn (this: *const nsITimer, aDelay: libc::uint32_t) -> nsresult,

    /* attribute unsigned long type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsITimer, aType: *mut libc::uint32_t) -> nsresult,
    pub set_type_: unsafe extern "C" fn (this: *const nsITimer, aType: libc::uint32_t) -> nsresult,

    /* [noscript] readonly attribute voidPtr closure; */
    pub get_closure: unsafe extern "C" fn (this: *const nsITimer, aClosure: *mut *const libc::c_void) -> nsresult,

    /* readonly attribute nsITimerCallback callback; */
    pub get_callback: unsafe extern "C" fn (this: *const nsITimer, aCallback: *mut *const nsITimerCallback) -> nsresult,

    /* attribute nsIEventTarget target; */
    pub get_target: unsafe extern "C" fn (this: *const nsITimer, aTarget: *mut *const nsIEventTarget) -> nsresult,
    pub set_target: unsafe extern "C" fn (this: *const nsITimer, aTarget: *const nsIEventTarget) -> nsresult,

}


impl nsITimer {
    /* void init (in nsIObserver aObserver, in unsigned long aDelay, in unsigned long aType); */
    #[inline]
    pub unsafe fn init(&self, aObserver: Option<&nsIObserver>, aDelay: libc::uint32_t, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aDelay, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initWithFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType); */


    /* void initWithCallback (in nsITimerCallback aCallback, in unsigned long aDelay, in unsigned long aType); */
    #[inline]
    pub unsafe fn initWithCallback(&self, aCallback: Option<&nsITimerCallback>, aDelay: libc::uint32_t, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).initWithCallback)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aDelay, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancel (); */
    #[inline]
    pub unsafe fn cancel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initWithNamedFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in string aName); */


    /* [noscript] void initWithNameableFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in nsTimerNameCallbackFunc aNameCallback); */


    /* attribute unsigned long delay; */
    #[inline]
    pub unsafe fn get_delay(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_delay)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_delay(&self, aDelay: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_delay)(self as *const _, aDelay) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_type_(&self, aType: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_type_)(self as *const _, aType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute voidPtr closure; */
    #[inline]
    pub unsafe fn get_closure(&self, ) -> Result<*const libc::c_void, nsresult> {
        let mut _retval: *const libc::c_void = ::std::ptr::null();
        match ((*self.vtable).get_closure)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsITimerCallback callback; */
    #[inline]
    pub unsafe fn get_callback(&self, ) -> Result<Option<RefPtr<nsITimerCallback>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_callback)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIEventTarget target; */
    #[inline]
    pub unsafe fn get_target(&self, ) -> Result<Option<RefPtr<nsIEventTarget>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_target)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_target(&self, aTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).set_target)(self as *const _, aTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


