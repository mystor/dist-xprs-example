//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINetworkInterceptController.idl
//


#[repr(C)]
pub struct nsIInterceptedChannel {
    vtable: *const nsIInterceptedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIInterceptedChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xf4b82975, 0x6a86, 0x4cc4,
            [0x87, 0xfe, 0x9a, 0x1f, 0xd4, 0x30, 0xc8, 0x6d])
    }
}

unsafe impl RefCounted for nsIInterceptedChannel {
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
pub trait nsIInterceptedChannelCoerce {
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self;
}

impl nsIInterceptedChannelCoerce for nsIInterceptedChannel {
    #[inline]
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self {
        v
    }
}

impl nsIInterceptedChannel {
    #[inline]
    pub fn coerce<T: nsIInterceptedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIInterceptedChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIInterceptedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIInterceptedChannelVTable {
    pub __base: nsISupportsVTable,

    /* void resetInterception (); */
    pub resetInterception: unsafe extern "C" fn (this: *const nsIInterceptedChannel) -> nsresult,

    /* void synthesizeStatus (in uint16_t status, in ACString reason); */
    pub synthesizeStatus: unsafe extern "C" fn (this: *const nsIInterceptedChannel, status: uint16_t, reason: *const nsACString) -> nsresult,

    /* void synthesizeHeader (in ACString name, in ACString value); */
    pub synthesizeHeader: unsafe extern "C" fn (this: *const nsIInterceptedChannel, name: *const nsACString, value: *const nsACString) -> nsresult,

    /* void finishSynthesizedResponse (in ACString finalURLSpec); */
    pub finishSynthesizedResponse: unsafe extern "C" fn (this: *const nsIInterceptedChannel, finalURLSpec: *const nsACString) -> nsresult,

    /* void cancel (in nsresult status); */
    pub cancel: unsafe extern "C" fn (this: *const nsIInterceptedChannel, status: nsresult) -> nsresult,

    /* readonly attribute nsIOutputStream responseBody; */
    pub get_responseBody: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aResponseBody: *mut *const nsIOutputStream) -> nsresult,

    /* readonly attribute nsIChannel channel; */
    pub get_channel: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aChannel: *mut *const nsIChannel) -> nsresult,

    /* readonly attribute nsIURI secureUpgradedChannelURI; */
    pub get_secureUpgradedChannelURI: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aSecureUpgradedChannelURI: *mut *const nsIURI) -> nsresult,

    /* [noscript] void setChannelInfo (in ChannelInfo channelInfo); */
    /// Unable to call function as its signature contains a non-rust type
    pub setChannelInfo: *const ::libc::c_void,

    /* [noscript] readonly attribute nsContentPolicyType internalContentPolicyType; */
    pub get_internalContentPolicyType: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aInternalContentPolicyType: *mut nsContentPolicyType) -> nsresult,

    /* [noscript] readonly attribute nsIConsoleReportCollector consoleReportCollector; */
    pub get_consoleReportCollector: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aConsoleReportCollector: *mut *const nsIConsoleReportCollector) -> nsresult,

    /* [noscript] void SetLaunchServiceWorkerStart (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetLaunchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] void SetLaunchServiceWorkerEnd (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetLaunchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] void SetDispatchFetchEventStart (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetDispatchFetchEventStart: *const ::libc::c_void,

    /* [noscript] void SetDispatchFetchEventEnd (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetDispatchFetchEventEnd: *const ::libc::c_void,

    /* [noscript] void SetHandleFetchEventStart (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetHandleFetchEventStart: *const ::libc::c_void,

    /* [noscript] void SetHandleFetchEventEnd (in TimeStamp aTimeStamp); */
    /// Unable to call function as its signature contains a non-rust type
    pub SetHandleFetchEventEnd: *const ::libc::c_void,

    /* [noscript] void SaveTimeStampsToUnderlyingChannel (); */
    pub SaveTimeStampsToUnderlyingChannel: unsafe extern "C" fn (this: *const nsIInterceptedChannel) -> nsresult,

    /* [noscript] void setReleaseHandle (in nsISupports aHandle); */
    pub setReleaseHandle: unsafe extern "C" fn (this: *const nsIInterceptedChannel, aHandle: *const nsISupports) -> nsresult,

}


impl nsIInterceptedChannel {
    /* void resetInterception (); */
    #[inline]
    pub unsafe fn resetInterception(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).resetInterception)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void synthesizeStatus (in uint16_t status, in ACString reason); */
    #[inline]
    pub unsafe fn synthesizeStatus(&self, status: uint16_t, reason: &[u8]) -> Result<(), nsresult> {
        let reason = nsCString::from(reason);
        match ((*self.vtable).synthesizeStatus)(self as *const _, status, &*reason) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void synthesizeHeader (in ACString name, in ACString value); */
    #[inline]
    pub unsafe fn synthesizeHeader(&self, name: &[u8], value: &[u8]) -> Result<(), nsresult> {
        let name = nsCString::from(name);
        let value = nsCString::from(value);
        match ((*self.vtable).synthesizeHeader)(self as *const _, &*name, &*value) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void finishSynthesizedResponse (in ACString finalURLSpec); */
    #[inline]
    pub unsafe fn finishSynthesizedResponse(&self, finalURLSpec: &[u8]) -> Result<(), nsresult> {
        let finalURLSpec = nsCString::from(finalURLSpec);
        match ((*self.vtable).finishSynthesizedResponse)(self as *const _, &*finalURLSpec) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void cancel (in nsresult status); */
    #[inline]
    pub unsafe fn cancel(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).cancel)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* readonly attribute nsIOutputStream responseBody; */
    #[inline]
    pub unsafe fn get_responseBody(&self, ) -> Result<Option<RefPtr<nsIOutputStream>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_responseBody)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIChannel channel; */
    #[inline]
    pub unsafe fn get_channel(&self, ) -> Result<Option<RefPtr<nsIChannel>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_channel)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* readonly attribute nsIURI secureUpgradedChannelURI; */
    #[inline]
    pub unsafe fn get_secureUpgradedChannelURI(&self, ) -> Result<Option<RefPtr<nsIURI>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_secureUpgradedChannelURI)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void setChannelInfo (in ChannelInfo channelInfo); */


    /* [noscript] readonly attribute nsContentPolicyType internalContentPolicyType; */
    #[inline]
    pub unsafe fn get_internalContentPolicyType(&self, ) -> Result<nsContentPolicyType, nsresult> {
        let mut _retval: nsContentPolicyType = ::std::mem::zeroed();
        match ((*self.vtable).get_internalContentPolicyType)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute nsIConsoleReportCollector consoleReportCollector; */
    #[inline]
    pub unsafe fn get_consoleReportCollector(&self, ) -> Result<Option<RefPtr<nsIConsoleReportCollector>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).get_consoleReportCollector)(self as *const _, _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

    /* [noscript] void SetLaunchServiceWorkerStart (in TimeStamp aTimeStamp); */


    /* [noscript] void SetLaunchServiceWorkerEnd (in TimeStamp aTimeStamp); */


    /* [noscript] void SetDispatchFetchEventStart (in TimeStamp aTimeStamp); */


    /* [noscript] void SetDispatchFetchEventEnd (in TimeStamp aTimeStamp); */


    /* [noscript] void SetHandleFetchEventStart (in TimeStamp aTimeStamp); */


    /* [noscript] void SetHandleFetchEventEnd (in TimeStamp aTimeStamp); */


    /* [noscript] void SaveTimeStampsToUnderlyingChannel (); */
    #[inline]
    pub unsafe fn SaveTimeStampsToUnderlyingChannel(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).SaveTimeStampsToUnderlyingChannel)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] void setReleaseHandle (in nsISupports aHandle); */
    #[inline]
    pub unsafe fn setReleaseHandle(&self, aHandle: Option<&nsISupports>) -> Result<(), nsresult> {

        match ((*self.vtable).setReleaseHandle)(self as *const _, aHandle.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


#[repr(C)]
pub struct nsINetworkInterceptController {
    vtable: *const nsINetworkInterceptControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINetworkInterceptController {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x70d2b4fe, 0xa552, 0x48cd,
            [0x8d, 0x93, 0x1d, 0x84, 0x37, 0xa5, 0x6b, 0x53])
    }
}

unsafe impl RefCounted for nsINetworkInterceptController {
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
pub trait nsINetworkInterceptControllerCoerce {
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self;
}

impl nsINetworkInterceptControllerCoerce for nsINetworkInterceptController {
    #[inline]
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self {
        v
    }
}

impl nsINetworkInterceptController {
    #[inline]
    pub fn coerce<T: nsINetworkInterceptControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINetworkInterceptController {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINetworkInterceptControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINetworkInterceptControllerVTable {
    pub __base: nsISupportsVTable,

    /* bool shouldPrepareForIntercept (in nsIURI aURI, in bool aIsNonSubresourceRequest); */
    pub shouldPrepareForIntercept: unsafe extern "C" fn (this: *const nsINetworkInterceptController, aURI: *const nsIURI, aIsNonSubresourceRequest: bool, _retval: *mut bool) -> nsresult,

    /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
    pub channelIntercepted: unsafe extern "C" fn (this: *const nsINetworkInterceptController, aChannel: *const nsIInterceptedChannel) -> nsresult,

}


impl nsINetworkInterceptController {
    /* bool shouldPrepareForIntercept (in nsIURI aURI, in bool aIsNonSubresourceRequest); */
    #[inline]
    pub unsafe fn shouldPrepareForIntercept(&self, aURI: Option<&nsIURI>, aIsNonSubresourceRequest: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).shouldPrepareForIntercept)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aIsNonSubresourceRequest, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
    #[inline]
    pub unsafe fn channelIntercepted(&self, aChannel: Option<&nsIInterceptedChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).channelIntercepted)(self as *const _, aChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


