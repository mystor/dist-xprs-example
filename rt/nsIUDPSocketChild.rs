//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUDPSocketChild.idl
//


#[repr(C)]
pub struct nsIUDPSocketChild {
    vtable: *const nsIUDPSocketChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUDPSocketChild {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1e6ad73b, 0x6c05, 0x4d78,
            [0x9a, 0x88, 0x2d, 0x35, 0x7b, 0x88, 0xf5, 0x8b])
    }
}

unsafe impl RefCounted for nsIUDPSocketChild {
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
pub trait nsIUDPSocketChildCoerce {
    fn coerce_from(v: &nsIUDPSocketChild) -> &Self;
}

impl nsIUDPSocketChildCoerce for nsIUDPSocketChild {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketChild) -> &Self {
        v
    }
}

impl nsIUDPSocketChild {
    #[inline]
    pub fn coerce<T: nsIUDPSocketChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUDPSocketChild {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUDPSocketChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketChild) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUDPSocketChildVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short localPort; */
    pub get_localPort: unsafe extern "C" fn (this: *const nsIUDPSocketChild, aLocalPort: *mut libc::uint16_t) -> nsresult,

    /* readonly attribute AUTF8String localAddress; */
    pub get_localAddress: unsafe extern "C" fn (this: *const nsIUDPSocketChild, aLocalAddress: *mut nsACString) -> nsresult,

    /* attribute AUTF8String filterName; */
    pub get_filterName: unsafe extern "C" fn (this: *const nsIUDPSocketChild, aFilterName: *mut nsACString) -> nsresult,
    pub set_filterName: unsafe extern "C" fn (this: *const nsIUDPSocketChild, aFilterName: *const nsACString) -> nsresult,

    /* [noscript] void setBackgroundSpinsEvents (); */
    pub setBackgroundSpinsEvents: unsafe extern "C" fn (this: *const nsIUDPSocketChild) -> nsresult,

    /* void bind (in nsIUDPSocketInternal socket, in nsIPrincipal principal, in AUTF8String host, in unsigned short port, in bool addressReuse, in bool loopback, in uint32_t recvBufferSize, in uint32_t sendBufferSize); */
    pub bind: unsafe extern "C" fn (this: *const nsIUDPSocketChild, socket: *const nsIUDPSocketInternal, principal: *const nsIPrincipal, host: *const nsACString, port: libc::uint16_t, addressReuse: bool, loopback: bool, recvBufferSize: uint32_t, sendBufferSize: uint32_t) -> nsresult,

    /* void connect (in nsIUDPSocketInternal socket, in AUTF8String host, in unsigned short port); */
    pub connect: unsafe extern "C" fn (this: *const nsIUDPSocketChild, socket: *const nsIUDPSocketInternal, host: *const nsACString, port: libc::uint16_t) -> nsresult,

    /* void send (in AUTF8String host, in unsigned short port, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub send: *const ::libc::c_void,

    /* void sendWithAddr (in nsINetAddr addr, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendWithAddr: *const ::libc::c_void,

    /* [noscript] void sendWithAddress ([const] in NetAddrPtr addr, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendWithAddress: *const ::libc::c_void,

    /* void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream); */
    pub sendBinaryStream: unsafe extern "C" fn (this: *const nsIUDPSocketChild, host: *const nsACString, port: libc::uint16_t, stream: *const nsIInputStream) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIUDPSocketChild) -> nsresult,

    /* void joinMulticast (in AUTF8String multicastAddress, in AUTF8String iface); */
    pub joinMulticast: unsafe extern "C" fn (this: *const nsIUDPSocketChild, multicastAddress: *const nsACString, iface: *const nsACString) -> nsresult,

    /* void leaveMulticast (in AUTF8String multicastAddress, in AUTF8String iface); */
    pub leaveMulticast: unsafe extern "C" fn (this: *const nsIUDPSocketChild, multicastAddress: *const nsACString, iface: *const nsACString) -> nsresult,

}


impl nsIUDPSocketChild {
    /* readonly attribute unsigned short localPort; */
    #[inline]
    pub unsafe fn get_localPort(&self, ) -> Result<libc::uint16_t, nsresult> {
        let mut _retval: libc::uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_localPort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute AUTF8String localAddress; */
    #[inline]
    pub unsafe fn get_localAddress(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_localAddress)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* attribute AUTF8String filterName; */
    #[inline]
    pub unsafe fn get_filterName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_filterName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_filterName(&self, aFilterName: &[u8]) -> Result<(), nsresult> {
        let aFilterName = nsCString::from(aFilterName);
        match ((*self.vtable).set_filterName)(self as *const _, &*aFilterName) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setBackgroundSpinsEvents (); */
    #[inline]
    pub unsafe fn setBackgroundSpinsEvents(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).setBackgroundSpinsEvents)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void bind (in nsIUDPSocketInternal socket, in nsIPrincipal principal, in AUTF8String host, in unsigned short port, in bool addressReuse, in bool loopback, in uint32_t recvBufferSize, in uint32_t sendBufferSize); */
    #[inline]
    pub unsafe fn bind(&self, socket: Option<&nsIUDPSocketInternal>, principal: Option<&nsIPrincipal>, host: &[u8], port: libc::uint16_t, addressReuse: bool, loopback: bool, recvBufferSize: uint32_t, sendBufferSize: uint32_t) -> Result<(), nsresult> {
        let host = nsCString::from(host);
        match ((*self.vtable).bind)(self as *const _, socket.map_or(::std::ptr::null(), |x| x as *const _), principal.map_or(::std::ptr::null(), |x| x as *const _), &*host, port, addressReuse, loopback, recvBufferSize, sendBufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void connect (in nsIUDPSocketInternal socket, in AUTF8String host, in unsigned short port); */
    #[inline]
    pub unsafe fn connect(&self, socket: Option<&nsIUDPSocketInternal>, host: &[u8], port: libc::uint16_t) -> Result<(), nsresult> {
        let host = nsCString::from(host);
        match ((*self.vtable).connect)(self as *const _, socket.map_or(::std::ptr::null(), |x| x as *const _), &*host, port) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void send (in AUTF8String host, in unsigned short port, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */


    /* void sendWithAddr (in nsINetAddr addr, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */


    /* [noscript] void sendWithAddress ([const] in NetAddrPtr addr, [array, size_is (byteLength), const] in uint8_t bytes, in unsigned long byteLength); */


    /* void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream); */
    #[inline]
    pub unsafe fn sendBinaryStream(&self, host: &[u8], port: libc::uint16_t, stream: Option<&nsIInputStream>) -> Result<(), nsresult> {
        let host = nsCString::from(host);
        match ((*self.vtable).sendBinaryStream)(self as *const _, &*host, port, stream.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void joinMulticast (in AUTF8String multicastAddress, in AUTF8String iface); */
    #[inline]
    pub unsafe fn joinMulticast(&self, multicastAddress: &[u8], iface: &[u8]) -> Result<(), nsresult> {
        let multicastAddress = nsCString::from(multicastAddress);
        let iface = nsCString::from(iface);
        match ((*self.vtable).joinMulticast)(self as *const _, &*multicastAddress, &*iface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void leaveMulticast (in AUTF8String multicastAddress, in AUTF8String iface); */
    #[inline]
    pub unsafe fn leaveMulticast(&self, multicastAddress: &[u8], iface: &[u8]) -> Result<(), nsresult> {
        let multicastAddress = nsCString::from(multicastAddress);
        let iface = nsCString::from(iface);
        match ((*self.vtable).leaveMulticast)(self as *const _, &*multicastAddress, &*iface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUDPSocketInternal {
    vtable: *const nsIUDPSocketInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUDPSocketInternal {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x613dd3ad, 0x598b, 0x4da9,
            [0xad, 0x63, 0xbb, 0xda, 0x50, 0xc2, 0x00, 0x98])
    }
}

unsafe impl RefCounted for nsIUDPSocketInternal {
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
pub trait nsIUDPSocketInternalCoerce {
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self;
}

impl nsIUDPSocketInternalCoerce for nsIUDPSocketInternal {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self {
        v
    }
}

impl nsIUDPSocketInternal {
    #[inline]
    pub fn coerce<T: nsIUDPSocketInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUDPSocketInternal {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUDPSocketInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUDPSocketInternalVTable {
    pub __base: nsISupportsVTable,

    /* void callListenerOpened (); */
    pub callListenerOpened: unsafe extern "C" fn (this: *const nsIUDPSocketInternal) -> nsresult,

    /* void callListenerConnected (); */
    pub callListenerConnected: unsafe extern "C" fn (this: *const nsIUDPSocketInternal) -> nsresult,

    /* void callListenerClosed (); */
    pub callListenerClosed: unsafe extern "C" fn (this: *const nsIUDPSocketInternal) -> nsresult,

    /* void callListenerReceivedData (in AUTF8String host, in unsigned short port, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub callListenerReceivedData: *const ::libc::c_void,

    /* void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber); */
    pub callListenerError: unsafe extern "C" fn (this: *const nsIUDPSocketInternal, message: *const nsACString, filename: *const nsACString, lineNumber: uint32_t) -> nsresult,

}


impl nsIUDPSocketInternal {
    /* void callListenerOpened (); */
    #[inline]
    pub unsafe fn callListenerOpened(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callListenerOpened)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void callListenerConnected (); */
    #[inline]
    pub unsafe fn callListenerConnected(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callListenerConnected)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void callListenerClosed (); */
    #[inline]
    pub unsafe fn callListenerClosed(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).callListenerClosed)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void callListenerReceivedData (in AUTF8String host, in unsigned short port, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */


    /* void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber); */
    #[inline]
    pub unsafe fn callListenerError(&self, message: &[u8], filename: &[u8], lineNumber: uint32_t) -> Result<(), nsresult> {
        let message = nsCString::from(message);
        let filename = nsCString::from(filename);
        match ((*self.vtable).callListenerError)(self as *const _, &*message, &*filename, lineNumber) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


