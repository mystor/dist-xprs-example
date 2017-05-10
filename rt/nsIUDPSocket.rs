//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIUDPSocket.idl
//


#[repr(C)]
pub struct nsIUDPSocket {
    vtable: *const nsIUDPSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUDPSocket {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd423bf4e, 0x4499, 0x40cf,
            [0xbc, 0x03, 0x15, 0x3e, 0x2b, 0xf2, 0x06, 0xd1])
    }
}

unsafe impl RefCounted for nsIUDPSocket {
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
pub trait nsIUDPSocketCoerce {
    fn coerce_from(v: &nsIUDPSocket) -> &Self;
}

impl nsIUDPSocketCoerce for nsIUDPSocket {
    #[inline]
    fn coerce_from(v: &nsIUDPSocket) -> &Self {
        v
    }
}

impl nsIUDPSocket {
    #[inline]
    pub fn coerce<T: nsIUDPSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUDPSocket {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUDPSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocket) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUDPSocketVTable {
    pub __base: nsISupportsVTable,

    /* [optional_argc] void init (in long aPort, in boolean aLoopbackOnly, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to call function as its signature contains a non-rust type
    pub init: *const ::libc::c_void,

    /* [optional_argc] void init2 (in AUTF8String aAddr, in long aPort, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to call function as its signature contains a non-rust type
    pub init2: *const ::libc::c_void,

    /* [noscript,optional_argc] void initWithAddress ([const] in NetAddrPtr aAddr, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithAddress: *const ::libc::c_void,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIUDPSocket) -> nsresult,

    /* void asyncListen (in nsIUDPSocketListener aListener); */
    pub asyncListen: unsafe extern "C" fn (this: *const nsIUDPSocket, aListener: *const nsIUDPSocketListener) -> nsresult,

    /* void connect ([const] in NetAddrPtr aAddr); */
    /// Unable to call function as its signature contains a non-rust type
    pub connect: *const ::libc::c_void,

    /* readonly attribute nsINetAddr localAddr; */
    pub get_localAddr: unsafe extern "C" fn (this: *const nsIUDPSocket, aLocalAddr: *mut *const nsINetAddr) -> nsresult,

    /* readonly attribute long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIUDPSocket, aPort: *mut libc::int32_t) -> nsresult,

    /* [noscript] NetAddr getAddress (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAddress: *const ::libc::c_void,

    /* unsigned long send (in AUTF8String host, in unsigned short port, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub send: *const ::libc::c_void,

    /* unsigned long sendWithAddr (in nsINetAddr addr, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendWithAddr: *const ::libc::c_void,

    /* [noscript] unsigned long sendWithAddress ([const] in NetAddrPtr addr, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendWithAddress: *const ::libc::c_void,

    /* void sendBinaryStream (in AUTF8String host, in unsigned short port, in nsIInputStream stream); */
    pub sendBinaryStream: unsafe extern "C" fn (this: *const nsIUDPSocket, host: *const nsACString, port: libc::uint16_t, stream: *const nsIInputStream) -> nsresult,

    /* void sendBinaryStreamWithAddress ([const] in NetAddrPtr addr, in nsIInputStream stream); */
    /// Unable to call function as its signature contains a non-rust type
    pub sendBinaryStreamWithAddress: *const ::libc::c_void,

    /* void joinMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    pub joinMulticast: unsafe extern "C" fn (this: *const nsIUDPSocket, addr: *const nsACString, iface: *const nsACString) -> nsresult,

    /* [noscript] void joinMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
    /// Unable to call function as its signature contains a non-rust type
    pub joinMulticastAddr: *const ::libc::c_void,

    /* void leaveMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    pub leaveMulticast: unsafe extern "C" fn (this: *const nsIUDPSocket, addr: *const nsACString, iface: *const nsACString) -> nsresult,

    /* [noscript] void leaveMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */
    /// Unable to call function as its signature contains a non-rust type
    pub leaveMulticastAddr: *const ::libc::c_void,

    /* attribute boolean multicastLoopback; */
    pub get_multicastLoopback: unsafe extern "C" fn (this: *const nsIUDPSocket, aMulticastLoopback: *mut bool) -> nsresult,
    pub set_multicastLoopback: unsafe extern "C" fn (this: *const nsIUDPSocket, aMulticastLoopback: bool) -> nsresult,

    /* attribute AUTF8String multicastInterface; */
    pub get_multicastInterface: unsafe extern "C" fn (this: *const nsIUDPSocket, aMulticastInterface: *mut nsACString) -> nsresult,
    pub set_multicastInterface: unsafe extern "C" fn (this: *const nsIUDPSocket, aMulticastInterface: *const nsACString) -> nsresult,

    /* [noscript] attribute NetAddr multicastInterfaceAddr; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_multicastInterfaceAddr: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_multicastInterfaceAddr: *const ::libc::c_void,

    /* [noscript] attribute long recvBufferSize; */
    pub get_recvBufferSize: unsafe extern "C" fn (this: *const nsIUDPSocket, aRecvBufferSize: *mut libc::int32_t) -> nsresult,
    pub set_recvBufferSize: unsafe extern "C" fn (this: *const nsIUDPSocket, aRecvBufferSize: libc::int32_t) -> nsresult,

    /* [noscript] attribute long sendBufferSize; */
    pub get_sendBufferSize: unsafe extern "C" fn (this: *const nsIUDPSocket, aSendBufferSize: *mut libc::int32_t) -> nsresult,
    pub set_sendBufferSize: unsafe extern "C" fn (this: *const nsIUDPSocket, aSendBufferSize: libc::int32_t) -> nsresult,

}


impl nsIUDPSocket {
    /* [optional_argc] void init (in long aPort, in boolean aLoopbackOnly, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */


    /* [optional_argc] void init2 (in AUTF8String aAddr, in long aPort, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */


    /* [noscript,optional_argc] void initWithAddress ([const] in NetAddrPtr aAddr, in nsIPrincipal aPrincipal, [optional] in boolean aAddressReuse); */


    /* void close (); */
    #[inline]
    pub unsafe fn close(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void asyncListen (in nsIUDPSocketListener aListener); */
    #[inline]
    pub unsafe fn asyncListen(&self, aListener: Option<&nsIUDPSocketListener>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncListen)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void connect ([const] in NetAddrPtr aAddr); */


    /* readonly attribute nsINetAddr localAddr; */
    #[inline]
    pub unsafe fn get_localAddr(&self, ) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_localAddr)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute long port; */
    #[inline]
    pub unsafe fn get_port(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_port)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] NetAddr getAddress (); */


    /* unsigned long send (in AUTF8String host, in unsigned short port, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */


    /* unsigned long sendWithAddr (in nsINetAddr addr, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */


    /* [noscript] unsigned long sendWithAddress ([const] in NetAddrPtr addr, [array, size_is (dataLength), const] in uint8_t data, in unsigned long dataLength); */


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

    /* void sendBinaryStreamWithAddress ([const] in NetAddrPtr addr, in nsIInputStream stream); */


    /* void joinMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    #[inline]
    pub unsafe fn joinMulticast(&self, addr: &[u8], iface: &[u8]) -> Result<(), nsresult> {
        let addr = nsCString::from(addr);
        let iface = nsCString::from(iface);
        match ((*self.vtable).joinMulticast)(self as *const _, &*addr, &*iface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void joinMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */


    /* void leaveMulticast (in AUTF8String addr, [optional] in AUTF8String iface); */
    #[inline]
    pub unsafe fn leaveMulticast(&self, addr: &[u8], iface: &[u8]) -> Result<(), nsresult> {
        let addr = nsCString::from(addr);
        let iface = nsCString::from(iface);
        match ((*self.vtable).leaveMulticast)(self as *const _, &*addr, &*iface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void leaveMulticastAddr ([const] in NetAddr addr, [const, optional] in NetAddrPtr iface); */


    /* attribute boolean multicastLoopback; */
    #[inline]
    pub unsafe fn get_multicastLoopback(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_multicastLoopback)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_multicastLoopback(&self, aMulticastLoopback: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_multicastLoopback)(self as *const _, aMulticastLoopback) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute AUTF8String multicastInterface; */
    #[inline]
    pub unsafe fn get_multicastInterface(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_multicastInterface)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_multicastInterface(&self, aMulticastInterface: &[u8]) -> Result<(), nsresult> {
        let aMulticastInterface = nsCString::from(aMulticastInterface);
        match ((*self.vtable).set_multicastInterface)(self as *const _, &*aMulticastInterface) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute NetAddr multicastInterfaceAddr; */



    /* [noscript] attribute long recvBufferSize; */
    #[inline]
    pub unsafe fn get_recvBufferSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_recvBufferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_recvBufferSize(&self, aRecvBufferSize: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_recvBufferSize)(self as *const _, aRecvBufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute long sendBufferSize; */
    #[inline]
    pub unsafe fn get_sendBufferSize(&self, ) -> Result<libc::int32_t, nsresult> {
        let mut _retval: libc::int32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sendBufferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sendBufferSize(&self, aSendBufferSize: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sendBufferSize)(self as *const _, aSendBufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUDPSocketListener {
    vtable: *const nsIUDPSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUDPSocketListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2e4b5dd3, 0x7358, 0x4281,
            [0xb8, 0x1f, 0x10, 0xc6, 0x2e, 0xf3, 0x9c, 0xb5])
    }
}

unsafe impl RefCounted for nsIUDPSocketListener {
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
pub trait nsIUDPSocketListenerCoerce {
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self;
}

impl nsIUDPSocketListenerCoerce for nsIUDPSocketListener {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self {
        v
    }
}

impl nsIUDPSocketListener {
    #[inline]
    pub fn coerce<T: nsIUDPSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUDPSocketListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUDPSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUDPSocketListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
    pub onPacketReceived: unsafe extern "C" fn (this: *const nsIUDPSocketListener, aSocket: *const nsIUDPSocket, aMessage: *const nsIUDPMessage) -> nsresult,

    /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
    pub onStopListening: unsafe extern "C" fn (this: *const nsIUDPSocketListener, aSocket: *const nsIUDPSocket, aStatus: nsresult) -> nsresult,

}


impl nsIUDPSocketListener {
    /* void onPacketReceived (in nsIUDPSocket aSocket, in nsIUDPMessage aMessage); */
    #[inline]
    pub unsafe fn onPacketReceived(&self, aSocket: Option<&nsIUDPSocket>, aMessage: Option<&nsIUDPMessage>) -> Result<(), nsresult> {

        match ((*self.vtable).onPacketReceived)(self as *const _, aSocket.map_or(::std::ptr::null(), |x| x as *const _), aMessage.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStopListening (in nsIUDPSocket aSocket, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onStopListening(&self, aSocket: Option<&nsIUDPSocket>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onStopListening)(self as *const _, aSocket.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIUDPMessage {
    vtable: *const nsIUDPMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIUDPMessage {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xafdc743f, 0x9cc0, 0x40d8,
            [0xb4, 0x42, 0x69, 0x5d, 0xc5, 0x4b, 0xbb, 0x74])
    }
}

unsafe impl RefCounted for nsIUDPMessage {
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
pub trait nsIUDPMessageCoerce {
    fn coerce_from(v: &nsIUDPMessage) -> &Self;
}

impl nsIUDPMessageCoerce for nsIUDPMessage {
    #[inline]
    fn coerce_from(v: &nsIUDPMessage) -> &Self {
        v
    }
}

impl nsIUDPMessage {
    #[inline]
    pub fn coerce<T: nsIUDPMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIUDPMessage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIUDPMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPMessage) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIUDPMessageVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsINetAddr fromAddr; */
    pub get_fromAddr: unsafe extern "C" fn (this: *const nsIUDPMessage, aFromAddr: *mut *const nsINetAddr) -> nsresult,

    /* readonly attribute ACString data; */
    pub get_data: unsafe extern "C" fn (this: *const nsIUDPMessage, aData: *mut nsACString) -> nsresult,

    /* readonly attribute nsIOutputStream outputStream; */
    pub get_outputStream: unsafe extern "C" fn (this: *const nsIUDPMessage, aOutputStream: *mut *const nsIOutputStream) -> nsresult,

    /* [implicit_jscontext] readonly attribute jsval rawData; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_rawData: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] Uint8TArrayRef getDataAsTArray (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDataAsTArray: *const ::libc::c_void,

}


impl nsIUDPMessage {
    /* readonly attribute nsINetAddr fromAddr; */
    #[inline]
    pub unsafe fn get_fromAddr(&self, ) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_fromAddr)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute ACString data; */
    #[inline]
    pub unsafe fn get_data(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_data)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIOutputStream outputStream; */
    #[inline]
    pub unsafe fn get_outputStream(&self, ) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_outputStream)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [implicit_jscontext] readonly attribute jsval rawData; */


    /* [noscript,nostdcall,notxpcom] Uint8TArrayRef getDataAsTArray (); */


}


