//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationDeviceProvider.idl
//


#[repr(C)]
pub struct nsIPresentationDeviceListener {
    vtable: *const nsIPresentationDeviceListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDeviceListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x46fd372b, 0x2e40, 0x4179,
            [0x9b, 0x36, 0x04, 0x78, 0xd1, 0x41, 0xe4, 0x40])
    }
}

unsafe impl RefCounted for nsIPresentationDeviceListener {
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
pub trait nsIPresentationDeviceListenerCoerce {
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self;
}

impl nsIPresentationDeviceListenerCoerce for nsIPresentationDeviceListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self {
        v
    }
}

impl nsIPresentationDeviceListener {
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDeviceListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDeviceListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDeviceListenerVTable {
    pub __base: nsISupportsVTable,

    /* void addDevice (in nsIPresentationDevice device); */
    pub addDevice: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> nsresult,

    /* void removeDevice (in nsIPresentationDevice device); */
    pub removeDevice: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> nsresult,

    /* void updateDevice (in nsIPresentationDevice device); */
    pub updateDevice: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice) -> nsresult,

    /* void onSessionRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
    pub onSessionRequest: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, url: *const nsAString, presentationId: *const nsAString, controlChannel: *const nsIPresentationControlChannel) -> nsresult,

    /* void onTerminateRequest (in nsIPresentationDevice device, in DOMString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
    pub onTerminateRequest: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, presentationId: *const nsAString, controlChannel: *const nsIPresentationControlChannel, aIsFromReceiver: bool) -> nsresult,

    /* void onReconnectRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
    pub onReconnectRequest: unsafe extern "C" fn (this: *const nsIPresentationDeviceListener, device: *const nsIPresentationDevice, url: *const nsAString, presentationId: *const nsAString, controlChannel: *const nsIPresentationControlChannel) -> nsresult,

}


impl nsIPresentationDeviceListener {
    /* void addDevice (in nsIPresentationDevice device); */
    #[inline]
    pub unsafe fn addDevice(&self, device: Option<&nsIPresentationDevice>) -> Result<(), nsresult> {

        match ((*self.vtable).addDevice)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void removeDevice (in nsIPresentationDevice device); */
    #[inline]
    pub unsafe fn removeDevice(&self, device: Option<&nsIPresentationDevice>) -> Result<(), nsresult> {

        match ((*self.vtable).removeDevice)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void updateDevice (in nsIPresentationDevice device); */
    #[inline]
    pub unsafe fn updateDevice(&self, device: Option<&nsIPresentationDevice>) -> Result<(), nsresult> {

        match ((*self.vtable).updateDevice)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSessionRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
    #[inline]
    pub unsafe fn onSessionRequest(&self, device: Option<&nsIPresentationDevice>, url: &[u16], presentationId: &[u16], controlChannel: Option<&nsIPresentationControlChannel>) -> Result<(), nsresult> {
        let url = nsString::from(url);
        let presentationId = nsString::from(presentationId);
        match ((*self.vtable).onSessionRequest)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _), &*url, &*presentationId, controlChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onTerminateRequest (in nsIPresentationDevice device, in DOMString presentationId, in nsIPresentationControlChannel controlChannel, in boolean aIsFromReceiver); */
    #[inline]
    pub unsafe fn onTerminateRequest(&self, device: Option<&nsIPresentationDevice>, presentationId: &[u16], controlChannel: Option<&nsIPresentationControlChannel>, aIsFromReceiver: bool) -> Result<(), nsresult> {
        let presentationId = nsString::from(presentationId);
        match ((*self.vtable).onTerminateRequest)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _), &*presentationId, controlChannel.map_or(::std::ptr::null(), |x| x as *const _), aIsFromReceiver) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onReconnectRequest (in nsIPresentationDevice device, in DOMString url, in DOMString presentationId, in nsIPresentationControlChannel controlChannel); */
    #[inline]
    pub unsafe fn onReconnectRequest(&self, device: Option<&nsIPresentationDevice>, url: &[u16], presentationId: &[u16], controlChannel: Option<&nsIPresentationControlChannel>) -> Result<(), nsresult> {
        let url = nsString::from(url);
        let presentationId = nsString::from(presentationId);
        match ((*self.vtable).onReconnectRequest)(self as *const _, device.map_or(::std::ptr::null(), |x| x as *const _), &*url, &*presentationId, controlChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationDeviceProvider {
    vtable: *const nsIPresentationDeviceProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDeviceProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3db2578a, 0x0f50, 0x44ad,
            [0xb0, 0x1b, 0x28, 0x42, 0x7b, 0x71, 0xb7, 0xbf])
    }
}

unsafe impl RefCounted for nsIPresentationDeviceProvider {
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
pub trait nsIPresentationDeviceProviderCoerce {
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self;
}

impl nsIPresentationDeviceProviderCoerce for nsIPresentationDeviceProvider {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self {
        v
    }
}

impl nsIPresentationDeviceProvider {
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDeviceProvider {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationDeviceProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDeviceProviderVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationDeviceListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIPresentationDeviceProvider, aListener: *mut *const nsIPresentationDeviceListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIPresentationDeviceProvider, aListener: *const nsIPresentationDeviceListener) -> nsresult,

    /* void forceDiscovery (); */
    pub forceDiscovery: unsafe extern "C" fn (this: *const nsIPresentationDeviceProvider) -> nsresult,

}


impl nsIPresentationDeviceProvider {
    /* attribute nsIPresentationDeviceListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIPresentationDeviceListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIPresentationDeviceListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
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

}


