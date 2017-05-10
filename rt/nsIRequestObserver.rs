//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIRequestObserver.idl
//


#[repr(C)]
pub struct nsIRequestObserver {
    vtable: *const nsIRequestObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIRequestObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfd91e2e0, 0x1481, 0x11d3,
            [0x93, 0x33, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40])
    }
}

unsafe impl RefCounted for nsIRequestObserver {
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
pub trait nsIRequestObserverCoerce {
    fn coerce_from(v: &nsIRequestObserver) -> &Self;
}

impl nsIRequestObserverCoerce for nsIRequestObserver {
    #[inline]
    fn coerce_from(v: &nsIRequestObserver) -> &Self {
        v
    }
}

impl nsIRequestObserver {
    #[inline]
    pub fn coerce<T: nsIRequestObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIRequestObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIRequestObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIRequestObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onStartRequest (in nsIRequest aRequest, in nsISupports aContext); */
    pub onStartRequest: unsafe extern "C" fn (this: *const nsIRequestObserver, aRequest: *const nsIRequest, aContext: *const nsISupports) -> nsresult,

    /* void onStopRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatusCode); */
    pub onStopRequest: unsafe extern "C" fn (this: *const nsIRequestObserver, aRequest: *const nsIRequest, aContext: *const nsISupports, aStatusCode: nsresult) -> nsresult,

}


impl nsIRequestObserver {
    /* void onStartRequest (in nsIRequest aRequest, in nsISupports aContext); */
    #[inline]
    pub unsafe fn onStartRequest(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).onStartRequest)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStopRequest (in nsIRequest aRequest, in nsISupports aContext, in nsresult aStatusCode); */
    #[inline]
    pub unsafe fn onStopRequest(&self, aRequest: Option<&nsIRequest>, aContext: Option<&nsISupports>, aStatusCode: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onStopRequest)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), aStatusCode) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


