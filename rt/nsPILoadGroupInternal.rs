//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsPILoadGroupInternal.idl
//


#[repr(C)]
pub struct nsPILoadGroupInternal {
    vtable: *const nsPILoadGroupInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsPILoadGroupInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6ef2f8ac, 0x9584, 0x48f3,
            [0x95, 0x7a, 0x0c, 0x94, 0xff, 0xf0, 0xc8, 0xc7])
    }
}

unsafe impl RefCounted for nsPILoadGroupInternal {
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
pub trait nsPILoadGroupInternalCoerce {
    fn coerce_from(v: &nsPILoadGroupInternal) -> &Self;
}

impl nsPILoadGroupInternalCoerce for nsPILoadGroupInternal {
    #[inline]
    fn coerce_from(v: &nsPILoadGroupInternal) -> &Self {
        v
    }
}

impl nsPILoadGroupInternal {
    #[inline]
    pub fn coerce<T: nsPILoadGroupInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsPILoadGroupInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsPILoadGroupInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPILoadGroupInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsPILoadGroupInternalVTable {
    pub __base: nsISupportsVTable,

    /* void OnEndPageLoad (in nsIChannel aDefaultChannel); */
    pub OnEndPageLoad: unsafe extern "C" fn (this: *const nsPILoadGroupInternal, aDefaultChannel: *const nsIChannel) -> nsresult,

}


impl nsPILoadGroupInternal {
    /* void OnEndPageLoad (in nsIChannel aDefaultChannel); */
    #[inline]
    pub unsafe fn OnEndPageLoad(&self, aDefaultChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).OnEndPageLoad)(self as *const _, aDefaultChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


