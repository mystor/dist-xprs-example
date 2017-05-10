//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsITraceableChannel.idl
//


#[repr(C)]
pub struct nsITraceableChannel {
    vtable: *const nsITraceableChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsITraceableChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x68167b0b, 0xef34, 0x4d79,
            [0xa0, 0x9a, 0x80, 0x45, 0xf7, 0xc5, 0x14, 0x0e])
    }
}

unsafe impl RefCounted for nsITraceableChannel {
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
pub trait nsITraceableChannelCoerce {
    fn coerce_from(v: &nsITraceableChannel) -> &Self;
}

impl nsITraceableChannelCoerce for nsITraceableChannel {
    #[inline]
    fn coerce_from(v: &nsITraceableChannel) -> &Self {
        v
    }
}

impl nsITraceableChannel {
    #[inline]
    pub fn coerce<T: nsITraceableChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsITraceableChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsITraceableChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITraceableChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsITraceableChannelVTable {
    pub __base: nsISupportsVTable,

    /* nsIStreamListener setNewListener (in nsIStreamListener aListener); */
    pub setNewListener: unsafe extern "C" fn (this: *const nsITraceableChannel, aListener: *const nsIStreamListener, _retval: *mut *const nsIStreamListener) -> nsresult,

}


impl nsITraceableChannel {
    /* nsIStreamListener setNewListener (in nsIStreamListener aListener); */
    #[inline]
    pub unsafe fn setNewListener(&self, aListener: Option<&nsIStreamListener>) -> Result<Option<RefPtr<nsIStreamListener>>, nsresult> {
        let mut _retval = GetterAddrefs::new();
        match ((*self.vtable).setNewListener)(self as *const _, aListener.map_or(::std::ptr::null(), |x| x as *const _), _retval.ptr()) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(_retval.refptr())
    }

}


