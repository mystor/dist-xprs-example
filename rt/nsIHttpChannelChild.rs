//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpChannelChild.idl
//


#[repr(C)]
pub struct nsIHttpChannelChild {
    vtable: *const nsIHttpChannelChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpChannelChild {
    #[inline]
    fn iid() -> nsIID {
        nsID(0xd02b96ed, 0x2789, 0x4f42,
            [0xa2, 0x5c, 0x7a, 0xbe, 0x63, 0xde, 0x7c, 0x18])
    }
}

unsafe impl RefCounted for nsIHttpChannelChild {
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
pub trait nsIHttpChannelChildCoerce {
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self;
}

impl nsIHttpChannelChildCoerce for nsIHttpChannelChild {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self {
        v
    }
}

impl nsIHttpChannelChild {
    #[inline]
    pub fn coerce<T: nsIHttpChannelChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpChannelChild {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpChannelChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpChannelChild) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpChannelChildVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void addCookiesToRequest (); */
    pub addCookiesToRequest: unsafe extern "C" fn (this: *const nsIHttpChannelChild) -> nsresult,

    /* [must_use] void forceIntercepted (in boolean postRedirectChannelShouldIntercept, in boolean postRedirectChannelShouldUpgrade); */
    pub forceIntercepted: unsafe extern "C" fn (this: *const nsIHttpChannelChild, postRedirectChannelShouldIntercept: bool, postRedirectChannelShouldUpgrade: bool) -> nsresult,

    /* [must_use] readonly attribute RequestHeaderTuples clientSetRequestHeaders; */
    /// Unable to call function as its signature contains a non-rust type
    pub get_clientSetRequestHeaders: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void GetClientSetCorsPreflightParameters (in OptionalCorsPreflightArgsRef args); */
    /// Unable to call function as its signature contains a non-rust type
    pub GetClientSetCorsPreflightParameters: *const ::libc::c_void,

    /* [must_use] void removeCorsPreflightCacheEntry (in nsIURI aURI, in nsIPrincipal aRequestingPrincipal); */
    pub removeCorsPreflightCacheEntry: unsafe extern "C" fn (this: *const nsIHttpChannelChild, aURI: *const nsIURI, aRequestingPrincipal: *const nsIPrincipal) -> nsresult,

}


impl nsIHttpChannelChild {
    /* [must_use] void addCookiesToRequest (); */
    #[inline]
    pub unsafe fn addCookiesToRequest(&self, ) -> Result<(), nsresult> {

        match ((*self.vtable).addCookiesToRequest)(self as *const _, ) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] void forceIntercepted (in boolean postRedirectChannelShouldIntercept, in boolean postRedirectChannelShouldUpgrade); */
    #[inline]
    pub unsafe fn forceIntercepted(&self, postRedirectChannelShouldIntercept: bool, postRedirectChannelShouldUpgrade: bool) -> Result<(), nsresult> {

        match ((*self.vtable).forceIntercepted)(self as *const _, postRedirectChannelShouldIntercept, postRedirectChannelShouldUpgrade) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* [must_use] readonly attribute RequestHeaderTuples clientSetRequestHeaders; */


    /* [nostdcall,notxpcom] void GetClientSetCorsPreflightParameters (in OptionalCorsPreflightArgsRef args); */


    /* [must_use] void removeCorsPreflightCacheEntry (in nsIURI aURI, in nsIPrincipal aRequestingPrincipal); */
    #[inline]
    pub unsafe fn removeCorsPreflightCacheEntry(&self, aURI: Option<&nsIURI>, aRequestingPrincipal: Option<&nsIPrincipal>) -> Result<(), nsresult> {

        match ((*self.vtable).removeCorsPreflightCacheEntry)(self as *const _, aURI.map_or(::std::ptr::null(), |x| x as *const _), aRequestingPrincipal.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


