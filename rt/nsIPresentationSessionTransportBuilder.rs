//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIPresentationSessionTransportBuilder.idl
//


#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderListener {
    vtable: *const nsIPresentationSessionTransportBuilderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionTransportBuilderListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x673f6de1, 0xe253, 0x41b8,
            [0x9b, 0xe8, 0xb7, 0xff, 0x16, 0x1f, 0xa8, 0xdc])
    }
}

unsafe impl RefCounted for nsIPresentationSessionTransportBuilderListener {
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
pub trait nsIPresentationSessionTransportBuilderListenerCoerce {
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self;
}

impl nsIPresentationSessionTransportBuilderListenerCoerce for nsIPresentationSessionTransportBuilderListener {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportBuilderListener {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportBuilderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionTransportBuilderListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportBuilderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilderListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onSessionTransport (in nsIPresentationSessionTransport transport); */
    pub onSessionTransport: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, transport: *const nsIPresentationSessionTransport) -> nsresult,

    /* void onError (in nsresult reason); */
    pub onError: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, reason: nsresult) -> nsresult,

    /* void sendOffer (in nsIPresentationChannelDescription offer); */
    pub sendOffer: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, offer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void sendAnswer (in nsIPresentationChannelDescription answer); */
    pub sendAnswer: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, answer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void sendIceCandidate (in DOMString candidate); */
    pub sendIceCandidate: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, candidate: *const nsAString) -> nsresult,

    /* void close (in nsresult reason); */
    pub close: unsafe extern "C" fn (this: *const nsIPresentationSessionTransportBuilderListener, reason: nsresult) -> nsresult,

}


impl nsIPresentationSessionTransportBuilderListener {
    /* void onSessionTransport (in nsIPresentationSessionTransport transport); */
    #[inline]
    pub unsafe fn onSessionTransport(&self, transport: Option<&nsIPresentationSessionTransport>) -> Result<(), nsresult> {

        match ((*self.vtable).onSessionTransport)(self as *const _, transport.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onError (in nsresult reason); */
    #[inline]
    pub unsafe fn onError(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onError)(self as *const _, reason) {
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

    /* void close (in nsresult reason); */
    #[inline]
    pub unsafe fn close(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).close)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationSessionTransportBuilder {
    vtable: *const nsIPresentationSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationSessionTransportBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2fdbe67d, 0x80f9, 0x48dc,
            [0x82, 0x37, 0x5b, 0xef, 0x8f, 0xa1, 0x98, 0x01])
    }
}

unsafe impl RefCounted for nsIPresentationSessionTransportBuilder {
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
pub trait nsIPresentationSessionTransportBuilderCoerce {
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self;
}

impl nsIPresentationSessionTransportBuilderCoerce for nsIPresentationSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportBuilder {
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationSessionTransportBuilder {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationSessionTransportBuilderVTable {
    pub __base: nsISupportsVTable,

}


impl nsIPresentationSessionTransportBuilder {
}


#[repr(C)]
pub struct nsIPresentationTransportBuilderConstructor {
    vtable: *const nsIPresentationTransportBuilderConstructorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationTransportBuilderConstructor {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x706482b2, 0x1b51, 0x4bed,
            [0xa2, 0x1d, 0x78, 0x5a, 0x9c, 0xfc, 0xfa, 0xc7])
    }
}

unsafe impl RefCounted for nsIPresentationTransportBuilderConstructor {
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
pub trait nsIPresentationTransportBuilderConstructorCoerce {
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self;
}

impl nsIPresentationTransportBuilderConstructorCoerce for nsIPresentationTransportBuilderConstructor {
    #[inline]
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self {
        v
    }
}

impl nsIPresentationTransportBuilderConstructor {
    #[inline]
    pub fn coerce<T: nsIPresentationTransportBuilderConstructorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationTransportBuilderConstructor {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIPresentationTransportBuilderConstructorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTransportBuilderConstructor) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationTransportBuilderConstructorVTable {
    pub __base: nsISupportsVTable,

    /* nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type); */
    pub createTransportBuilder: unsafe extern "C" fn (this: *const nsIPresentationTransportBuilderConstructor, type_: uint8_t, _retval: *mut *const nsIPresentationSessionTransportBuilder) -> nsresult,

}


impl nsIPresentationTransportBuilderConstructor {
    /* nsIPresentationSessionTransportBuilder createTransportBuilder (in uint8_t type); */
    #[inline]
    pub unsafe fn createTransportBuilder(&self, type_: uint8_t) -> Result<Option<RefPtr<nsIPresentationSessionTransportBuilder>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).createTransportBuilder)(self as *const _, type_, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


#[repr(C)]
pub struct nsIPresentationTCPSessionTransportBuilder {
    vtable: *const nsIPresentationTCPSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationTCPSessionTransportBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcde36d6e, 0xf471, 0x4262,
            [0xa7, 0x0d, 0xf9, 0x32, 0xa2, 0x6b, 0x21, 0xd9])
    }
}

unsafe impl RefCounted for nsIPresentationTCPSessionTransportBuilder {
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
pub trait nsIPresentationTCPSessionTransportBuilderCoerce {
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self;
}

impl nsIPresentationTCPSessionTransportBuilderCoerce for nsIPresentationTCPSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationTCPSessionTransportBuilder {
    #[inline]
    pub fn coerce<T: nsIPresentationTCPSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationTCPSessionTransportBuilder {
    type Target = nsIPresentationSessionTransportBuilder;
    #[inline]
    fn deref(&self) -> &nsIPresentationSessionTransportBuilder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPresentationSessionTransportBuilderCoerce> nsIPresentationTCPSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationTCPSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationTCPSessionTransportBuilderVTable {
    pub __base: nsIPresentationSessionTransportBuilderVTable,

    /* void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub buildTCPSenderTransport: unsafe extern "C" fn (this: *const nsIPresentationTCPSessionTransportBuilder, aTransport: *const nsISocketTransport, aListener: *const nsIPresentationSessionTransportBuilderListener) -> nsresult,

    /* void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub buildTCPReceiverTransport: unsafe extern "C" fn (this: *const nsIPresentationTCPSessionTransportBuilder, aDescription: *const nsIPresentationChannelDescription, aListener: *const nsIPresentationSessionTransportBuilderListener) -> nsresult,

}


impl nsIPresentationTCPSessionTransportBuilder {
    /* void buildTCPSenderTransport (in nsISocketTransport aTransport, in nsIPresentationSessionTransportBuilderListener aListener); */
    #[inline]
    pub unsafe fn buildTCPSenderTransport(&self, aTransport: Option<&nsISocketTransport>, aListener: Option<&nsIPresentationSessionTransportBuilderListener>) -> Result<(), nsresult> {

        match ((*self.vtable).buildTCPSenderTransport)(self as *const _, aTransport.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void buildTCPReceiverTransport (in nsIPresentationChannelDescription aDescription, in nsIPresentationSessionTransportBuilderListener aListener); */
    #[inline]
    pub unsafe fn buildTCPReceiverTransport(&self, aDescription: Option<&nsIPresentationChannelDescription>, aListener: Option<&nsIPresentationSessionTransportBuilderListener>) -> Result<(), nsresult> {

        match ((*self.vtable).buildTCPReceiverTransport)(self as *const _, aDescription.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsIPresentationDataChannelSessionTransportBuilder {
    vtable: *const nsIPresentationDataChannelSessionTransportBuilderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIPresentationDataChannelSessionTransportBuilder {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x8131c4e0, 0x3a8c, 0x4bc1,
            [0xa9, 0x2a, 0x84, 0x31, 0x47, 0x3d, 0x2f, 0xe8])
    }
}

unsafe impl RefCounted for nsIPresentationDataChannelSessionTransportBuilder {
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
pub trait nsIPresentationDataChannelSessionTransportBuilderCoerce {
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self;
}

impl nsIPresentationDataChannelSessionTransportBuilderCoerce for nsIPresentationDataChannelSessionTransportBuilder {
    #[inline]
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self {
        v
    }
}

impl nsIPresentationDataChannelSessionTransportBuilder {
    #[inline]
    pub fn coerce<T: nsIPresentationDataChannelSessionTransportBuilderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIPresentationDataChannelSessionTransportBuilder {
    type Target = nsIPresentationSessionTransportBuilder;
    #[inline]
    fn deref(&self) -> &nsIPresentationSessionTransportBuilder {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIPresentationSessionTransportBuilderCoerce> nsIPresentationDataChannelSessionTransportBuilderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDataChannelSessionTransportBuilder) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIPresentationDataChannelSessionTransportBuilderVTable {
    pub __base: nsIPresentationSessionTransportBuilderVTable,

    /* void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener); */
    pub buildDataChannelTransport: unsafe extern "C" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, aRole: uint8_t, aWindow: *const mozIDOMWindow, aListener: *const nsIPresentationSessionTransportBuilderListener) -> nsresult,

    /* void onOffer (in nsIPresentationChannelDescription offer); */
    pub onOffer: unsafe extern "C" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, offer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void onAnswer (in nsIPresentationChannelDescription answer); */
    pub onAnswer: unsafe extern "C" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, answer: *const nsIPresentationChannelDescription) -> nsresult,

    /* void onIceCandidate (in DOMString candidate); */
    pub onIceCandidate: unsafe extern "C" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, candidate: *const nsAString) -> nsresult,

    /* void notifyDisconnected (in nsresult reason); */
    pub notifyDisconnected: unsafe extern "C" fn (this: *const nsIPresentationDataChannelSessionTransportBuilder, reason: nsresult) -> nsresult,

}


impl nsIPresentationDataChannelSessionTransportBuilder {
    /* void buildDataChannelTransport (in uint8_t aRole, in mozIDOMWindow aWindow, in nsIPresentationSessionTransportBuilderListener aListener); */
    #[inline]
    pub unsafe fn buildDataChannelTransport(&self, aRole: uint8_t, aWindow: Option<&mozIDOMWindow>, aListener: Option<&nsIPresentationSessionTransportBuilderListener>) -> Result<(), nsresult> {

        match ((*self.vtable).buildDataChannelTransport)(self as *const _, aRole, aWindow.map_or(::std::ptr::null(), |x| x as *const _), aListener.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

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

    /* void notifyDisconnected (in nsresult reason); */
    #[inline]
    pub unsafe fn notifyDisconnected(&self, reason: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).notifyDisconnected)(self as *const _, reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


