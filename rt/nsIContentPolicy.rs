//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIContentPolicy.idl
//


#[repr(C)]
pub struct nsIContentPolicy {
    vtable: *const nsIContentPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIContentPolicy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xcaad4f1f, 0xd047, 0x46ac,
            [0xae, 0x9d, 0xdc, 0x59, 0x8e, 0x4f, 0xb9, 0x1b])
    }
}

unsafe impl RefCounted for nsIContentPolicy {
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
pub trait nsIContentPolicyCoerce {
    fn coerce_from(v: &nsIContentPolicy) -> &Self;
}

impl nsIContentPolicyCoerce for nsIContentPolicy {
    #[inline]
    fn coerce_from(v: &nsIContentPolicy) -> &Self {
        v
    }
}

impl nsIContentPolicy {
    #[inline]
    pub fn coerce<T: nsIContentPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIContentPolicy {
    type Target = nsIContentPolicyBase;
    #[inline]
    fn deref(&self) -> &nsIContentPolicyBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIContentPolicyBaseCoerce> nsIContentPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPolicy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIContentPolicyVTable {
    pub __base: nsIContentPolicyBaseVTable,

    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeTypeGuess, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
    pub shouldLoad: unsafe extern "C" fn (this: *const nsIContentPolicy, aContentType: nsContentPolicyType, aContentLocation: *const nsIURI, aRequestOrigin: *const nsIURI, aContext: *const nsISupports, aMimeTypeGuess: *const nsACString, aExtra: *const nsISupports, aRequestPrincipal: *const nsIPrincipal, _retval: *mut libc::int16_t) -> nsresult,

    /* short shouldProcess (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeType, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
    pub shouldProcess: unsafe extern "C" fn (this: *const nsIContentPolicy, aContentType: nsContentPolicyType, aContentLocation: *const nsIURI, aRequestOrigin: *const nsIURI, aContext: *const nsISupports, aMimeType: *const nsACString, aExtra: *const nsISupports, aRequestPrincipal: *const nsIPrincipal, _retval: *mut libc::int16_t) -> nsresult,

}


impl nsIContentPolicy {
    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeTypeGuess, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
    #[inline]
    pub unsafe fn shouldLoad(&self, aContentType: nsContentPolicyType, aContentLocation: Option<&nsIURI>, aRequestOrigin: Option<&nsIURI>, aContext: Option<&nsISupports>, aMimeTypeGuess: &[u8], aExtra: Option<&nsISupports>, aRequestPrincipal: Option<&nsIPrincipal>) -> Result<libc::int16_t, nsresult> {
        let aMimeTypeGuess = nsCString::from(aMimeTypeGuess);
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).shouldLoad)(self as *const _, aContentType, aContentLocation.map_or(::std::ptr::null(), |x| x as *const _), aRequestOrigin.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeTypeGuess, aExtra.map_or(::std::ptr::null(), |x| x as *const _), aRequestPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* short shouldProcess (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsISupports aContext, in ACString aMimeType, in nsISupports aExtra, [optional] in nsIPrincipal aRequestPrincipal); */
    #[inline]
    pub unsafe fn shouldProcess(&self, aContentType: nsContentPolicyType, aContentLocation: Option<&nsIURI>, aRequestOrigin: Option<&nsIURI>, aContext: Option<&nsISupports>, aMimeType: &[u8], aExtra: Option<&nsISupports>, aRequestPrincipal: Option<&nsIPrincipal>) -> Result<libc::int16_t, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).shouldProcess)(self as *const _, aContentType, aContentLocation.map_or(::std::ptr::null(), |x| x as *const _), aRequestOrigin.map_or(::std::ptr::null(), |x| x as *const _), aContext.map_or(::std::ptr::null(), |x| x as *const _), &*aMimeType, aExtra.map_or(::std::ptr::null(), |x| x as *const _), aRequestPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


