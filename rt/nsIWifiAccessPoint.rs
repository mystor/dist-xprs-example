//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWifiAccessPoint.idl
//


#[repr(C)]
pub struct nsIWifiAccessPoint {
    vtable: *const nsIWifiAccessPointVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWifiAccessPoint {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe28e614f, 0x8f86, 0x44ff,
            [0xbc, 0xf5, 0x5f, 0x18, 0x22, 0x58, 0x34, 0xa0])
    }
}

unsafe impl RefCounted for nsIWifiAccessPoint {
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
pub trait nsIWifiAccessPointCoerce {
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self;
}

impl nsIWifiAccessPointCoerce for nsIWifiAccessPoint {
    #[inline]
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self {
        v
    }
}

impl nsIWifiAccessPoint {
    #[inline]
    pub fn coerce<T: nsIWifiAccessPointCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWifiAccessPoint {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWifiAccessPointCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWifiAccessPoint) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWifiAccessPointVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString mac; */
    pub get_mac: unsafe extern "C" fn (this: *const nsIWifiAccessPoint, aMac: *mut nsACString) -> nsresult,

    /* readonly attribute AString ssid; */
    pub get_ssid: unsafe extern "C" fn (this: *const nsIWifiAccessPoint, aSsid: *mut nsAString) -> nsresult,

    /* readonly attribute ACString rawSSID; */
    pub get_rawSSID: unsafe extern "C" fn (this: *const nsIWifiAccessPoint, aRawSSID: *mut nsACString) -> nsresult,

    /* readonly attribute long signal; */
    pub get_signal: unsafe extern "C" fn (this: *const nsIWifiAccessPoint, aSignal: *mut libc::int32_t) -> nsresult,

}


impl nsIWifiAccessPoint {
    /* readonly attribute ACString mac; */
    #[inline]
    pub unsafe fn get_mac(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_mac)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AString ssid; */
    #[inline]
    pub unsafe fn get_ssid(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_ssid)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString rawSSID; */
    #[inline]
    pub unsafe fn get_rawSSID(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_rawSSID)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute long signal; */
    #[inline]
    pub unsafe fn get_signal(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_signal)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


