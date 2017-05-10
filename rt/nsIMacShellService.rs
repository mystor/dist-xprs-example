//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIMacShellService.idl
//


pub mod nsIMacShellService_consts {
    pub const APPLICATION_KEYCHAIN_ACCESS: i64 = 2;
    pub const APPLICATION_NETWORK: i64 = 3;
    pub const APPLICATION_DESKTOP: i64 = 4;
}


#[repr(C)]
pub struct nsIMacShellService {
    vtable: *const nsIMacShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIMacShellService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x387fdc80, 0x0077, 0x4b60,
            [0xa0, 0xd9, 0xd9, 0xe8, 0x0a, 0x83, 0xba, 0x64])
    }
}

unsafe impl RefCounted for nsIMacShellService {
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
pub trait nsIMacShellServiceCoerce {
    fn coerce_from(v: &nsIMacShellService) -> &Self;
}

impl nsIMacShellServiceCoerce for nsIMacShellService {
    #[inline]
    fn coerce_from(v: &nsIMacShellService) -> &Self {
        v
    }
}

impl nsIMacShellService {
    #[inline]
    pub fn coerce<T: nsIMacShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIMacShellService {
    type Target = nsIShellService;
    #[inline]
    fn deref(&self) -> &nsIShellService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIShellServiceCoerce> nsIMacShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMacShellService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIMacShellServiceVTable {
    pub __base: nsIShellServiceVTable,

}


impl nsIMacShellService {
}


