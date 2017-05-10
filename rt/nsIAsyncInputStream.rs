//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncInputStream.idl
//


pub mod nsIAsyncInputStream_consts {
    pub const WAIT_CLOSURE_ONLY: i64 = 1;
}


#[repr(C)]
pub struct nsIAsyncInputStream {
    vtable: *const nsIAsyncInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncInputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5f255ab, 0x4801, 0x4161,
            [0x88, 0x16, 0x27, 0x7a, 0xc9, 0x2f, 0x6a, 0xd1])
    }
}

unsafe impl RefCounted for nsIAsyncInputStream {
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
pub trait nsIAsyncInputStreamCoerce {
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self;
}

impl nsIAsyncInputStreamCoerce for nsIAsyncInputStream {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self {
        v
    }
}

impl nsIAsyncInputStream {
    #[inline]
    pub fn coerce<T: nsIAsyncInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIInputStreamCoerce> nsIAsyncInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncInputStreamVTable {
    pub __base: nsIInputStreamVTable,

    /* void closeWithStatus (in nsresult aStatus); */
    pub closeWithStatus: unsafe extern "C" fn (this: *const nsIAsyncInputStream, aStatus: nsresult) -> nsresult,

    /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    pub asyncWait: unsafe extern "C" fn (this: *const nsIAsyncInputStream, aCallback: *const nsIInputStreamCallback, aFlags: libc::uint32_t, aRequestedCount: libc::uint32_t, aEventTarget: *const nsIEventTarget) -> nsresult,

}


impl nsIAsyncInputStream {
    /* void closeWithStatus (in nsresult aStatus); */
    #[inline]
    pub unsafe fn closeWithStatus(&self, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).closeWithStatus)(self as *const _, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncWait (in nsIInputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    #[inline]
    pub unsafe fn asyncWait(&self, aCallback: Option<&nsIInputStreamCallback>, aFlags: libc::uint32_t, aRequestedCount: libc::uint32_t, aEventTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncWait)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aRequestedCount, aEventTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIInputStreamCallback {
    vtable: *const nsIInputStreamCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInputStreamCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd1f28e94, 0x3a6e, 0x4050,
            [0xa5, 0xf5, 0x2e, 0x81, 0xb1, 0xfc, 0x2a, 0x43])
    }
}

unsafe impl RefCounted for nsIInputStreamCallback {
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
pub trait nsIInputStreamCallbackCoerce {
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self;
}

impl nsIInputStreamCallbackCoerce for nsIInputStreamCallback {
    #[inline]
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self {
        v
    }
}

impl nsIInputStreamCallback {
    #[inline]
    pub fn coerce<T: nsIInputStreamCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInputStreamCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInputStreamCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInputStreamCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
    pub onInputStreamReady: unsafe extern "C" fn (this: *const nsIInputStreamCallback, aStream: *const nsIAsyncInputStream) -> nsresult,

}


impl nsIInputStreamCallback {
    /* void onInputStreamReady (in nsIAsyncInputStream aStream); */
    #[inline]
    pub unsafe fn onInputStreamReady(&self, aStream: Option<&nsIAsyncInputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).onInputStreamReady)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


