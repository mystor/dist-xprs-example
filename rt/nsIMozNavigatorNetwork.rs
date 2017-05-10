//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMozNavigatorNetwork.idl
//


#[repr(C)]
pub struct nsIMozNavigatorNetwork {
    vtable: *const nsIMozNavigatorNetworkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMozNavigatorNetwork {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7956523b, 0x631e, 0x4f80,
            [0x94, 0xa5, 0x38, 0x83, 0xbc, 0xfd, 0x6b, 0xf3])
    }
}

unsafe impl RefCounted for nsIMozNavigatorNetwork {
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
pub trait nsIMozNavigatorNetworkCoerce {
    fn coerce_from(v: &nsIMozNavigatorNetwork) -> &Self;
}

impl nsIMozNavigatorNetworkCoerce for nsIMozNavigatorNetwork {
    #[inline]
    fn coerce_from(v: &nsIMozNavigatorNetwork) -> &Self {
        v
    }
}

impl nsIMozNavigatorNetwork {
    #[inline]
    pub fn coerce<T: nsIMozNavigatorNetworkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMozNavigatorNetwork {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIMozNavigatorNetworkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozNavigatorNetwork) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMozNavigatorNetworkVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsINetworkProperties properties; */
    pub get_properties: unsafe extern "C" fn (this: *const nsIMozNavigatorNetwork, aProperties: *mut *const nsINetworkProperties) -> nsresult,

}


impl nsIMozNavigatorNetwork {
    /* readonly attribute nsINetworkProperties properties; */
    #[inline]
    pub unsafe fn get_properties(&self, ) -> Result<Option<RefPtr<nsINetworkProperties>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_properties)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


