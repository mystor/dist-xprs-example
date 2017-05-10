//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWifiMonitor.idl
//


#[repr(C)]
pub struct nsIWifiMonitor {
    vtable: *const nsIWifiMonitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWifiMonitor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf289701e, 0xd9af, 0x4685,
            [0xbc, 0x2f, 0xe4, 0x22, 0x6f, 0xf7, 0xc0, 0x18])
    }
}

unsafe impl RefCounted for nsIWifiMonitor {
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
pub trait nsIWifiMonitorCoerce {
    fn coerce_from(v: &nsIWifiMonitor) -> &Self;
}

impl nsIWifiMonitorCoerce for nsIWifiMonitor {
    #[inline]
    fn coerce_from(v: &nsIWifiMonitor) -> &Self {
        v
    }
}

impl nsIWifiMonitor {
    #[inline]
    pub fn coerce<T: nsIWifiMonitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWifiMonitor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWifiMonitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWifiMonitor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWifiMonitorVTable {
    pub __base: nsISupportsVTable,

    /* void startWatching (in nsIWifiListener aListener); */
    pub startWatching: unsafe extern "C" fn (this: *const nsIWifiMonitor, aListener: *const nsIWifiListener) -> nsresult,

    /* void stopWatching (in nsIWifiListener aListener); */
    pub stopWatching: unsafe extern "C" fn (this: *const nsIWifiMonitor, aListener: *const nsIWifiListener) -> nsresult,

}


impl nsIWifiMonitor {
    /* void startWatching (in nsIWifiListener aListener); */
    #[inline]
    pub unsafe fn startWatching(&self, aListener: Option<&nsIWifiListener>) -> Result<(), nsresult> {

        match ((*self.vtable).startWatching)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void stopWatching (in nsIWifiListener aListener); */
    #[inline]
    pub unsafe fn stopWatching(&self, aListener: Option<&nsIWifiListener>) -> Result<(), nsresult> {

        match ((*self.vtable).stopWatching)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


