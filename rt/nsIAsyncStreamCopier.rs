//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncStreamCopier.idl
//


#[repr(C)]
pub struct nsIAsyncStreamCopier {
    vtable: *const nsIAsyncStreamCopierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncStreamCopier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x5a19ca27, 0xe041, 0x4aca,
            [0x82, 0x87, 0xeb, 0x24, 0x8d, 0x4c, 0x50, 0xc0])
    }
}

unsafe impl RefCounted for nsIAsyncStreamCopier {
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
pub trait nsIAsyncStreamCopierCoerce {
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self;
}

impl nsIAsyncStreamCopierCoerce for nsIAsyncStreamCopier {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self {
        v
    }
}

impl nsIAsyncStreamCopier {
    #[inline]
    pub fn coerce<T: nsIAsyncStreamCopierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncStreamCopier {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsIAsyncStreamCopierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncStreamCopierVTable {
    pub __base: nsIRequestVTable,

    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
    pub init: unsafe extern "C" fn (this: *const nsIAsyncStreamCopier, aSource: *const nsIInputStream, aSink: *const nsIOutputStream, aTarget: *const nsIEventTarget, aSourceBuffered: bool, aSinkBuffered: bool, aChunkSize: libc::uint32_t, aCloseSource: bool, aCloseSink: bool) -> nsresult,

    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
    pub asyncCopy: unsafe extern "C" fn (this: *const nsIAsyncStreamCopier, aObserver: *const nsIRequestObserver, aObserverContext: *const nsISupports) -> nsresult,

}


impl nsIAsyncStreamCopier {
    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in boolean aSourceBuffered, in boolean aSinkBuffered, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
    #[inline]
    pub unsafe fn init(&self, aSource: Option<&nsIInputStream>, aSink: Option<&nsIOutputStream>, aTarget: Option<&nsIEventTarget>, aSourceBuffered: bool, aSinkBuffered: bool, aChunkSize: libc::uint32_t, aCloseSource: bool, aCloseSink: bool) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aSink.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aSourceBuffered, aSinkBuffered, aChunkSize, aCloseSource, aCloseSink) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
    #[inline]
    pub unsafe fn asyncCopy(&self, aObserver: Option<&nsIRequestObserver>, aObserverContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncCopy)(self as *const _, aObserver.map_or(::std::ptr::null(), |x| x as *const _), aObserverContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


