//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkProperties.idl
//


#[repr(C)]
pub struct nsINetworkProperties {
    vtable: *const nsINetworkPropertiesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkProperties {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0af94dec, 0x7ffc, 0x4301,
            [0x89, 0x37, 0x76, 0x6c, 0x21, 0x4a, 0xc6, 0x88])
    }
}

unsafe impl RefCounted for nsINetworkProperties {
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
pub trait nsINetworkPropertiesCoerce {
    fn coerce_from(v: &nsINetworkProperties) -> &Self;
}

impl nsINetworkPropertiesCoerce for nsINetworkProperties {
    #[inline]
    fn coerce_from(v: &nsINetworkProperties) -> &Self {
        v
    }
}

impl nsINetworkProperties {
    #[inline]
    pub fn coerce<T: nsINetworkPropertiesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkProperties {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkPropertiesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkProperties) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkPropertiesVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isWifi; */
    pub get_isWifi: unsafe extern "C" fn (this: *const nsINetworkProperties, aIsWifi: *mut bool) -> nsresult,

    /* readonly attribute unsigned long dhcpGateway; */
    pub get_dhcpGateway: unsafe extern "C" fn (this: *const nsINetworkProperties, aDhcpGateway: *mut libc::uint32_t) -> nsresult,

}


impl nsINetworkProperties {
    /* readonly attribute boolean isWifi; */
    #[inline]
    pub unsafe fn get_isWifi(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_isWifi)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long dhcpGateway; */
    #[inline]
    pub unsafe fn get_dhcpGateway(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_dhcpGateway)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


