//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISimpleStreamListener.idl
//


#[repr(C)]
pub struct nsISimpleStreamListener {
    vtable: *const nsISimpleStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISimpleStreamListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa9b84f6a, 0x0824, 0x4278,
            [0xba, 0xe6, 0xbf, 0xca, 0x05, 0x70, 0xa2, 0x6e])
    }
}

unsafe impl RefCounted for nsISimpleStreamListener {
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
pub trait nsISimpleStreamListenerCoerce {
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self;
}

impl nsISimpleStreamListenerCoerce for nsISimpleStreamListener {
    #[inline]
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self {
        v
    }
}

impl nsISimpleStreamListener {
    #[inline]
    pub fn coerce<T: nsISimpleStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISimpleStreamListener {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIStreamListenerCoerce> nsISimpleStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISimpleStreamListenerVTable {
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver); */
    pub init: unsafe extern "C" fn (this: *const nsISimpleStreamListener, aSink: *const nsIOutputStream, aObserver: *const nsIRequestObserver) -> nsresult,

}


impl nsISimpleStreamListener {
    /* void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver); */
    #[inline]
    pub unsafe fn init(&self, aSink: Option<&nsIOutputStream>, aObserver: Option<&nsIRequestObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aSink.map_or(::std::ptr::null(), |x| x as *const _), aObserver.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


