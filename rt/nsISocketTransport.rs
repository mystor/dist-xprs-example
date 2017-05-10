//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISocketTransport.idl
//


pub mod nsISocketTransport_consts {
    pub const TIMEOUT_CONNECT: i64 = 0;
    pub const TIMEOUT_READ_WRITE: i64 = 1;
    pub const STATUS_RESOLVING: i64 = 2152398851;
    pub const STATUS_RESOLVED: i64 = 2152398859;
    pub const STATUS_CONNECTING_TO: i64 = 2152398855;
    pub const STATUS_CONNECTED_TO: i64 = 2152398852;
    pub const STATUS_SENDING_TO: i64 = 2152398853;
    pub const STATUS_WAITING_FOR: i64 = 2152398858;
    pub const STATUS_RECEIVING_FROM: i64 = 2152398854;
    pub const BYPASS_CACHE: i64 = 1;
    pub const ANONYMOUS_CONNECT: i64 = 2;
    pub const DISABLE_IPV6: i64 = 4;
    pub const NO_PERMANENT_STORAGE: i64 = 8;
    pub const DISABLE_IPV4: i64 = 16;
    pub const DISABLE_RFC1918: i64 = 32;
    pub const MITM_OK: i64 = 64;
    pub const BE_CONSERVATIVE: i64 = 128;
}


#[repr(C)]
pub struct nsISocketTransport {
    vtable: *const nsISocketTransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISocketTransport {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x79221831, 0x85e2, 0x43a8,
            [0x81, 0x52, 0x05, 0xd7, 0x7d, 0x6f, 0xde, 0x31])
    }
}

unsafe impl RefCounted for nsISocketTransport {
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
pub trait nsISocketTransportCoerce {
    fn coerce_from(v: &nsISocketTransport) -> &Self;
}

impl nsISocketTransportCoerce for nsISocketTransport {
    #[inline]
    fn coerce_from(v: &nsISocketTransport) -> &Self {
        v
    }
}

impl nsISocketTransport {
    #[inline]
    pub fn coerce<T: nsISocketTransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISocketTransport {
    type Target = nsITransport;
    #[inline]
    fn deref(&self) -> &nsITransport {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsITransportCoerce> nsISocketTransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISocketTransport) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISocketTransportVTable {
    pub __base: nsITransportVTable,

    /* readonly attribute AUTF8String host; */
    pub get_host: unsafe extern "C" fn (this: *const nsISocketTransport, aHost: *mut nsACString) -> nsresult,

    /* readonly attribute long port; */
    pub get_port: unsafe extern "C" fn (this: *const nsISocketTransport, aPort: *mut libc::int32_t) -> nsresult,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_ScriptableOriginAttributes: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_ScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetOriginAttributes: *const ::libc::c_void,

    /* attribute ACString networkInterfaceId; */
    pub get_networkInterfaceId: unsafe extern "C" fn (this: *const nsISocketTransport, aNetworkInterfaceId: *mut nsACString) -> nsresult,
    pub set_networkInterfaceId: unsafe extern "C" fn (this: *const nsISocketTransport, aNetworkInterfaceId: *const nsACString) -> nsresult,

    /* [noscript] NetAddr getPeerAddr (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getPeerAddr: *const ::libc::c_void,

    /* [noscript] NetAddr getSelfAddr (); */
    /// Unable to call function as its signature contains a non-rust type
    pub getSelfAddr: *const ::libc::c_void,

    /* [noscript] void bind (in NetAddrPtr aLocalAddr); */
    /// Unable to call function as its signature contains a non-rust type
    pub bind: *const ::libc::c_void,

    /* nsINetAddr getScriptablePeerAddr (); */
    pub getScriptablePeerAddr: unsafe extern "C" fn (this: *const nsISocketTransport, _retval: *mut *const nsINetAddr) -> nsresult,

    /* nsINetAddr getScriptableSelfAddr (); */
    pub getScriptableSelfAddr: unsafe extern "C" fn (this: *const nsISocketTransport, _retval: *mut *const nsINetAddr) -> nsresult,

    /* readonly attribute nsISupports securityInfo; */
    pub get_securityInfo: unsafe extern "C" fn (this: *const nsISocketTransport, aSecurityInfo: *mut *const nsISupports) -> nsresult,

    /* attribute nsIInterfaceRequestor securityCallbacks; */
    pub get_securityCallbacks: unsafe extern "C" fn (this: *const nsISocketTransport, aSecurityCallbacks: *mut *const nsIInterfaceRequestor) -> nsresult,
    pub set_securityCallbacks: unsafe extern "C" fn (this: *const nsISocketTransport, aSecurityCallbacks: *const nsIInterfaceRequestor) -> nsresult,

    /* boolean isAlive (); */
    pub isAlive: unsafe extern "C" fn (this: *const nsISocketTransport, _retval: *mut bool) -> nsresult,

    /* unsigned long getTimeout (in unsigned long aType); */
    pub getTimeout: unsafe extern "C" fn (this: *const nsISocketTransport, aType: libc::uint32_t, _retval: *mut libc::uint32_t) -> nsresult,

    /* void setTimeout (in unsigned long aType, in unsigned long aValue); */
    pub setTimeout: unsafe extern "C" fn (this: *const nsISocketTransport, aType: libc::uint32_t, aValue: libc::uint32_t) -> nsresult,

    /* void setReuseAddrPort (in bool reuseAddrPort); */
    pub setReuseAddrPort: unsafe extern "C" fn (this: *const nsISocketTransport, reuseAddrPort: bool) -> nsresult,

    /* attribute unsigned long connectionFlags; */
    pub get_connectionFlags: unsafe extern "C" fn (this: *const nsISocketTransport, aConnectionFlags: *mut libc::uint32_t) -> nsresult,
    pub set_connectionFlags: unsafe extern "C" fn (this: *const nsISocketTransport, aConnectionFlags: libc::uint32_t) -> nsresult,

    /* attribute octet QoSBits; */
    pub get_QoSBits: unsafe extern "C" fn (this: *const nsISocketTransport, aQoSBits: *mut libc::uint8_t) -> nsresult,
    pub set_QoSBits: unsafe extern "C" fn (this: *const nsISocketTransport, aQoSBits: libc::uint8_t) -> nsresult,

    /* attribute unsigned long recvBufferSize; */
    pub get_recvBufferSize: unsafe extern "C" fn (this: *const nsISocketTransport, aRecvBufferSize: *mut libc::uint32_t) -> nsresult,
    pub set_recvBufferSize: unsafe extern "C" fn (this: *const nsISocketTransport, aRecvBufferSize: libc::uint32_t) -> nsresult,

    /* attribute unsigned long sendBufferSize; */
    pub get_sendBufferSize: unsafe extern "C" fn (this: *const nsISocketTransport, aSendBufferSize: *mut libc::uint32_t) -> nsresult,
    pub set_sendBufferSize: unsafe extern "C" fn (this: *const nsISocketTransport, aSendBufferSize: libc::uint32_t) -> nsresult,

    /* attribute boolean keepaliveEnabled; */
    pub get_keepaliveEnabled: unsafe extern "C" fn (this: *const nsISocketTransport, aKeepaliveEnabled: *mut bool) -> nsresult,
    pub set_keepaliveEnabled: unsafe extern "C" fn (this: *const nsISocketTransport, aKeepaliveEnabled: bool) -> nsresult,

    /* void setKeepaliveVals (in long keepaliveIdleTime, in long keepaliveRetryInterval); */
    pub setKeepaliveVals: unsafe extern "C" fn (this: *const nsISocketTransport, keepaliveIdleTime: libc::int32_t, keepaliveRetryInterval: libc::int32_t) -> nsresult,

    /* [noscript] void setFastOpenCallback (in TCPFastOpenPtr aFastOpen); */
    /// Unable to call function as its signature contains a non-rust type
    pub setFastOpenCallback: *const ::libc::c_void,

}


impl nsISocketTransport {
    /* readonly attribute AUTF8String host; */
    #[inline]
    pub unsafe fn get_host(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_host)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
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

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */



    /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */


    /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */


    /* attribute ACString networkInterfaceId; */
    #[inline]
    pub unsafe fn get_networkInterfaceId(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_networkInterfaceId)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_networkInterfaceId(&self, aNetworkInterfaceId: &[u8]) -> Result<(), nsresult> {
        let aNetworkInterfaceId = nsCString::from(aNetworkInterfaceId);
        match ((*self.vtable).set_networkInterfaceId)(self as *const _, &*aNetworkInterfaceId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] NetAddr getPeerAddr (); */


    /* [noscript] NetAddr getSelfAddr (); */


    /* [noscript] void bind (in NetAddrPtr aLocalAddr); */


    /* nsINetAddr getScriptablePeerAddr (); */
    #[inline]
    pub unsafe fn getScriptablePeerAddr(&self, ) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getScriptablePeerAddr)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* nsINetAddr getScriptableSelfAddr (); */
    #[inline]
    pub unsafe fn getScriptableSelfAddr(&self, ) -> Result<Option<RefPtr<nsINetAddr>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).getScriptableSelfAddr)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsISupports securityInfo; */
    #[inline]
    pub unsafe fn get_securityInfo(&self, ) -> Result<Option<RefPtr<nsISupports>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityInfo)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* attribute nsIInterfaceRequestor securityCallbacks; */
    #[inline]
    pub unsafe fn get_securityCallbacks(&self, ) -> Result<Option<RefPtr<nsIInterfaceRequestor>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_securityCallbacks)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_securityCallbacks(&self, aSecurityCallbacks: Option<&nsIInterfaceRequestor>) -> Result<(), nsresult> {

        match ((*self.vtable).set_securityCallbacks)(self as *const _, aSecurityCallbacks.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean isAlive (); */
    #[inline]
    pub unsafe fn isAlive(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).isAlive)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* unsigned long getTimeout (in unsigned long aType); */
    #[inline]
    pub unsafe fn getTimeout(&self, aType: libc::uint32_t) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).getTimeout)(self as *const _, aType, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void setTimeout (in unsigned long aType, in unsigned long aValue); */
    #[inline]
    pub unsafe fn setTimeout(&self, aType: libc::uint32_t, aValue: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setTimeout)(self as *const _, aType, aValue) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setReuseAddrPort (in bool reuseAddrPort); */
    #[inline]
    pub unsafe fn setReuseAddrPort(&self, reuseAddrPort: bool) -> Result<(), nsresult> {

        match ((*self.vtable).setReuseAddrPort)(self as *const _, reuseAddrPort) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long connectionFlags; */
    #[inline]
    pub unsafe fn get_connectionFlags(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_connectionFlags)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_connectionFlags(&self, aConnectionFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_connectionFlags)(self as *const _, aConnectionFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute octet QoSBits; */
    #[inline]
    pub unsafe fn get_QoSBits(&self, ) -> Result<libc::uint8_t, nsresult> {
        let mut _retval: libc::uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).get_QoSBits)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_QoSBits(&self, aQoSBits: libc::uint8_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_QoSBits)(self as *const _, aQoSBits) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long recvBufferSize; */
    #[inline]
    pub unsafe fn get_recvBufferSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_recvBufferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_recvBufferSize(&self, aRecvBufferSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_recvBufferSize)(self as *const _, aRecvBufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute unsigned long sendBufferSize; */
    #[inline]
    pub unsafe fn get_sendBufferSize(&self, ) -> Result<libc::uint32_t, nsresult> {
        let mut _retval: libc::uint32_t = ::std::mem::zeroed();
        match ((*self.vtable).get_sendBufferSize)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_sendBufferSize(&self, aSendBufferSize: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_sendBufferSize)(self as *const _, aSendBufferSize) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute boolean keepaliveEnabled; */
    #[inline]
    pub unsafe fn get_keepaliveEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_keepaliveEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_keepaliveEnabled(&self, aKeepaliveEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_keepaliveEnabled)(self as *const _, aKeepaliveEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void setKeepaliveVals (in long keepaliveIdleTime, in long keepaliveRetryInterval); */
    #[inline]
    pub unsafe fn setKeepaliveVals(&self, keepaliveIdleTime: libc::int32_t, keepaliveRetryInterval: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).setKeepaliveVals)(self as *const _, keepaliveIdleTime, keepaliveRetryInterval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setFastOpenCallback (in TCPFastOpenPtr aFastOpen); */


}


