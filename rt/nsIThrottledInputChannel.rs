//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThrottledInputChannel.idl
//


#[repr(C)]
pub struct nsIInputChannelThrottleQueue {
    vtable: *const nsIInputChannelThrottleQueueVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputChannelThrottleQueue {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6b4b96fe, 0x3c67, 0x4587,
            [0xaf, 0x7b, 0x58, 0xb6, 0xb1, 0x7d, 0xa4, 0x11])
    }
}

unsafe impl RefCounted for nsIInputChannelThrottleQueue {
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
pub trait nsIInputChannelThrottleQueueCoerce {
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self;
}

impl nsIInputChannelThrottleQueueCoerce for nsIInputChannelThrottleQueue {
    #[inline]
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self {
        v
    }
}

impl nsIInputChannelThrottleQueue {
    #[inline]
    pub fn coerce<T: nsIInputChannelThrottleQueueCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputChannelThrottleQueue {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputChannelThrottleQueueCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputChannelThrottleQueue) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputChannelThrottleQueueVTable {
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
    pub init: unsafe extern "C" fn (this: *const nsIInputChannelThrottleQueue, aMeanBytesPerSecond: libc::uint32_t, aMaxBytesPerSecond: libc::uint32_t) -> nsresult,

    /* unsigned long available (in unsigned long aRemaining); */
    pub available: unsafe extern "C" fn (this: *const nsIInputChannelThrottleQueue, aRemaining: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void recordRead (in unsigned long aBytesRead); */
    pub recordRead: unsafe extern "C" fn (this: *const nsIInputChannelThrottleQueue, aBytesRead: libc::uint32_t) -> nsresult,

    /* unsigned long long bytesProcessed (); */
    pub bytesProcessed: unsafe extern "C" fn (this: *const nsIInputChannelThrottleQueue, _retval: *mut libc::uint64_t) -> nsresult,

    /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
    pub wrapStream: unsafe extern "C" fn (this: *const nsIInputChannelThrottleQueue, aInputStream: *const nsIInputStream, _retval: *mut *const nsIAsyncInputStream) -> nsresult,

}


impl nsIInputChannelThrottleQueue {
    /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
    #[inline]
    pub unsafe fn init(&self, aMeanBytesPerSecond: libc::uint32_t, aMaxBytesPerSecond: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aMeanBytesPerSecond, aMaxBytesPerSecond) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long available (in unsigned long aRemaining); */
    #[inline]
    pub unsafe fn available(&self, aRemaining: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).available)(self as *const _, aRemaining, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void recordRead (in unsigned long aBytesRead); */
    #[inline]
    pub unsafe fn recordRead(&self, aBytesRead: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).recordRead)(self as *const _, aBytesRead) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* unsigned long long bytesProcessed (); */
    #[inline]
    pub unsafe fn bytesProcessed(&self, ) -> Result<libc::uint64_t, nsresult> {
        let mut _retval: libc::uint64_t = ::std::mem::zeroed();
        match ((*self.vtable).bytesProcessed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
    #[inline]
    pub unsafe fn wrapStream(&self, aInputStream: Option<&nsIInputStream>) -> Result<Option<RefPtr<nsIAsyncInputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).wrapStream)(self as *const _, aInputStream.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIThrottledInputChannel {
    vtable: *const nsIThrottledInputChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThrottledInputChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0a32a100, 0xc031, 0x45b6,
            [0x9e, 0x8b, 0x04, 0x44, 0xc7, 0xd4, 0xa1, 0x43])
    }
}

unsafe impl RefCounted for nsIThrottledInputChannel {
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
pub trait nsIThrottledInputChannelCoerce {
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self;
}

impl nsIThrottledInputChannelCoerce for nsIThrottledInputChannel {
    #[inline]
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self {
        v
    }
}

impl nsIThrottledInputChannel {
    #[inline]
    pub fn coerce<T: nsIThrottledInputChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThrottledInputChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThrottledInputChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThrottledInputChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThrottledInputChannelVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
    pub get_throttleQueue: unsafe extern "C" fn (this: *const nsIThrottledInputChannel, aThrottleQueue: *mut *const nsIInputChannelThrottleQueue) -> nsresult,
    pub set_throttleQueue: unsafe extern "C" fn (this: *const nsIThrottledInputChannel, aThrottleQueue: *const nsIInputChannelThrottleQueue) -> nsresult,

}


impl nsIThrottledInputChannel {
    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
    #[inline]
    pub unsafe fn get_throttleQueue(&self, ) -> Result<Option<RefPtr<nsIInputChannelThrottleQueue>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_throttleQueue)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_throttleQueue(&self, aThrottleQueue: Option<&nsIInputChannelThrottleQueue>) -> Result<(), nsresult> {

        match ((*self.vtable).set_throttleQueue)(self as *const _, aThrottleQueue.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


