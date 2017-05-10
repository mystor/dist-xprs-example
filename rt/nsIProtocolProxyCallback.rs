//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProtocolProxyCallback.idl
//


#[repr(C)]
pub struct nsIProtocolProxyCallback {
    vtable: *const nsIProtocolProxyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProtocolProxyCallback {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xfbb6eff6, 0x0cc2, 0x4d99,
            [0x8d, 0x6f, 0x0a, 0x12, 0xb4, 0x62, 0xbd, 0xeb])
    }
}

unsafe impl RefCounted for nsIProtocolProxyCallback {
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
pub trait nsIProtocolProxyCallbackCoerce {
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self;
}

impl nsIProtocolProxyCallbackCoerce for nsIProtocolProxyCallback {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self {
        v
    }
}

impl nsIProtocolProxyCallback {
    #[inline]
    pub fn coerce<T: nsIProtocolProxyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProtocolProxyCallback {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProtocolProxyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyCallback) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProtocolProxyCallbackVTable {
    pub __base: nsISupportsVTable,

    /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
    pub onProxyAvailable: unsafe extern "C" fn (this: *const nsIProtocolProxyCallback, aRequest: *const nsICancelable, aChannel: *const nsIChannel, aProxyInfo: *const nsIProxyInfo, aStatus: nsresult) -> nsresult,

}


impl nsIProtocolProxyCallback {
    /* void onProxyAvailable (in nsICancelable aRequest, in nsIChannel aChannel, in nsIProxyInfo aProxyInfo, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onProxyAvailable(&self, aRequest: Option<&nsICancelable>, aChannel: Option<&nsIChannel>, aProxyInfo: Option<&nsIProxyInfo>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onProxyAvailable)(self as *const _, aRequest.map_or(::std::ptr::null(), |x| x as *const _), aChannel.map_or(::std::ptr::null(), |x| x as *const _), aProxyInfo.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


