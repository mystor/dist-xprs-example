//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamListener.idl
//


#[repr(C)]
pub struct nsIStreamListener {
    vtable: *const nsIStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3b4c8a77, 0x76ba, 0x4610,
            [0xb3, 0x16, 0x67, 0x8c, 0x73, 0xa3, 0xb8, 0x8c])
    }
}

unsafe impl RefCounted for nsIStreamListener {
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
pub trait nsIStreamListenerCoerce {
    fn coerce_from(v: &nsIStreamListener) -> &Self;
}

impl nsIStreamListenerCoerce for nsIStreamListener {
    #[inline]
    fn coerce_from(v: &nsIStreamListener) -> &Self {
        v
    }
}

impl nsIStreamListener {
    #[inline]
    pub fn coerce<T: nsIStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamListener {
    type Target = nsIRequestObserver;
    #[inline]
    fn deref(&self) -> &nsIRequestObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestObserverCoerce> nsIStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamListenerVTable {
    pub __base: nsIRequestObserverVTable,

    /* void onDataAvailable (in nsIRequest aRequest, in nsISupports aContext, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
    pub onDataAvailable: unsafe extern "C" fn (this: *const nsIStreamListener, aRequest: *const nsIRequest, aContext: *const nsISupports, aInputStream: *const nsIInputStream, aOffset: libc::uint64_t, aCount: libc::uint32_t) -> nsresult,

}


impl nsIStreamListener {
    /* void onDataAvailable (in nsIRequest aRequest, in nsISupports aContext, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
    #[inline]
    pub unsafe fn onDataAvailable(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>, aInputStream: Option<&nsIInputStream>, aOffset: libc::uint64_t, aCount: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onDataAvailable)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aInputStream.map_or(::std::ptr::null(), |x| x as *const _), aOffset, aCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


