//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIIdleServiceInternal.idl
//


#[repr(C)]
pub struct nsIIdleServiceInternal {
    vtable: *const nsIIdleServiceInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIIdleServiceInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7b89a2e7, 0xed12, 0x42e0,
            [0xb8, 0x6d, 0x49, 0x84, 0x23, 0x9a, 0xbd, 0x7b])
    }
}

unsafe impl RefCounted for nsIIdleServiceInternal {
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
pub trait nsIIdleServiceInternalCoerce {
    fn coerce_from(v: &nsIIdleServiceInternal) -> &Self;
}

impl nsIIdleServiceInternalCoerce for nsIIdleServiceInternal {
    #[inline]
    fn coerce_from(v: &nsIIdleServiceInternal) -> &Self {
        v
    }
}

impl nsIIdleServiceInternal {
    #[inline]
    pub fn coerce<T: nsIIdleServiceInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIIdleServiceInternal {
    type Target = nsIIdleService;
    #[inline]
    fn deref(&self) -> &nsIIdleService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIIdleServiceCoerce> nsIIdleServiceInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIdleServiceInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIIdleServiceInternalVTable {
    pub __base: nsIIdleServiceVTable,

    /* void resetIdleTimeOut (in unsigned long idleDeltaInMS); */
    pub resetIdleTimeOut: unsafe extern "C" fn (this: *const nsIIdleServiceInternal, idleDeltaInMS: libc::uint32_t) -> nsresult,

}


impl nsIIdleServiceInternal {
    /* void resetIdleTimeOut (in unsigned long idleDeltaInMS); */
    #[inline]
    pub unsafe fn resetIdleTimeOut(&self, idleDeltaInMS: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).resetIdleTimeOut)(self as *const _, idleDeltaInMS) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


