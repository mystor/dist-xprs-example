//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannelAuthProvider.idl
//


#[repr(C)]
pub struct nsIHttpChannelAuthProvider {
    vtable: *const nsIHttpChannelAuthProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpChannelAuthProvider {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x788f331b, 0x2e1f, 0x436c,
            [0xb4, 0x05, 0x4f, 0x88, 0xa3, 0x1a, 0x10, 0x5b])
    }
}

unsafe impl RefCounted for nsIHttpChannelAuthProvider {
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
pub trait nsIHttpChannelAuthProviderCoerce {
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self;
}

impl nsIHttpChannelAuthProviderCoerce for nsIHttpChannelAuthProvider {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self {
        v
    }
}

impl nsIHttpChannelAuthProvider {
    #[inline]
    pub fn coerce<T: nsIHttpChannelAuthProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpChannelAuthProvider {
    type Target = nsICancelable;
    #[inline]
    fn deref(&self) -> &nsICancelable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsICancelableCoerce> nsIHttpChannelAuthProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelAuthProvider) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpChannelAuthProviderVTable {
    pub __base: nsICancelableVTable,

    /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
    pub init: unsafe extern "C" fn (this: *const nsIHttpChannelAuthProvider, channel: *const nsIHttpAuthenticableChannel) -> nsresult,

    /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
    pub processAuthentication: unsafe extern "C" fn (this: *const nsIHttpChannelAuthProvider, httpStatus: libc::uint32_t, sslConnectFailed: bool) -> nsresult,

    /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
    pub addAuthorizationHeaders: unsafe extern "C" fn (this: *const nsIHttpChannelAuthProvider, dontUseCachedWWWCreds: bool) -> nsresult,

    /* [must_use] void checkForSuperfluousAuth (); */
    pub checkForSuperfluousAuth: unsafe extern "C" fn (this: *const nsIHttpChannelAuthProvider) -> nsresult,

    /* [must_use] void disconnect (in nsresult status); */
    pub disconnect: unsafe extern "C" fn (this: *const nsIHttpChannelAuthProvider, status: nsresult) -> nsresult,

}


impl nsIHttpChannelAuthProvider {
    /* [must_use] void init (in nsIHttpAuthenticableChannel channel); */
    #[inline]
    pub unsafe fn init(&self, channel: Option<&nsIHttpAuthenticableChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).init)(self as *const _, channel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void processAuthentication (in unsigned long httpStatus, in boolean sslConnectFailed); */
    #[inline]
    pub unsafe fn processAuthentication(&self, httpStatus: libc::uint32_t, sslConnectFailed: bool) -> Result<(), nsresult> {

        match ((*self.vtable).processAuthentication)(self as *const _, httpStatus, sslConnectFailed) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void addAuthorizationHeaders (in boolean dontUseCachedWWWCreds); */
    #[inline]
    pub unsafe fn addAuthorizationHeaders(&self, dontUseCachedWWWCreds: bool) -> Result<(), nsresult> {

        match ((*self.vtable).addAuthorizationHeaders)(self as *const _, dontUseCachedWWWCreds) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void checkForSuperfluousAuth (); */
    #[inline]
    pub unsafe fn checkForSuperfluousAuth(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).checkForSuperfluousAuth)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void disconnect (in nsresult status); */
    #[inline]
    pub unsafe fn disconnect(&self, status: nsresult) -> Result<(), nsresult> {

        match ((*self.vtable).disconnect)(self as *const _, status) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


