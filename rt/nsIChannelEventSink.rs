//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIChannelEventSink.idl
//


pub mod nsIChannelEventSink_consts {
    pub const REDIRECT_TEMPORARY: i64 = 1;
    pub const REDIRECT_PERMANENT: i64 = 2;
    pub const REDIRECT_INTERNAL: i64 = 4;
    pub const REDIRECT_STS_UPGRADE: i64 = 8;
}


#[repr(C)]
pub struct nsIChannelEventSink {
    vtable: *const nsIChannelEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIChannelEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x0197720d, 0x37ed, 0x4e75,
            [0x89, 0x56, 0xd0, 0xd2, 0x96, 0xe4, 0xd8, 0xa6])
    }
}

unsafe impl RefCounted for nsIChannelEventSink {
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
pub trait nsIChannelEventSinkCoerce {
    fn coerce_from(v: &nsIChannelEventSink) -> &Self;
}

impl nsIChannelEventSinkCoerce for nsIChannelEventSink {
    #[inline]
    fn coerce_from(v: &nsIChannelEventSink) -> &Self {
        v
    }
}

impl nsIChannelEventSink {
    #[inline]
    pub fn coerce<T: nsIChannelEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIChannelEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIChannelEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChannelEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIChannelEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
    pub asyncOnChannelRedirect: unsafe extern "C" fn (this: *const nsIChannelEventSink, oldChannel: *const nsIChannel, newChannel: *const nsIChannel, flags: libc::uint32_t, callback: *const nsIAsyncVerifyRedirectCallback) -> nsresult,

}


impl nsIChannelEventSink {
    /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
    #[inline]
    pub unsafe fn asyncOnChannelRedirect(&self, oldChannel: Option<&nsIChannel>, newChannel: Option<&nsIChannel>, flags: libc::uint32_t, callback: Option<&nsIAsyncVerifyRedirectCallback>) -> Result<(), nsresult> {

        match ((*self.vtable).asyncOnChannelRedirect)(self as *const _, oldChannel.map_or(::std::ptr::null(), |x| x as *const _), newChannel.map_or(::std::ptr::null(), |x| x as *const _), flags, callback.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


