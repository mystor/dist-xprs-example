//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIAsyncStreamCopier2.idl
//


#[repr(C)]
pub struct nsIAsyncStreamCopier2 {
    vtable: *const nsIAsyncStreamCopier2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIAsyncStreamCopier2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa5b2decf, 0x4ede, 0x4801,
            [0x8b, 0x38, 0xe5, 0xfe, 0x5d, 0xb4, 0x6b, 0xf2])
    }
}

unsafe impl RefCounted for nsIAsyncStreamCopier2 {
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
pub trait nsIAsyncStreamCopier2Coerce {
    fn coerce_from(v: &nsIAsyncStreamCopier2) -> &Self;
}

impl nsIAsyncStreamCopier2Coerce for nsIAsyncStreamCopier2 {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier2) -> &Self {
        v
    }
}

impl nsIAsyncStreamCopier2 {
    #[inline]
    pub fn coerce<T: nsIAsyncStreamCopier2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIAsyncStreamCopier2 {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestCoerce> nsIAsyncStreamCopier2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncStreamCopier2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIAsyncStreamCopier2VTable {
    pub __base: nsIRequestVTable,

    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
    pub init: unsafe extern "C" fn (this: *const nsIAsyncStreamCopier2, aSource: *const nsIInputStream, aSink: *const nsIOutputStream, aTarget: *const nsIEventTarget, aChunkSize: libc::uint32_t, aCloseSource: bool, aCloseSink: bool) -> nsresult,

    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
    pub asyncCopy: unsafe extern "C" fn (this: *const nsIAsyncStreamCopier2, aObserver: *const nsIRequestObserver, aObserverContext: *const nsISupports) -> nsresult,

}


impl nsIAsyncStreamCopier2 {
    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
    #[inline]
    pub unsafe fn init(&self, aSource: Option<&nsIInputStream>, aSink: Option<&nsIOutputStream>, aTarget: Option<&nsIEventTarget>, aChunkSize: libc::uint32_t, aCloseSource: bool, aCloseSink: bool) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aSource.map_or(::std::ptr::null(), |x| x as *const _), aSink.map_or(::std::ptr::null(), |x| x as *const _), aTarget.map_or(::std::ptr::null(), |x| x as *const _), aChunkSize, aCloseSource, aCloseSink) {
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


