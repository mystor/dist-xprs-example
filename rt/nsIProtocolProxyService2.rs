//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyService2.idl
//


#[repr(C)]
pub struct nsIProtocolProxyService2 {
    vtable: *const nsIProtocolProxyService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolProxyService2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb2e5b2c0, 0xe21e, 0x4845,
            [0xb3, 0x36, 0xbe, 0x6d, 0x60, 0xa3, 0x89, 0x51])
    }
}

unsafe impl RefCounted for nsIProtocolProxyService2 {
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
pub trait nsIProtocolProxyService2Coerce {
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self;
}

impl nsIProtocolProxyService2Coerce for nsIProtocolProxyService2 {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self {
        v
    }
}

impl nsIProtocolProxyService2 {
    #[inline]
    pub fn coerce<T: nsIProtocolProxyService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolProxyService2 {
    type Target = nsIProtocolProxyService;
    #[inline]
    fn deref(&self) -> &nsIProtocolProxyService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIProtocolProxyServiceCoerce> nsIProtocolProxyService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolProxyService2VTable {
    pub __base: nsIProtocolProxyServiceVTable,

    /* void reloadPAC (); */
    pub reloadPAC: unsafe extern "C" fn (this: *const nsIProtocolProxyService2) -> nsresult,

    /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
    pub asyncResolve2: unsafe extern "C" fn (this: *const nsIProtocolProxyService2, aChannel: *const nsIChannel, aFlags: libc::uint32_t, aCallback: *const nsIProtocolProxyCallback, _retval: *mut *const nsICancelable) -> nsresult,

}


impl nsIProtocolProxyService2 {
    /* void reloadPAC (); */
    #[inline]
    pub unsafe fn reloadPAC(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).reloadPAC)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback); */
    #[inline]
    pub unsafe fn asyncResolve2(&self, aChannel: Option<&nsIChannel>, aFlags: libc::uint32_t, aCallback: Option<&nsIProtocolProxyCallback>) -> Result<Option<RefPtr<nsICancelable>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).asyncResolve2)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _), aFlags, aCallback.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


