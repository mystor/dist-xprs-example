//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkPredictorVerifier.idl
//


#[repr(C)]
pub struct nsINetworkPredictorVerifier {
    vtable: *const nsINetworkPredictorVerifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkPredictorVerifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2e43bb32, 0xdabf, 0x4494,
            [0x9f, 0x90, 0x2b, 0x31, 0x95, 0xb1, 0xc7, 0x3d])
    }
}

unsafe impl RefCounted for nsINetworkPredictorVerifier {
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
pub trait nsINetworkPredictorVerifierCoerce {
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self;
}

impl nsINetworkPredictorVerifierCoerce for nsINetworkPredictorVerifier {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self {
        v
    }
}

impl nsINetworkPredictorVerifier {
    #[inline]
    pub fn coerce<T: nsINetworkPredictorVerifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkPredictorVerifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkPredictorVerifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkPredictorVerifierVTable {
    pub __base: nsISupportsVTable,

    /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
    pub onPredictPrefetch: unsafe extern "C" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI, status: uint32_t) -> nsresult,

    /* void onPredictPreconnect (in nsIURI uri); */
    pub onPredictPreconnect: unsafe extern "C" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI) -> nsresult,

    /* void onPredictDNS (in nsIURI uri); */
    pub onPredictDNS: unsafe extern "C" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI) -> nsresult,

}


impl nsINetworkPredictorVerifier {
    /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
    #[inline]
    pub unsafe fn onPredictPrefetch(&self, uri: Option<&nsIURI>, status: uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onPredictPrefetch)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _), status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPredictPreconnect (in nsIURI uri); */
    #[inline]
    pub unsafe fn onPredictPreconnect(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).onPredictPreconnect)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onPredictDNS (in nsIURI uri); */
    #[inline]
    pub unsafe fn onPredictDNS(&self, uri: Option<&nsIURI>) -> Result<(), nsresult> {

        match ((*self.vtable).onPredictDNS)(self as *const _, uri.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


