//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncOutputStream.idl
//


pub mod nsIAsyncOutputStream_consts {
    pub const WAIT_CLOSURE_ONLY: i64 = 1;
}


#[repr(C)]
pub struct nsIAsyncOutputStream {
    vtable: *const nsIAsyncOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncOutputStream {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbeb632d3, 0xd77a, 0x4e90,
            [0x91, 0x34, 0xf9, 0xec, 0xe6, 0x9e, 0x82, 0x00])
    }
}

unsafe impl RefCounted for nsIAsyncOutputStream {
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
pub trait nsIAsyncOutputStreamCoerce {
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self;
}

impl nsIAsyncOutputStreamCoerce for nsIAsyncOutputStream {
    #[inline]
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self {
        v
    }
}

impl nsIAsyncOutputStream {
    #[inline]
    pub fn coerce<T: nsIAsyncOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIOutputStreamCoerce> nsIAsyncOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncOutputStreamVTable {
    pub __base: nsIOutputStreamVTable,

    /* void closeWithStatus (in nsresult reason); */
    pub closeWithStatus: unsafe extern "C" fn (this: *const nsIAsyncOutputStream, reason: nsresult) -> nsresult,

    /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    pub asyncWait: unsafe extern "C" fn (this: *const nsIAsyncOutputStream, aCallback: *const nsIOutputStreamCallback, aFlags: libc::uint32_t, aRequestedCount: libc::uint32_t, aEventTarget: *const nsIEventTarget) -> nsresult,

}


impl nsIAsyncOutputStream {
    /* void closeWithStatus (in nsresult reason); */
    #[inline]
    pub unsafe fn closeWithStatus(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).closeWithStatus)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncWait (in nsIOutputStreamCallback aCallback, in unsigned long aFlags, in unsigned long aRequestedCount, in nsIEventTarget aEventTarget); */
    #[inline]
    pub unsafe fn asyncWait(&self, aCallback: Option<&nsIOutputStreamCallback>, aFlags: libc::uint32_t, aRequestedCount: libc::uint32_t, aEventTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncWait)(self as *const _, aCallback.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aRequestedCount, aEventTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIOutputStreamCallback {
    vtable: *const nsIOutputStreamCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIOutputStreamCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x40dbcdff, 0x9053, 0x42c5,
            [0xa5, 0x7c, 0x3e, 0xc9, 0x10, 0xd0, 0xf1, 0x48])
    }
}

unsafe impl RefCounted for nsIOutputStreamCallback {
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
pub trait nsIOutputStreamCallbackCoerce {
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self;
}

impl nsIOutputStreamCallbackCoerce for nsIOutputStreamCallback {
    #[inline]
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self {
        v
    }
}

impl nsIOutputStreamCallback {
    #[inline]
    pub fn coerce<T: nsIOutputStreamCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIOutputStreamCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIOutputStreamCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputStreamCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIOutputStreamCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
    pub onOutputStreamReady: unsafe extern "C" fn (this: *const nsIOutputStreamCallback, aStream: *const nsIAsyncOutputStream) -> nsresult,

}


impl nsIOutputStreamCallback {
    /* void onOutputStreamReady (in nsIAsyncOutputStream aStream); */
    #[inline]
    pub unsafe fn onOutputStreamReady(&self, aStream: Option<&nsIAsyncOutputStream>) -> Result<(), nsresult> {

        match ((*self.vtable).onOutputStreamReady)(self as *const _, aStream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


