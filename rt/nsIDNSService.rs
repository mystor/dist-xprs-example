//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIDNSService.idl
//


pub mod nsIDNSService_consts {
    pub const RESOLVE_BYPASS_CACHE: i64 = 1;
    pub const RESOLVE_CANONICAL_NAME: i64 = 2;
    pub const RESOLVE_PRIORITY_MEDIUM: i64 = 4;
    pub const RESOLVE_PRIORITY_LOW: i64 = 8;
    pub const RESOLVE_SPECULATE: i64 = 16;
    pub const RESOLVE_DISABLE_IPV6: i64 = 32;
    pub const RESOLVE_OFFLINE: i64 = 64;
    pub const RESOLVE_DISABLE_IPV4: i64 = 128;
    pub const RESOLVE_ALLOW_NAME_COLLISION: i64 = 256;
}


#[repr(C)]
pub struct nsIDNSService {
    vtable: *const nsIDNSServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIDNSService {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xde5642c6, 0x61fc, 0x4fcf,
            [0x9a, 0x47, 0x03, 0x22, 0x6b, 0x0d, 0x4e, 0x21])
    }
}

unsafe impl RefCounted for nsIDNSService {
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
pub trait nsIDNSServiceCoerce {
    fn coerce_from(v: &nsIDNSService) -> &Self;
}

impl nsIDNSServiceCoerce for nsIDNSService {
    #[inline]
    fn coerce_from(v: &nsIDNSService) -> &Self {
        v
    }
}

impl nsIDNSService {
    #[inline]
    pub fn coerce<T: nsIDNSServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIDNSService {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIDNSServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSService) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIDNSServiceVTable {
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,optional_argc] nsICancelable asyncResolve (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub asyncResolve: *const ::libc::c_void,

    /* [notxpcom] nsresult asyncResolveNative (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub asyncResolveNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] void cancelAsyncResolve (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cancelAsyncResolve: *const ::libc::c_void,

    /* [notxpcom] nsresult cancelAsyncResolveNative (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cancelAsyncResolveNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] nsIDNSRecord resolve (in AUTF8String aHostName, in unsigned long aFlags, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub resolve: *const ::libc::c_void,

    /* [notxpcom] nsresult resolveNative (in AUTF8String aHostName, in unsigned long aFlags, in OriginAttributes aOriginAttributes, out nsIDNSRecord aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub resolveNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] nsICancelable asyncResolveExtended (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub asyncResolveExtended: *const ::libc::c_void,

    /* [notxpcom] nsresult asyncResolveExtendedNative (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */
    /// Unable to call function as its signature contains a non-rust type
    pub asyncResolveExtendedNative: *const ::libc::c_void,

    /* [implicit_jscontext,optional_argc] void cancelAsyncResolveExtended (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cancelAsyncResolveExtended: *const ::libc::c_void,

    /* [notxpcom] nsresult cancelAsyncResolveExtendedNative (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */
    /// Unable to call function as its signature contains a non-rust type
    pub cancelAsyncResolveExtendedNative: *const ::libc::c_void,

    /* [noscript] void getDNSCacheEntries (in EntriesArray args); */
    /// Unable to call function as its signature contains a non-rust type
    pub getDNSCacheEntries: *const ::libc::c_void,

    /* readonly attribute AUTF8String myHostName; */
    pub get_myHostName: unsafe extern "C" fn (this: *const nsIDNSService, aMyHostName: *mut nsACString) -> nsresult,

}


impl nsIDNSService {
    /* [implicit_jscontext,optional_argc] nsICancelable asyncResolve (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult asyncResolveNative (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */


    /* [implicit_jscontext,optional_argc] void cancelAsyncResolve (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult cancelAsyncResolveNative (in AUTF8String aHostName, in unsigned long aFlags, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */


    /* [implicit_jscontext,optional_argc] nsIDNSRecord resolve (in AUTF8String aHostName, in unsigned long aFlags, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult resolveNative (in AUTF8String aHostName, in unsigned long aFlags, in OriginAttributes aOriginAttributes, out nsIDNSRecord aResult); */


    /* [implicit_jscontext,optional_argc] nsICancelable asyncResolveExtended (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult asyncResolveExtendedNative (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */


    /* [implicit_jscontext,optional_argc] void cancelAsyncResolveExtended (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */


    /* [notxpcom] nsresult cancelAsyncResolveExtendedNative (in AUTF8String aHostName, in unsigned long aFlags, in AUTF8String aNetworkInterface, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */


    /* [noscript] void getDNSCacheEntries (in EntriesArray args); */


    /* readonly attribute AUTF8String myHostName; */
    #[inline]
    pub unsafe fn get_myHostName(&self, ) -> Result<nsCString, nsresult> {
        let mut _retval = nsCString::new();
        match ((*self.vtable).get_myHostName)(self as *const _, &mut *_retval) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


