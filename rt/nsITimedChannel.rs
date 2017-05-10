//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITimedChannel.idl
//


#[repr(C)]
pub struct nsITimedChannel {
    vtable: *const nsITimedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITimedChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xca63784d, 0x959c, 0x4c3a,
            [0x9a, 0x59, 0x23, 0x4a, 0x2a, 0x52, 0x0d, 0xe0])
    }
}

unsafe impl RefCounted for nsITimedChannel {
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
pub trait nsITimedChannelCoerce {
    fn coerce_from(v: &nsITimedChannel) -> &Self;
}

impl nsITimedChannelCoerce for nsITimedChannel {
    #[inline]
    fn coerce_from(v: &nsITimedChannel) -> &Self {
        v
    }
}

impl nsITimedChannel {
    #[inline]
    pub fn coerce<T: nsITimedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITimedChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITimedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimedChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITimedChannelVTable {
    pub __base: nsISupportsVTable,

    /* attribute boolean timingEnabled; */
    pub get_timingEnabled: unsafe extern "C" fn (this: *const nsITimedChannel, aTimingEnabled: *mut bool) -> nsresult,
    pub set_timingEnabled: unsafe extern "C" fn (this: *const nsITimedChannel, aTimingEnabled: bool) -> nsresult,

    /* attribute uint16_t redirectCount; */
    pub get_redirectCount: unsafe extern "C" fn (this: *const nsITimedChannel, aRedirectCount: *mut uint16_t) -> nsresult,
    pub set_redirectCount: unsafe extern "C" fn (this: *const nsITimedChannel, aRedirectCount: uint16_t) -> nsresult,

    /* [noscript] readonly attribute TimeStamp channelCreation; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_channelCreation: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp asyncOpen; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_asyncOpen: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_launchServiceWorkerStart: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_launchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_launchServiceWorkerEnd: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_launchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_dispatchFetchEventStart: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_dispatchFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_dispatchFetchEventEnd: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_dispatchFetchEventEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_handleFetchEventStart: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_handleFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_handleFetchEventEnd: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_handleFetchEventEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp domainLookupStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_domainLookupStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp domainLookupEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_domainLookupEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp connectStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_connectStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp connectEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_connectEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp requestStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_requestStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp responseStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_responseStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp responseEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_responseEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_redirectStart: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_redirectStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_redirectEnd: *const ::libc::c_void,
    /// Unable to call function as its signature contains a non-rust type
    pub set_redirectEnd: *const ::libc::c_void,

    /* [noscript] attribute AString initiatorType; */
    pub get_initiatorType: unsafe extern "C" fn (this: *const nsITimedChannel, aInitiatorType: *mut nsAString) -> nsresult,
    pub set_initiatorType: unsafe extern "C" fn (this: *const nsITimedChannel, aInitiatorType: *const nsAString) -> nsresult,

    /* [noscript] attribute boolean allRedirectsSameOrigin; */
    pub get_allRedirectsSameOrigin: unsafe extern "C" fn (this: *const nsITimedChannel, aAllRedirectsSameOrigin: *mut bool) -> nsresult,
    pub set_allRedirectsSameOrigin: unsafe extern "C" fn (this: *const nsITimedChannel, aAllRedirectsSameOrigin: bool) -> nsresult,

    /* [noscript] attribute boolean allRedirectsPassTimingAllowCheck; */
    pub get_allRedirectsPassTimingAllowCheck: unsafe extern "C" fn (this: *const nsITimedChannel, aAllRedirectsPassTimingAllowCheck: *mut bool) -> nsresult,
    pub set_allRedirectsPassTimingAllowCheck: unsafe extern "C" fn (this: *const nsITimedChannel, aAllRedirectsPassTimingAllowCheck: bool) -> nsresult,

    /* [noscript] boolean timingAllowCheck (in nsIPrincipal origin); */
    pub timingAllowCheck: unsafe extern "C" fn (this: *const nsITimedChannel, origin: *const nsIPrincipal, _retval: *mut bool) -> nsresult,

    /* [noscript] readonly attribute TimeStamp cacheReadStart; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_cacheReadStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp cacheReadEnd; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_cacheReadEnd: *const ::libc::c_void,

    /* readonly attribute PRTime channelCreationTime; */
    pub get_channelCreationTime: unsafe extern "C" fn (this: *const nsITimedChannel, aChannelCreationTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime asyncOpenTime; */
    pub get_asyncOpenTime: unsafe extern "C" fn (this: *const nsITimedChannel, aAsyncOpenTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime launchServiceWorkerStartTime; */
    pub get_launchServiceWorkerStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aLaunchServiceWorkerStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime launchServiceWorkerEndTime; */
    pub get_launchServiceWorkerEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aLaunchServiceWorkerEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime dispatchFetchEventStartTime; */
    pub get_dispatchFetchEventStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aDispatchFetchEventStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime dispatchFetchEventEndTime; */
    pub get_dispatchFetchEventEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aDispatchFetchEventEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime handleFetchEventStartTime; */
    pub get_handleFetchEventStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aHandleFetchEventStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime handleFetchEventEndTime; */
    pub get_handleFetchEventEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aHandleFetchEventEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime domainLookupStartTime; */
    pub get_domainLookupStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aDomainLookupStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime domainLookupEndTime; */
    pub get_domainLookupEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aDomainLookupEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime connectStartTime; */
    pub get_connectStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aConnectStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime connectEndTime; */
    pub get_connectEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aConnectEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime requestStartTime; */
    pub get_requestStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aRequestStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime responseStartTime; */
    pub get_responseStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aResponseStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime responseEndTime; */
    pub get_responseEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aResponseEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime cacheReadStartTime; */
    pub get_cacheReadStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aCacheReadStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime cacheReadEndTime; */
    pub get_cacheReadEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aCacheReadEndTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime redirectStartTime; */
    pub get_redirectStartTime: unsafe extern "C" fn (this: *const nsITimedChannel, aRedirectStartTime: *mut PRTime) -> nsresult,

    /* readonly attribute PRTime redirectEndTime; */
    pub get_redirectEndTime: unsafe extern "C" fn (this: *const nsITimedChannel, aRedirectEndTime: *mut PRTime) -> nsresult,

}


impl nsITimedChannel {
    /* attribute boolean timingEnabled; */
    #[inline]
    pub unsafe fn get_timingEnabled(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_timingEnabled)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_timingEnabled(&self, aTimingEnabled: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_timingEnabled)(self as *const _, aTimingEnabled) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* attribute uint16_t redirectCount; */
    #[inline]
    pub unsafe fn get_redirectCount(&self, ) -> Result<uint16_t, nsresult> {
        let mut _retval: uint16_t = ::std::mem::zeroed();
        match ((*self.vtable).get_redirectCount)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_redirectCount(&self, aRedirectCount: uint16_t) -> Result<(), nsresult> {

        match ((*self.vtable).set_redirectCount)(self as *const _, aRedirectCount) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] readonly attribute TimeStamp channelCreation; */


    /* [noscript] readonly attribute TimeStamp asyncOpen; */


    /* [noscript] attribute TimeStamp launchServiceWorkerStart; */



    /* [noscript] attribute TimeStamp launchServiceWorkerEnd; */



    /* [noscript] attribute TimeStamp dispatchFetchEventStart; */



    /* [noscript] attribute TimeStamp dispatchFetchEventEnd; */



    /* [noscript] attribute TimeStamp handleFetchEventStart; */



    /* [noscript] attribute TimeStamp handleFetchEventEnd; */



    /* [noscript] readonly attribute TimeStamp domainLookupStart; */


    /* [noscript] readonly attribute TimeStamp domainLookupEnd; */


    /* [noscript] readonly attribute TimeStamp connectStart; */


    /* [noscript] readonly attribute TimeStamp connectEnd; */


    /* [noscript] readonly attribute TimeStamp requestStart; */


    /* [noscript] readonly attribute TimeStamp responseStart; */


    /* [noscript] readonly attribute TimeStamp responseEnd; */


    /* [noscript] attribute TimeStamp redirectStart; */



    /* [noscript] attribute TimeStamp redirectEnd; */



    /* [noscript] attribute AString initiatorType; */
    #[inline]
    pub unsafe fn get_initiatorType(&self, ) -> Result<nsString, nsresult> {
        let mut _retval = nsString::new();
        match ((*self.vtable).get_initiatorType)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_initiatorType(&self, aInitiatorType: &[u16]) -> Result<(), nsresult> {
        let aInitiatorType = nsString::from(aInitiatorType);
        match ((*self.vtable).set_initiatorType)(self as *const _, &*aInitiatorType) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute boolean allRedirectsSameOrigin; */
    #[inline]
    pub unsafe fn get_allRedirectsSameOrigin(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allRedirectsSameOrigin)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allRedirectsSameOrigin(&self, aAllRedirectsSameOrigin: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allRedirectsSameOrigin)(self as *const _, aAllRedirectsSameOrigin) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] attribute boolean allRedirectsPassTimingAllowCheck; */
    #[inline]
    pub unsafe fn get_allRedirectsPassTimingAllowCheck(&self, ) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).get_allRedirectsPassTimingAllowCheck)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }
    #[inline]
    pub unsafe fn set_allRedirectsPassTimingAllowCheck(&self, aAllRedirectsPassTimingAllowCheck: bool) -> Result<(), nsresult> {

        match ((*self.vtable).set_allRedirectsPassTimingAllowCheck)(self as *const _, aAllRedirectsPassTimingAllowCheck) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [noscript] boolean timingAllowCheck (in nsIPrincipal origin); */
    #[inline]
    pub unsafe fn timingAllowCheck(&self, origin: Option<&nsIPrincipal>) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).timingAllowCheck)(self as *const _, origin.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* [noscript] readonly attribute TimeStamp cacheReadStart; */


    /* [noscript] readonly attribute TimeStamp cacheReadEnd; */


    /* readonly attribute PRTime channelCreationTime; */
    #[inline]
    pub unsafe fn get_channelCreationTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_channelCreationTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime asyncOpenTime; */
    #[inline]
    pub unsafe fn get_asyncOpenTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_asyncOpenTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime launchServiceWorkerStartTime; */
    #[inline]
    pub unsafe fn get_launchServiceWorkerStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_launchServiceWorkerStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime launchServiceWorkerEndTime; */
    #[inline]
    pub unsafe fn get_launchServiceWorkerEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_launchServiceWorkerEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime dispatchFetchEventStartTime; */
    #[inline]
    pub unsafe fn get_dispatchFetchEventStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_dispatchFetchEventStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime dispatchFetchEventEndTime; */
    #[inline]
    pub unsafe fn get_dispatchFetchEventEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_dispatchFetchEventEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime handleFetchEventStartTime; */
    #[inline]
    pub unsafe fn get_handleFetchEventStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_handleFetchEventStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime handleFetchEventEndTime; */
    #[inline]
    pub unsafe fn get_handleFetchEventEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_handleFetchEventEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime domainLookupStartTime; */
    #[inline]
    pub unsafe fn get_domainLookupStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_domainLookupStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime domainLookupEndTime; */
    #[inline]
    pub unsafe fn get_domainLookupEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_domainLookupEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime connectStartTime; */
    #[inline]
    pub unsafe fn get_connectStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_connectStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime connectEndTime; */
    #[inline]
    pub unsafe fn get_connectEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_connectEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime requestStartTime; */
    #[inline]
    pub unsafe fn get_requestStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_requestStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime responseStartTime; */
    #[inline]
    pub unsafe fn get_responseStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_responseStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime responseEndTime; */
    #[inline]
    pub unsafe fn get_responseEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_responseEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime cacheReadStartTime; */
    #[inline]
    pub unsafe fn get_cacheReadStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheReadStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime cacheReadEndTime; */
    #[inline]
    pub unsafe fn get_cacheReadEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_cacheReadEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime redirectStartTime; */
    #[inline]
    pub unsafe fn get_redirectStartTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_redirectStartTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* readonly attribute PRTime redirectEndTime; */
    #[inline]
    pub unsafe fn get_redirectEndTime(&self, ) -> Result<PRTime, nsresult> {
        let mut _retval: PRTime = ::std::mem::zeroed();
        match ((*self.vtable).get_redirectEndTime)(self as *const _, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


