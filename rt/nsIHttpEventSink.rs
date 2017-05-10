//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIHttpEventSink.idl
//


#[repr(C)]
pub struct nsIHttpEventSink {
    vtable: *const nsIHttpEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIHttpEventSink {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x9475a6af, 0x6352, 0x4251,
            [0x90, 0xf9, 0xd6, 0x5b, 0x1c, 0xd2, 0xea, 0x15])
    }
}

unsafe impl RefCounted for nsIHttpEventSink {
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
pub trait nsIHttpEventSinkCoerce {
    fn coerce_from(v: &nsIHttpEventSink) -> &Self;
}

impl nsIHttpEventSinkCoerce for nsIHttpEventSink {
    #[inline]
    fn coerce_from(v: &nsIHttpEventSink) -> &Self {
        v
    }
}

impl nsIHttpEventSink {
    #[inline]
    pub fn coerce<T: nsIHttpEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIHttpEventSink {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIHttpEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpEventSink) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIHttpEventSinkVTable {
    pub __base: nsISupportsVTable,

    /* [must_use] void onRedirect (in nsIHttpChannel httpChannel, in nsIChannel newChannel); */
    pub onRedirect: unsafe extern "C" fn (this: *const nsIHttpEventSink, httpChannel: *const nsIHttpChannel, newChannel: *const nsIChannel) -> nsresult,

}


impl nsIHttpEventSink {
    /* [must_use] void onRedirect (in nsIHttpChannel httpChannel, in nsIChannel newChannel); */
    #[inline]
    pub unsafe fn onRedirect(&self, httpChannel: Option<&nsIHttpChannel>, newChannel: Option<&nsIChannel>) -> Result<(), nsresult> {

        match ((*self.vtable).onRedirect)(self as *const _, httpChannel.map_or(::std::ptr::null(), |x| x as *const _), newChannel.map_or(::std::ptr::null(), |x| x as *const _)) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


