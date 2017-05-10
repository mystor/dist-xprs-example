//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIStreamListenerTee.idl
//


#[repr(C)]
pub struct nsIStreamListenerTee {
    vtable: *const nsIStreamListenerTeeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIStreamListenerTee {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x62b27fc1, 0x6e8c, 0x4225,
            [0x8a, 0xd0, 0xb9, 0xd4, 0x42, 0x52, 0x97, 0x3a])
    }
}

unsafe impl RefCounted for nsIStreamListenerTee {
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
pub trait nsIStreamListenerTeeCoerce {
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self;
}

impl nsIStreamListenerTeeCoerce for nsIStreamListenerTee {
    #[inline]
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self {
        v
    }
}

impl nsIStreamListenerTee {
    #[inline]
    pub fn coerce<T: nsIStreamListenerTeeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIStreamListenerTee {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsIStreamListenerTeeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamListenerTee) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIStreamListenerTeeVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    pub init: unsafe extern "C" fn (this: *const nsIStreamListenerTee, listener: *const nsIStreamListener, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> nsresult,

    /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    pub initAsync: unsafe extern "C" fn (this: *const nsIStreamListenerTee, listener: *const nsIStreamListener, eventTarget: *const nsIEventTarget, sink: *const nsIOutputStream, requestObserver: *const nsIRequestObserver) -> nsresult,

}


impl nsIStreamListenerTee {
    /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    #[inline]
    pub unsafe fn init(&self, listener: Option<&nsIStreamListener>, sink: Option<&nsIOutputStream>, requestObserver: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _), sink.map_or(::std::ptr::null(), |x| x as *const _), requestObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
    #[inline]
    pub unsafe fn initAsync(&self, listener: Option<&nsIStreamListener>, eventTarget: Option<&nsIEventTarget>, sink: Option<&nsIOutputStream>, requestObserver: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).initAsync)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _), eventTarget.map_or(::std::ptr::null(), |x| x as *const _), sink.map_or(::std::ptr::null(), |x| x as *const _), requestObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


