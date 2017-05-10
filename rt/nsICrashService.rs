//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsICrashService.idl
//


pub mod nsICrashService_consts {
    pub const PROCESS_TYPE_MAIN: i64 = 0;
    pub const PROCESS_TYPE_CONTENT: i64 = 1;
    pub const PROCESS_TYPE_PLUGIN: i64 = 2;
    pub const PROCESS_TYPE_GMPLUGIN: i64 = 3;
    pub const PROCESS_TYPE_GPU: i64 = 4;
    pub const CRASH_TYPE_CRASH: i64 = 0;
    pub const CRASH_TYPE_HANG: i64 = 1;
}


#[repr(C)]
pub struct nsICrashService {
    vtable: *const nsICrashServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsICrashService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf60d76e5, 0x62c3, 0x4f58,
            [0x89, 0xf6, 0xb7, 0x26, 0xc2, 0xb7, 0xbc, 0x20])
    }
}

unsafe impl RefCounted for nsICrashService {
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
pub trait nsICrashServiceCoerce {
    fn coerce_from(v: &nsICrashService) -> &Self;
}

impl nsICrashServiceCoerce for nsICrashService {
    #[inline]
    fn coerce_from(v: &nsICrashService) -> &Self {
        v
    }
}

impl nsICrashService {
    #[inline]
    pub fn coerce<T: nsICrashServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsICrashService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsICrashServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICrashService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsICrashServiceVTable {
    pub __base: nsISupportsVTable,

    /* void addCrash (in long processType, in long crashType, in AString id); */
    pub addCrash: unsafe extern "C" fn (this: *const nsICrashService, processType: libc::int32_t, crashType: libc::int32_t, id: *const nsAString) -> nsresult,

}


impl nsICrashService {
    /* void addCrash (in long processType, in long crashType, in AString id); */
    #[inline]
    pub unsafe fn addCrash(&self, processType: libc::int32_t, crashType: libc::int32_t, id: &[u16]) -> Result<(), nsresult> {
        let id = nsString::from(id);
        match ((*self.vtable).addCrash)(self as *const _, processType, crashType, &*id) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


