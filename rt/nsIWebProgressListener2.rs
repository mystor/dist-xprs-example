//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIWebProgressListener2.idl
//


#[repr(C)]
pub struct nsIWebProgressListener2 {
    vtable: *const nsIWebProgressListener2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIWebProgressListener2 {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xdde39de0, 0xe4e0, 0x11da,
            [0x8a, 0xd9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66])
    }
}

unsafe impl RefCounted for nsIWebProgressListener2 {
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
pub trait nsIWebProgressListener2Coerce {
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self;
}

impl nsIWebProgressListener2Coerce for nsIWebProgressListener2 {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self {
        v
    }
}

impl nsIWebProgressListener2 {
    #[inline]
    pub fn coerce<T: nsIWebProgressListener2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIWebProgressListener2 {
    type Target = nsIWebProgressListener;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIWebProgressListenerCoerce> nsIWebProgressListener2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebProgressListener2) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIWebProgressListener2VTable {
    pub __base: nsIWebProgressListenerVTable,

    /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
    pub onProgressChange64: unsafe extern "C" fn (this: *const nsIWebProgressListener2, aWebProgress: *const nsIWebProgress, aRequest: *const nsIRequest, aCurSelfProgress: libc::int64_t, aMaxSelfProgress: libc::int64_t, aCurTotalProgress: libc::int64_t, aMaxTotalProgress: libc::int64_t) -> nsresult,

    /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
    pub onRefreshAttempted: unsafe extern "C" fn (this: *const nsIWebProgressListener2, aWebProgress: *const nsIWebProgress, aRefreshURI: *const nsIURI, aMillis: libc::int32_t, aSameURI: bool, _retval: *mut bool) -> nsresult,

}


impl nsIWebProgressListener2 {
    /* void onProgressChange64 (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long long aCurSelfProgress, in long long aMaxSelfProgress, in long long aCurTotalProgress, in long long aMaxTotalProgress); */
    #[inline]
    pub unsafe fn onProgressChange64(&self, aWebProgress: Option<&nsIWebProgress>, aRequest: Option<&nsIRequest>, aCurSelfProgress: libc::int64_t, aMaxSelfProgress: libc::int64_t, aCurTotalProgress: libc::int64_t, aMaxTotalProgress: libc::int64_t) -> Result<(), nsresult> {

        match ((*self.vtable).onProgressChange64)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRequest.map_or(::std::ptr::null(), |x| x as *const _), aCurSelfProgress, aMaxSelfProgress, aCurTotalProgress, aMaxTotalProgress) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* boolean onRefreshAttempted (in nsIWebProgress aWebProgress, in nsIURI aRefreshURI, in long aMillis, in boolean aSameURI); */
    #[inline]
    pub unsafe fn onRefreshAttempted(&self, aWebProgress: Option<&nsIWebProgress>, aRefreshURI: Option<&nsIURI>, aMillis: libc::int32_t, aSameURI: bool) -> Result<bool, nsresult> {
        let mut _retval: bool = ::std::mem::zeroed();
        match ((*self.vtable).onRefreshAttempted)(self as *const _, aWebProgress.map_or(::std::ptr::null(), |x| x as *const _), aRefreshURI.map_or(::std::ptr::null(), |x| x as *const _), aMillis, aSameURI, &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


