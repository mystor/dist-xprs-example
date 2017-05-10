//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketProviderService.idl
//


#[repr(C)]
pub struct nsISocketProviderService {
    vtable: *const nsISocketProviderServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketProviderService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8f8a23d0, 0x5472, 0x11d3,
            [0xbb, 0xc8, 0x00, 0x00, 0x86, 0x1d, 0x12, 0x37])
    }
}

unsafe impl RefCounted for nsISocketProviderService {
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
pub trait nsISocketProviderServiceCoerce {
    fn coerce_from(v: &nsISocketProviderService) -> &Self;
}

impl nsISocketProviderServiceCoerce for nsISocketProviderService {
    #[inline]
    fn coerce_from(v: &nsISocketProviderService) -> &Self {
        v
    }
}

impl nsISocketProviderService {
    #[inline]
    pub fn coerce<T: nsISocketProviderServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketProviderService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsISocketProviderServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketProviderService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketProviderServiceVTable {
    pub __base: nsISupportsVTable,

    /* nsISocketProvider getSocketProvider (in string socketType); */
    pub getSocketProvider: unsafe extern "C" fn (this: *const nsISocketProviderService, socketType: *const libc::c_char, _retval: *mut *const nsISocketProvider) -> nsresult,

}


impl nsISocketProviderService {
    /* nsISocketProvider getSocketProvider (in string socketType); */
    #[inline]
    pub unsafe fn getSocketProvider(&self, socketType: *const libc::c_char) -> Result<Option<RefPtr<nsISocketProvider>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getSocketProvider)(self as *const _, socketType, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


