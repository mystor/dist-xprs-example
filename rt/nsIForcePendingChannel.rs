//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsIForcePendingChannel.idl
//


#[repr(C)]
pub struct nsIForcePendingChannel {
    vtable: *const nsIForcePendingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsIForcePendingChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x2ac3e1ca, 0x049f, 0x44c3,
            [0xa5, 0x19, 0xf0, 0x68, 0x1f, 0x51, 0xe9, 0xb1])
    }
}

unsafe impl RefCounted for nsIForcePendingChannel {
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
pub trait nsIForcePendingChannelCoerce {
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self;
}

impl nsIForcePendingChannelCoerce for nsIForcePendingChannel {
    #[inline]
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self {
        v
    }
}

impl nsIForcePendingChannel {
    #[inline]
    pub fn coerce<T: nsIForcePendingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsIForcePendingChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsIForcePendingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsIForcePendingChannelVTable {
    pub __base: nsISupportsVTable,

    /* void forcePending (in boolean aForcePending); */
    pub forcePending: unsafe extern "C" fn (this: *const nsIForcePendingChannel, aForcePending: bool) -> nsresult,

}


impl nsIForcePendingChannel {
    /* void forcePending (in boolean aForcePending); */
    #[inline]
    pub unsafe fn forcePending(&self, aForcePending: bool) -> Result<(), nsresult> {

        match ((*self.vtable).forcePending)(self as *const _, aForcePending) {
            NS_OK => {},
            e => return Err(e),
        }
        Ok(())
    }

}


