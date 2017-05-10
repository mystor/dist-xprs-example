//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsISimpleContentPolicy.idl
//


#[repr(C)]
pub struct nsISimpleContentPolicy {
    vtable: *const nsISimpleContentPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsISimpleContentPolicy {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xb9df71e3, 0xa9b3, 0x4706,
            [0xb2, 0xd5, 0xe6, 0xc0, 0xd3, 0xd6, 0x8e, 0xc7])
    }
}

unsafe impl RefCounted for nsISimpleContentPolicy {
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
pub trait nsISimpleContentPolicyCoerce {
    fn coerce_from(v: &nsISimpleContentPolicy) -> &Self;
}

impl nsISimpleContentPolicyCoerce for nsISimpleContentPolicy {
    #[inline]
    fn coerce_from(v: &nsISimpleContentPolicy) -> &Self {
        v
    }
}

impl nsISimpleContentPolicy {
    #[inline]
    pub fn coerce<T: nsISimpleContentPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsISimpleContentPolicy {
    type Target = nsIContentPolicyBase;
    #[inline]
    fn deref(&self) -> &nsIContentPolicyBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIContentPolicyBaseCoerce> nsISimpleContentPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleContentPolicy) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsISimpleContentPolicyVTable {
    pub __base: nsIContentPolicyBaseVTable,

    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsIDOMElement aTopFrameElement, in boolean aIsTopLevel, in ACString aMimeTypeGuess, in nsISupports aExtra, in nsIPrincipal aRequestPrincipal); */
    pub shouldLoad: unsafe extern "C" fn (this: *const nsISimpleContentPolicy, aContentType: nsContentPolicyType, aContentLocation: *const nsIURI, aRequestOrigin: *const nsIURI, aTopFrameElement: *const nsIDOMElement, aIsTopLevel: bool, aMimeTypeGuess: *const nsACString, aExtra: *const nsISupports, aRequestPrincipal: *const nsIPrincipal, _retval: *mut libc::int16_t) -> nsresult,

    /* short shouldProcess (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsIDOMElement aTopFrameElement, in boolean aIsTopLevel, in ACString aMimeType, in nsISupports aExtra, in nsIPrincipal aRequestPrincipal); */
    pub shouldProcess: unsafe extern "C" fn (this: *const nsISimpleContentPolicy, aContentType: nsContentPolicyType, aContentLocation: *const nsIURI, aRequestOrigin: *const nsIURI, aTopFrameElement: *const nsIDOMElement, aIsTopLevel: bool, aMimeType: *const nsACString, aExtra: *const nsISupports, aRequestPrincipal: *const nsIPrincipal, _retval: *mut libc::int16_t) -> nsresult,

}


impl nsISimpleContentPolicy {
    /* short shouldLoad (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsIDOMElement aTopFrameElement, in boolean aIsTopLevel, in ACString aMimeTypeGuess, in nsISupports aExtra, in nsIPrincipal aRequestPrincipal); */
    #[inline]
    pub unsafe fn shouldLoad(&self, aContentType: nsContentPolicyType, aContentLocation: Option<&nsIURI>, aRequestOrigin: Option<&nsIURI>, aTopFrameElement: Option<&nsIDOMElement>, aIsTopLevel: bool, aMimeTypeGuess: &[u8], aExtra: Option<&nsISupports>, aRequestPrincipal: Option<&nsIPrincipal>) -> Result<libc::int16_t, nsresult> {
        let aMimeTypeGuess = nsCString::from(aMimeTypeGuess);
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).shouldLoad)(self as *const _, aContentType, aContentLocation.map_or(::std::ptr::null(), |x| x as *const _), aRequestOrigin.map_or(::std::ptr::null(), |x| x as *const _), aTopFrameElement.map_or(::std::ptr::null(), |x| x as *const _), aIsTopLevel, &*aMimeTypeGuess, aExtra.map_or(::std::ptr::null(), |x| x as *const _), aRequestPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

    /* short shouldProcess (in nsContentPolicyType aContentType, in nsIURI aContentLocation, in nsIURI aRequestOrigin, in nsIDOMElement aTopFrameElement, in boolean aIsTopLevel, in ACString aMimeType, in nsISupports aExtra, in nsIPrincipal aRequestPrincipal); */
    #[inline]
    pub unsafe fn shouldProcess(&self, aContentType: nsContentPolicyType, aContentLocation: Option<&nsIURI>, aRequestOrigin: Option<&nsIURI>, aTopFrameElement: Option<&nsIDOMElement>, aIsTopLevel: bool, aMimeType: &[u8], aExtra: Option<&nsISupports>, aRequestPrincipal: Option<&nsIPrincipal>) -> Result<libc::int16_t, nsresult> {
        let aMimeType = nsCString::from(aMimeType);
        let mut _retval: libc::int16_t = ::std::mem::zeroed();
        match ((*self.vtable).shouldProcess)(self as *const _, aContentType, aContentLocation.map_or(::std::ptr::null(), |x| x as *const _), aRequestOrigin.map_or(::std::ptr::null(), |x| x as *const _), aTopFrameElement.map_or(::std::ptr::null(), |x| x as *const _), aIsTopLevel, &*aMimeType, aExtra.map_or(::std::ptr::null(), |x| x as *const _), aRequestPrincipal.map_or(::std::ptr::null(), |x| x as *const _), &mut _retval as *mut _) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval)
    }

}


