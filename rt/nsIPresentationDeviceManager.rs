//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDeviceManager.idl
//


#[repr(C)]
pub struct nsIPresentationDeviceManager {
    vtable: *const nsIPresentationDeviceManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDeviceManager {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xbeb61db5, 0x3d5f, 0x454f,
            [0xa1, 0x5a, 0xdb, 0xfa, 0x03, 0x37, 0xc5, 0x69])
    }
}

unsafe impl RefCounted for nsIPresentationDeviceManager {
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
pub trait nsIPresentationDeviceManagerCoerce {
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self;
}

impl nsIPresentationDeviceManagerCoerce for nsIPresentationDeviceManager {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self {
        v
    }
}

impl nsIPresentationDeviceManager {
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDeviceManager {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDeviceManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceManager) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDeviceManagerVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean deviceAvailable; */
    pub get_deviceAvailable: unsafe extern "C" fn (this: *const nsIPresentationDeviceManager, aDeviceAvailable: *mut bool) -> nsresult,

    /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
    pub addDeviceProvider: unsafe extern "C" fn (this: *const nsIPresentationDeviceManager, provider: *const nsIPresentationDeviceProvider) -> nsresult,

    /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
    pub removeDeviceProvider: unsafe extern "C" fn (this: *const nsIPresentationDeviceManager, provider: *const nsIPresentationDeviceProvider) -> nsresult,

    /* void forceDiscovery (); */
    pub forceDiscovery: unsafe extern "C" fn (this: *const nsIPresentationDeviceManager) -> nsresult,

    /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
    pub getAvailableDevices: unsafe extern "C" fn (this: *const nsIPresentationDeviceManager, presentationUrls: *const nsIArray, _retval: *mut *const nsIArray) -> nsresult,

}


impl nsIPresentationDeviceManager {
    /* readonly attribute boolean deviceAvailable; */
    #[inline]
    pub unsafe fn get_deviceAvailable(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_deviceAvailable)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void addDeviceProvider (in nsIPresentationDeviceProvider provider); */
    #[inline]
    pub unsafe fn addDeviceProvider(&self, provider: Option<&nsIPresentationDeviceProvider>) -> Result<(), nsresult> {

        match ((*self.vtable).addDeviceProvider)(self as *const _, provider.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDeviceProvider (in nsIPresentationDeviceProvider provider); */
    #[inline]
    pub unsafe fn removeDeviceProvider(&self, provider: Option<&nsIPresentationDeviceProvider>) -> Result<(), nsresult> {

        match ((*self.vtable).removeDeviceProvider)(self as *const _, provider.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void forceDiscovery (); */
    #[inline]
    pub unsafe fn forceDiscovery(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).forceDiscovery)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* nsIArray getAvailableDevices ([optional] in nsIArray presentationUrls); */
    #[inline]
    pub unsafe fn getAvailableDevices(&self, presentationUrls: Option<&nsIArray>) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getAvailableDevices)(self as *const _, presentationUrls.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


