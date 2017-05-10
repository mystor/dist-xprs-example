//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebRequestListener.idl
//


#[repr(C)]
pub struct nsIWebRequestListener {
    vtable: *const nsIWebRequestListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebRequestListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x699a50bb, 0x1f18, 0x2844,
            [0xb9, 0xea, 0x9f, 0x21, 0x6f, 0x62, 0xcb, 0x18])
    }
}

unsafe impl RefCounted for nsIWebRequestListener {
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
pub trait nsIWebRequestListenerCoerce {
    fn coerce_from(v: &nsIWebRequestListener) -> &Self;
}

impl nsIWebRequestListenerCoerce for nsIWebRequestListener {
    #[inline]
    fn coerce_from(v: &nsIWebRequestListener) -> &Self {
        v
    }
}

impl nsIWebRequestListener {
    #[inline]
    pub fn coerce<T: nsIWebRequestListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebRequestListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebRequestListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebRequestListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebRequestListenerVTable {
    pub __base: nsISupportsVTable,

    /* void init (in nsIStreamListener aStreamListener, in nsITraceableChannel aTraceableChannel); */
    pub init: unsafe extern "C" fn (this: *const nsIWebRequestListener, aStreamListener: *const nsIStreamListener, aTraceableChannel: *const nsITraceableChannel) -> nsresult,

}


impl nsIWebRequestListener {
    /* void init (in nsIStreamListener aStreamListener, in nsITraceableChannel aTraceableChannel); */
    #[inline]
    pub unsafe fn init(&self, aStreamListener: Option<&nsIStreamListener>, aTraceableChannel: Option<&nsITraceableChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aStreamListener.map_or(::std::ptr::null(), |x| x as *const _), aTraceableChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


