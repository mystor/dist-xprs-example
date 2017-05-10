//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISOCKSSocketInfo.idl
//


#[repr(C)]
pub struct nsISOCKSSocketInfo {
    vtable: *const nsISOCKSSocketInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISOCKSSocketInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd5c0d1f9, 0x22d7, 0x47dc,
            [0xbf, 0x91, 0xd9, 0xac, 0x6e, 0x12, 0x51, 0xa6])
    }
}

unsafe impl RefCounted for nsISOCKSSocketInfo {
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
pub trait nsISOCKSSocketInfoCoerce {
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self;
}

impl nsISOCKSSocketInfoCoerce for nsISOCKSSocketInfo {
    #[inline]
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self {
        v
    }
}

impl nsISOCKSSocketInfo {
    #[inline]
    pub fn coerce<T: nsISOCKSSocketInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISOCKSSocketInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISOCKSSocketInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISOCKSSocketInfoVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] attribute NetAddrPtr destinationAddr; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_destinationAddr: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_destinationAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr externalProxyAddr; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_externalProxyAddr: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_externalProxyAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr internalProxyAddr; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_internalProxyAddr: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_internalProxyAddr: *const ::libc::c_void,

}


impl nsISOCKSSocketInfo {
    /* [noscript] attribute NetAddrPtr destinationAddr; */



    /* [noscript] attribute NetAddrPtr externalProxyAddr; */



    /* [noscript] attribute NetAddrPtr internalProxyAddr; */



}


