//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIParentRedirectingChannel.idl
//


#[repr(C)]
pub struct nsIParentRedirectingChannel {
    vtable: *const nsIParentRedirectingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIParentRedirectingChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x3ed1d288, 0x5324, 0x46ee,
            [0x8a, 0x98, 0x33, 0xac, 0x37, 0xd1, 0x08, 0x0b])
    }
}

unsafe impl RefCounted for nsIParentRedirectingChannel {
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
pub trait nsIParentRedirectingChannelCoerce {
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self;
}

impl nsIParentRedirectingChannelCoerce for nsIParentRedirectingChannel {
    #[inline]
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self {
        v
    }
}

impl nsIParentRedirectingChannel {
    #[inline]
    pub fn coerce<T: nsIParentRedirectingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIParentRedirectingChannel {
    type Target = nsIParentChannel;
    #[inline]
    fn deref(&self) -> &nsIParentChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsIParentChannelCoerce> nsIParentRedirectingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentRedirectingChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIParentRedirectingChannelVTable {
    pub __base: nsIParentChannelVTable,

    /* void startRedirect (in uint32_t newChannelId, in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
    pub startRedirect: unsafe extern "C" fn (this: *const nsIParentRedirectingChannel, newChannelId: uint32_t, newChannel: *const nsIChannel, redirectFlags: uint32_t, callback: *const nsIAsyncVerifyRedirectCallback) -> nsresult,

    /* void completeRedirect (in boolean succeeded); */
    pub completeRedirect: unsafe extern "C" fn (this: *const nsIParentRedirectingChannel, succeeded: bool) -> nsresult,

}


impl nsIParentRedirectingChannel {
    /* void startRedirect (in uint32_t newChannelId, in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
    #[inline]
    pub unsafe fn startRedirect(&self, newChannelId: uint32_t, newChannel: Option<&nsIChannel>, redirectFlags: uint32_t, callback: Option<&nsIAsyncVerifyRedirectCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).startRedirect)(self as *const _, newChannelId, newChannel.map_or(::std::ptr::null(), |x| x as *const _), redirectFlags, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

    /* void completeRedirect (in boolean succeeded); */
    #[inline]
    pub unsafe fn completeRedirect(&self, succeeded: bool) -> Result<(), nsresult> {

        match ((*self.vtable).completeRedirect)(self as *const _, succeeded) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


