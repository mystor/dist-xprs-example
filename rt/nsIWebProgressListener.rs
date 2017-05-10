//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgressListener.idl
//


pub mod nsIWebProgressListener_consts {
    pub const STATE_START: i64 = 1;
    pub const STATE_REDIRECTING: i64 = 2;
    pub const STATE_TRANSFERRING: i64 = 4;
    pub const STATE_NEGOTIATING: i64 = 8;
    pub const STATE_STOP: i64 = 16;
    pub const STATE_IS_REQUEST: i64 = 65536;
    pub const STATE_IS_DOCUMENT: i64 = 131072;
    pub const STATE_IS_NETWORK: i64 = 262144;
    pub const STATE_IS_WINDOW: i64 = 524288;
    pub const STATE_IS_REDIRECTED_DOCUMENT: i64 = 1048576;
    pub const STATE_RESTORING: i64 = 16777216;
    pub const STATE_IS_INSECURE: i64 = 4;
    pub const STATE_IS_BROKEN: i64 = 1;
    pub const STATE_IS_SECURE: i64 = 2;
    pub const STATE_BLOCKED_MIXED_ACTIVE_CONTENT: i64 = 16;
    pub const STATE_LOADED_MIXED_ACTIVE_CONTENT: i64 = 32;
    pub const STATE_BLOCKED_MIXED_DISPLAY_CONTENT: i64 = 256;
    pub const STATE_LOADED_MIXED_DISPLAY_CONTENT: i64 = 512;
    pub const STATE_BLOCKED_TRACKING_CONTENT: i64 = 4096;
    pub const STATE_LOADED_TRACKING_CONTENT: i64 = 8192;
    pub const STATE_BLOCKED_UNSAFE_CONTENT: i64 = 16384;
    pub const STATE_SECURE_HIGH: i64 = 262144;
    pub const STATE_SECURE_MED: i64 = 65536;
    pub const STATE_SECURE_LOW: i64 = 131072;
    pub const STATE_IDENTITY_EV_TOPLEVEL: i64 = 1048576;
    pub const STATE_USES_SSL_3: i64 = 16777216;
    pub const STATE_USES_WEAK_CRYPTO: i64 = 33554432;
    pub const STATE_CERT_USER_OVERRIDDEN: i64 = 67108864;
    pub const LOCATION_CHANGE_SAME_DOCUMENT: i64 = 1;
    pub const LOCATION_CHANGE_ERROR_PAGE: i64 = 2;
}


#[repr(C)]
pub struct nsIWebProgressListener {
    vtable: *const nsIWebProgressListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebProgressListener {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xa9df523b, 0xefe2, 0x421e,
            [0x9d, 0x8e, 0x3d, 0x7f, 0x80, 0x7d, 0xda, 0x4c])
    }
}

unsafe impl RefCounted for nsIWebProgressListener {
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
pub trait nsIWebProgressListenerCoerce {
    fn coerce_from(v: &nsIWebProgressListener) -> &Self;
}

impl nsIWebProgressListenerCoerce for nsIWebProgressListener {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener) -> &Self {
        v
    }
}

impl nsIWebProgressListener {
    #[inline]
    pub fn coerce<T: nsIWebProgressListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebProgressListener {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIWebProgressListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebProgressListenerVTable {
    pub __base: nsISupportsVTable,

    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus); */
    pub onStateChange: unsafe extern "C" fn (this: *const nsIWebProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aStateFlags: libc::uint32_t, aStatus: nsresult) -> nsresult,

    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long aCurSelfProgress, in long aMaxSelfProgress, in long aCurTotalProgress, in long aMaxTotalProgress); */
    pub onProgressChange: unsafe extern "C" fn (this: *const nsIWebProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aCurSelfProgress: libc::int32_t, aMaxSelfProgress: libc::int32_t, aCurTotalProgress: libc::int32_t, aMaxTotalProgress: libc::int32_t) -> nsresult,

    /* void onLocationChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsIURI aLocation, [optional] in unsigned long aFlags); */
    pub onLocationChange: unsafe extern "C" fn (this: *const nsIWebProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aLocation: *const nsIURI, aFlags: libc::uint32_t) -> nsresult,

    /* void onStatusChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsresult aStatus, in wstring aMessage); */
    pub onStatusChange: unsafe extern "C" fn (this: *const nsIWebProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aStatus: nsresult, aMessage: *const libc::int16_t) -> nsresult,

    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState); */
    pub onSecurityChange: unsafe extern "C" fn (this: *const nsIWebProgressListener, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aState: libc::uint32_t) -> nsresult,

}


impl nsIWebProgressListener {
    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus); */
    #[inline]
    pub unsafe fn onStateChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aStateFlags: libc::uint32_t, aStatus: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).onStateChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aStateFlags, aStatus) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long aCurSelfProgress, in long aMaxSelfProgress, in long aCurTotalProgress, in long aMaxTotalProgress); */
    #[inline]
    pub unsafe fn onProgressChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aCurSelfProgress: libc::int32_t, aMaxSelfProgress: libc::int32_t, aCurTotalProgress: libc::int32_t, aMaxTotalProgress: libc::int32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onProgressChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onLocationChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsIURI aLocation, [optional] in unsigned long aFlags); */
    #[inline]
    pub unsafe fn onLocationChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aLocation: Option<&nsIURI>, aFlags: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onLocationChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aLocation.map_or(::std::ptr::null(), |x| x as *const _), aFlags) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onStatusChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsresult aStatus, in wstring aMessage); */
    #[inline]
    pub unsafe fn onStatusChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aStatus: nsresult, aMessage: *const libc::int16_t) -> Result<(), nsresult> {

        match ((*self.vtable).onStatusChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aStatus, aMessage) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState); */
    #[inline]
    pub unsafe fn onSecurityChange(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aState: libc::uint32_t) -> Result<(), nsresult> {

        match ((*self.vtable).onSecurityChange)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aState) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


