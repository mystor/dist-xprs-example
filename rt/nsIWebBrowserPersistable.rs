//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebBrowserPersistable.idl
//


#[repr(C)]
pub struct nsIWebBrowserPersistable {
    vtable: *const nsIWebBrowserPersistableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebBrowserPersistable {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf4c3fa8e, 0x83e9, 0x49f8,
            [0xac, 0x6f, 0x95, 0x1f, 0xc7, 0x54, 0x1f, 0xe4])
    }
}

unsafe impl RefCounted for nsIWebBrowserPersistable {
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
pub trait nsIWebBrowserPersistableCoerce {
    fn coerce_from(v: &nsIWebBrowserPersistable) -> &Self;
}

impl nsIWebBrowserPersistableCoerce for nsIWebBrowserPersistable {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistable) -> &Self {
        v
    }
}

impl nsIWebBrowserPersistable {
    #[inline]
    pub fn coerce<T: nsIWebBrowserPersistableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebBrowserPersistable {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebBrowserPersistableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserPersistable) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebBrowserPersistableVTable {
    pub __base: nsISupportsVTable,

    /* void startPersistence (in unsigned long long aOuterWindowID, in nsIWebBrowserPersistDocumentReceiver aRecv); */
    pub startPersistence: unsafe extern "C" fn (this: *const nsIWebBrowserPersistable, aOuterWindowID: libc::uint64_t, aRecv: *const nsIWebBrowserPersistDocumentReceiver) -> nsresult,

}


impl nsIWebBrowserPersistable {
    /* void startPersistence (in unsigned long long aOuterWindowID, in nsIWebBrowserPersistDocumentReceiver aRecv); */
    #[inline]
    pub unsafe fn startPersistence(&self, aOuterWindowID: libc::uint64_t, aRecv: Option<&nsIWebBrowserPersistDocumentReceiver>) -> Result<(), nsresult> {

        match ((*self.vtable).startPersistence)(self as *const _, aOuterWindowID, aRecv.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


