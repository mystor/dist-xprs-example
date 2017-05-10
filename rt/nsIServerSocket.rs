//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIServerSocket.idl
//


pub type nsServerSocketFlag = libc::uint32_t;


pub mod nsIServerSocket_consts {
    pub const LoopbackOnly: i64 = 1;
    pub const KeepWhenOffline: i64 = 2;
}


#[repr(C)]
pub struct nsIServerSocket {
    vtable: *const nsIServerSocketVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServerSocket {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x7a9c39cb, 0xa13f, 0x4eef,
            [0x9b, 0xdf, 0xa7, 0x43, 0x01, 0x62, 0x87, 0x42])
    }
}

unsafe impl RefCounted for nsIServerSocket {
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
pub trait nsIServerSocketCoerce {
    fn coerce_from(v: &nsIServerSocket) -> &Self;
}

impl nsIServerSocketCoerce for nsIServerSocket {
    #[inline]
    fn coerce_from(v: &nsIServerSocket) -> &Self {
        v
    }
}

impl nsIServerSocket {
    #[inline]
    pub fn coerce<T: nsIServerSocketCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServerSocket {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServerSocketCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServerSocket) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServerSocketVTable {
    pub __base: nsISupportsVTable,

    /* void init (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
    pub init: unsafe extern "C" fn (this: *const nsIServerSocket, aPort: libc::int32_t, aLoopbackOnly: bool, aBackLog: libc::int32_t) -> nsresult,

    /* void initSpecialConnection (in long aPort, in nsServerSocketFlag aFlags, in long aBackLog); */
    pub initSpecialConnection: unsafe extern "C" fn (this: *const nsIServerSocket, aPort: libc::int32_t, aFlags: nsServerSocketFlag, aBackLog: libc::int32_t) -> nsresult,

    /* [noscript] void initWithAddress ([const] in PRNetAddrPtr aAddr, in long aBackLog); */
    /// Unable to call function as its signature contains a non-rust type
    pub initWithAddress: *const ::libc::c_void,

    /* void initWithFilename (in nsIFile aPath, in unsigned long aPermissions, in long aBacklog); */
    pub initWithFilename: unsafe extern "C" fn (this: *const nsIServerSocket, aPath: *const nsIFile, aPermissions: libc::uint32_t, aBacklog: libc::int32_t) -> nsresult,

    /* void close (); */
    pub close: unsafe extern "C" fn (this: *const nsIServerSocket) -> nsresult,

    /* void asyncListen (in nsIServerSocketListener aListener); */
    pub asyncListen: unsafe extern "C" fn (this: *const nsIServerSocket, aListener: *const nsIServerSocketListener) -> nsresult,

    /* readonly attribute long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsIServerSocket, aPort: *mut libc::int32_t) -> nsresult,

    /* [noscript] PRNetAddr getAddress (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getAddress: *const ::libc::c_void,

}


impl nsIServerSocket {
    /* void init (in long aPort, in boolean aLoopbackOnly, in long aBackLog); */
    #[inline]
    pub unsafe fn init(&self, aPort: libc::int32_t, aLoopbackOnly: bool, aBackLog: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, aPort, aLoopbackOnly, aBackLog) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void initSpecialConnection (in long aPort, in nsServerSocketFlag aFlags, in long aBackLog); */
    #[inline]
    pub unsafe fn initSpecialConnection(&self, aPort: libc::int32_t, aFlags: nsServerSocketFlag, aBackLog: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).initSpecialConnection)(self as *const _, aPort, aFlags, aBackLog) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void initWithAddress ([const] in PRNetAddrPtr aAddr, in long aBackLog); */


    /* void initWithFilename (in nsIFile aPath, in unsigned long aPermissions, in long aBacklog); */
    #[inline]
    pub unsafe fn initWithFilename(&self, aPath: Option<&nsIFile>, aPermissions: libc::uint32_t, aBacklog: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).initWithFilename)(self as *const _, aPath.map_or(::std::ptr::null(), |x| x as *const _), aPermissions, aBacklog) {
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

    /* void asyncListen (in nsIServerSocketListener aListener); */
    #[inline]
    pub unsafe fn asyncListen(&self, aListener: Option<&nsIServerSocketListener>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncListen)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
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

    /* [noscript] PRNetAddr getAddress (); */


}


#[repr(C)]
pub struct nsIServerSocketListener {
    vtable: *const nsIServerSocketListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIServerSocketListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x836d98ec, 0xfee2, 0x4bde,
            [0xb6, 0x09, 0xab, 0xd5, 0xe9, 0x66, 0xea, 0xbd])
    }
}

unsafe impl RefCounted for nsIServerSocketListener {
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
pub trait nsIServerSocketListenerCoerce {
    fn coerce_from(v: &nsIServerSocketListener) -> &Self;
}

impl nsIServerSocketListenerCoerce for nsIServerSocketListener {
    #[inline]
    fn coerce_from(v: &nsIServerSocketListener) -> &Self {
        v
    }
}

impl nsIServerSocketListener {
    #[inline]
    pub fn coerce<T: nsIServerSocketListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIServerSocketListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIServerSocketListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServerSocketListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIServerSocketListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
    pub onSocketAccepted: unsafe extern "C" fn (this: *const nsIServerSocketListener, aServ: *const nsIServerSocket, aTransport: *const nsISocketTransport) -> nsresult,

    /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
    pub onStopListening: unsafe extern "C" fn (this: *const nsIServerSocketListener, aServ: *const nsIServerSocket, aStatus: nsresult) -> nsresult,

}


impl nsIServerSocketListener {
    /* void onSocketAccepted (in nsIServerSocket aServ, in nsISocketTransport aTransport); */
    #[inline]
    pub unsafe fn onSocketAccepted(&self, aServ: Option<&nsIServerSocket>, aTransport: Option<&nsISocketTransport>) -> Result<(), nsresult> {

        match ((*self.vtable).onSocketAccepted)(self as *const _, aServ.map_or(::std::ptr::null(), |x| x as *const _), aTransport.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStopListening (in nsIServerSocket aServ, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onStopListening(&self, aServ: Option<&nsIServerSocket>, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onStopListening)(self as *const _, aServ.map_or(::std::ptr::null(), |x| x as *const _), aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


