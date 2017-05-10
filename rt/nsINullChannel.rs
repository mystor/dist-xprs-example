//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM ../../../dist/idl/nsINullChannel.idl
//


#[repr(C)]
pub struct nsINullChannel {
    vtable: *const nsINullChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

unsafe impl XpCom for nsINullChannel {
    #[inline]
    fn iid() -> nsIID {
        nsID(0x4610b901, 0xdf41, 0x4bb4,
            [0xbd, 0x3f, 0xfd, 0x4d, 0x6b, 0x6d, 0x8d, 0x68])
    }
}

unsafe impl RefCounted for nsINullChannel {
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
pub trait nsINullChannelCoerce {
    fn coerce_from(v: &nsINullChannel) -> &Self;
}

impl nsINullChannelCoerce for nsINullChannel {
    #[inline]
    fn coerce_from(v: &nsINullChannel) -> &Self {
        v
    }
}

impl nsINullChannel {
    #[inline]
    pub fn coerce<T: nsINullChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

impl ::std::ops::Deref for nsINullChannel {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Enable coercing to base classes
impl<T: nsISupportsCoerce> nsINullChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINullChannel) -> &Self {
        T::coerce_from(v)
    }
}

#[repr(C)]
pub struct nsINullChannelVTable {
    pub __base: nsISupportsVTable,

}


impl nsINullChannel {
}


