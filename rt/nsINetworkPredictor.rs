//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkPredictor.idl
//


pub type PredictorPredictReason = libc::uint32_t;


pub type PredictorLearnReason = libc::uint32_t;


pub mod nsINetworkPredictor_consts {
    pub const PREDICT_LINK: i64 = 0;
    pub const PREDICT_LOAD: i64 = 1;
    pub const PREDICT_STARTUP: i64 = 2;
    pub const LEARN_LOAD_TOPLEVEL: i64 = 0;
    pub const LEARN_LOAD_SUBRESOURCE: i64 = 1;
    pub const LEARN_LOAD_REDIRECT: i64 = 2;
    pub const LEARN_STARTUP: i64 = 3;
}


#[repr(C)]
pub struct nsINetworkPredictor {
    vtable: *const nsINetworkPredictorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkPredictor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xacc88e7c, 0x3f39, 0x42c7,
            [0xac, 0x31, 0x63, 0x77, 0xc2, 0xc3, 0xd7, 0x3e])
    }
}

unsafe impl RefCounted for nsINetworkPredictor {
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
pub trait nsINetworkPredictorCoerce {
    fn coerce_from(v: &nsINetworkPredictor) -> &Self;
}

impl nsINetworkPredictorCoerce for nsINetworkPredictor {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictor) -> &Self {
        v
    }
}

impl nsINetworkPredictor {
    #[inline]
    pub fn coerce<T: nsINetworkPredictorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkPredictor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkPredictorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkPredictorVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void predict (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in jsval originAttributes, in nsINetworkPredictorVerifier verifier); */
    /// Unable to call function as its signature contains a non-rust type
    pub predict: *const ::libc::c_void,

    /* [notxpcom] nsresult predictNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in OriginAttributes originAttributes, in nsINetworkPredictorVerifier verifier); */
    /// Unable to call function as its signature contains a non-rust type
    pub predictNative: *const ::libc::c_void,

    /* [implicit_jscontext] void learn (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in jsval originAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub learn: *const ::libc::c_void,

    /* [notxpcom] nsresult learnNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in OriginAttributes originAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub learnNative: *const ::libc::c_void,

    /* void reset (); */
    pub reset: unsafe extern "C" fn (this: *const nsINetworkPredictor) -> nsresult,

}


impl nsINetworkPredictor {
    /* [implicit_jscontext] void predict (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in jsval originAttributes, in nsINetworkPredictorVerifier verifier); */


    /* [notxpcom] nsresult predictNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in OriginAttributes originAttributes, in nsINetworkPredictorVerifier verifier); */


    /* [implicit_jscontext] void learn (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in jsval originAttributes); */


    /* [notxpcom] nsresult learnNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in OriginAttributes originAttributes); */


    /* void reset (); */
    #[inline]
    pub unsafe fn reset(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reset)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


