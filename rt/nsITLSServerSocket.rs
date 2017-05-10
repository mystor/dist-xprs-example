//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITLSServerSocket.idl
//


pub mod nsITLSServerSocket_consts {
    pub const REQUEST_NEVER: i64 = 0;
    pub const REQUEST_FIRST_HANDSHAKE: i64 = 1;
    pub const REQUEST_ALWAYS: i64 = 2;
    pub const REQUIRE_FIRST_HANDSHAKE: i64 = 3;
    pub const REQUIRE_ALWAYS: i64 = 4;
}


#[repr(C)]
pub struct nsITLSServerSocket {
    vtable: *const nsITLSServerSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITLSServerSocket {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcc2c30f9, 0xcfaa, 0x4b8a,
            [0xbd, 0x44, 0xc2, 0x48, 0x81, 0x98, 0x1b, 0x74])
    }
}

unsafe impl RefCounted for nsITLSServerSocket {
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
pub trait nsITLSServerSocketCoerce {
    fn coerce_from(v: &nsITLSServerSocket) -> &Self;
}

impl nsITLSServerSocketCoerce for nsITLSServerSocket {
    #[inline]
    fn coerce_from(v: &nsITLSServerSocket) -> &Self {
        v
    }
}

impl nsITLSServerSocket {
    #[inline]
    pub fn coerce<T: nsITLSServerSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITLSServerSocket {
    type Target = nsIServerSocket;
    #[inline]
    fn deref(&self) -> &nsIServerSocket {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIServerSocketCoerce> nsITLSServerSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerSocket) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITLSServerSocketVTable {
    pub __base: nsIServerSocketVTable,

    /* attribute nsIX509Cert serverCert; */
    pub get_serverCert: unsafe extern "C" fn (this: *const nsITLSServerSocket, aServerCert: *mut *const nsIX509Cert) -> nsresult,
    pub set_serverCert: unsafe extern "C" fn (this: *const nsITLSServerSocket, aServerCert: *const nsIX509Cert) -> nsresult,

    /* void setSessionCache (in boolean aSessionCache); */
    pub setSessionCache: unsafe extern "C" fn (this: *const nsITLSServerSocket, aSessionCache: bool) -> nsresult,

    /* void setSessionTickets (in boolean aSessionTickets); */
    pub setSessionTickets: unsafe extern "C" fn (this: *const nsITLSServerSocket, aSessionTickets: bool) -> nsresult,

    /* void setRequestClientCertificate (in unsigned long aRequestClientCert); */
    pub setRequestClientCertificate: unsafe extern "C" fn (this: *const nsITLSServerSocket, aRequestClientCert: libc::uint32_t) -> nsresult,

    /* void setCipherSuites ([array, size_is (aLength)] in unsigned short aCipherSuites, in unsigned long aLength); */
    /// Unable to call function as its signature contains a non-rust type
    pub setCipherSuites: *const ::libc::c_void,

    /* void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion); */
    pub setVersionRange: unsafe extern "C" fn (this: *const nsITLSServerSocket, aMinVersion: libc::uint16_t, aMaxVersion: libc::uint16_t) -> nsresult,

}


impl nsITLSServerSocket {
    /* attribute nsIX509Cert serverCert; */
    #[inline]
    pub unsafe fn get_serverCert(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_serverCert)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_serverCert(&self, aServerCert: Option<&nsIX509Cert>) -> Result<(), nsresult> {

        match ((*self.vtable).set_serverCert)(self as *const _, aServerCert.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSessionCache (in boolean aSessionCache); */
    #[inline]
    pub unsafe fn setSessionCache(&self, aSessionCache: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSessionCache)(self as *const _, aSessionCache) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setSessionTickets (in boolean aSessionTickets); */
    #[inline]
    pub unsafe fn setSessionTickets(&self, aSessionTickets: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setSessionTickets)(self as *const _, aSessionTickets) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setRequestClientCertificate (in unsigned long aRequestClientCert); */
    #[inline]
    pub unsafe fn setRequestClientCertificate(&self, aRequestClientCert: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setRequestClientCertificate)(self as *const _, aRequestClientCert) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setCipherSuites ([array, size_is (aLength)] in unsigned short aCipherSuites, in unsigned long aLength); */


    /* void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion); */
    #[inline]
    pub unsafe fn setVersionRange(&self, aMinVersion: libc::uint16_t, aMaxVersion: libc::uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).setVersionRange)(self as *const _, aMinVersion, aMaxVersion) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


pub mod nsITLSClientStatus_consts {
    pub const SSL_VERSION_3: i64 = 768;
    pub const TLS_VERSION_1: i64 = 769;
    pub const TLS_VERSION_1_1: i64 = 770;
    pub const TLS_VERSION_1_2: i64 = 771;
    pub const TLS_VERSION_1_3: i64 = 772;
    pub const TLS_VERSION_UNKNOWN: i64 = -1;
}


#[repr(C)]
pub struct nsITLSClientStatus {
    vtable: *const nsITLSClientStatusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITLSClientStatus {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x19668ea4, 0xe5ad, 0x4182,
            [0x96, 0x98, 0x7e, 0x89, 0x0d, 0x48, 0xf3, 0x27])
    }
}

unsafe impl RefCounted for nsITLSClientStatus {
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
pub trait nsITLSClientStatusCoerce {
    fn coerce_from(v: &nsITLSClientStatus) -> &Self;
}

impl nsITLSClientStatusCoerce for nsITLSClientStatus {
    #[inline]
    fn coerce_from(v: &nsITLSClientStatus) -> &Self {
        v
    }
}

impl nsITLSClientStatus {
    #[inline]
    pub fn coerce<T: nsITLSClientStatusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITLSClientStatus {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITLSClientStatusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSClientStatus) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITLSClientStatusVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIX509Cert peerCert; */
    pub get_peerCert: unsafe extern "C" fn (this: *const nsITLSClientStatus, aPeerCert: *mut *const nsIX509Cert) -> nsresult,

    /* readonly attribute short tlsVersionUsed; */
    pub get_tlsVersionUsed: unsafe extern "C" fn (this: *const nsITLSClientStatus, aTlsVersionUsed: *mut libc::int16_t) -> nsresult,

    /* readonly attribute ACString cipherName; */
    pub get_cipherName: unsafe extern "C" fn (this: *const nsITLSClientStatus, aCipherName: *mut nsACString) -> nsresult,

    /* readonly attribute unsigned long keyLength; */
    pub get_keyLength: unsafe extern "C" fn (this: *const nsITLSClientStatus, aKeyLength: *mut libc::uint32_t) -> nsresult,

    /* readonly attribute unsigned long macLength; */
    pub get_macLength: unsafe extern "C" fn (this: *const nsITLSClientStatus, aMacLength: *mut libc::uint32_t) -> nsresult,

}


impl nsITLSClientStatus {
    /* readonly attribute nsIX509Cert peerCert; */
    #[inline]
    pub unsafe fn get_peerCert(&self, ) -> Result<Option<RefPtr<nsIX509Cert>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_peerCert)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute short tlsVersionUsed; */
    #[inline]
    pub unsafe fn get_tlsVersionUsed(&self, ) -> Result<libc::int16_t, nsresult> {
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tlsVersionUsed)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute ACString cipherName; */
    #[inline]
    pub unsafe fn get_cipherName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_cipherName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long keyLength; */
    #[inline]
    pub unsafe fn get_keyLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_keyLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute unsigned long macLength; */
    #[inline]
    pub unsafe fn get_macLength(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_macLength)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsITLSServerConnectionInfo {
    vtable: *const nsITLSServerConnectionInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITLSServerConnectionInfo {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8a93f5d5, 0xeddd, 0x4c62,
            [0xa4, 0xbd, 0xbf, 0xd2, 0x97, 0x65, 0x31, 0x84])
    }
}

unsafe impl RefCounted for nsITLSServerConnectionInfo {
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
pub trait nsITLSServerConnectionInfoCoerce {
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self;
}

impl nsITLSServerConnectionInfoCoerce for nsITLSServerConnectionInfo {
    #[inline]
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self {
        v
    }
}

impl nsITLSServerConnectionInfo {
    #[inline]
    pub fn coerce<T: nsITLSServerConnectionInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITLSServerConnectionInfo {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITLSServerConnectionInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerConnectionInfo) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITLSServerConnectionInfoVTable {
    pub __base: nsISupportsVTable,

    /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
    pub setSecurityObserver: unsafe extern "C" fn (this: *const nsITLSServerConnectionInfo, observer: *const nsITLSServerSecurityObserver) -> nsresult,

    /* readonly attribute nsITLSServerSocket serverSocket; */
    pub get_serverSocket: unsafe extern "C" fn (this: *const nsITLSServerConnectionInfo, aServerSocket: *mut *const nsITLSServerSocket) -> nsresult,

    /* readonly attribute nsITLSClientStatus status; */
    pub get_status: unsafe extern "C" fn (this: *const nsITLSServerConnectionInfo, aStatus: *mut *const nsITLSClientStatus) -> nsresult,

}


impl nsITLSServerConnectionInfo {
    /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
    #[inline]
    pub unsafe fn setSecurityObserver(&self, observer: Option<&nsITLSServerSecurityObserver>) -> Result<(), nsresult> {

        match ((*self.vtable).setSecurityObserver)(self as *const _, observer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsITLSServerSocket serverSocket; */
    #[inline]
    pub unsafe fn get_serverSocket(&self, ) -> Result<Option<RefPtr<nsITLSServerSocket>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_serverSocket)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsITLSClientStatus status; */
    #[inline]
    pub unsafe fn get_status(&self, ) -> Result<Option<RefPtr<nsITLSClientStatus>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_status)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsITLSServerSecurityObserver {
    vtable: *const nsITLSServerSecurityObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITLSServerSecurityObserver {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x1f62e1ae, 0xe546, 0x4a38,
            [0x89, 0x17, 0xd4, 0x28, 0x47, 0x2e, 0xd7, 0x36])
    }
}

unsafe impl RefCounted for nsITLSServerSecurityObserver {
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
pub trait nsITLSServerSecurityObserverCoerce {
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self;
}

impl nsITLSServerSecurityObserverCoerce for nsITLSServerSecurityObserver {
    #[inline]
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self {
        v
    }
}

impl nsITLSServerSecurityObserver {
    #[inline]
    pub fn coerce<T: nsITLSServerSecurityObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITLSServerSecurityObserver {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITLSServerSecurityObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITLSServerSecurityObserver) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITLSServerSecurityObserverVTable {
    pub __base: nsISupportsVTable,

    /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
    pub onHandshakeDone: unsafe extern "C" fn (this: *const nsITLSServerSecurityObserver, aServer: *const nsITLSServerSocket, aStatus: *const nsITLSClientStatus) -> nsresult,

}


impl nsITLSServerSecurityObserver {
    /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
    #[inline]
    pub unsafe fn onHandshakeDone(&self, aServer: Option<&nsITLSServerSocket>, aStatus: Option<&nsITLSClientStatus>) -> Result<(), nsresult> {

        match ((*self.vtable).onHandshakeDone)(self as *const _, aServer.map_or(::std::ptr::null(), |x| x as *const _), aStatus.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


