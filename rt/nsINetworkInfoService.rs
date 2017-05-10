//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkInfoService.idl
//


#[repr(C)]
pub struct nsIListNetworkAddressesListener {
    vtable: *const nsIListNetworkAddressesListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIListNetworkAddressesListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xc4bdaac1, 0x3ab1, 0x4fdb,
            [0x9a, 0x16, 0x17, 0xcb, 0xed, 0x79, 0x46, 0x03])
    }
}

unsafe impl RefCounted for nsIListNetworkAddressesListener {
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
pub trait nsIListNetworkAddressesListenerCoerce {
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self;
}

impl nsIListNetworkAddressesListenerCoerce for nsIListNetworkAddressesListener {
    #[inline]
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self {
        v
    }
}

impl nsIListNetworkAddressesListener {
    #[inline]
    pub fn coerce<T: nsIListNetworkAddressesListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIListNetworkAddressesListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIListNetworkAddressesListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIListNetworkAddressesListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIListNetworkAddressesListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onListedNetworkAddresses ([array, size_is (aAddressArraySize)] in string aAddressArray, in unsigned long aAddressArraySize); */
    /// Unable to call function as its signature contains a non-rust type
    pub onListedNetworkAddresses: *const ::libc::c_void,

    /* void onListNetworkAddressesFailed (); */
    pub onListNetworkAddressesFailed: unsafe extern "C" fn (this: *const nsIListNetworkAddressesListener) -> nsresult,

}


impl nsIListNetworkAddressesListener {
    /* void onListedNetworkAddresses ([array, size_is (aAddressArraySize)] in string aAddressArray, in unsigned long aAddressArraySize); */


    /* void onListNetworkAddressesFailed (); */
    #[inline]
    pub unsafe fn onListNetworkAddressesFailed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onListNetworkAddressesFailed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIGetHostnameListener {
    vtable: *const nsIGetHostnameListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIGetHostnameListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ebdcb62, 0x2df4, 0x4042,
            [0x88, 0x64, 0x3f, 0xa8, 0x1a, 0xbd, 0x46, 0x93])
    }
}

unsafe impl RefCounted for nsIGetHostnameListener {
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
pub trait nsIGetHostnameListenerCoerce {
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self;
}

impl nsIGetHostnameListenerCoerce for nsIGetHostnameListener {
    #[inline]
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self {
        v
    }
}

impl nsIGetHostnameListener {
    #[inline]
    pub fn coerce<T: nsIGetHostnameListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIGetHostnameListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIGetHostnameListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGetHostnameListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIGetHostnameListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onGotHostname (in AUTF8String aHostname); */
    pub onGotHostname: unsafe extern "C" fn (this: *const nsIGetHostnameListener, aHostname: *const nsACString) -> nsresult,

    /* void onGetHostnameFailed (); */
    pub onGetHostnameFailed: unsafe extern "C" fn (this: *const nsIGetHostnameListener) -> nsresult,

}


impl nsIGetHostnameListener {
    /* void onGotHostname (in AUTF8String aHostname); */
    #[inline]
    pub unsafe fn onGotHostname(&self, aHostname: &[u8]) -> Result<(), nsresult> {
        let aHostname = nsCString::from(aHostname);
        match ((*self.vtable).onGotHostname)(self as *const _, &*aHostname) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onGetHostnameFailed (); */
    #[inline]
    pub unsafe fn onGetHostnameFailed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).onGetHostnameFailed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINetworkInfoService {
    vtable: *const nsINetworkInfoServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkInfoService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x55fc8dae, 0x4a58, 0x4e0f,
            [0xa4, 0x9b, 0x90, 0x1c, 0xba, 0xba, 0xe8, 0x09])
    }
}

unsafe impl RefCounted for nsINetworkInfoService {
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
pub trait nsINetworkInfoServiceCoerce {
    fn coerce_from(v: &nsINetworkInfoService) -> &Self;
}

impl nsINetworkInfoServiceCoerce for nsINetworkInfoService {
    #[inline]
    fn coerce_from(v: &nsINetworkInfoService) -> &Self {
        v
    }
}

impl nsINetworkInfoService {
    #[inline]
    pub fn coerce<T: nsINetworkInfoServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkInfoService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkInfoServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkInfoService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkInfoServiceVTable {
    pub __base: nsISupportsVTable,

    /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
    pub listNetworkAddresses: unsafe extern "C" fn (this: *const nsINetworkInfoService, aListener: *const nsIListNetworkAddressesListener) -> nsresult,

    /* void getHostname (in nsIGetHostnameListener aListener); */
    pub getHostname: unsafe extern "C" fn (this: *const nsINetworkInfoService, aListener: *const nsIGetHostnameListener) -> nsresult,

}


impl nsINetworkInfoService {
    /* void listNetworkAddresses (in nsIListNetworkAddressesListener aListener); */
    #[inline]
    pub unsafe fn listNetworkAddresses(&self, aListener: Option<&nsIListNetworkAddressesListener>) -> Result<(), nsresult> {

        match ((*self.vtable).listNetworkAddresses)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void getHostname (in nsIGetHostnameListener aListener); */
    #[inline]
    pub unsafe fn getHostname(&self, aListener: Option<&nsIGetHostnameListener>) -> Result<(), nsresult> {

        match ((*self.vtable).getHostname)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


