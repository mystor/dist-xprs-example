//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISecurityInfoProvider.idl
//


#[repr(C)]
pub struct nsISecurityInfoProvider {
    vtable: *const nsISecurityInfoProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISecurityInfoProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb8cc9126, 0x9319, 0x4415,
            [0xaf, 0xd9, 0xb8, 0x22, 0x20, 0xd4, 0x53, 0xed])
    }
}

unsafe impl RefCounted for nsISecurityInfoProvider {
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
pub trait nsISecurityInfoProviderCoerce {
    fn coerce_from(v: &nsISecurityInfoProvider) -> &Self;
}

impl nsISecurityInfoProviderCoerce for nsISecurityInfoProvider {
    #[inline]
    fn coerce_from(v: &nsISecurityInfoProvider) -> &Self {
        v
    }
}

impl nsISecurityInfoProvider {
    #[inline]
    pub fn coerce<T: nsISecurityInfoProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISecurityInfoProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISecurityInfoProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityInfoProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISecurityInfoProviderVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsISecurityInfoProvider, aSecurityInfo: *mut *const nsISupports) -> nsresult,

    /* readonly attribute boolean hasTransferredData; */
    pub get_hasTransferredData: unsafe extern "C" fn (this: *const nsISecurityInfoProvider, aHasTransferredData: *mut bool) -> nsresult,

}


impl nsISecurityInfoProvider {
    /* readonly attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute boolean hasTransferredData; */
    #[inline]
    pub unsafe fn get_hasTransferredData(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_hasTransferredData)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


