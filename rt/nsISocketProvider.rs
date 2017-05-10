//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketProvider.idl
//


pub mod nsISocketProvider_consts {
    pub const PROXY_RESOLVES_HOST: i64 = 1;
    pub const ANONYMOUS_CONNECT: i64 = 2;
    pub const NO_PERMANENT_STORAGE: i64 = 4;
    pub const MITM_OK: i64 = 8;
    pub const BE_CONSERVATIVE: i64 = 16;
}


#[repr(C)]
pub struct nsISocketProvider {
    vtable: *const nsISocketProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x508d5469, 0x9e1e, 0x4a08,
            [0xb5, 0xb0, 0x7c, 0xfe, 0xbb, 0xa1, 0xe5, 0x1a])
    }
}

unsafe impl RefCounted for nsISocketProvider {
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
pub trait nsISocketProviderCoerce {
    fn coerce_from(v: &nsISocketProvider) -> &Self;
}

impl nsISocketProviderCoerce for nsISocketProvider {
    #[inline]
    fn coerce_from(v: &nsISocketProvider) -> &Self {
        v
    }
}

impl nsISocketProvider {
    #[inline]
    pub fn coerce<T: nsISocketProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISocketProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketProviderVTable {
    pub __base: nsISupportsVTable,

    /* [noscript] void newSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, out PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub newSocket: *const ::libc::c_void,

    /* [noscript] void addToSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub addToSocket: *const ::libc::c_void,

}


impl nsISocketProvider {
    /* [noscript] void newSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, out PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */


    /* [noscript] void addToSocket (in long aFamily, in string aHost, in long aPort, in nsIProxyInfo aProxy, in const_OriginAttributesRef aOriginAttributes, in unsigned long aFlags, in PRFileDescStar aFileDesc, out nsISupports aSecurityInfo); */


}


