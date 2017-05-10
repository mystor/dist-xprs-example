//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequestObserverProxy.idl
//


#[repr(C)]
pub struct nsIRequestObserverProxy {
    vtable: *const nsIRequestObserverProxyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRequestObserverProxy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc2b06151, 0x1bf8, 0x4eef,
            [0xae, 0xa9, 0x15, 0x32, 0xf1, 0x2f, 0x5a, 0x10])
    }
}

unsafe impl RefCounted for nsIRequestObserverProxy {
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
pub trait nsIRequestObserverProxyCoerce {
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self;
}

impl nsIRequestObserverProxyCoerce for nsIRequestObserverProxy {
    #[inline]
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self {
        v
    }
}

impl nsIRequestObserverProxy {
    #[inline]
    pub fn coerce<T: nsIRequestObserverProxyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRequestObserverProxy {
    type Target = nsIRequestObserver;
    #[inline]
    fn deref(&self) -> &nsIRequestObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIRequestObserverCoerce> nsIRequestObserverProxyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRequestObserverProxyVTable {
    pub __base: nsIRequestObserverVTable,

    /* void init (in nsIRequestObserver observer, in nsISupports context); */
    pub init: unsafe extern "C" fn (this: *const nsIRequestObserverProxy, observer: *const nsIRequestObserver, context: *const nsISupports) -> nsresult,

}


impl nsIRequestObserverProxy {
    /* void init (in nsIRequestObserver observer, in nsISupports context); */
    #[inline]
    pub unsafe fn init(&self, observer: Option<&nsIRequestObserver>, context: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _), context.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


