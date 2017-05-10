//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationControlChannel.idl
//


pub mod nsIPresentationChannelDescription_consts {
    pub const TYPE_TCP: i64 = 1;
    pub const TYPE_DATACHANNEL: i64 = 2;
}


#[repr(C)]
pub struct nsIPresentationChannelDescription {
    vtable: *const nsIPresentationChannelDescriptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationChannelDescription {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xae318e05, 0x2a4e, 0x4f85,
            [0x95, 0xc0, 0xe8, 0xb1, 0x91, 0xad, 0x81, 0x2c])
    }
}

unsafe impl RefCounted for nsIPresentationChannelDescription {
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
pub trait nsIPresentationChannelDescriptionCoerce {
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self;
}

impl nsIPresentationChannelDescriptionCoerce for nsIPresentationChannelDescription {
    #[inline]
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self {
        v
    }
}

impl nsIPresentationChannelDescription {
    #[inline]
    pub fn coerce<T: nsIPresentationChannelDescriptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationChannelDescription {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationChannelDescriptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationChannelDescription) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationChannelDescriptionVTable {
    pub __base: nsISupportsVTable,

    /* readonly attribute uint8_t type; */
    pub get_type_: unsafe extern "C" fn (this: *const nsIPresentationChannelDescription, aType: *mut uint8_t) -> nsresult,

    /* readonly attribute nsIArray tcpAddress; */
    pub get_tcpAddress: unsafe extern "C" fn (this: *const nsIPresentationChannelDescription, aTcpAddress: *mut *const nsIArray) -> nsresult,

    /* readonly attribute uint16_t tcpPort; */
    pub get_tcpPort: unsafe extern "C" fn (this: *const nsIPresentationChannelDescription, aTcpPort: *mut uint16_t) -> nsresult,

    /* readonly attribute DOMString dataChannelSDP; */
    pub get_dataChannelSDP: unsafe extern "C" fn (this: *const nsIPresentationChannelDescription, aDataChannelSDP: *mut nsAString) -> nsresult,

}


impl nsIPresentationChannelDescription {
    /* readonly attribute uint8_t type; */
    #[inline]
    pub unsafe fn get_type_(&self, ) -> Result<uint8_t, nsresult> {
        let mut _retval: uint8_t = ::std::mem::zeroed();
        match ((*self.vtable).get_type_)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute nsIArray tcpAddress; */
    #[inline]
    pub unsafe fn get_tcpAddress(&self, ) -> Result<Option<RefPtr<nsIArray>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_tcpAddress)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute uint16_t tcpPort; */
    #[inline]
    pub unsafe fn get_tcpPort(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_tcpPort)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute DOMString dataChannelSDP; */
    #[inline]
    pub unsafe fn get_dataChannelSDP(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_dataChannelSDP)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


#[repr(C)]
pub struct nsIPresentationControlChannelListener {
    vtable: *const nsIPresentationControlChannelListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationControlChannelListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x96dd548f, 0x7d0f, 0x43c1,
            [0xb1, 0xad, 0x28, 0xe6, 0x66, 0xcf, 0x1e, 0x82])
    }
}

unsafe impl RefCounted for nsIPresentationControlChannelListener {
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
pub trait nsIPresentationControlChannelListenerCoerce {
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self;
}

impl nsIPresentationControlChannelListenerCoerce for nsIPresentationControlChannelListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self {
        v
    }
}

impl nsIPresentationControlChannelListener {
    #[inline]
    pub fn coerce<T: nsIPresentationControlChannelListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationControlChannelListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationControlChannelListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannelListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationControlChannelListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onOffer (in nsIPresentationChannelDescription offer); */
    pub onOffer: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener, offer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void onAnswer (in nsIPresentationChannelDescription answer); */
    pub onAnswer: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener, answer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void onIceCandidate (in DOMString candidate); */
    pub onIceCandidate: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener, candidate: *const nsAString) -> nsresult,

    /* void notifyConnected (); */
    pub notifyConnected: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener) -> nsresult,

    /* void notifyDisconnected (in nsresult reason); */
    pub notifyDisconnected: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener, reason: nsresult) -> nsresult,

    /* void notifyReconnected (); */
    pub notifyReconnected: unsafe extern "C" fn (this: *const nsIPresentationControlChannelListener) -> nsresult,

}


impl nsIPresentationControlChannelListener {
    /* void onOffer (in nsIPresentationChannelDescription offer); */
    #[inline]
    pub unsafe fn onOffer(&self, offer: Option<&nsIPresentationChannelDescription>) -> Result<(), nsresult> {

        match ((*self.vtable).onOffer)(self as *const _, offer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onAnswer (in nsIPresentationChannelDescription answer); */
    #[inline]
    pub unsafe fn onAnswer(&self, answer: Option<&nsIPresentationChannelDescription>) -> Result<(), nsresult> {

        match ((*self.vtable).onAnswer)(self as *const _, answer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onIceCandidate (in DOMString candidate); */
    #[inline]
    pub unsafe fn onIceCandidate(&self, candidate: &[u16]) -> Result<(), nsresult> {
        let candidate = nsString::from(candidate);
        match ((*self.vtable).onIceCandidate)(self as *const _, &*candidate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyConnected (); */
    #[inline]
    pub unsafe fn notifyConnected(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyConnected)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyDisconnected (in nsresult reason); */
    #[inline]
    pub unsafe fn notifyDisconnected(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).notifyDisconnected)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void notifyReconnected (); */
    #[inline]
    pub unsafe fn notifyReconnected(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).notifyReconnected)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationControlChannel {
    vtable: *const nsIPresentationControlChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationControlChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xe60e208c, 0xa9f5, 0x4bc6,
            [0x9a, 0x3e, 0x47, 0xf3, 0xe4, 0xae, 0x9c, 0x57])
    }
}

unsafe impl RefCounted for nsIPresentationControlChannel {
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
pub trait nsIPresentationControlChannelCoerce {
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self;
}

impl nsIPresentationControlChannelCoerce for nsIPresentationControlChannel {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self {
        v
    }
}

impl nsIPresentationControlChannel {
    #[inline]
    pub fn coerce<T: nsIPresentationControlChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationControlChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationControlChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationControlChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationControlChannelVTable {
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationControlChannelListener listener; */
    pub get_listener: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, aListener: *mut *const nsIPresentationControlChannelListener) -> nsresult,
    pub set_listener: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, aListener: *const nsIPresentationControlChannelListener) -> nsresult,

    /* void sendOffer (in nsIPresentationChannelDescription offer); */
    pub sendOffer: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, offer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
    pub sendAnswer: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, answer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void sendIceCandidate (in DOMString candidate); */
    pub sendIceCandidate: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, candidate: *const nsAString) -> nsresult,

    /* void launch (in DOMString presentationId, in DOMString url); */
    pub launch: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, presentationId: *const nsAString, url: *const nsAString) -> nsresult,

    /* void terminate (in DOMString presentationId); */
    pub terminate: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, presentationId: *const nsAString) -> nsresult,

    /* void disconnect (in nsresult reason); */
    pub disconnect: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, reason: nsresult) -> nsresult,

    /* void reconnect (in DOMString presentationId, in DOMString url); */
    pub reconnect: unsafe extern "C" fn (this: *const nsIPresentationControlChannel, presentationId: *const nsAString, url: *const nsAString) -> nsresult,

}


impl nsIPresentationControlChannel {
    /* attribute nsIPresentationControlChannelListener listener; */
    #[inline]
    pub unsafe fn get_listener(&self, ) -> Result<Option<RefPtr<nsIPresentationControlChannelListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_listener)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }
    #[inline]
    pub unsafe fn set_listener(&self, aListener: Option<&nsIPresentationControlChannelListener>) -> Result<(), nsresult> {

        match ((*self.vtable).set_listener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendOffer (in nsIPresentationChannelDescription offer); */
    #[inline]
    pub unsafe fn sendOffer(&self, offer: Option<&nsIPresentationChannelDescription>) -> Result<(), nsresult> {

        match ((*self.vtable).sendOffer)(self as *const _, offer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
    #[inline]
    pub unsafe fn sendAnswer(&self, answer: Option<&nsIPresentationChannelDescription>) -> Result<(), nsresult> {

        match ((*self.vtable).sendAnswer)(self as *const _, answer.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void sendIceCandidate (in DOMString candidate); */
    #[inline]
    pub unsafe fn sendIceCandidate(&self, candidate: &[u16]) -> Result<(), nsresult> {
        let candidate = nsString::from(candidate);
        match ((*self.vtable).sendIceCandidate)(self as *const _, &*candidate) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void launch (in DOMString presentationId, in DOMString url); */
    #[inline]
    pub unsafe fn launch(&self, presentationId: &[u16], url: &[u16]) -> Result<(), nsresult> {
        let presentationId = nsString::from(presentationId);
        let url = nsString::from(url);
        match ((*self.vtable).launch)(self as *const _, &*presentationId, &*url) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void terminate (in DOMString presentationId); */
    #[inline]
    pub unsafe fn terminate(&self, presentationId: &[u16]) -> Result<(), nsresult> {
        let presentationId = nsString::from(presentationId);
        match ((*self.vtable).terminate)(self as *const _, &*presentationId) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void disconnect (in nsresult reason); */
    #[inline]
    pub unsafe fn disconnect(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).disconnect)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void reconnect (in DOMString presentationId, in DOMString url); */
    #[inline]
    pub unsafe fn reconnect(&self, presentationId: &[u16], url: &[u16]) -> Result<(), nsresult> {
        let presentationId = nsString::from(presentationId);
        let url = nsString::from(url);
        match ((*self.vtable).reconnect)(self as *const _, &*presentationId, &*url) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


