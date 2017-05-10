//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITransportProvider.idl
//


#[repr(C)]
pub struct nsITransportProvider {
    vtable: *const nsITransportProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITransportProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6fcec704, 0xcfd2, 0x46ef,
            [0xa3, 0x94, 0xa6, 0x4d, 0x5c, 0xb1, 0x47, 0x5c])
    }
}

unsafe impl RefCounted for nsITransportProvider {
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
pub trait nsITransportProviderCoerce {
    fn coerce_from(v: &nsITransportProvider) -> &Self;
}

impl nsITransportProviderCoerce for nsITransportProvider {
    #[inline]
    fn coerce_from(v: &nsITransportProvider) -> &Self {
        v
    }
}

impl nsITransportProvider {
    #[inline]
    pub fn coerce<T: nsITransportProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITransportProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITransportProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITransportProviderVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void setListener (in nsIHttpUpgradeListener listener); */
    pub setListener: unsafe extern "C" fn (this: *const nsITransportProvider, listener: *const nsIHttpUpgradeListener) -> nsresult,

    /* [must_use,noscript] PTransportProviderChild getIPCChild (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getIPCChild: *const ::libc::c_void,

}


impl nsITransportProvider {
    /* [must_use] void setListener (in nsIHttpUpgradeListener listener); */
    #[inline]
    pub unsafe fn setListener(&self, listener: Option<&nsIHttpUpgradeListener>) -> Result<(), nsresult> {

        match ((*self.vtable).setListener)(self as *const _, listener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use,noscript] PTransportProviderChild getIPCChild (); */


}


