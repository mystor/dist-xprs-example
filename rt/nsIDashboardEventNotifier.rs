//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDashboardEventNotifier.idl
//


#[repr(C)]
pub struct nsIDashboardEventNotifier {
    vtable: *const nsIDashboardEventNotifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDashboardEventNotifier {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x24fdfcbe, 0x54cb, 0x4997,
            [0x83, 0x92, 0x3c, 0x47, 0x61, 0x26, 0xea, 0x3b])
    }
}

unsafe impl RefCounted for nsIDashboardEventNotifier {
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
pub trait nsIDashboardEventNotifierCoerce {
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self;
}

impl nsIDashboardEventNotifierCoerce for nsIDashboardEventNotifier {
    #[inline]
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self {
        v
    }
}

impl nsIDashboardEventNotifier {
    #[inline]
    pub fn coerce<T: nsIDashboardEventNotifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDashboardEventNotifier {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDashboardEventNotifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDashboardEventNotifier) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDashboardEventNotifierVTable {
    pub __base: nsISupportsVTable,

    /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
    pub addHost: unsafe extern "C" fn (this: *const nsIDashboardEventNotifier, aHost: *const nsACString, aSerial: libc::uint32_t, aEncrypted: bool) -> nsresult,

    /* void removeHost (in ACString aHost, in unsigned long aSerial); */
    pub removeHost: unsafe extern "C" fn (this: *const nsIDashboardEventNotifier, aHost: *const nsACString, aSerial: libc::uint32_t) -> nsresult,

    /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    pub newMsgSent: unsafe extern "C" fn (this: *const nsIDashboardEventNotifier, aHost: *const nsACString, aSerial: libc::uint32_t, aLength: libc::uint32_t) -> nsresult,

    /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    pub newMsgReceived: unsafe extern "C" fn (this: *const nsIDashboardEventNotifier, aHost: *const nsACString, aSerial: libc::uint32_t, aLength: libc::uint32_t) -> nsresult,

}


impl nsIDashboardEventNotifier {
    /* void addHost (in ACString aHost, in unsigned long aSerial, in boolean aEncrypted); */
    #[inline]
    pub unsafe fn addHost(&self, aHost: &[u8], aSerial: libc::uint32_t, aEncrypted: bool) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).addHost)(self as *const _, &*aHost, aSerial, aEncrypted) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeHost (in ACString aHost, in unsigned long aSerial); */
    #[inline]
    pub unsafe fn removeHost(&self, aHost: &[u8], aSerial: libc::uint32_t) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).removeHost)(self as *const _, &*aHost, aSerial) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void newMsgSent (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    #[inline]
    pub unsafe fn newMsgSent(&self, aHost: &[u8], aSerial: libc::uint32_t, aLength: libc::uint32_t) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).newMsgSent)(self as *const _, &*aHost, aSerial, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void newMsgReceived (in ACString aHost, in unsigned long aSerial, in unsigned long aLength); */
    #[inline]
    pub unsafe fn newMsgReceived(&self, aHost: &[u8], aSerial: libc::uint32_t, aLength: libc::uint32_t) -> Result<(), nsresult> {
        let aHost = nsCString::from(aHost);
        match ((*self.vtable).newMsgReceived)(self as *const _, &*aHost, aSerial, aLength) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


