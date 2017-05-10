//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIProxiedChannel.idl
//


#[repr(C)]
pub struct nsIProxiedChannel {
    vtable: *const nsIProxiedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIProxiedChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x6238f134, 0x8c3f, 0x4354,
            [0x95, 0x8f, 0xdf, 0xd9, 0xd5, 0x4a, 0x44, 0x46])
    }
}

unsafe impl RefCounted for nsIProxiedChannel {
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
pub trait nsIProxiedChannelCoerce {
    fn coerce_from(v: &nsIProxiedChannel) -> &Self;
}

impl nsIProxiedChannelCoerce for nsIProxiedChannel {
    #[inline]
    fn coerce_from(v: &nsIProxiedChannel) -> &Self {
        v
    }
}

impl nsIProxiedChannel {
    #[inline]
    pub fn coerce<T: nsIProxiedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIProxiedChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIProxiedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxiedChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIProxiedChannelVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIProxyInfo proxyInfo; */
    pub get_proxyInfo: unsafe extern "C" fn (this: *const nsIProxiedChannel, aProxyInfo: *mut *const nsIProxyInfo) -> nsresult,

}


impl nsIProxiedChannel {
    /* readonly attribute nsIProxyInfo proxyInfo; */
    #[inline]
    pub unsafe fn get_proxyInfo(&self, ) -> Result<Option<RefPtr<nsIProxyInfo>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_proxyInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


