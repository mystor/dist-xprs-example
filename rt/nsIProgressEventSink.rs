//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProgressEventSink.idl
//


#[repr(C)]
pub struct nsIProgressEventSink {
    vtable: *const nsIProgressEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProgressEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x87d55fba, 0xcb7e, 0x4f38,
            [0x84, 0xc1, 0x5c, 0x6c, 0x2b, 0x2a, 0x55, 0xe9])
    }
}

unsafe impl RefCounted for nsIProgressEventSink {
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
pub trait nsIProgressEventSinkCoerce {
    fn coerce_from(v: &nsIProgressEventSink) -> &Self;
}

impl nsIProgressEventSinkCoerce for nsIProgressEventSink {
    #[inline]
    fn coerce_from(v: &nsIProgressEventSink) -> &Self {
        v
    }
}

impl nsIProgressEventSink {
    #[inline]
    pub fn coerce<T: nsIProgressEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProgressEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProgressEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProgressEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProgressEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* void onProgress (in nsIRequest aRequest, in nsISupports aContext, in long long aProgress, in long long aProgressMax); */
    pub onProgress: unsafe extern "C" fn (this: *const nsIProgressEventSink, aRequest: *const nsIRequest, aContext: *const nsISupports, aProgress: libc::int64_t, aProgressMax: libc::int64_t) -> nsresult,

    /* void onStatus (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus, in wstring aStatusArg); */
    pub onStatus: unsafe extern "C" fn (this: *const nsIProgressEventSink, aRequest: *const nsIRequest, aContext: *const nsISupports, aStatus: nsresult, aStatusArg: *const libc::int16_t) -> nsresult,

}


impl nsIProgressEventSink {
    /* void onProgress (in nsIRequest aRequest, in nsISupports aContext, in long long aProgress, in long long aProgressMax); */
    #[inline]
    pub unsafe fn onProgress(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>, aProgress: libc::int64_t, aProgressMax: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).onProgress)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aProgress, aProgressMax) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStatus (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatus, in wstring aStatusArg); */
    #[inline]
    pub unsafe fn onStatus(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>, aStatus: nsresult, aStatusArg: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onStatus)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aStatus, aStatusArg) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


