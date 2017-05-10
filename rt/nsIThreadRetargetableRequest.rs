//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIThreadRetargetableRequest.idl
//


#[repr(C)]
pub struct nsIThreadRetargetableRequest {
    vtable: *const nsIThreadRetargetableRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIThreadRetargetableRequest {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x27b84c48, 0x5a73, 0x4ba4,
            [0xa8, 0xa4, 0x8b, 0x5e, 0x64, 0x9a, 0x14, 0x5e])
    }
}

unsafe impl RefCounted for nsIThreadRetargetableRequest {
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
pub trait nsIThreadRetargetableRequestCoerce {
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self;
}

impl nsIThreadRetargetableRequestCoerce for nsIThreadRetargetableRequest {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self {
        v
    }
}

impl nsIThreadRetargetableRequest {
    #[inline]
    pub fn coerce<T: nsIThreadRetargetableRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIThreadRetargetableRequest {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIThreadRetargetableRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIThreadRetargetableRequest) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIThreadRetargetableRequestVTable {
    pub __base: nsISupportsVTable,

    /* void retargetDeliveryTo (in nsIEventTarget aNewTarget); */
    pub retargetDeliveryTo: unsafe extern "C" fn (this: *const nsIThreadRetargetableRequest, aNewTarget: *const nsIEventTarget) -> nsresult,

}


impl nsIThreadRetargetableRequest {
    /* void retargetDeliveryTo (in nsIEventTarget aNewTarget); */
    #[inline]
    pub unsafe fn retargetDeliveryTo(&self, aNewTarget: Option<&nsIEventTarget>) -> Result<(), nsresult> {

        match ((*self.vtable).retargetDeliveryTo)(self as *const _, aNewTarget.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


